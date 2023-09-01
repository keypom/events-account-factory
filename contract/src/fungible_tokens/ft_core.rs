use crate::*;

#[near_bindgen]
impl Contract {
    /// Allows an admin to mint an amount of tokens to a desired account ID.
    /// Useful for dropping tokens to users for things like attending talks
    pub fn ft_mint(&mut self, account_id: AccountId, amount: U128) {
        self.assert_admin();
        self.internal_deposit_mint(&account_id, amount.0);
    }

    /// Allows a user to specify a list of items for a specific vendor to purchase.
    /// This will transfer their tokens to the vendor (assuming they have enough).
    /// Alternatively, if no memo is specified, the user can simply transfer tokens to another account.
    /// The receiving account *must* either be a valid vendor or a sub-account of this contract.
    #[handle_result]
    pub fn ft_transfer(&mut self, receiver_id: AccountId, memo: Option<String>, amount: Option<U128>) -> Result<U128, String> {
        let amount_to_transfer = if let Some(memo) = memo {
            let item_ids: Vec<u64> = serde_json::from_str(&memo).expect("Failed to parse memo");
            let vendor_data = self.data_by_vendor.get(&receiver_id).expect("No vendor found");
    
            // Tally the total price across all the items being purchased
            let mut total_price = 0;
            for id in item_ids.iter() {
                let item = vendor_data.item_by_id.get(id).expect("No item found");
                total_price += item.price.0;
            }

            total_price
        } else {
            // Tested: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=54a4a26cf62b44a178286431fe10e7f4
            require!(receiver_id.to_string().ends_with(env::current_account_id().as_str()), "Invalid receiver ID");
            amount.expect("No amount specified").0
        };

        // Transfer the tokens to the vendor
        let sender_id = env::predecessor_account_id();
        self.internal_transfer(&sender_id, &receiver_id, amount_to_transfer);

        Ok(U128(amount_to_transfer))
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