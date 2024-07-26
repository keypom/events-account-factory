use crate::*;

// Outlines the different types of drops that can be created and claimed
#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum InternalDropData {
    token(TokenDropData),
    nft(NFTDropData),
}

impl InternalDropData {
    pub fn get_id(&self) -> String {
        match self {
            InternalDropData::token(data) => data.id.clone(),
            InternalDropData::nft(data) => data.id.clone(),
        }
    }

    pub fn get_scavenger_ids(&self) -> Option<Vec<String>> {
        match self {
            InternalDropData::token(data) => data.scavenger_ids.clone(),
            InternalDropData::nft(data) => data.scavenger_ids.clone(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            InternalDropData::token(data) => data.name.clone(),
            InternalDropData::nft(data) => data.name.clone(),
        }
    }

    pub fn get_image(&self) -> String {
        match self {
            InternalDropData::token(data) => data.image.clone(),
            InternalDropData::nft(data) => data.image.clone(),
        }
    }
}

// Allows users to claim a set of tokens. If scavenger_ids are set, all the ids need to be claimed
// before the user gets the `amount`
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenDropData {
    pub id: String,
    pub scavenger_ids: Option<Vec<String>>,
    pub amount: U128,

    // Metadata for the drop
    pub name: String,
    pub image: String,
}

// Allows users to claim NFTs. If scavenger_ids are set, all the ids need to be claimed
// before the user gets the NFT
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTDropData {
    pub id: String,
    pub scavenger_ids: Option<Vec<String>>,
    
    // Outlines which NFT this drop corresponds to
    pub series_id: String,

    // Metadata for the drop
    pub name: String,
    pub image: String,
}

// Internal data kept on the contract corresponding to what's been claimed by each user
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InternalClaimedDropData {
    pub id: String,
    pub scavenger_ids: Option<Vec<String>>,
}

// External data returned to the frontends to display what assets have been claimed by each user
#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ExtClaimedDropData {
    token(ExtClaimedTokenDropData),
    nft(ExtClaimedNFTDropData),
}

// Data returned to the frontend to display the progress of claims for a token drop
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtClaimedTokenDropData {
    pub id: String,
    pub scavs_found: Option<Vec<String>>,

    pub amount: U128,

    // Metadata for the drop
    pub name: String,
    pub image: String,
}

// Data returned to the frontend to display the progress of claims for an NFT drop
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtClaimedNFTDropData {
    pub id: String,
    pub scavs_found: Option<Vec<String>>,

     // Outlines which NFT this drop corresponds to
     pub series_id: String,

     // Metadata for the drop
     pub name: String,
     pub image: String,
}