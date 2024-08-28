use crate::*;

pub const KEYPOM_CONFERENCE_METADATA_SPEC: &str = "1.0.0";
pub const KEYPOM_STANDARD_NAME: &str = "kpom101";

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
#[allow(non_camel_case_types)]
pub enum DropClaimReward {
    NFT,
    Token(U128),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomTokenMintLog {
    pub amount: U128,
    pub receiver_id: String,
    pub drop_id: Option<String>,
    pub new_balance: U128,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomTokenTransferLog {
    pub amount: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub new_sender_balance: U128,
    pub new_receiver_balance: U128,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomDropCreationLog {
    pub drop_reward: DropClaimReward,
    pub creator_id: String,
    pub num_scavengers: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomDropClaimLog {
    pub claimer_id: String,
    pub reward: Option<DropClaimReward>,
    pub pieces_found: Option<u16>,
    pub pieces_required: Option<u16>,
}
