use near_sdk::{Gas, ext_contract, PromiseOrValue, assert_one_yocto, PromiseResult};

use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        // Assert that the user attached exactly 1 yoctoNEAR. This is for security and so that the user will be required to sign with a FAK.
        assert_one_yocto();
        // The sender is the user who called the method
        let sender_id = env::predecessor_account_id();
        // How many tokens the user wants to withdraw
        let amount: Balance = amount.into();
        // Transfer the tokens
        self.internal_transfer(&sender_id, &receiver_id, amount, memo);
    }

    fn ft_total_supply(&self) -> U128 {
        // Return the total supply casted to a U128
        self.total_supply.into()
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        // Return the balance of the account casted to a U128
        self.balance_by_account.get(&account_id).unwrap_or(0).into()
    }

    // Finalize an `ft_transfer_call` chain of cross-contract calls.
    //
    // The `ft_transfer_call` process:
    //
    // 1. Sender calls `ft_transfer_call` on FT contract
    // 2. FT contract transfers `amount` tokens from sender to receiver
    // 3. FT contract calls `ft_on_transfer` on receiver contract
    // 4+. [receiver contract may make other cross-contract calls]
    // N. FT contract resolves promise chain with `ft_resolve_transfer`, and may
    //    refund sender some or all of original `amount`
    //
    // Requirements:
    // * Contract MUST forbid calls to this function by any account except self
    // * If promise chain failed, contract MUST revert token transfer
    // * If promise chain resolves with a non-zero amount given as a string,
    //   contract MUST return this amount of tokens to `sender_id`
    //
    // Arguments:
    // * `sender_id`: the sender of `ft_transfer_call`
    // * `receiver_id`: the `receiver_id` argument given to `ft_transfer_call`
    // * `amount`: the `amount` argument given to `ft_transfer_call`
    //
    // Returns a string representing a string version of an unsigned 128-bit
    // integer of how many total tokens were spent by sender_id. Example: if sender
    // calls `ft_transfer_call({ "amount": "100" })`, but `receiver_id` only uses
    // 80, `ft_on_transfer` will resolve with `"20"`, and `ft_resolve_transfer`
    // will return `"80"`.
    #[private]
    pub fn ft_resolve_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128 {
        let amount: Balance = amount.into();

        // Get the unused amount from the `ft_on_transfer` call result.
        let unused_amount = match env::promise_result(0) {
            PromiseResult::NotReady => env::abort(),
            // If the promise was successful, get the return value and cast it to a U128.
            PromiseResult::Successful(value) => {
                // If we can properly parse the value, the unused amount is equal to whatever is smaller - the unused amount or the original amount (to prevent malicious contracts)
                if let Ok(unused_amount) = near_sdk::serde_json::from_slice::<U128>(&value) {
                    std::cmp::min(amount, unused_amount.0)
                // If we can't properly parse the value, the original amount is returned.
                } else {
                    amount
                }
            }
            // If the promise wasn't successful, return the original amount.
            PromiseResult::Failed => amount,
        };

        // If there is some unused amount, we should refund the sender
        if unused_amount > 0 {
            // Get the receiver's balance. We can only refund the sender if the receiver has enough balance.
            let receiver_balance = self.balance_by_account.get(&receiver_id).unwrap_or(0);
            if receiver_balance > 0 {
                // The amount to refund is the smaller of the unused amount and the receiver's balance as we can only refund up to what the receiver currently has.
                let refund_amount = std::cmp::min(receiver_balance, unused_amount);
                
                // Refund the sender for the unused amount.
                self.internal_transfer(&receiver_id, &sender_id, refund_amount, Some("Refund".to_string()));
                
                // Return what was actually used (the amount sent - refund)
                let used_amount = amount
                    .checked_sub(refund_amount)
                    .unwrap_or_else(|| env::panic_str("Total supply overflow"));
                return used_amount.into();
            }
        }

        // If the unused amount is 0, return the original amount.
        amount.into()
    }
}