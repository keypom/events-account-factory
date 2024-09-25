use cleanup::helpers::on_storage_cleared;

use crate::*;

#[near]
impl Contract {
    /// Allows an admin to freeze all token transactions on the contract
    ///
    /// Panics if the account is not authorized.
    pub fn toggle_freeze(&mut self, is_freeze: bool) {
        self.assert_admin();
        self.is_contract_frozen = is_freeze;
    }

    /// Clears storage by removing account details in batches.
    ///
    /// This function will only work when the contract is frozen. It iterates
    /// over the `account_details_by_id` map, processing up to 1000 accounts at
    /// a time. For each account, it removes the account from the outer map and
    /// clears any associated data in the inner maps (`drops_claimed` and
    /// `drops_created`).
    ///
    /// # Returns
    ///
    /// Returns the number of accounts left to clear after this operation.
    ///
    /// # Panics
    ///
    /// Panics if the contract is not frozen or if the caller is not an admin.
    pub fn clear_storage(&mut self, limit: Option<u32>, refund_account: AccountId) -> u64 {
        // Ensure that only an admin can perform this operation.
        self.assert_admin();
        let storage_initial = env::storage_usage();
        // Ensure that the contract is frozen before clearing storage.
        require!(
            self.is_contract_frozen,
            "Storage can only be cleared once the contract is frozen"
        );

        // Clear the external maps if they're not already cleared
        if !self.drop_by_id.is_empty() {
            self.drop_by_id.clear();
        }
        if !self.ticket_data_by_id.is_empty() {
            self.ticket_data_by_id.clear();
        }
        if self.agenda.is_empty() {
            self.agenda = String::new();
            self.agenda_timestamp = 0;
        }
        if self.alerts.is_empty() {
            self.alerts = String::new();
            self.alerts_timestamp = 0;
        }
        if self.attendee_ticket_by_pk.is_empty() {
            self.attendee_ticket_by_pk.clear();
        }
        if self.token_leaderboard.is_empty() {
            self.token_leaderboard.clear();
        }
        if self.poap_leaderboard.is_empty() {
            self.poap_leaderboard.clear();
        }
        if self.recent_transactions.is_empty() {
            self.recent_transactions.clear();
        }

        // Get the total number of accounts to clear.
        let total = self.account_details_by_id.len();

        // Define the maximum number of accounts to process in one batch.
        let batch_size = limit.unwrap_or(1000);

        // Collect the keys of the accounts to be processed in this batch (immutable borrow).
        let account_ids_to_process: Vec<_> = self
            .account_details_by_id
            .keys()
            .take(batch_size as usize)
            .cloned() // Clone the keys to avoid borrowing issues
            .collect();

        // Initialize a counter to track the number of accounts processed in this batch.
        let mut processed = 0;

        // Now perform the removal in a separate mutable borrow (mutable borrow starts here).
        for account_id in account_ids_to_process {
            // Get the account details.
            if let Some(mut account_details) = self.account_details_by_id.remove(&account_id) {
                // Clear the inner data structures associated with this account.
                account_details.drops_claimed.clear();
                account_details.drops_created.clear();

                // Increment the counter for each processed account.
                processed += 1;
            }
        }

        // Calculate the storage usage after the removals.
        let storage_used = env::storage_usage() - storage_initial;
        on_storage_cleared(refund_account, storage_used);

        // Calculate and return the number of accounts left to clear.

        (total - processed) as u64
    }
}
