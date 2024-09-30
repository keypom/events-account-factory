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

    // Query for the status of the contract (i.e is the conference over?)
    pub fn is_contract_frozen(&self) -> bool {
        self.is_contract_frozen
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
            let before = env::storage_usage();
            let num_drops = self.drop_by_id.len();
            self.drop_by_id.clear();
            self.drop_by_id.flush();
            near_sdk::log!(
                "Cleared {} drops. {} bytes cleared. Initial {} Final {}",
                num_drops,
                before - env::storage_usage(),
                before,
                env::storage_usage()
            );
        }
        if !self.ticket_data_by_id.is_empty() {
            let before = env::storage_usage();
            let num_tickets = self.ticket_data_by_id.len();
            self.ticket_data_by_id.clear();
            self.ticket_data_by_id.flush();
            near_sdk::log!(
                "Cleared {} ticket data. {} bytes cleared. Initial {} Final {}",
                num_tickets,
                before - env::storage_usage(),
                before,
                env::storage_usage()
            );
        }
        if self.agenda.is_empty() {
            let before = env::storage_usage();
            self.agenda = String::new();
            self.agenda_timestamp = 0;
            near_sdk::log!(
                "Cleared the agenda. {} bytes cleared. Initial {} Final {}",
                before - env::storage_usage(),
                before,
                env::storage_usage()
            );
        }
        if self.alerts.is_empty() {
            let before = env::storage_usage();
            near_sdk::log!(
                "Cleared the alerts. {} bytes cleared. Initial {} Final {}",
                before - env::storage_usage(),
                before,
                env::storage_usage()
            );
            self.alerts = String::new();
            self.alerts_timestamp = 0;
        }
        if self.attendee_ticket_by_pk.is_empty() {
            let before = env::storage_usage();
            let num_tickets = self.attendee_ticket_by_pk.len();
            self.attendee_ticket_by_pk.clear();
            self.attendee_ticket_by_pk.flush();
            near_sdk::log!(
                "Cleared {} attendee tickets. {} bytes cleared. Initial {} Final {}",
                num_tickets,
                before - env::storage_usage(),
                before,
                env::storage_usage()
            );
        }
        if self.token_leaderboard.is_empty() {
            let before = env::storage_usage();
            self.token_leaderboard = Vec::new();
            near_sdk::log!(
                "Cleared the token leaderboard. {} bytes cleared. Initial {} Final {}",
                before - env::storage_usage(),
                before,
                env::storage_usage()
            );
        }
        if self.poap_leaderboard.is_empty() {
            let before = env::storage_usage();
            self.poap_leaderboard = Vec::new();
            near_sdk::log!(
                "Cleared the POAP leaderboard. {} bytes cleared. Initial {} Final {}",
                before - env::storage_usage(),
                before,
                env::storage_usage()
            );
        }
        if self.recent_transactions.is_empty() {
            let before = env::storage_usage();
            self.recent_transactions = Vec::new();
            near_sdk::log!(
                "Cleared the recent transactions. {} bytes cleared. Initial {} Final {}",
                before - env::storage_usage(),
                before,
                env::storage_usage()
            );
        }

        // Get the total number of accounts to clear.
        let total = self.account_details_by_id.len();

        // Define the maximum number of accounts to process in one batch.
        let batch_size = limit.unwrap_or(1000);

        // Collect the keys of the accounts to be processed in this batch (immutable borrow).
        let accounts_to_process: Vec<_> = self
            .account_details_by_id
            .drain()
            .take(batch_size as usize)
            .collect();

        // Initialize a counter to track the number of accounts processed in this batch.
        let mut processed = 0;

        // Now perform the removal in a separate mutable borrow (mutable borrow starts here).
        for (_account_id, mut account_details) in accounts_to_process {
            // Clear the inner data structures associated with this account.
            account_details.drops_claimed.clear();
            account_details.drops_created.clear();

            // Increment the counter for each processed account.
            processed += 1;
        }
        near_sdk::log!("Cleared {} accounts.", processed);

        // Calculate the storage usage after the removals.
        let storage_used = storage_initial - env::storage_usage();
        near_sdk::log!(
            "Cleared {} bytes. Initial {} Final {}",
            storage_used,
            storage_initial,
            env::storage_usage()
        );
        on_storage_cleared(refund_account, storage_used);

        // Calculate and return the number of accounts left to clear.

        (total - processed) as u64
    }
}
