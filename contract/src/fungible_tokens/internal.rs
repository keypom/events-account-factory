use crate::*;
use near_sdk::require;

impl Contract {
    /// Internal method for depositing some amount of FTs into an account and updating the total supply.
    pub(crate) fn internal_deposit_ft_mint(
        &mut self,
        account_id: &AccountId,
        amount: Balance,
        drop_id: Option<String>,
        add_to_leaderboard: bool,
    ) {
        // Get the current balance of the account. If they're not registered, panic.
        let mut account_details = self
            .account_details_by_id
            .get(account_id)
            .expect("Receiver not found in map");
        let balance = account_details.ft_balance;

        if add_to_leaderboard {
            let mut tokens_collected = account_details.tokens_collected.0;
            tokens_collected += amount;
            self.update_token_leaderboard(account_id.clone(), tokens_collected);
            account_details.tokens_collected.0 = tokens_collected;
        }

        // Add the amount to the balance and insert the new balance into the accounts map
        if let Some(new_balance) = balance.checked_add(amount) {
            account_details.ft_balance = new_balance;
            self.account_details_by_id
                .insert(account_id, &account_details);
        } else {
            env::panic_str("Balance overflow");
        }

        // Increment the total supply and log a mint event
        self.ft_total_supply += amount;
        self.total_tokens_transferred += amount;
        env::log_str(
            &EventLog {
                standard: FT_STANDARD_NAME.to_string(),
                version: FT_METADATA_SPEC.to_string(),
                event: EventLogVariant::FtMint(FtMintLog {
                    owner_id: account_id.to_string(),
                    amount: amount.to_string(),
                    memo: None,
                }),
            }
            .to_string(),
        );
        env::log_str(
            &EventLog {
                standard: KEYPOM_STANDARD_NAME.to_string(),
                version: KEYPOM_CONFERENCE_METADATA_SPEC.to_string(),
                event: EventLogVariant::KeypomTokenMint(KeypomTokenMintLog {
                    drop_id,
                    receiver_id: account_id.to_string(),
                    amount: U128(amount),
                    new_balance: self.ft_balance_of(account_id.clone()),
                }),
            }
            .to_string(),
        );
    }

    /// Internal method for depositing some amount of FTs into an account.
    pub(crate) fn internal_ft_deposit(
        &mut self,
        account_id: &AccountId,
        amount: Balance,
        add_to_leaderboard: bool,
    ) {
        // Get the current balance of the account. If they're not registered, panic.
        let mut account_details = self
            .account_details_by_id
            .get(account_id)
            .expect("Receiver not found in map");
        let balance = account_details.ft_balance;

        if add_to_leaderboard {
            let mut tokens_collected = account_details.tokens_collected.0;
            tokens_collected += amount;
            self.update_token_leaderboard(account_id.clone(), tokens_collected);
            account_details.tokens_collected.0 = tokens_collected;
        }

        // Add the amount to the balance and insert the new balance into the accounts map
        if let Some(new_balance) = balance.checked_add(amount) {
            account_details.ft_balance = new_balance;
            self.account_details_by_id
                .insert(account_id, &account_details);
        } else {
            env::panic_str("Balance overflow");
        }
    }

    /// Internal method for withdrawing some amount of FTs from an account.
    pub(crate) fn internal_ft_withdraw(&mut self, account_id: &AccountId, amount: Balance) {
        // Get the current balance of the account. If they're not registered, panic.
        let mut account_details = self
            .account_details_by_id
            .get(account_id)
            .expect("Receiver not found in map");
        let balance = account_details.ft_balance;

        // Decrease the amount from the balance and insert the new balance into the accounts map
        if let Some(new_balance) = balance.checked_sub(amount) {
            account_details.ft_balance = new_balance;
            self.account_details_by_id
                .insert(account_id, &account_details);
        } else {
            env::panic_str("The account doesn't have enough balance");
        }
    }

    /// Internal method for performing a transfer of FTs from one account to another.
    pub(crate) fn internal_ft_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        amount: Balance,
        add_to_leaderboard: bool,
    ) {
        // Ensure the sender can't transfer to themselves
        require!(
            sender_id != receiver_id,
            "Sender and receiver should be different"
        );
        // Ensure the sender can't transfer 0 tokens
        require!(amount > 0, "The amount should be a positive number");

        // Withdraw from the sender and deposit into the receiver
        self.internal_ft_withdraw(sender_id, amount);
        self.internal_ft_deposit(receiver_id, amount, add_to_leaderboard);

        self.total_tokens_transferred += amount;
        // Emit a Transfer event
        env::log_str(
            &EventLog {
                standard: FT_STANDARD_NAME.to_string(),
                version: FT_METADATA_SPEC.to_string(),
                event: EventLogVariant::FtTransfer(FtTransferLog {
                    old_owner_id: sender_id.to_string(),
                    new_owner_id: receiver_id.to_string(),
                    amount: amount.to_string(),
                    memo: None,
                }),
            }
            .to_string(),
        );

        // Emit the Keypom transfer event
        env::log_str(
            &EventLog {
                standard: KEYPOM_STANDARD_NAME.to_string(),
                version: KEYPOM_CONFERENCE_METADATA_SPEC.to_string(),
                event: EventLogVariant::KeypomTokenTransfer(KeypomTokenTransferLog {
                    sender_id: sender_id.to_string(),
                    receiver_id: receiver_id.to_string(),
                    amount: amount.to_string(),
                    new_sender_balance: self.ft_balance_of(sender_id.clone()),
                    new_receiver_balance: self.ft_balance_of(receiver_id.clone()),
                }),
            }
            .to_string(),
        );
    }
}
