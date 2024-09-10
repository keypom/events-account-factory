use crate::*;

/// Used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub fn hash_string(string: &String) -> CryptoHash {
    env::sha256_array(string.as_bytes())
}

#[near_bindgen]
impl Contract {
    pub(crate) fn assert_admin(&self) -> AccountId {
        let caller_id = env::predecessor_account_id();
        let account_details = self
            .account_details_by_id
            .get(&caller_id)
            .expect("No account details found");
        require!(
            account_details
                .account_status
                .expect("No account status found")
                .is_admin(),
            "Unauthorized"
        );
        caller_id
    }

    pub(crate) fn assert_vendor(&self) -> AccountId {
        let caller_id = self.caller_id_by_signing_pk();
        let account_details = self
            .account_details_by_id
            .get(&caller_id)
            .expect("No account details found");
        require!(
            account_details
                .account_status
                .expect("No account status found")
                .is_vendor(),
            "Unauthorized"
        );
        caller_id
    }

    pub(crate) fn assert_sponsor(&self) -> AccountId {
        let caller_id = self.caller_id_by_signing_pk();
        let account_details = self
            .account_details_by_id
            .get(&caller_id)
            .expect("No account details found");
        require!(
            account_details
                .account_status
                .expect("No account status found")
                .is_sponsor(),
            "Unauthorized"
        );
        caller_id
    }

    pub(crate) fn assert_data_setter(&self) -> AccountId {
        let caller_id = self.caller_id_by_signing_pk();
        near_sdk::env::log_str(&format!("caller_id: {}", caller_id));
        let account_details = self
            .account_details_by_id
            .get(&caller_id)
            .expect("No account details found");
        require!(
            account_details
                .account_status
                .expect("No account status found")
                .is_data_sponsor(),
            "Unauthorized"
        );
        caller_id
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
            .map(|t| t.account_id)
            .unwrap_or(Some(env::predecessor_account_id()))
            .expect("Account has not been scanned in yet")
    }
}
