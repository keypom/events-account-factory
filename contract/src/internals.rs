use crate::*;

/// Used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub fn hash_string(string: &String) -> CryptoHash {
    env::sha256_array(string.as_bytes())
}

#[near]
impl Contract {
    pub(crate) fn assert_role<F>(&self, role_checker: F, role_name: &str) -> AccountId
    where
        F: Fn(&AccountStatus) -> bool,
    {
        let caller_id = self.caller_id_by_signing_pk();
        let account_details = self
            .account_details_by_id
            .get(&caller_id)
            .expect("No account details found");

        // Borrow the account status instead of moving it
        let account_status = account_details
            .account_status
            .as_ref()
            .expect("No account status found");

        require!(
            role_checker(account_status),
            &format!("Unauthorized: Not a {}", role_name)
        );

        caller_id
    }

    pub(crate) fn assert_admin(&self) -> AccountId {
        self.assert_role(AccountStatus::is_admin, "admin")
    }

    pub(crate) fn assert_sponsor(&self) -> AccountId {
        self.assert_role(AccountStatus::is_sponsor, "sponsor")
    }

    pub(crate) fn assert_data_setter(&self) -> AccountId {
        self.assert_role(AccountStatus::is_data_sponsor, "data setter")
    }

    pub(crate) fn assert_no_freeze(&self) {
        require!(
            !self.is_contract_frozen,
            "The conference is over, only NFT assets can be transacted with"
        );
    }

    /// Retrieves the account ID associated with the public key of the caller.
    ///
    /// This function maps the public key of the signer (caller) to the corresponding account ID.
    ///
    /// # Returns
    ///
    /// Returns the `AccountId` associated with the signer's public key.
    ///
    /// # Panics
    ///
    /// Panics if no account ID is found for the given public key.
    pub(crate) fn caller_id_by_signing_pk(&self) -> AccountId {
        self.attendee_ticket_by_pk
            .get(&env::signer_account_pk())
            .and_then(|t| t.account_id.clone()) // Clone the account_id to move it
            .unwrap_or_else(|| env::predecessor_account_id()) // Use the predecessor account ID if not found
    }
}
