use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, CryptoHash, PanicOnDefault,
    PublicKey,
};

mod drops;
mod events;
mod factory;
mod fungible_tokens;
mod internals;
mod models;
mod non_fungible_tokens;
mod vendors;
mod cleanup;

use drops::*;
use events::*;
use fungible_tokens::*;
use internals::*;
use models::*;
use non_fungible_tokens::*;
use vendors::*;

// ------------------------ Access Key Method Names ------------------------ //
pub const ATTENDEE_KEY_METHOD_NAMES: &str = "claim_drop,ft_transfer";
pub const SPONSOR_KEY_METHOD_NAMES: &str = "create_token_drop,create_nft_drop,delete_drop";

pub const DROP_DELIMITER: &str = "||";

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    // ------------------------ Contract Global ---------------------------- //
    pub account_details_by_id: UnorderedMap<AccountId, AccountDetails>, // clearable
    pub account_id_by_pub_key: UnorderedMap<PublicKey, AccountId>, // clearable
    pub is_contract_frozen: bool,

    // ------------------------ Fungible Tokens ---------------------------- //
    pub ft_total_supply: Balance,
    pub ft_metadata: FungibleTokenMetadata,

    // ------------------------ Non Fungible Tokens ------------------------ //
    pub nft_tokens_by_id: UnorderedMap<TokenId, Token>,
    pub nft_tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub series_by_id: UnorderedMap<SeriesId, Series>,
    pub nft_metadata: NFTContractMetadata,

    // ------------------------ Drops -------------------------------------- //
    pub drop_by_id: UnorderedMap<DropId, DropData>, // clearable

    // ------------------------ Account Factory ---------------------------- //
    pub ticket_data_by_id: UnorderedMap<DropId, TicketType>, // clearable
    pub keypom_contract: AccountId,
}

#[near_bindgen]
impl Contract {
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
            self.account_id_by_pub_key
                .get(&public_key)
                .expect("No account found for the given public key")
        } else {
            key_or_account_id
                .parse::<AccountId>()
                .expect("Invalid account ID format")
        };

        let account_details = self.account_details_by_id
            .get(&account_id)
            .expect("No account details found");

        ExtAccountDetails {
            account_id: account_id.to_string(),
            ft_balance: U128(account_details.ft_balance),
            vendor_data: account_details.vendor_data.map(|d| d.metadata),
            account_status: account_details.account_status,
        }
    }

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
        keypom_contract: AccountId,
        ticket_data: HashMap<DropId, TicketType>,
        token_name: Option<String>,
        symbol: Option<String>,
        icon: Option<String>,
        admin: Vec<AccountId>,
    ) -> Self {
        let mut ticket_data_by_id: UnorderedMap<String, TicketType> =
        UnorderedMap::new(StorageKeys::TicketDataById);

        for (drop_id, ticket_type) in ticket_data.into_iter() {
            ticket_data_by_id.insert(&drop_id, &ticket_type);
        }

        let mut account_details_by_id: UnorderedMap<AccountId, AccountDetails> =
        UnorderedMap::new(StorageKeys::AccountDetailsById);
        for account in admin {
            let mut account_details = AccountDetails::new(&account);
            account_details.account_status = Some(AccountStatus::Admin);
            account_details_by_id.insert(&account, &account_details);
        }

        Self {
            nft_tokens_per_owner: LookupMap::new(StorageKeys::TokensForOwner),
            is_contract_frozen: false,
            account_details_by_id,
            ft_total_supply: 0,
            ft_metadata: FungibleTokenMetadata {
                spec: "ft-1.0.0".to_string(),
                name: token_name.unwrap_or("Redacted Fungible Token".to_string()),
                symbol: symbol.unwrap_or("SOV3".to_string()),
                icon: icon.or(Some(DATA_IMAGE_SVG_GT_ICON.to_string())),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },

            series_by_id: UnorderedMap::new(StorageKeys::SeriesById),
            nft_tokens_by_id: UnorderedMap::new(StorageKeys::TokensById),
            nft_metadata: NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Redacted NFT Contract".to_string(),
                symbol: "SOV3".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },

            drop_by_id: UnorderedMap::new(StorageKeys::DropById),

            keypom_contract,
            ticket_data_by_id,
            account_id_by_pub_key: UnorderedMap::new(StorageKeys::AccountIdByPubKey),
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
        let mut account_details = self
            .account_details_by_id
            .get(&account_id)
            .unwrap_or(AccountDetails::new(&account_id));
        account_details.account_status = Some(status);
        self.account_details_by_id
            .insert(&account_id, &account_details);
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
            let mut account_details = self
                .account_details_by_id
                .get(&account_id)
                .unwrap_or(AccountDetails::new(&account_id));
            account_details.account_status = None;
            self.account_details_by_id
                .insert(&account_id, &account_details);
        }
    }
}
