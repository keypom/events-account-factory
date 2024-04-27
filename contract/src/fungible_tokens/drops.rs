use std::collections::HashMap;

use crate::*;

#[near_bindgen]
impl Contract {
    /// Allows an admin to create a drop so people can scan a QR code and get the amount of tokens
    pub fn create_drop(&mut self, drop_data: InternalDropData) {
        self.assert_admin();
        let drop_id = drop_data.get_id();
        self.drop_by_id.insert(&drop_id, &drop_data);
    }

    /// Allows a user to claim an existing drop (if they haven't already)
    pub fn claim_drop(&mut self, drop_id: String, scavenger_id: Option<String>) {
        let drop_data = self.drop_by_id.get(&drop_id).expect("Drop not found");

        let receiver_id = env::predecessor_account_id();
        let mut claimed_drops = self
            .drops_claimed_by_account
            .get(&receiver_id)
            .expect("User not registered");

        match drop_data {
            InternalDropData::token(data) => {
                self.handle_token_drop(data, &receiver_id, scavenger_id, &mut claimed_drops);
            }
            InternalDropData::nft(data) => {
                self.handle_nft_drop(data, &receiver_id, scavenger_id, &mut claimed_drops);
            }
        }
        self.drops_claimed_by_account
            .insert(&receiver_id, &claimed_drops);
    }

    fn handle_token_drop(
        &mut self,
        data: TokenDropData,
        receiver_id: &AccountId,
        scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<String, Vec<String>>,
    ) {
        match data.scavenger_ids {
            Some(scavenger_ids) => {
                near_sdk::log!("Scavenger IDs: {:?}", &scavenger_ids);
                let mut claimed_scavenger_ids = claimed_drops.get(&data.id).unwrap_or(Vec::new());
                near_sdk::log!("Claimed scavenger IDs: {:?}", &claimed_scavenger_ids);
                near_sdk::log!("Scavenger ID: {:?}", &scavenger_id);

                let scav_id = scavenger_id.expect("Scavenger ID is required");
                require!(
                    !claimed_scavenger_ids.contains(&scav_id),
                    "Scavenger item already claimed"
                );
                claimed_scavenger_ids.push(scav_id);
                claimed_drops.insert(&data.id, &claimed_scavenger_ids);
                if scavenger_ids.len() == claimed_scavenger_ids.len() {
                    // All scavenger items claimed, now claim the main reward
                    self.internal_deposit_mint(receiver_id, data.amount.0);
                }
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(
                    claimed_drops.get(&data.id).is_none(),
                    "Drop already claimed"
                );
                claimed_drops.insert(&data.id, &vec![data.id.clone()]);
                self.internal_deposit_mint(receiver_id, data.amount.0);
            }
        }
    }

    fn handle_nft_drop(
        &mut self,
        data: NFTDropData,
        receiver_id: &AccountId,
        scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<String, Vec<String>>,
    ) {
    }

    /// Query for the total amount of tokens currently circulating.
    pub fn get_drop_information(&self, drop_id: String) -> Option<InternalDropData> {
        self.drop_by_id.get(&drop_id)
    }

    pub fn claims_for_account(&self, account_id: AccountId, drop_id: String) -> Vec<String> {
        self.drops_claimed_by_account
            .get(&account_id)
            .expect("Account not registered")
            .get(&drop_id)
            .unwrap_or(Vec::new())
    }

    pub fn get_scavengers_for_account(
        &self,
        account_id: AccountId,
    ) -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();

        if let Some(claimed_drops) = self.drops_claimed_by_account.get(&account_id) {
            for (drop_id, claimed) in claimed_drops.iter() {
                if let Some(drop_data) = self.drop_by_id.get(&drop_id) {
                    if let Some(scavenger_ids) = drop_data.get_scavenger_ids() {
                        result.insert(drop_id, claimed);
                    }
                }
            }
        } else {
            panic!("Account not registered");
        }

        result
    }
}
