use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LeaderboardInformation {
    pub recent_transactions: Vec<TransactionType>,
    pub total_transactions: u64,
    pub total_tokens_transferred: U128,
    pub token_leaderboard: Vec<(AccountId, U128)>,
    pub poap_leaderboard: Vec<(AccountId, u16)>,
}

#[near_bindgen]
impl Contract {
    /// View function to get all leaderboard information in one call.
    ///
    /// # Returns
    ///
    /// A `LeaderboardInformation` struct containing the recent transactions, total transactions,
    /// total tokens transferred, token leaderboard, and POAP leaderboard.
    pub fn get_leaderboard_information(&self) -> LeaderboardInformation {
        // Get recent transactions
        let recent_transactions = self.recent_transactions.clone();

        // Get total number of transactions
        let total_transactions = self.total_transactions;

        // Get total tokens transferred
        let total_tokens_transferred = self.total_tokens_transferred;

        // Get token leaderboard
        let token_leaderboard = self
            .token_leaderboard
            .iter()
            .map(|account_id| {
                let account_details = self.account_details_by_id.get(account_id).unwrap();
                (account_id.clone(), U128(account_details.tokens_collected.0))
            })
            .collect();

        // Get POAP leaderboard
        let poap_leaderboard = self
            .poap_leaderboard
            .iter()
            .map(|account_id| {
                let num_poaps = self.nft_tokens_per_owner.get(account_id).unwrap().len() as u16;
                (account_id.clone(), num_poaps)
            })
            .collect();

        // Return all the information
        LeaderboardInformation {
            recent_transactions,
            total_transactions,
            total_tokens_transferred: U128(total_tokens_transferred),
            token_leaderboard,
            poap_leaderboard,
        }
    }
}
