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
    /// Scans the ticket into the event
    ///
    /// # Panics
    ///
    /// Panics if the ticket has already been scanned or if the ticket does not exist.
    #[payable]
    pub fn scan_ticket(&mut self) {
        self.assert_no_freeze();
        let ticket_pk = env::signer_account_pk();

        let mut attendee_ticket = self
            .attendee_ticket_by_pk
            .get(&ticket_pk)
            .expect("No ticket information found for public key");
        require!(
            !attendee_ticket.has_scanned,
            "Ticket has already been scanned"
        );
        attendee_ticket.has_scanned = true;

        self.attendee_ticket_by_pk
            .insert(&ticket_pk, &attendee_ticket);
    }

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
    pub fn create_account(&mut self, new_account_id: AccountId) -> Promise {
        self.assert_no_freeze();
        let ticket_pk = env::signer_account_pk();

        let mut attendee_ticket = self
            .attendee_ticket_by_pk
            .get(&ticket_pk)
            .expect("No ticket information found for public key");
        require!(
            attendee_ticket.has_scanned,
            "Ticket needs to be scanned first"
        );

        // Get the next available account ID in case the one passed in is taken
        let account_id: AccountId = self.find_available_account_id(new_account_id);

        // Update the attendee ticket with the new account ID and make sure that it doesnt get
        // scanned in again
        attendee_ticket.account_id = Some(account_id.clone());
        self.attendee_ticket_by_pk
            .insert(&ticket_pk, &attendee_ticket);

        let ticket_drop_id = attendee_ticket.drop_id;
        let ticket_data = self.ticket_data_by_id.get(&ticket_drop_id).unwrap();
        self.internal_create_account(account_id, ticket_pk, ticket_data, false)
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

        let attendee_info = AttendeeTicketInformation {
            drop_id: '.'.to_string(),
            has_scanned: false,
            account_id: Some(new_account_id.clone()),
            metadata: '.'.to_string(),
        };
        require!(
            self.attendee_ticket_by_pk
                .insert(&new_public_key, &attendee_info)
                .is_none(),
            "Key already exists"
        );
        self.internal_create_account(new_account_id, new_public_key, ticket_data, true)
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
        add_key: bool,
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
            AccountStatus::DataSetter => {
                account_details.account_status = Some(AccountStatus::DataSetter);
                access_key_method_names = DATA_SETTER_KEY_METHOD_NAMES;
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

        self.account_details_by_id
            .insert(&new_account_id, &account_details);

        // Deposit the starting balance into the account and then create it
        self.internal_deposit_ft_mint(&new_account_id, tokens_to_start.0, None);

        let final_storage_usage = env::storage_usage();
        near_sdk::log!(
            "Storage used: {}",
            final_storage_usage - initial_storage_usage
        );

        // Add the ticket access key for this account so they can sign transactions only if they
        // are not an attendee (since their original ticket contains the key already and its been
        // created)
        if add_key {
            Promise::new(env::current_account_id()).add_access_key(
                new_public_key.clone(),
                0, // unlimited allowance
                env::current_account_id(),
                access_key_method_names.to_string(),
            );
        }

        // Add the same full access key to the account so that they can offboard later
        Promise::new(new_account_id.clone())
            .create_account()
            .transfer(near_to_start.0)
            .add_full_access_key(new_public_key)
    }
}
