use crate::*;

/// Represents the different types of drops that can be created and claimed.
#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum InternalDropData {
    token(InternalTokenDropData),
    nft(InternalNFTDropData),
}

impl InternalDropData {
    /// Returns the ID of the drop.
    pub fn get_id(&self) -> String {
        match self {
            InternalDropData::token(data) => data.base.base.id.clone(),
            InternalDropData::nft(data) => data.base.base.id.clone(),
        }
    }

    /// Returns the scavenger IDs associated with the drop.
    pub fn get_scavenger_ids(&self) -> Option<Vec<String>> {
        match self {
            InternalDropData::token(data) => data.base.base.scavenger_ids.clone(),
            InternalDropData::nft(data) => data.base.base.scavenger_ids.clone(),
        }
    }

    /// Returns the name of the drop.
    pub fn get_name(&self) -> String {
        match self {
            InternalDropData::token(data) => data.base.base.name.clone(),
            InternalDropData::nft(data) => data.base.base.name.clone(),
        }
    }

    /// Returns the image URL of the drop.
    pub fn get_image(&self) -> String {
        match self {
            InternalDropData::token(data) => data.base.base.image.clone(),
            InternalDropData::nft(data) => data.base.base.image.clone(),
        }
    }
}

/// Base struct for external claimed drop data.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtClaimedDropBase {
    pub id: String,
    pub scavenger_ids: Option<Vec<String>>,
    pub name: String,
    pub image: String,
}

/// Base struct for internal drop data, extends the external claimed drop base with creator ID.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct InternalDropBase {
    pub base: ExtClaimedDropBase,
    pub creator_id: AccountId,
}

/// Represents the internal data for a token drop.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct InternalTokenDropData {
    pub base: InternalDropBase,
    pub amount: U128,
}

/// Represents the internal data for an NFT drop.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct InternalNFTDropData {
    pub base: InternalDropBase,
    pub series_id: String,
}

/// Represents the data for a claimed drop.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InternalClaimedDropData {
    pub id: String,
    pub scavenger_ids: Option<Vec<String>>,
}

/// Represents the different types of claimed drops to be returned to the frontend.
#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum ExtClaimedDropData {
    token(ExtClaimedTokenDropData),
    nft(ExtClaimedNFTDropData),
}

impl ExtClaimedDropData {
    /// Returns the ID of the claimed drop.
    pub fn get_id(&self) -> String {
        match self {
            ExtClaimedDropData::token(data) => data.base.id.clone(),
            ExtClaimedDropData::nft(data) => data.base.id.clone(),
        }
    }

    /// Returns the scavenger IDs associated with the claimed drop.
    pub fn get_scavenger_ids(&self) -> Option<Vec<String>> {
        match self {
            ExtClaimedDropData::token(data) => data.base.scavenger_ids.clone(),
            ExtClaimedDropData::nft(data) => data.base.scavenger_ids.clone(),
        }
    }

    /// Returns the name of the claimed drop.
    pub fn get_name(&self) -> String {
        match self {
            ExtClaimedDropData::token(data) => data.base.name.clone(),
            ExtClaimedDropData::nft(data) => data.base.name.clone(),
        }
    }

    /// Returns the image URL of the claimed drop.
    pub fn get_image(&self) -> String {
        match self {
            ExtClaimedDropData::token(data) => data.base.image.clone(),
            ExtClaimedDropData::nft(data) => data.base.image.clone(),
        }
    }

    /// Converts external drop data to internal drop data with a hard-coded creator ID.
    pub fn convert_to_internal(&self, creator_id: AccountId) -> InternalDropData {
        match self {
            ExtClaimedDropData::token(ext_data) => {
                InternalDropData::token(InternalTokenDropData {
                    base: InternalDropBase {
                        base: ExtClaimedDropBase {
                            id: ext_data.base.id.clone(),
                            scavenger_ids: ext_data.base.scavenger_ids.clone(),
                            name: ext_data.base.name.clone(),
                            image: ext_data.base.image.clone(),
                        },
                        creator_id: creator_id.clone(),
                    },
                    amount: ext_data.amount,
                })
            }
            ExtClaimedDropData::nft(ext_data) => {
                InternalDropData::nft(InternalNFTDropData {
                    base: InternalDropBase {
                        base: ExtClaimedDropBase {
                            id: ext_data.base.id.clone(),
                            scavenger_ids: ext_data.base.scavenger_ids.clone(),
                            name: ext_data.base.name.clone(),
                            image: ext_data.base.image.clone(),
                        },
                        creator_id: creator_id.clone(),
                    },
                    series_id: ext_data.series_id.clone(),
                })
            }
        }
    }
}

/// Represents the data for a claimed token drop.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtClaimedTokenDropData {
    pub base: ExtClaimedDropBase,
    pub amount: U128,
}

/// Represents the data for a claimed NFT drop.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtClaimedNFTDropData {
    pub base: ExtClaimedDropBase,
    pub series_id: String,
}
