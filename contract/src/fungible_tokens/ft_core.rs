use crate::*;

#[near_bindgen]
impl Contract {
    /// Allows a user to specify a list of items for a specific vendor to purchase.
    /// This will transfer their tokens to the vendor (assuming they have enough)
    #[handle_result]
    pub fn purchase_item(&mut self, vendor_id: AccountId, item_ids: Vec<String>) -> Result<Vec<String>, String> {
        let vendor_data = self.data_by_vendor.get(&vendor_id).expect("No vendor found");

        // Tally the total price across all the items being purchased
        let mut total_price = 0;
        for id in item_ids.iter() {
            let item = vendor_data.item_by_id.get(id).expect("No item found");
            total_price += item.price.0;
        }

        // Transfer the tokens to the vendor
        let sender_id = env::predecessor_account_id();
        self.internal_transfer(&sender_id, &vendor_id, total_price);

        Ok(item_ids)
    }

    /// Query for the total amount of tokens currently circulating.
    pub fn ft_total_supply(&self) -> U128 {
        // Return the total supply casted to a U128
        self.total_supply.into()
    }

    /// Query for the balance of tokens for a specific account.
    pub fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        // Return the balance of the account casted to a U128
        self.balance_by_account.get(&account_id).unwrap_or(0).into()
    }
}