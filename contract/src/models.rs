use crate::*;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKeys {
    VendorInfo,
    Admins,
    BalanceByAccount
}

/// For each vendor, there's a store-front and list of items for sale
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VendorInformation {
    /// Info to render on the store-front 
    pub metadata: VendorMetadata,
    /// List of items for sale
    pub items: Vec<VendorItem>
}

/// Represents an asset that is purchasable.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VendorItem {
    pub name: String,
    pub image: String,
    /// Price in $NCON
    pub price: U128,
    /// Is the item currently purchasable?
    pub in_stock: bool,
}


/// Store-front information for a vendor
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VendorMetadata {
    pub name: String,
    pub description: String,
    pub cover_image: String,
}