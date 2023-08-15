use near_sdk::{require};

use crate::*;

impl Contract {
    /// Internal method for force getting the balance of an account. If the account doesn't have a balance, panic with a custom message.
    pub(crate) fn internal_unwrap_balance_of(&self, account_id: &AccountId) -> Balance {
        match self.balance_by_account.get(account_id) {
            Some(balance) => balance,
            None => {
                env::panic_str(format!("The account {} is not registered", &account_id).as_str())
            }
        }
    }

    /// Internal method for depositing some amount of FTs into an account and updating the total supply.
    pub(crate) fn internal_deposit_mint(&mut self, account_id: &AccountId, amount: Balance) {
        // Get the current balance of the account. If they're not registered, panic.
        let balance = self.balance_by_account.get(account_id).unwrap_or(0);
        
        // Add the amount to the balance and insert the new balance into the accounts map
        if let Some(new_balance) = balance.checked_add(amount) {
            self.balance_by_account.insert(account_id, &new_balance);
        } else {
            env::panic_str("Balance overflow");
        }
    }

    /// Internal method for depositing some amount of FTs into an account. 
    pub(crate) fn internal_deposit(&mut self, account_id: &AccountId, amount: Balance) {
        // Get the current balance of the account. If they're not registered, panic.
        let balance = self.internal_unwrap_balance_of(account_id);
        
        // Add the amount to the balance and insert the new balance into the accounts map
        if let Some(new_balance) = balance.checked_add(amount) {
            self.balance_by_account.insert(account_id, &new_balance);
        } else {
            env::panic_str("Balance overflow");
        }
    }

    /// Internal method for withdrawing some amount of FTs from an account. 
    pub(crate) fn internal_withdraw(&mut self, account_id: &AccountId, amount: Balance) {
        // Get the current balance of the account. If they're not registered, panic.
        let balance = self.internal_unwrap_balance_of(account_id);
        
        // Decrease the amount from the balance and insert the new balance into the accounts map
        if let Some(new_balance) = balance.checked_sub(amount) {
            self.balance_by_account.insert(account_id, &new_balance);
        } else {
            env::panic_str("The account doesn't have enough balance");
        }
    }

    /// Internal method for performing a transfer of FTs from one account to another.
    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        amount: Balance,
        memo: Option<String>,
    ) {
        // Ensure the sender can't transfer to themselves
        require!(sender_id != receiver_id, "Sender and receiver should be different");
        // Ensure the sender can't transfer 0 tokens
        require!(amount > 0, "The amount should be a positive number");
        
        // Withdraw from the sender and deposit into the receiver
        self.internal_withdraw(sender_id, amount);
        self.internal_deposit(receiver_id, amount);
        
        // Emit a Transfer event
        env::log_str(&EventLog {
            standard: FT_STANDARD_NAME.to_string(),
            version: FT_METADATA_SPEC.to_string(),
            event: EventLogVariant::FtTransfer(FtTransferLog {
                old_owner_id: sender_id.to_string(),
                new_owner_id: receiver_id.to_string(),
                amount: amount.to_string(),
                memo,
            })
        }.to_string());
    }

    /// Internal method for registering an account with the contract.
    pub(crate) fn internal_register_account(&mut self, account_id: &AccountId) {
        if self.balance_by_account.insert(account_id, &0).is_some() {
            env::panic_str("The account is already registered");
        }
    }
}