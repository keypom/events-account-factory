use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet, UnorderedMap, LookupSet};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen, BorshStorageKey, PanicOnDefault, PublicKey, Promise, AccountId, require, env};

mod models;
use models::*;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Mapping {
    vendor_info: UnorderedMap<AccountId, VendorInformation>,
    admins: LookupSet<AccountId>
}

#[near_bindgen]
impl Mapping {
    #[init]
    pub fn new() -> Self {
        Self {
            vendor_info: UnorderedMap::new(StorageKeys::VendorInfo),
            admins: LookupSet::new(StorageKeys::Admins)
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