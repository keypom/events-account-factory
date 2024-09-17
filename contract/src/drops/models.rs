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
    pub image: Option<String>, // For token drops!
}

/// Base struct for external claimed drop data.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DropBase {
    pub scavenger_hunt: Option<Vec<ScavengerHuntData>>,
    pub name: String,
    pub id: String,
    pub num_claimed: u64,
    pub image: Option<String>,
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

/// Represents the internal data for a token drop.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MultichainDropData {
    pub base: DropBase,

    // FOR MPC
    pub chain_id: u64,
    // Receiving NFT contract on external chain
    pub contract_id: String,
    // Arguments that I pass in to the NFT mint function call on external chain
    // **NEEDS TO HAVE BEEN CREATED ON THE NFT CONTRACT BEFORE CALLING CREATE DROP**
    pub series_id: SeriesId,
}

/// Represents the different types of claimed drops to be returned to the frontend.
#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum DropData {
    Token(TokenDropData),
    Multichain(MultichainDropData),
    Nft(NFTDropData),
}

impl DropData {
    pub fn is_nft_drop(&self) -> bool {
        match self {
            DropData::Token(_) => false,
            DropData::Nft(_) => true,
            DropData::Multichain(_) => false,
        }
    }

    pub fn is_token_drop(&self) -> bool {
        match self {
            DropData::Token(_) => true,
            DropData::Nft(_) => false,
            DropData::Multichain(_) => false,
        }
    }

    pub fn is_multichain_drop(&self) -> bool {
        match self {
            DropData::Token(_) => false,
            DropData::Nft(_) => false,
            DropData::Multichain(_) => true,
        }
    }

    /// Returns the scavenger IDs associated with the claimed drop.
    pub fn get_scavenger_ids(&self) -> Option<Vec<String>> {
        match self {
            DropData::Token(data) => data
                .base
                .scavenger_hunt
                .as_ref()
                .map(|h| h.iter().map(|i| i.piece.clone()).collect()),
            DropData::Nft(data) => data
                .base
                .scavenger_hunt
                .as_ref()
                .map(|h| h.iter().map(|i| i.piece.clone()).collect()),
            DropData::Multichain(data) => data
                .base
                .scavenger_hunt
                .as_ref()
                .map(|h| h.iter().map(|i| i.piece.clone()).collect()),
        }
    }

    pub fn get_scavenger_data(&self) -> Option<Vec<ScavengerHuntData>> {
        match self {
            DropData::Token(data) => data.base.scavenger_hunt.clone(),
            DropData::Nft(data) => data.base.scavenger_hunt.clone(),
            DropData::Multichain(data) => data.base.scavenger_hunt.clone(),
        }
    }

    /// Returns the name of the claimed drop.
    pub fn get_name(&self) -> String {
        match self {
            DropData::Token(data) => data.base.name.clone(),
            DropData::Nft(data) => data.base.name.clone(),
            DropData::Multichain(data) => data.base.name.clone(),
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
    Token(Option<Vec<String>>),
    Nft(Option<Vec<String>>),
    Multichain(Option<Vec<String>>),
}

impl ClaimedDropData {
    /// Returns the scavenger IDs associated with the claimed drop.
    pub fn get_found_scavenger_ids(&self) -> Option<Vec<String>> {
        match self {
            ClaimedDropData::Token(token_drop_scavs) => token_drop_scavs.clone(),
            ClaimedDropData::Nft(nft_drop_scavs) => nft_drop_scavs.clone(),
            ClaimedDropData::Multichain(multichain_drop_scavs) => multichain_drop_scavs.clone(),
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
            ClaimedDropData::Token(ref mut token_drop_scavs) => {
                *token_drop_scavs = Some(found_scavs)
            }
            ClaimedDropData::Nft(ref mut nft_drop_scavs) => *nft_drop_scavs = Some(found_scavs),
            ClaimedDropData::Multichain(ref mut multichain_drop_scavs) => {
                *multichain_drop_scavs = Some(found_scavs)
            }
        }
    }
}
