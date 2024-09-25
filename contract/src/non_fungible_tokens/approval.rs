use crate::*;
use near_sdk::serde_json::json;
use near_sdk::{assert_one_yocto, Gas, GasWeight, Promise};

#[near]
impl Contract {
    //allow a specific account ID to approve a token on your behalf
    #[payable]
    pub fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        /*
            assert at least one yocto for security reasons - this will cause a redirect to the NEAR wallet.
            The user needs to attach enough to pay for storage on the contract
        */
        assert_at_least_one_yocto();

        //get the token object from the token ID
        let token = self.nft_tokens_by_id.get_mut(&token_id).expect("No token");

        //make sure that the person calling the function is the owner of the token
        assert_eq!(
            &env::predecessor_account_id(),
            &token.owner_id,
            "Predecessor must be the token owner."
        );

        //get the next approval ID if we need a new approval
        let approval_id: u64 = token.next_approval_id;

        //check if the account has been approved already for this token
        let is_new_approval = token
            .approved_account_ids
            //insert returns none if the key was not present.
            .insert(account_id.clone(), approval_id)
            //if the key was not present, .is_none() will return true so it is a new approval.
            .is_none();

        //if it was a new approval, we need to calculate how much storage is being used to add the account.
        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id)
        //if it was not a new approval, we used no storage.
        } else {
            0
        };

        //increment the token's next approval ID by 1
        token.next_approval_id += 1;

        //refund any excess storage attached by the user. If the user didn't attach enough, panic.
        refund_deposit(storage_used);

        //if some message was passed into the function, we initiate a cross contract call on the
        //account we're giving access to.
        if let Some(msg) = msg {
            // Defaulting GAS weight to 1, no attached deposit, and no static GAS to attach.
            Promise::new(account_id)
                .function_call_weight(
                    "nft_on_approve".to_string(),
                    json!({ "token_id": token_id, "owner_id": token.owner_id, "msg": msg })
                        .to_string()
                        .into_bytes(),
                    NearToken::from_yoctonear(0),
                    Gas::from_tgas(0),
                    GasWeight(1),
                )
                .as_return();
        }
    }

    //check if the passed in account has access to approve the token ID
    pub fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        //get the token object from the token_id
        let token = self.nft_tokens_by_id.get(&token_id).expect("No token");

        //get the approval number for the passed in account ID
        let approval = token.approved_account_ids.get(&approved_account_id);

        //if there was some approval ID found for the account ID
        if let Some(approval) = approval {
            //if a specific approval_id was passed into the function
            if let Some(approval_id) = approval_id {
                //return if the approval ID passed in matches the actual approval ID for the account
                approval_id == *approval
                //if there was no approval_id passed into the function, we simply return true
            } else {
                true
            }
            //if there was no approval ID found for the account ID, we simply return false
        } else {
            false
        }
    }

    //revoke a specific account from transferring the token on your behalf
    #[payable]
    pub fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        //assert that the user attached exactly 1 yoctoNEAR for security reasons
        assert_one_yocto();
        //get the token object using the passed in token_id
        let token = self.nft_tokens_by_id.get_mut(&token_id).expect("No token");

        //get the caller of the function and assert that they are the owner of the token
        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &token.owner_id);

        token.approved_account_ids.remove(&account_id);
    }

    //revoke all accounts from transferring the token on your behalf
    #[payable]
    pub fn nft_revoke_all(&mut self, token_id: TokenId) {
        //assert that the caller attached exactly 1 yoctoNEAR for security
        assert_one_yocto();

        //get the token object from the passed in token ID
        let token = self.nft_tokens_by_id.get_mut(&token_id).expect("No token");
        //get the caller and make sure they are the owner of the tokens
        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &token.owner_id);

        token.approved_account_ids.clear();
    }
}
