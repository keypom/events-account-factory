use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault, PublicKey,
};

mod events;
mod factory;
mod fungible_tokens;
mod models;
mod vendors;

use events::*;
use fungible_tokens::*;
use models::*;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    // ------------------------ Vendor Information ------------------------ //
    pub data_by_vendor: UnorderedMap<AccountId, VendorInformation>,
    pub account_status_by_id: LookupMap<AccountId, AccountStatus>,

    // ------------------------ Fungible Tokens ------------------------ //
    pub balance_by_account: LookupMap<AccountId, Balance>,
    pub total_supply: Balance,
    pub metadata: FungibleTokenMetadata,

    pub drop_by_id: UnorderedMap<DropId, InternalDropData>,
    pub drops_claimed_by_account: LookupMap<AccountId, UnorderedMap<DropId, Vec<ScavengerId>>>,

    // ------------------------ Account Factory ------------------------ //
    pub ticket_data_by_id: LookupMap<DropId, TicketType>,
    pub keypom_contract: AccountId,

    pub account_id_by_pub_key: LookupMap<PublicKey, AccountId>,
}

#[near_bindgen]
impl Contract {
    /// Allows
    pub fn recover_account(&self, key: PublicKey) -> AccountId {
        self.account_id_by_pub_key
            .get(&key)
            .expect("No account found")
    }

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
            account_status_by_id: LookupMap::new(StorageKeys::AccontStatusById),

            balance_by_account: LookupMap::new(StorageKeys::BalanceByAccount),
            total_supply: 0,
            metadata: FungibleTokenMetadata {
                spec: "ft-1.0.0".to_string(),
                name: token_name.unwrap_or("NEARCon Fungible Token".to_string()),
                symbol: symbol.unwrap_or("token".to_string()),
                icon: icon.or(Some(DATA_IMAGE_SVG_GT_ICON.to_string())),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
            drop_by_id: UnorderedMap::new(StorageKeys::DropById),
            drops_claimed_by_account: LookupMap::new(StorageKeys::DropsClaimedByAccount),

            keypom_contract,
            ticket_data_by_id,
            account_id_by_pub_key: LookupMap::new(StorageKeys::AccountIdByPubKey),
        }
    }

    #[private]
    pub fn add_account_status(&mut self, account_id: AccountId, status: AccountStatus) {
        self.account_status_by_id.insert(&account_id, &status);
    }

    #[private]
    pub fn remove_accont_status(&mut self, account_ids: Vec<AccountId>) {
        for account_id in account_ids {
            self.account_status_by_id.remove(&account_id);
        }
    }
}
