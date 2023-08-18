use crate::*;

#[near_bindgen]
impl Contract {
    /// Query for the metadata associated with a vendor
    pub fn get_vendor_metadata(&self, vendor_id: AccountId) -> VendorMetadata {
        self.data_by_vendor.get(&vendor_id).expect("No vendor found").metadata
    }

    /// Paginate through the items for a specific vendor
    pub fn get_items_for_vendor(&self, vendor_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<InternalVendorItem> {
        let vendor_data = self.data_by_vendor.get(&vendor_id).expect("No vendor found");
        let start = u128::from(from_index.unwrap_or(U128(0)));

        vendor_data.item_by_id.values()
            .skip(start as usize) 
            .take(limit.unwrap_or(50) as usize) 
            .collect()
    }

    /// Query for the information for a specific vendor's item
    pub fn get_item_information(&self, vendor_id: AccountId, item_id: u64) -> InternalVendorItem {
        let vendor_data = self.data_by_vendor.get(&vendor_id).expect("No vendor found");
        vendor_data.item_by_id.get(&item_id).expect("No item found")
    }
}