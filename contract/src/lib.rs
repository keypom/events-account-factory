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

use drops::*;
use events::*;
use fungible_tokens::*;
use internals::*;
use models::*;
use non_fungible_tokens::*;
use vendors::*;

// ------------------------ Access Key Method Names ------------------------ //
pub const GLOBAL_KEY_METHOD_NAMES: &str = "claim_drop,ft_transfer";

pub const DROP_DELIMITER: &str = "||";

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pub account_details_by_id: LookupMap<AccountId, AccountDetails>,
    // ------------------------ Fungible Tokens ---------------------------- //
    pub ft_total_supply: Balance,
    pub ft_metadata: FungibleTokenMetadata,

    // ------------------------ Non Fungible Tokens ------------------------ //
    pub tokens_by_id: UnorderedMap<TokenId, Token>,
    pub series_by_id: UnorderedMap<SeriesId, Series>,
    pub nft_metadata: NFTContractMetadata,

    // ------------------------ Drops -------------------------------------- //
    pub drop_by_id: UnorderedMap<DropId, DropData>,

    // ------------------------ Account Factory ---------------------------- //
    pub ticket_data_by_id: LookupMap<DropId, TicketType>,
    pub keypom_contract: AccountId,

    pub account_id_by_pub_key: LookupMap<PublicKey, AccountId>,
}

#[near_bindgen]
impl Contract {
    /// Queries for the account ID that claimed the ticket associated with the given public key.
    ///
    /// # Arguments
    ///
    /// * `key` - The public key to query for the associated account ID.
    ///
    /// # Returns
    ///
    /// Returns the `AccountId` associated with the given public key.
    ///
    /// # Panics
    ///
    /// Panics if no account is found for the given public key.
    pub fn recover_account(&self, key: PublicKey) -> AccountId {
        self.account_id_by_pub_key
            .get(&key)
            .expect("No account found")
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
        let mut ticket_data_by_id: LookupMap<String, TicketType> =
            LookupMap::new(StorageKeys::TicketDataById);

        for (drop_id, ticket_type) in ticket_data.into_iter() {
            ticket_data_by_id.insert(&drop_id, &ticket_type);
        }

        let mut account_details_by_id: LookupMap<AccountId, AccountDetails> =
            LookupMap::new(StorageKeys::AccountDetailsById);
        for account in admin {
            let mut account_details = AccountDetails::new(&account);
            account_details.account_status = Some(AccountStatus::Admin);
            account_details_by_id.insert(&account, &account_details);
        }

        Self {
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
            tokens_by_id: UnorderedMap::new(StorageKeys::TokensById),
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
            account_id_by_pub_key: LookupMap::new(StorageKeys::AccountIdByPubKey),
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

    /// Retrieves the account ID associated with the public key of the caller.
    ///
    /// This function maps the public key of the signer (caller) to the corresponding account ID.
    ///
    /// # Returns
    ///
    /// Returns the `AccountId` associated with the signer's public key.
    ///
    /// # Panics
    ///
    /// Panics if no account ID is found for the given public key.
    fn caller_id_by_signing_pk(&self) -> AccountId {
        self.account_id_by_pub_key
            .get(&env::signer_account_pk())
            .expect("No associated account ID found for the given public key")
    }
}
