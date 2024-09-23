use crate::*;

#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub enum TransactionType {
    Claim {
        account_id: AccountId,
        reward: String, // Could be an image URL, scavenger piece, or token amount
        timestamp: u64,
    },
    Transfer {
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: NearToken,
        timestamp: u64,
    },
}
