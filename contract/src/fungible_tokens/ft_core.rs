use crate::*;

#[near]
impl Contract {
    /// Allows an admin to mint an amount of tokens to a specified account ID.
    ///
    /// This function is useful for minting tokens to users by an admin for various reasons,
    /// such as rewarding them for attending talks or participating in events.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account ID to which the tokens will be minted.
    /// * `amount` - The amount of tokens to mint, represented as a `U128`.
    ///
    /// # Panics
    ///
    /// Panics if the caller is not an admin.
    pub fn ft_mint(&mut self, account_id: AccountId, amount: U128) {
        self.assert_no_freeze();
        self.assert_admin();

        self.total_transactions += 1;
        self.internal_deposit_ft_mint(
            &account_id,
            NearToken::from_yoctonear(amount.0),
            None,
            false,
        );
    }

    /// Allows a user to transfer tokens to another account or purchase items from a vendor.
    ///
    /// This function facilitates the transfer of tokens between users. If a memo is provided, it specifies
    /// a list of items to purchase from a vendor, and the user's tokens will be transferred to the vendor
    /// accordingly, provided the user has sufficient tokens. If no memo is specified, the user can simply
    /// transfer tokens to another account. The receiving account must either be a valid vendor or a sub-account
    /// of this contract.
    ///
    /// # Arguments
    ///
    /// * `receiver_id` - The account ID of the receiver.
    /// * `memo` - An optional string memo that specifies the items to purchase.
    /// * `amount` - An optional amount of tokens to transfer.
    ///
    /// # Returns
    ///
    /// Returns the amount of tokens transferred, wrapped in a `Result<U128, String>`.
    ///
    /// # Panics
    ///
    /// Panics if the memo is invalid or if the receiver ID is not valid when no memo is provided.
    #[handle_result]
    pub fn ft_transfer(
        &mut self,
        receiver_id: AccountId,
        amount: NearToken,
    ) -> Result<NearToken, String> {
        self.assert_no_freeze();
        // Tested: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=54a4a26cf62b44a178286431fe10e7f4
        require!(
            receiver_id
                .to_string()
                .ends_with(env::current_account_id().as_str()),
            "Invalid receiver ID"
        );

        // Transfer the tokens
        let sender_id = self.caller_id_by_signing_pk();
        self.internal_ft_transfer(&sender_id, &receiver_id, amount, false);

        // Record the transfer transaction
        self.add_transaction(TransactionType::Transfer {
            sender_id: sender_id.clone(),
            receiver_id: receiver_id.clone(),
            amount,
            timestamp: env::block_timestamp(),
        });
        self.total_transactions += 1;

        self.total_tokens_transferred
            .checked_add(amount)
            .expect("Overflow");
        Ok(amount)
    }

    /// Queries for the total amount of tokens currently circulating.
    ///
    /// # Returns
    ///
    /// Returns the total supply of tokens as a `U128`.
    pub fn ft_total_supply(&self) -> NearToken {
        // Return the total supply casted to a U128
        self.ft_total_supply
    }

    /// Queries for the balance of tokens for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account ID for which to query the balance.
    ///
    /// # Returns
    ///
    /// Returns the balance of tokens for the specified account as a `U128`.
    pub fn ft_balance_of(&self, account_id: AccountId) -> NearToken {
        // Return the balance of the account casted to a U128
        self.account_details_by_id
            .get(&account_id)
            .map(|d| d.ft_balance)
            .unwrap_or(NearToken::from_yoctonear(0))
            .into()
    }
}
