use std::collections::HashMap;

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
    pub starting_near_balance: LookupMap<String, Balance>,
    pub starting_token_balance: LookupMap<String, Balance>,
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
        starting_near_balance: HashMap<String, U128>,
        starting_token_balance: HashMap<String, U128>,
        token_name: Option<String>,
        symbol: Option<String>,
        icon: Option<String>,
    ) -> Self {
        let mut token_balance_map: LookupMap<String, Balance> =
            LookupMap::new(StorageKeys::StartingTokenBalance);
        for (drop_id, amount) in starting_token_balance.into_iter() {
            token_balance_map.insert(&drop_id, &amount.0);
        }
        let mut near_balance_map: LookupMap<String, Balance> =
            LookupMap::new(StorageKeys::StartingNEARBalance);
        for (drop_id, amount) in starting_near_balance.into_iter() {
            near_balance_map.insert(&drop_id, &amount.0);
        }

        Self {
            data_by_vendor: UnorderedMap::new(StorageKeys::DataByVendor),
            admin_accounts: LookupSet::new(StorageKeys::AdminAccounts),

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
                minted_per_claim: None,
            },
            drop_by_id: UnorderedMap::new(StorageKeys::DropById),
            drops_claimed_by_account: LookupMap::new(StorageKeys::DropsClaimedByAccount),

            allowed_drop_id,
            keypom_contract,
            starting_near_balance: near_balance_map,
            starting_token_balance: token_balance_map,
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
