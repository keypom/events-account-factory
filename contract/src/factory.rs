use near_sdk::{Promise, PublicKey};

use crate::*;

/// Keypom Args struct to be sent to external contracts
#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomArgs {
    pub account_id_field: Option<String>,
    pub drop_id_field: Option<String>,
    pub key_id_field: Option<String>,
    pub funder_id_field: Option<String>,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_account(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
        drop_id: String,
        keypom_args: KeypomArgs,
    ) -> Promise {
        let initial_storage_usage = env::storage_usage();

        self.assert_keypom();
        // Ensure the incoming args are correct from Keypom
        require!(keypom_args.drop_id_field.expect("No keypom args sent") == "drop_id".to_string());
        if let Some(required_drop_id) = self.allowed_drop_id.clone() {
            require!(drop_id == required_drop_id, "Invalid drop ID");
        }

        // Get the next available account ID in case the one passed in is taken
        let account_id: AccountId = self.find_available_account_id(new_account_id);
        let tokens_to_start = self
            .starting_near_balance
            .get(&drop_id)
            .expect("Drop ID not found");
        let near_to_start = self
            .starting_token_balance
            .get(&drop_id)
            .expect("Drop ID not found");

        near_sdk::log!(
            "Creating account: {} with starting balance: {}",
            account_id,
            near_to_start
        );
        // Add the account ID to the map
        self.account_id_by_pub_key
            .insert(&new_public_key, &account_id);
        let drop_set = UnorderedMap::new(StorageKeys::DropsClaimedByAccountInner {
            account_id_hash: env::sha256_array(account_id.as_bytes()),
        });
        self.drops_claimed_by_account.insert(&account_id, &drop_set);
        // Deposit the starting balance into the account and then create it
        self.internal_deposit_mint(&account_id, tokens_to_start);

        let final_storage_usage = env::storage_usage();
        near_sdk::log!(
            "Storage used: {}",
            final_storage_usage - initial_storage_usage
        );

        Promise::new(account_id.clone())
            .create_account()
            .transfer(near_to_start)
            .add_full_access_key(new_public_key.into())
    }

    /// In the case that multiple people choose the same username (i.e ben.nearcon.near) at the same time
    /// Before the frontend can validate, we should simply append a number to the end of the username i.e ben1.nearcon.near & ben2.nearcon.near etc...
    pub(crate) fn find_available_account_id(&self, new_account_id: AccountId) -> AccountId {
        let delim = format!(".{}", env::current_account_id()).to_string();
        let binding = new_account_id.to_string();
        let split: Vec<&str> = binding.split(&delim).collect();
        let prefix = split[0].to_string();

        let mut account_id = new_account_id.clone();
        let found = false;
        let mut i = 0;

        while !found {
            let is_new_account = !self.balance_by_account.contains_key(&account_id);

            if is_new_account {
                return account_id;
            }

            i += 1;
            account_id = format!("{}-{}.{}", prefix, i, env::current_account_id())
                .parse()
                .unwrap();
        }

        new_account_id
    }

    pub fn get_starting_token_balance(&self, drop_id: String) -> U128 {
        U128(
            self.starting_token_balance
                .get(&drop_id)
                .expect("no drop id found"),
        )
    }

    pub fn get_starting_near_balance(&self, drop_id: String) -> U128 {
        U128(
            self.starting_near_balance
                .get(&drop_id)
                .expect("no drop id found"),
        )
    }

    /// Update the starting balance for NEAR
    pub fn update_starting_near_balance(&mut self, drop_id: String, new_balance: U128) {
        self.assert_admin();
        self.starting_near_balance.insert(&drop_id, &new_balance.0);
    }

    /// Update the starting balance for token
    pub fn update_starting_token_balance(&mut self, drop_id: String, new_balance: U128) {
        self.assert_admin();
        self.starting_token_balance.insert(&drop_id, &new_balance.0);
    }

    /// Update the drop ID that is allowed to create accounts
    pub fn update_drop_id(&mut self, new_drop_id: Option<String>) {
        self.assert_admin();
        self.allowed_drop_id = new_drop_id;
    }

    /// Update the drop ID that is allowed to create accounts
    pub fn update_keypom_contract(&mut self, keypom_contract: AccountId) {
        self.assert_admin();
        self.keypom_contract = keypom_contract;
    }

    /// Assert that the caller is either keypom or the current account
    pub(crate) fn assert_keypom(&self) {
        let caller = env::predecessor_account_id();
        if caller != env::current_account_id() {
            require!(
                env::predecessor_account_id() == self.keypom_contract,
                "Only Keypom can call this method"
            );
        }
    }
}
