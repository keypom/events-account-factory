use crate::*;

/// Represents the different types of claimed drops to be returned to the frontend.
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[near(serializers = [json, borsh])]
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
    pub fn get_scavenger_keys(&self) -> Option<Vec<PublicKey>> {
        match self {
            DropData::Token(data) => data
                .scavenger_hunt
                .as_ref()
                .map(|h| h.iter().map(|i| i.key.clone()).collect()),
            DropData::Nft(data) => data
                .scavenger_hunt
                .as_ref()
                .map(|h| h.iter().map(|i| i.key.clone()).collect()),
            DropData::Multichain(data) => data
                .scavenger_hunt
                .as_ref()
                .map(|h| h.iter().map(|i| i.key.clone()).collect()),
        }
    }

    pub fn get_scavenger_data(&self) -> Option<Vec<ScavengerHuntData>> {
        match self {
            DropData::Token(data) => data.scavenger_hunt.clone(),
            DropData::Nft(data) => data.scavenger_hunt.clone(),
            DropData::Multichain(data) => data.scavenger_hunt.clone(),
        }
    }

    /// Returns the name of the claimed drop.
    pub fn get_name(&self) -> String {
        match self {
            DropData::Token(data) => data.name.clone(),
            DropData::Nft(data) => data.name.clone(),
            DropData::Multichain(data) => data.name.clone(),
        }
    }
}

#[derive(Clone, Debug)]
#[near(serializers = [json, borsh])]
pub struct ScavengerHuntData {
    pub key: PublicKey,
    pub id: u16,
    pub description: String,
}

pub type ScavengerKeys = Option<Vec<PublicKey>>;

#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub struct TokenDropData {
    pub id: String,
    pub key: PublicKey,
    pub name: String,
    pub image: String,
    pub scavenger_hunt: Option<Vec<ScavengerHuntData>>,
    pub num_claimed: u64,

    pub token_amount: U128,
}

/// Represents the internal data for a token drop.
#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub struct NFTDropData {
    pub id: String,
    pub key: PublicKey,
    pub name: String,
    pub image: String,
    pub scavenger_hunt: Option<Vec<ScavengerHuntData>>,
    pub num_claimed: u64,

    pub nft_metadata: TokenMetadata,
    pub nft_series_id: SeriesId,
}

/// Represents the internal data for a token drop.
#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub struct MultichainDropData {
    pub id: String,
    pub key: PublicKey,
    pub name: String,
    pub image: String,
    pub scavenger_hunt: Option<Vec<ScavengerHuntData>>,
    pub num_claimed: u64,

    pub nft_metadata: TokenMetadata,
    pub mc_metadata: MultichainMetadata,
}
