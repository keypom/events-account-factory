use crate::*;
use near_sdk::CryptoHash;

/// Used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_string(string: &String) -> CryptoHash {
    env::sha256_array(string.as_bytes())
}

#[near_bindgen]
impl Contract {
    /// Adds a vendor to the list of vendors
    pub fn add_vendor(&mut self, vendor_id: AccountId, vendor_metadata: VendorMetadata) {
        self.assert_admin();

        let vendor_info = VendorInformation {
            metadata: vendor_metadata,
            item_by_id: UnorderedMap::new(StorageKeys::VendorItems {
                vendor_id_hash: hash_string(&vendor_id.to_string()),
            })
        };

        require!(self.data_by_vendor.insert(&vendor_id, &vendor_info).is_none(), "Vendor already exists");
    }

    /// Adds a vendor to the list of vendors
    pub fn update_vendor(&mut self, vendor_id: AccountId, vendor_metadata: VendorMetadata) {
        self.assert_admin();

        let mut vendor_info= self.data_by_vendor.get(&vendor_id).expect("No vendor found");
        vendor_info.metadata = vendor_metadata;
        self.data_by_vendor.insert(&vendor_id, &vendor_info);
    }

    /// Adds a list of items to a specific vendor's store-front
    pub fn add_item_to_vendor(&mut self, vendor_id: AccountId, items: Vec<ExtVendorItem>) {
        self.assert_admin_or_vendor(&vendor_id);
        
        let mut vendor_info = self.data_by_vendor.get(&vendor_id).expect("No vendor found");
        let mut next_id = vendor_info.item_by_id.len() as u64;
        for ext_item in items.iter() {
            let internal_item = InternalVendorItem {
                id: next_id.to_string(),
                name: ext_item.name.clone(),
                image: ext_item.image.clone(),
                price: ext_item.price.clone(),
                in_stock: ext_item.in_stock,
            };

            require!(vendor_info.item_by_id.insert(&next_id, &internal_item).is_none(), "Item already exists");
            next_id += 1;
        }

        self.data_by_vendor.insert(&vendor_id, &vendor_info);
    }

    /// Update a specific item in a vendor's store-front
    pub fn update_vendor_item(&mut self, vendor_id: AccountId, item_id: u64, new_item: ExtVendorItem) {
        self.assert_admin_or_vendor(&vendor_id);
        
        let mut vendor_info = self.data_by_vendor.get(&vendor_id).expect("No vendor found");
        let internal_item = InternalVendorItem {
            id: item_id.to_string(),
            name: new_item.name.clone(),
            image: new_item.image.clone(),
            price: new_item.price.clone(),
            in_stock: new_item.in_stock,
        };
        require!(vendor_info.item_by_id.insert(&item_id, &internal_item).is_some(), "Item doesn't exist");
        self.data_by_vendor.insert(&vendor_id, &vendor_info);
    }

    pub(crate) fn assert_admin(&self) {
        require!(self.admin_accounts.contains(&env::predecessor_account_id()), "Unauthorized");
    }

    pub(crate) fn assert_admin_or_vendor(&self, vendor_id: &AccountId) {
        // If the caller isn't the vendor, ensure they're an admin
        if env::predecessor_account_id() != vendor_id.clone() {
            self.assert_admin();
        }
    }
}