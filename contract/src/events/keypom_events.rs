use crate::*;

pub const KEYPOM_CONFERENCE_METADATA_SPEC: &str = "1.0.0";
pub const KEYPOM_STANDARD_NAME: &str = "kpom101";

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum DropType {
    NFT,
    Token,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomAccountCreatedLog {
    pub account_id: String,
    pub pub_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomTokenMintLog {
    pub receiver_id: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomTokenTransferLog {
    pub old_owner_id: String,
    pub new_owner_id: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomDropCreationLog {
    pub creator_id: String,
    pub num_scavengers: u16,
    pub drop_type: DropType,
    pub drop_name: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomDropClaimLog {
    pub receiver_id: String,
    pub drop_name: String,
    pub scavenger
}