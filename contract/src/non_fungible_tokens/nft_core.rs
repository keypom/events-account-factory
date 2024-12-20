use crate::*;
use near_sdk::serde_json::json;
use near_sdk::{assert_one_yocto, Gas, GasWeight, Promise, PromiseOrValue, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas::from_tgas(10); // 10 TGas = 10^13 gas units
const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas::from_tgas(25); // 25 TGas = 25^13 gas units

#[near]
impl Contract {
    //implementation of the nft_transfer method. This transfers the NFT from the current owner to the receiver.
    #[payable]
    pub fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        //we introduce an approval ID so that people with that approval ID can transfer the token
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        //assert that the user attached exactly 1 yoctoNEAR. This is for security and so that the user will be redirected to the NEAR wallet.
        assert_one_yocto();
        //get the sender to transfer the token from the sender to the receiver
        let sender_id = env::predecessor_account_id();

        //call the internal transfer method and get back the previous token so we can refund the approved account IDs
        self.internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, memo);
    }

    //implementation of the transfer call method. This will transfer the NFT and call a method on the receiver_id contract
    #[payable]
    pub fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        //we introduce an approval ID so that people with that approval ID can transfer the token
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        //assert that the user attached exactly 1 yocto for security reasons.
        assert_one_yocto();

        //get the sender ID
        let sender_id = env::predecessor_account_id();

        //transfer the token and get the previous token object
        let previous_token = self.internal_transfer(
            &sender_id,
            &receiver_id,
            &token_id,
            approval_id,
            memo.clone(),
        );

        //default the authorized_id to none
        let mut authorized_id = None;
        //if the sender isn't the owner of the token, we set the authorized ID equal to the sender.
        if sender_id != previous_token.owner_id {
            authorized_id = Some(sender_id.to_string());
        }

        // Initiating receiver's call and the callback
        // Defaulting GAS weight to 1, no attached deposit, and static GAS equal to the GAS for nft on transfer.

        Promise::new(receiver_id.clone())
                .function_call_weight(
                    "nft_on_transfer".to_string(),
                    json!({ "sender_id": sender_id, "previous_owner_id": previous_token.owner_id, "token_id": token_id, "msg": msg })
                        .to_string()
                        .into_bytes(),
                    NearToken::from_yoctonear(0),
                    GAS_FOR_NFT_ON_TRANSFER,
                    GasWeight(0),
                )
            // We then resolve the promise and call nft_resolve_transfer on our own contract
            .then(
                // Defaulting GAS weight to 1, no attached deposit, and static GAS equal to the GAS for resolve transfer
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_RESOLVE_TRANSFER)
                    .nft_resolve_transfer(
                        authorized_id, // we introduce an authorized ID so that we can log the transfer
                        previous_token.owner_id,
                        receiver_id,
                        token_id,
                        previous_token.approved_account_ids,
                        memo, // we introduce a memo for logging in the events standard
                    ),
            )
            .into()
    }

    //get the information for a specific token ID
    pub fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        //if there is some token ID in the nft_tokens_by_id collection
        if let Some(token) = self.nft_tokens_by_id.get(&token_id) {
            let cur_series = self
                .series_by_id
                .get(&token.series_id)
                .expect("Not a series");
            let mut metadata = cur_series.metadata.clone();

            let split: Vec<&str> = token_id.split(":").collect();
            let edition_number = split[1];
            // If there is a title for the NFT, add the token ID to it.
            if let Some(title) = metadata.title {
                metadata.title = Some(format!("{} - {}", title, edition_number));
            } else {
                // If there is no title, we simply create one based on the series number and edition number
                metadata.title = Some(format!("Series {} : Edition {}", split[0], split[1]));
            }

            //we return the JsonToken (wrapped by Some since we return an option)
            Some(JsonToken {
                series_id: token.series_id,
                token_id,
                owner_id: token.owner_id.clone(),
                metadata,
                approved_account_ids: token.approved_account_ids.clone(),
                royalty: cur_series.royalty.clone(),
            })
        } else {
            //if there wasn't a token ID in the nft_tokens_by_id collection, we return None
            None
        }
    }

    //resolves the cross contract call when calling nft_on_transfer in the nft_transfer_call method
    //returns true if the token was successfully transferred to the receiver_id
    #[private]
    pub fn nft_resolve_transfer(
        &mut self,
        //we introduce an authorized ID for logging the transfer event
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        //we introduce the approval map so we can keep track of what the approvals were before the transfer
        approved_account_ids: HashMap<AccountId, u64>,
        //we introduce a memo for logging the transfer event
        memo: Option<String>,
    ) -> bool {
        // Whether receiver wants to return token back to the sender, based on `nft_on_transfer`
        // call result.
        if let PromiseResult::Successful(value) = env::promise_result(0) {
            //As per the standard, the nft_on_transfer should return whether we should return the token to it's owner or not
            if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
                //if we need don't need to return the token, we simply return true meaning everything went fine
                if !return_token {
                    /*
                        since we've already transferred the token and nft_on_transfer returned false, we don't have to
                        revert the original transfer and thus we can just return true since nothing went wrong.
                    */
                    return true;
                }
            }
        }

        //get the token object if there is some token object
        let mut token = if let Some(token) = self.nft_tokens_by_id.get(&token_id).cloned() {
            if token.owner_id != receiver_id {
                // The token is not owner by the receiver anymore. Can't return it.
                return true;
            }
            token
        //if there isn't a token object, it was burned and so we return true
        } else {
            return true;
        };

        //we remove the token from the receiver
        self.internal_remove_token_from_owner(&receiver_id.clone(), &token_id);
        //we add the token to the original owner
        self.internal_add_token_to_owner(&owner_id, &token_id);

        //we change the token struct's owner to be the original owner
        token.owner_id = owner_id.clone();

        //reset the approved account IDs to what they were before the transfer
        token.approved_account_ids = approved_account_ids;

        //we inset the token back into the nft_tokens_by_id collection
        self.nft_tokens_by_id
            .insert(token_id.clone(), token.clone());

        /*
            We need to log that the NFT was reverted back to the original owner.
            The old_owner_id will be the receiver and the new_owner_id will be the
            original owner of the token since we're reverting the transfer.
        */
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
                old_owner_id: receiver_id.to_string(),
                // The account ID of the new owner of the token.
                new_owner_id: owner_id.to_string(),
                // A vector containing the token IDs as strings.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo,
            }]),
        };

        //we perform the actual logging
        env::log_str(&nft_transfer_log.to_string());

        //return false
        false
    }
}
