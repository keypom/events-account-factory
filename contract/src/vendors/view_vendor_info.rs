use crate::*;

/// An event log to capture tokens minting
/// Arguments
/// * `owner_id`: "account.near"
/// * `amount`: the number of tokens to mint, wrapped in quotes and treated
///   like a string, although the number will be stored as an unsigned integer
///   with 128 bits.
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FtMintLog {
    pub owner_id: String,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// An event log to capture tokens burning
/// Arguments
/// * `owner_id`: owner of tokens to burn
/// * `amount`: the number of tokens to burn, wrapped in quotes and treated
///   like a string, although the number will be stored as an unsigned integer
///   with 128 bits.
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FtBurnLog {
    pub owner_id: String,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// An event log to capture tokens transfer
/// Arguments
/// * `old_owner_id`: "owner.near"
/// * `new_owner_id`: "receiver.near"
/// * `amount`: the number of tokens to transfer, wrapped in quotes and treated
///   like a string, although the number will be stored as an unsigned integer
///   with 128 bits.
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FtTransferLog {
    pub old_owner_id: String,
    pub new_owner_id: String,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}