use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum TransactionType {
    Claim {
        account_id: AccountId,
        reward: String, // Could be an image URL, scavenger piece, or token amount
        timestamp: u64,
    },
    Transfer {
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        timestamp: u64,
    },
}
