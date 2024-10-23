use std::collections::HashMap;

use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::{IterableMap, IterableSet, LookupMap};
use near_sdk::{
    env, near, require, AccountId, BorshStorageKey, CryptoHash, NearToken, PanicOnDefault,
    PublicKey,
};

mod cleanup;
mod drops;
mod events;
mod ext_database;
mod factory;
mod fungible_tokens;
mod internals;
mod leaderboard;
mod models;
mod multichain;
mod non_fungible_tokens;
mod tickets;

use drops::*;
use events::*;
use fungible_tokens::*;
use internals::*;
use leaderboard::*;
use models::*;
use multichain::*;
use non_fungible_tokens::*;

// ------------------------ Access Key Method Names ------------------------ //
pub const ATTENDEE_KEY_METHOD_NAMES: &str = "scan_ticket,create_account,claim_drop,ft_transfer";
pub const SPONSOR_KEY_METHOD_NAMES: &str =
    "create_token_drop,create_nft_drop,delete_drop,ft_transfer,create_multichain_drop";
pub const ADMIN_KEY_METHOD_NAMES: &str = "";
pub const DATA_SETTER_KEY_METHOD_NAMES: &str = "set_alerts,set_agenda";
pub const TICKET_ADDER_KEY_METHOD_NAMES: &str = "add_tickets";

pub const DROP_DELIMITER: &str = "||";

#[near(contract_state, serializers = [borsh])]
#[derive(PanicOnDefault)]
pub struct Contract {
    // ------------------------ Contract Global ---------------------------- //
    pub account_details_by_id: IterableMap<AccountId, AccountDetails>, // clearable
    pub is_contract_frozen: bool,
    pub contract_key: PublicKey,

    // ------------------------ Fungible Tokens ---------------------------- //
    pub ft_total_supply: NearToken,
    pub ft_metadata: FungibleTokenMetadata,

    // ------------------------ Non Fungible Tokens ------------------------ //
    pub nft_tokens_by_id: IterableMap<TokenId, Token>,
    pub nft_tokens_per_owner: LookupMap<AccountId, IterableSet<TokenId>>,
    pub series_by_id: IterableMap<SeriesId, Series>,
    pub nft_metadata: NFTContractMetadata,

    // ------------------------ Drops -------------------------------------- //
    pub drop_by_id: IterableMap<DropId, DropData>, // clearable

    // ------------------------ Account Factory ---------------------------- //
    pub ticket_data_by_id: IterableMap<DropId, TicketType>, // clearable

    // ------------------------ Leaderboard ------------------------------------ //
    pub token_leaderboard: Vec<AccountId>,         // clearable
    pub poap_leaderboard: Vec<AccountId>,          // clearable
    pub recent_transactions: Vec<TransactionType>, // clearable
    pub total_transactions: u64,
    pub total_tokens_transferred: NearToken,

    // ------------------------ Tickets ------------------------------------ //
    pub attendee_ticket_by_pk: IterableMap<PublicKey, AttendeeTicketInformation>, // clearable

    // ------------------------ External Databases ------------------------- //
    pub agenda: String,        // clearable
    pub alerts: String,        // clearable
    pub alerts_timestamp: u64, // clearable
    pub agenda_timestamp: u64, // clearable
}

