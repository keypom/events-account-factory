use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ScavengerHuntData {
    pub piece: String,
    pub description: String,
}

/// Base struct for external claimed drop data.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtDropBase {
    pub scavenger_hunt: Option<Vec<ScavengerHuntData>>,
    pub name: String,
    pub image: String,
}

/// Base struct for external claimed drop data.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DropBase {
    pub scavenger_hunt: Option<Vec<ScavengerHuntData>>,
    pub name: String,
    pub image: String,
    pub id: String,
    pub num_claimed: u64,
}

/// Represents the internal data for a token drop.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenDropData {
    pub base: DropBase,
    pub amount: U128,
}

/// Represents the internal data for a token drop.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTDropData {
    pub base: DropBase,
    pub series_id: SeriesId,
}

/// Represents the different types of claimed drops to be returned to the frontend.
#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum DropData {
    token(TokenDropData),
    nft(NFTDropData),
}

impl DropData {
    pub fn is_nft_drop(&self) -> bool {
        match self {
            DropData::token(_) => false,
            DropData::nft(_) => true,
        }
    }

    pub fn is_token_drop(&self) -> bool {
        match self {
            DropData::token(_) => true,
            DropData::nft(_) => false,
        }
    }

    /// Returns the scavenger IDs associated with the claimed drop.
    pub fn get_scavenger_ids(&self) -> Option<Vec<String>> {
        match self {
            DropData::token(data) => data
                .base
                .scavenger_hunt
                .as_ref()
                .map(|h| h.iter().map(|i| i.piece.clone()).collect()),
            DropData::nft(data) => data
                .base
                .scavenger_hunt
                .as_ref()
                .map(|h| h.iter().map(|i| i.piece.clone()).collect()),
        }
    }

    /// Returns the name of the claimed drop.
    pub fn get_name(&self) -> String {
        match self {
            DropData::token(data) => data.base.name.clone(),
            DropData::nft(data) => data.base.name.clone(),
        }
    }

    /// Returns the image URL of the claimed drop.
    pub fn get_image(&self) -> String {
        match self {
            DropData::token(data) => data.base.image.clone(),
            DropData::nft(data) => data.base.image.clone(),
        }
    }
}

/// Represents what the user has claimed for a specific drop. If scavenger IDs is none, the drop contains no scavengers
/// If scavengers is Some, the drop needs X amount of scavenger Ids to be found before the reward is allocated
/// On the frontend, query for drop data to see how many are needed and cross reference with this data structure to see
/// how many more IDs are left to be found
/// This is done to optimize the contract and not require duplicate data to be stored
#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum ClaimedDropData {
    token(Option<Vec<String>>),
    nft(Option<Vec<String>>),
}

impl ClaimedDropData {
    /// Returns the scavenger IDs associated with the claimed drop.
    pub fn get_found_scavenger_ids(&self) -> Option<Vec<String>> {
        match self {
            ClaimedDropData::token(token_drop_scavs) => token_drop_scavs.clone(),
            ClaimedDropData::nft(nft_drop_scavs) => nft_drop_scavs.clone(),
        }
    }

    /// Adds a scavenger ID to the list of found scavenger IDs.
    ///
    /// # Arguments
    ///
    /// * `scavenger_id` - The scavenger ID to be added.
    ///
    /// # Panics
    ///
    /// Panics if the scavenger ID has already been claimed.
    pub fn add_scavenger_id(&mut self, scavenger_id: String) {
        let mut found_scavs = self.get_found_scavenger_ids().unwrap_or_default();
        require!(
            !found_scavs.contains(&scavenger_id),
            "Scavenger item already claimed"
        );
        found_scavs.push(scavenger_id);

        match self {
            ClaimedDropData::token(ref mut token_drop_scavs) => {
                *token_drop_scavs = Some(found_scavs)
            }
            ClaimedDropData::nft(ref mut nft_drop_scavs) => *nft_drop_scavs = Some(found_scavs),
        }
    }
}
