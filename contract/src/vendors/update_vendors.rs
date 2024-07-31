use crate::*;

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

        let mut account_details = self.account_details_by_id.get(&vendor_id).unwrap_or(AccountDetails::new(&vendor_id));
        account_details.vendor_data = Some(vendor_info);
        self.account_details_by_id.insert(&vendor_id, &account_details);
    }

    /// Adds a list of items to a specific vendor's store-front
    pub fn add_item_to_vendor(&mut self, vendor_id: AccountId, items: Vec<ExtVendorItem>) {
        self.assert_vendor();
        
        let mut account_details = self.account_details_by_id.get(&vendor_id).unwrap_or(AccountDetails::new(&vendor_id));
        let mut vendor_info = account_details.vendor_data.expect("User is not a vendor");

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

        account_details.vendor_data = Some(vendor_info);
        self.account_details_by_id.insert(&vendor_id, &account_details);

    }

    /// Update a specific item in a vendor's store-front
    pub fn update_vendor_item(&mut self, vendor_id: AccountId, item_id: u64, new_item: ExtVendorItem) {
        self.assert_vendor();
        
        let mut account_details = self.account_details_by_id.get(&vendor_id).unwrap_or(AccountDetails::new(&vendor_id));
        let mut vendor_info = account_details.vendor_data.expect("User is not a vendor");

        let internal_item = InternalVendorItem {
            id: item_id.to_string(),
            name: new_item.name.clone(),
            image: new_item.image.clone(),
            price: new_item.price.clone(),
            in_stock: new_item.in_stock,
        };
        require!(vendor_info.item_by_id.insert(&item_id, &internal_item).is_some(), "Item doesn't exist");
        account_details.vendor_data = Some(vendor_info);
        self.account_details_by_id.insert(&vendor_id, &account_details);
    }

    pub(crate) fn assert_admin(&self) -> AccountId {
        let caller_id = env::predecessor_account_id();
        let account_details = self.account_details_by_id.get(&caller_id).expect("Unauthorized");
        require!(account_details.account_status.expect("Unauthorized").is_admin(), "Unauthorized");
        caller_id
    }

    pub(crate) fn assert_vendor(&self) -> AccountId {
        let caller_id = env::predecessor_account_id();
        let account_details = self.account_details_by_id.get(&caller_id).expect("Unauthorized");
        require!(account_details.account_status.expect("Unauthorized").is_vendor(), "Unauthorized");
        caller_id
    }

    pub(crate) fn assert_sponsor(&self) -> AccountId {
        let caller_id = env::predecessor_account_id();
        let account_details = self.account_details_by_id.get(&caller_id).expect("Unauthorized");
        require!(account_details.account_status.expect("Unauthorized").is_sponsor(), "Unauthorized");
        caller_id
    }
}