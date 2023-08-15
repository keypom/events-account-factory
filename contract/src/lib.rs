use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupSet, LookupMap};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen, BorshStorageKey, PanicOnDefault, AccountId, require, env, Balance};

mod models;
mod events;
mod fungible_tokens;

use models::*;
use fungible_tokens::*;
use events::*;

const TRIAL_CONTRACT: &[u8] = include_bytes!("../../out/trial-accounts.wasm");
const INITIAL_TOTAL_SUPPLY: u128 = 1_000_000_000;

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

    // ------------------------ Account Factory ------------------------ //
    pub allowed_drop_id: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(allowed_drop_id: String) -> Self {
        let mut contract = Self {
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

            allowed_drop_id
        };
        contract.internal_deposit_mint(&env::current_account_id(), INITIAL_TOTAL_SUPPLY);
        contract
    }

    // pub fn add_vendor(&mut self, vendor_id: AccountId, vendor_info: VendorInformation) {
    //     self.assert_admin();
    //     self.vendor_info.insert(&vendor_id, &vendor_info);
    // }

    // pub fn get_vendor_info(&self, vendor_id: AccountId) -> VendorInformation {
    //     self.vendor_info.get(&vendor_id).expect("No vendor found")
    // }

    pub(crate) fn assert_admin(&self) {
        require!(self.admin_accounts.contains(&env::predecessor_account_id()), "Unauthorized");
    }
}