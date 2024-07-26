use crate::*;

#[near_bindgen]
impl Contract {
    /// Allows a user to claim an existing drop (if they haven't already)
    pub fn claim_drop(&mut self, drop_id: String, scavenger_id: Option<String>) {
        let drop_data = self.drop_by_id.get(&drop_id).expect("Drop not found");

        let receiver_id = env::predecessor_account_id();
        let mut claimed_drops = self
            .claims_by_account
            .get(&receiver_id)
            .expect("User not registered");

        match drop_data {
            InternalDropData::token(data) => {
                self.handle_claim_token_drop(data, &receiver_id, scavenger_id, &mut claimed_drops);
            }
            InternalDropData::nft(data) => {
                self.handle_claim_nft_drop(data, &receiver_id, scavenger_id, &mut claimed_drops);
            }
        }
        self.claims_by_account
            .insert(&receiver_id, &claimed_drops);
    }

    fn handle_claim_token_drop(
        &mut self,
        data: TokenDropData,
        receiver_id: &AccountId,
        scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<DropId, InternalClaimedDropData>,
    ) {
        match data.scavenger_ids {
            Some(scavenger_ids) => {
                near_sdk::log!("Scavenger IDs: {:?}", &scavenger_ids);
                let mut claimed_drop = claimed_drops.get(&data.id).unwrap_or(InternalClaimedDropData { id: data.id.clone(), scavenger_ids: None });
                let mut claimed_scavenger_ids = claimed_drop.scavenger_ids.unwrap_or(Vec::new());

                near_sdk::log!("Claimed scavenger IDs: {:?}", &claimed_scavenger_ids);
                near_sdk::log!("Scavenger ID: {:?}", &scavenger_id);

                let scav_id = scavenger_id.expect("Scavenger ID is required");
                require!(
                    !claimed_scavenger_ids.contains(&scav_id),
                    "Scavenger item already claimed"
                );
                claimed_scavenger_ids.push(scav_id);

                if scavenger_ids.len() == claimed_scavenger_ids.len() {
                    // All scavenger items claimed, now claim the main reward
                    self.internal_deposit_mint(receiver_id, data.amount.0);
                }

                claimed_drop.scavenger_ids = Some(claimed_scavenger_ids);
                claimed_drops.insert(&data.id, &claimed_drop);
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(
                    claimed_drops.get(&data.id).is_none(),
                    "Drop already claimed"
                );
                claimed_drops.insert(&data.id, &InternalClaimedDropData { id: data.id.clone(), scavenger_ids: None });
                self.internal_deposit_mint(receiver_id, data.amount.0);
            }
        }
    }

    fn handle_claim_nft_drop(
        &mut self,
        data: NFTDropData,
        receiver_id: &AccountId,
        scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<DropId, InternalClaimedDropData>,
    ) {
        match data.scavenger_ids {
            Some(scavenger_ids) => {
                near_sdk::log!("Scavenger IDs: {:?}", &scavenger_ids);
                let mut claimed_drop = claimed_drops.get(&data.id).unwrap_or(InternalClaimedDropData { id: data.id.clone(), scavenger_ids: None });
                let mut claimed_scavenger_ids = claimed_drop.scavenger_ids.unwrap_or(Vec::new());

                near_sdk::log!("Claimed scavenger IDs: {:?}", &claimed_scavenger_ids);
                near_sdk::log!("Scavenger ID: {:?}", &scavenger_id);

                let scav_id = scavenger_id.expect("Scavenger ID is required");
                require!(
                    !claimed_scavenger_ids.contains(&scav_id),
                    "Scavenger item already claimed"
                );
                claimed_scavenger_ids.push(scav_id);
                if scavenger_ids.len() == claimed_scavenger_ids.len() {
                    // TODO: actually mint NFT
                    todo!();
                }
                
                claimed_drop.scavenger_ids = Some(claimed_scavenger_ids);
                claimed_drops.insert(&data.id, &claimed_drop);
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(
                    claimed_drops.get(&data.id).is_none(),
                    "Drop already claimed"
                );
                claimed_drops.insert(&data.id, &InternalClaimedDropData { id: data.id.clone(), scavenger_ids: None });
                // TODO: actually mint NFT
            }
        }
    }
}
