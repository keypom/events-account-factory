use std::mem::size_of;

use near_sdk::Promise;

use crate::*;

//convert the royalty percentage and amount to pay into a payout (U128)
pub(crate) fn royalty_to_payout(royalty_percentage: u32, amount_to_pay: NearToken) -> U128 {
    U128(royalty_percentage as u128 * amount_to_pay.as_yoctonear() / 10_000u128)
}

//Assert that the user has attached at least 1 yoctoNEAR (for security reasons and to pay for storage)
pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit().as_yoctonear() >= 1,
        "Requires attached deposit of at least 1 yoctoNEAR",
    )
}

//refund the initial deposit based on the amount of storage that was used up
pub(crate) fn refund_deposit(storage_used: u64) {
    // Get how much it would cost to store the information, ensuring the value exists
    let required_cost = env::storage_byte_cost()
        .checked_mul(storage_used as u128)
        .expect("Overflow in calculating required cost");

    // Get the attached deposit
    let attached_deposit = env::attached_deposit();

    // Make sure that the attached deposit is greater than or equal to the required cost
    assert!(
        attached_deposit >= required_cost,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost
    );

    // Get the refund amount from the attached deposit - required cost
    let refund = attached_deposit
        .checked_sub(required_cost)
        .expect("Unexpected calculation overflow");

    // If the refund is greater than 1 yoctoNEAR, we refund the predecessor that amount
    if refund.as_yoctonear() > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

//calculate how many bytes the account ID is taking up
pub fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
    // The extra 4 bytes are coming from Borsh serialization to store the length of the string.
    account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}

impl Contract {
    // Add a token to the set of tokens an owner has
    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) -> u16 {
        // Use the entry API to get or initialize the IterableSet
        let token_set = self
            .nft_tokens_per_owner
            .entry(account_id.clone()) // Clone the account_id since entry takes ownership
            .or_insert_with(|| {
                IterableSet::new(StorageKeys::TokensForOwnerInner {
                    account_id_hash: hash_string(&account_id.to_string()),
                })
            });

        // Insert the token ID into the set
        token_set.insert(token_id.to_string());

        // Return the number of tokens the user owns
        token_set.len() as u16
    }

    //remove a token from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_remove_token_from_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        //we get the set of tokens that the owner has
        let token_set = self
            .nft_tokens_per_owner
            .get_mut(account_id)
            .expect("Trying to send NFTs to a non registered account");

        //we remove the the token_id from the set of tokens
        token_set.remove(token_id);
    }

    //transfers the NFT to the receiver_id (internal method and can't be called directly via CLI).
    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
        //we introduce an approval ID so that people with that approval ID can transfer the token
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> Token {
        //get the token object by passing in the token_id
        let token = self
            .nft_tokens_by_id
            .get(token_id)
            .cloned()
            .expect("No token");

        //if the sender doesn't equal the owner, we check if the sender is in the approval list
        if sender_id != &token.owner_id {
            //if the token's approved account IDs doesn't contain the sender, we panic
            if !token.approved_account_ids.contains_key(sender_id) {
                env::panic_str("Unauthorized");
            }

            // If they included an approval_id, check if the sender's actual approval_id is the same as the one included
            if let Some(enforced_approval_id) = approval_id {
                //get the actual approval ID
                let actual_approval_id = token
                    .approved_account_ids
                    .get(sender_id)
                    //if the sender isn't in the map, we panic
                    .expect("Sender is not approved account");

                //make sure that the actual approval ID is the same as the one provided
                assert_eq!(
                    actual_approval_id, &enforced_approval_id,
                    "The actual approval_id {} is different from the given approval_id {}",
                    actual_approval_id, enforced_approval_id,
                );
            }
        }

        //we make sure that the sender isn't sending the token to themselves
        assert_ne!(
            &token.owner_id, receiver_id,
            "The token owner and the receiver should be different"
        );

        //we remove the token from it's current owner's set
        self.internal_remove_token_from_owner(&token.owner_id, token_id);
        //we then add the token to the receiver_id's set
        self.internal_add_token_to_owner(receiver_id, token_id);

        //we create a new token struct
        let new_token = Token {
            series_id: token.series_id,
            owner_id: receiver_id.clone(),
            //reset the approval account IDs
            approved_account_ids: Default::default(),
            next_approval_id: token.next_approval_id,
        };
        //insert that new token into the nft_tokens_by_id, replacing the old entry
        self.nft_tokens_by_id
            .insert(token_id.to_string(), new_token);

        //if there was some memo attached, we log it.
        if let Some(memo) = memo.as_ref() {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }

        // Default the authorized ID to be None for the logs.
        let mut authorized_id = None;
        //if the approval ID was provided, set the authorized ID equal to the sender
        if approval_id.is_some() {
            authorized_id = Some(sender_id.to_string());
        }

        // Construct the transfer log as per the events standard.
        let nft_transfer_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                // The optional authorized account ID to transfer the token on behalf of the old owner.
                authorized_id,
                // The old owner's account ID.
                old_owner_id: token.owner_id.to_string(),
                // The account ID of the new owner of the token.
                new_owner_id: receiver_id.to_string(),
                // A vector containing the token IDs as strings.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_transfer_log.to_string());

        //return the previous token object that was transferred.
        token.clone()
    }
}