#[near]
impl Contract {
    /// Initializes a new contract instance.
    ///
    /// # Arguments
    ///
    /// * `keypom_contract` - The Keypom contract account ID.
    /// * `ticket_data` - A hashmap containing drop IDs and their associated ticket data.
    /// * `token_name` - An optional name for the fungible token.
    /// * `symbol` - An optional symbol for the fungible token.
    /// * `icon` - An optional icon for the fungible token.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the contract.
    #[init]
    pub fn new(
        ticket_data: HashMap<DropId, TicketType>,
        token_name: Option<String>,
        symbol: Option<String>,
        icon: Option<String>,
        admin: Vec<AccountId>,
        contract_key: PublicKey,
    ) -> Self {
        let mut ticket_data_by_id: IterableMap<String, TicketType> =
            IterableMap::new(StorageKeys::TicketDataById);

        for (drop_id, ticket_type) in ticket_data.into_iter() {
            ticket_data_by_id.insert(drop_id, ticket_type);
        }

        let mut account_details_by_id: IterableMap<AccountId, AccountDetails> =
            IterableMap::new(StorageKeys::AccountDetailsById);
        for account in admin {
            let mut account_details = AccountDetails::new(&account);
            account_details.account_status = Some(AccountStatus::Admin);
            account_details_by_id.insert(account, account_details);
        }

        Self {
            agenda: "[{}]".to_string(),
            alerts: "[{}]".to_string(),
            token_leaderboard: Vec::new(),
            poap_leaderboard: Vec::new(),
            agenda_timestamp: 0,
            alerts_timestamp: 0,
            nft_tokens_per_owner: LookupMap::new(StorageKeys::TokensForOwner),
            contract_key,
            is_contract_frozen: false,
            account_details_by_id,
            ft_total_supply: NearToken::from_yoctonear(0),
            recent_transactions: Vec::new(),
            total_transactions: 0,
            total_tokens_transferred: NearToken::from_yoctonear(0),
            ft_metadata: FungibleTokenMetadata {
                spec: "ft-1.0.0".to_string(),
                name: token_name.unwrap_or("Redacted Fungible Token".to_string()),
                symbol: symbol.unwrap_or("SOV3".to_string()),
                icon: icon.or(Some(DATA_IMAGE_SVG_GT_ICON.to_string())),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },

            series_by_id: IterableMap::new(StorageKeys::SeriesById),
            nft_tokens_by_id: IterableMap::new(StorageKeys::TokensById),
            nft_metadata: NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Redacted NFT Contract".to_string(),
                symbol: "SOV3".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },

            drop_by_id: IterableMap::new(StorageKeys::DropById),

            ticket_data_by_id,
            attendee_ticket_by_pk: IterableMap::new(StorageKeys::AttendeeTicketInformation),
        }
    }

    /// Queries for the account details associated with the given public key or account ID.
    ///
    /// # Arguments
    ///
    /// * `key_or_account_id` - Either the public key or the account ID to query for the associated account details.
    ///
    /// # Returns
    ///
    /// Returns `ExtAccountDetails` associated with the given public key or account ID.
    ///
    /// # Panics
    ///
    /// Panics if no account is found for the given public key or account ID.
    pub fn recover_account(&self, key_or_account_id: String) -> ExtAccountDetails {
        let account_id = if let Ok(public_key) = key_or_account_id.parse::<PublicKey>() {
            self.attendee_ticket_by_pk
                .get(&public_key)
                .expect("No account found for the given public key")
                .account_id
                .as_ref() // Borrow the account_id to avoid moving it
                .expect("Account has not been scanned in yet")
                .clone() // Clone the account_id so we can move it
        } else {
            key_or_account_id
                .parse::<AccountId>()
                .expect("Invalid account ID format")
        };

        let account_details = self
            .account_details_by_id
            .get(&account_id)
            .expect("No account details found");

        ExtAccountDetails {
            ft_collected: account_details.tokens_collected,
            account_id: account_id.to_string(),
            ft_balance: account_details.ft_balance,
            account_status: account_details.account_status.clone(), // Clone account_status
        }
    }

    /// Adds an account status for a given account ID.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account to add the status for.
    /// * `status` - The status to be added for the account.
    ///
    /// # Panics
    ///
    /// Panics if the caller is not an admin.
    pub fn add_account_status(&mut self, account_id: AccountId, status: AccountStatus) {
        self.assert_admin();
        self.assert_no_freeze();

        // Get mutable reference to account details if it exists
        let account_details = self
            .account_details_by_id
            .entry(account_id.clone())
            .or_insert_with(|| AccountDetails::new(&account_id));

        // Set the account status
        account_details.account_status = Some(status);
    }

    /// Removes the account status for the given list of account IDs.
    ///
    /// # Arguments
    ///
    /// * `account_ids` - A vector of account IDs to remove the status for.
    ///
    /// # Panics
    ///
    /// Panics if the caller is not an admin.
    pub fn remove_account_status(&mut self, account_ids: Vec<AccountId>) {
        self.assert_admin();
        self.assert_no_freeze();

        for account_id in account_ids {
            if let Some(account_details) = self.account_details_by_id.get_mut(&account_id) {
                // Set the account status to None
                account_details.account_status = None;
            }
        }
    }
}
