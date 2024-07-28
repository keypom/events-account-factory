use crate::*;

#[near_bindgen]
impl Contract {
    /// Assert that the caller is the keypom contract
    pub(crate) fn assert_keypom(&self) {
        require!(
            env::predecessor_account_id() == self.keypom_contract,
            "Only Keypom can call this method"
        );
    }
}