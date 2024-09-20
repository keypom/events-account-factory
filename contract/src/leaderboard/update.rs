use crate::*;

#[near_bindgen]
impl Contract {
    // Updates tokens collected leaderboard
    pub(crate) fn update_token_leaderboard(
        &mut self,
        account_id: AccountId,
        tokens_collected: u128,
    ) {
        // Prevent admins or sponsors from getting onto the leaderboard
        let account_details = self
            .account_details_by_id
            .get(&account_id)
            .expect("Account not found in map");
        let creator_status = account_details
            .account_status
            .as_ref()
            .expect("Account not found for status");

        if creator_status.is_admin() || creator_status.is_sponsor() {
            return;
        }

        // Find or add account in the leaderboard
        if let Some(pos) = self
            .token_leaderboard
            .iter()
            .position(|id| id == &account_id)
        {
            self.token_leaderboard.remove(pos);
        }

        // Insert the account in the correct position based on tokens_collected
        let insert_pos = self.token_leaderboard.iter().position(|id| {
            let details = self.account_details_by_id.get(id).unwrap();
            details.tokens_collected.0 < tokens_collected
        });

        if let Some(pos) = insert_pos {
            self.token_leaderboard.insert(pos, account_id);
        } else {
            self.token_leaderboard.push(account_id);
        }

        // Limit the leaderboard to top 10 accounts
        if self.token_leaderboard.len() > 10 {
            self.token_leaderboard.pop();
        }
    }

    // Updates POAPs collected leaderboard
    pub(crate) fn update_poap_leaderboard(&mut self, account_id: &AccountId, poaps_collected: u16) {
        // Prevent admins or sponsors from getting onto the leaderboard
        let account_details = self
            .account_details_by_id
            .get(account_id)
            .expect("Account not found in map");
        let creator_status = account_details
            .account_status
            .as_ref()
            .expect("Account not found for status");

        if creator_status.is_admin() || creator_status.is_sponsor() {
            return;
        }

        // Similar logic to token leaderboard
        if let Some(pos) = self.poap_leaderboard.iter().position(|id| id == account_id) {
            self.poap_leaderboard.remove(pos);
        }

        let insert_pos = self.poap_leaderboard.iter().position(|id| {
            let num_tokens = self.nft_tokens_per_owner.get(id).unwrap().len() as u16;
            num_tokens < poaps_collected
        });

        if let Some(pos) = insert_pos {
            self.poap_leaderboard.insert(pos, account_id.clone());
        } else {
            self.poap_leaderboard.push(account_id.clone());
        }

        // Limit the leaderboard to top 10 accounts
        if self.poap_leaderboard.len() > 10 {
            self.poap_leaderboard.pop();
        }
    }

    pub(crate) fn add_transaction(&mut self, transaction: TransactionType) {
        // Add the transaction to the list
        self.recent_transactions.push(transaction);

        // Ensure the vector contains only the last 10 transactions
        if self.recent_transactions.len() > 10 {
            self.recent_transactions.remove(0); // Remove the oldest transaction
        }
    }
}
