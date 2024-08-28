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
    /// Creates a new account with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `new_account_id` - The ID of the new account to be created.
    /// * `new_public_key` - The public key for the new account.
    /// * `drop_id` - The ID of the drop associated with the account creation.
    /// * `keypom_args` - Additional arguments from Keypom.
    ///
    /// # Returns
    ///
    /// Returns a promise to create the new account.
    ///
    /// # Panics
    ///
    /// Panics if the Keypom arguments are invalid or the drop ID does not exist.
    #[payable]
    pub fn create_account(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
        drop_id: String,
        keypom_args: KeypomArgs,
    ) -> Promise {
        self.assert_no_freeze();
        self.assert_keypom();
        // Ensure the incoming args are correct from Keypom
        require!(
            keypom_args.drop_id_field.expect("No keypom args sent") == *"drop_id",
            "Invalid Keypom arguments"
        );
        require!(
            self.ticket_data_by_id.get(&drop_id).is_some(),
            "Invalid drop ID"
        );

        // Get the next available account ID in case the one passed in is taken
        let account_id: AccountId = self.find_available_account_id(new_account_id);
        let ticket_data = self.ticket_data_by_id.get(&drop_id).unwrap();
        self.internal_create_account(account_id, new_public_key, ticket_data)
    }

    /// Finds the next available account ID if the given one is already taken.
    /// If multiple users choose the same username (e.g., ben.nearcon.near) simultaneously,
    /// appends a number to the end of the username (e.g., ben1.nearcon.near, ben2.nearcon.near).
    ///
    /// # Arguments
    ///
    /// * `new_account_id` - The desired account ID.
    ///
    /// # Returns
    ///
    /// Returns the available account ID.
    pub(crate) fn find_available_account_id(&self, new_account_id: AccountId) -> AccountId {
        let delim = format!(".{}", env::current_account_id()).to_string();
        let binding = new_account_id.to_string();
        let split: Vec<&str> = binding.split(&delim).collect();
        let prefix = split[0].to_string();

        let mut account_id = new_account_id.clone();
        let mut i = 0;

        loop {
            let is_new_account = self.account_details_by_id.get(&account_id).is_none();

            if is_new_account {
                return account_id;
            }

            i += 1;
            account_id = format!("{}-{}.{}", prefix, i, env::current_account_id())
                .parse()
                .unwrap();
        }
    }

    /// Creates a new account with the given parameters.
    /// Initializes the account with the starting balances and account type.
    ///
    /// # Arguments
    ///
    /// * `new_account_id` - The ID of the new account.
    /// * `new_public_key` - The public key for the new account.
    /// * `ticket_data` - The ticket data associated with the account creation.
    ///
    /// # Returns
    ///
    /// Returns a promise to create the new account.
    pub fn admin_create_account(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
        ticket_data: TicketType,
    ) -> Promise {
        self.assert_no_freeze();
        self.assert_admin();
        self.internal_create_account(new_account_id, new_public_key, ticket_data)
    }

    /// Internally creates a new account with the given parameters.
    /// Initializes the account with the starting balances and account type.
    ///
    /// # Arguments
    ///
    /// * `new_account_id` - The ID of the new account.
    /// * `new_public_key` - The public key for the new account.
    /// * `ticket_data` - The ticket data associated with the account creation.
    ///
    /// # Returns
    ///
    /// Returns a promise to create the new account.
    fn internal_create_account(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
        ticket_data: TicketType,
    ) -> Promise {
        let initial_storage_usage = env::storage_usage();

        let tokens_to_start = ticket_data.starting_token_balance;
        let near_to_start = ticket_data.starting_near_balance;

        let mut account_details = AccountDetails::new(&new_account_id);
        let mut access_key_method_names = ATTENDEE_KEY_METHOD_NAMES;
        match ticket_data.account_type {
            AccountStatus::Sponsor => {
                account_details.account_status = Some(AccountStatus::Sponsor);
                access_key_method_names = SPONSOR_KEY_METHOD_NAMES;
            }
            AccountStatus::Admin => {
                account_details.account_status = Some(AccountStatus::Admin);
            }
            _ => {
                // Do nothing for other cases, including AccountStatus::Basic
            }
        }

        near_sdk::log!(
            "Creating account: {} with starting balance: {}",
            new_account_id,
            near_to_start.0
        );

        // Add the account ID to the map
        self.account_id_by_pub_key
            .insert(&new_public_key, &new_account_id);
        self.account_details_by_id
            .insert(&new_account_id, &account_details);

        // Deposit the starting balance into the account and then create it
        self.internal_deposit_ft_mint(&new_account_id, tokens_to_start.0, None);

        let final_storage_usage = env::storage_usage();
        near_sdk::log!(
            "Storage used: {}",
            final_storage_usage - initial_storage_usage
        );

        // Add the ticket access key for this account so they can sign transactions
        Promise::new(env::current_account_id()).add_access_key(
            new_public_key.clone(),
            0, // unlimited allowance
            env::current_account_id(),
            access_key_method_names.to_string(),
        );

        // Add the same full access key to the account so that they can offboard later
        Promise::new(new_account_id.clone())
            .create_account()
            .transfer(near_to_start.0)
            .add_full_access_key(new_public_key)
    }
}
