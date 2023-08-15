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

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    // ------------------------ Vendor Information ------------------------ //
    pub vendor_info: UnorderedMap<AccountId, VendorInformation>,
    pub admins: LookupSet<AccountId>,

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
        Self {
            vendor_info: UnorderedMap::new(StorageKeys::VendorInfo),
            admins: LookupSet::new(StorageKeys::Admins),
            
            balance_by_account: LookupMap::new(StorageKeys::BalanceByAccount),
            total_supply: 1_000_000_000_000, // Todo: change
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
        }
    }

    pub fn add_vendor(&mut self, vendor_id: AccountId, vendor_info: VendorInformation) {
        self.assert_admin();
        self.vendor_info.insert(&vendor_id, &vendor_info);
    }

    pub fn get_vendor_info(&self, vendor_id: AccountId) -> VendorInformation {
        self.vendor_info.get(&vendor_id).expect("No vendor found")
    }

    pub(crate) fn assert_admin(&self) {
        require!(self.admins.contains(&env::predecessor_account_id()), "Unauthorized");
    }
}