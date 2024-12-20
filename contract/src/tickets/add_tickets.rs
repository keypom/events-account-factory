use near_sdk::{Allowance, Promise, PublicKey};

use crate::*;

/// Data for each new ticket key issued such as the users encrypted metadata
#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub struct KeyData {
    pub public_key: PublicKey,
    pub metadata: Option<String>,
}

#[near]
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
    pub fn add_tickets(&mut self, drop_id: DropId, key_data: Vec<KeyData>) {
        self.assert_no_freeze();
        self.assert_ticket_adder();

        // More than 100 keys leads to promise rejection
        require!(key_data.len() < 100, "Maximum number of keys exceeded");

        // Get the current account ID (which will be cloned later as needed)
        let current_account_id = env::current_account_id();

        // Loop through each key and add it to the account and insert into the maps
        for key in key_data.iter() {
            let attendee_info = AttendeeTicketInformation {
                drop_id: Some(drop_id.clone()),
                has_scanned: false,
                account_id: None,
                metadata: key.metadata.clone(),
            };

            require!(
                self.attendee_ticket_by_pk
                    .insert(key.public_key.clone(), attendee_info.clone())
                    .is_none(),
                "Key already exists"
            );

            // Add this key to the batch, clone the `current_account_id`
            Promise::new(current_account_id.clone()).add_access_key_allowance(
                key.public_key.clone(),
                Allowance::unlimited(),
                current_account_id.clone(), // Clone again as it's being moved
                ATTENDEE_KEY_METHOD_NAMES.to_string(),
            );
        }

        self.total_transactions += 1;
    }
}
