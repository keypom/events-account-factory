use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap};
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
    pub admin_accounts: LookupSet<AccountId>,

    // ------------------------ Fungible Tokens ------------------------ //
    pub balance_by_account: LookupMap<AccountId, Balance>,
    pub total_supply: Balance,
    pub metadata: FungibleTokenMetadata,
    pub drop_by_id: UnorderedMap<String, InternalDropData>,
    pub drops_claimed_by_account: LookupMap<AccountId, UnorderedMap<String, Vec<String>>>,

    // ------------------------ Account Factory ------------------------ //
    pub allowed_drop_id: Option<String>,
    pub keypom_contract: AccountId,
    pub starting_near_balance: Balance,
    pub starting_ncon_balance: Balance,
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
        allowed_drop_id: Option<String>,
        keypom_contract: AccountId,
        starting_near_balance: U128,
        starting_ncon_balance: U128,
    ) -> Self {
        Self {
            data_by_vendor: UnorderedMap::new(StorageKeys::DataByVendor),
            admin_accounts: LookupSet::new(StorageKeys::AdminAccounts),

            balance_by_account: LookupMap::new(StorageKeys::BalanceByAccount),
            total_supply: 0,
            metadata: FungibleTokenMetadata {
                spec: "ft-1.0.0".to_string(),
                name: "NEARCon Fungible Token".to_string(),
                symbol: "NCON".to_string(),
                icon: Some(DATA_IMAGE_SVG_GT_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
            drop_by_id: UnorderedMap::new(StorageKeys::DropById),
            drops_claimed_by_account: LookupMap::new(StorageKeys::DropsClaimedByAccount),

            allowed_drop_id,
            keypom_contract,
            starting_near_balance: starting_near_balance.into(),
            starting_ncon_balance: starting_ncon_balance.into(),
            account_id_by_pub_key: LookupMap::new(StorageKeys::AccountIdByPubKey),
        }
    }

    #[private]
    pub fn add_admin(&mut self, account_ids: Vec<AccountId>) {
        for account_id in account_ids {
            self.admin_accounts.insert(&account_id);
        }
    }

    #[private]
    pub fn remove_admin(&mut self, account_ids: Vec<AccountId>) {
        for account_id in account_ids {
            self.admin_accounts.remove(&account_id);
        }
    }
}
