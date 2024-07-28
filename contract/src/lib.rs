use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault, PublicKey,
};

mod events;
mod drops;
mod factory;
mod fungible_tokens;
mod models;
mod vendors;
mod non_fungible_tokens;

use events::*;
use fungible_tokens::*;
use non_fungible_tokens::*;
use vendors::*;
use models::*;
use drops::*;

// ------------------------ Access Key Method Names ------------------------ //
pub const GLOBAL_KEY_METHOD_NAMES: &str =
    "claim_drop,ft_transfer";

pub const DROP_DELIMITER: &str = "||";


#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    // ------------------------ Vendor Information ------------------------- //
    pub data_by_vendor: UnorderedMap<AccountId, VendorInformation>,
    pub account_status_by_id: LookupMap<AccountId, AccountStatus>,

    // ------------------------ Fungible Tokens ---------------------------- //
    pub ft_balance_by_account: LookupMap<AccountId, Balance>,
    pub ft_total_supply: Balance,
    pub ft_metadata: FungibleTokenMetadata,

    // ------------------------ Non Fungible Tokens ------------------------ //
    pub tokens_by_id: UnorderedMap<TokenId, Token>,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub series_by_id: UnorderedMap<SeriesId, Series>,
    pub nft_metadata: NFTContractMetadata,

    // ------------------------ Drops ==========---------------------------- //
    pub drop_ids_by_creator: LookupMap<AccountId, UnorderedSet<DropId>>,
    pub drop_by_id: UnorderedMap<DropId, DropData>,
    pub claims_by_account: LookupMap<AccountId, UnorderedMap<DropId, ClaimedDropData>>,

    // ------------------------ Account Factory ------------------------ //
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
    ) -> Self {
        let mut ticket_data_by_id: LookupMap<String, TicketType> =
            LookupMap::new(StorageKeys::TicketDataById);

        for (drop_id, ticket_type) in ticket_data.into_iter() {
            ticket_data_by_id.insert(&drop_id, &ticket_type);
        }

        Self {
            data_by_vendor: UnorderedMap::new(StorageKeys::DataByVendor),
            account_status_by_id: LookupMap::new(StorageKeys::AccountStatusById),
            
            ft_balance_by_account: LookupMap::new(StorageKeys::BalanceByAccount),
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
            tokens_per_owner: LookupMap::new(StorageKeys::TokensPerOwner),
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
            claims_by_account: LookupMap::new(StorageKeys::DropsClaimedByAccount),
            drop_ids_by_creator: LookupMap::new(StorageKeys::DropIdsByCreator),

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
        self.account_status_by_id.insert(&account_id, &status);
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
            self.account_status_by_id.remove(&account_id);
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
        self.account_id_by_pub_key.get(&env::signer_account_pk()).expect("No associated account ID found for the given public key")
    }
}
