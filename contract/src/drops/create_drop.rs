use crate::*;

#[near_bindgen]
impl Contract {
    /// Allows an admin to create a drop so people can scan a QR code and get the amount of tokens
    pub fn create_drop(&mut self, drop: InternalDropData) {
        self.assert_sponsor();
        
        let drop_creator = env::predecessor_account_id();

        let drop_id = drop.get_id();
        self.drop_by_id.insert(&drop_id, &drop);
    }

    /// Allows an admin to create a drop so people can scan a QR code and get the amount of tokens
    pub fn create_drop_batch(&mut self, drops: Vec<InternalDropData>) {
        self.assert_admin();
        for drop in drops {
            let drop_id = drop.get_id();
            self.drop_by_id.insert(&drop_id, &drop);
        }
    }
}
