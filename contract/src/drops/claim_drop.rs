use crate::*;

#[near_bindgen]
impl Contract {
    /// Allows a user to claim an existing drop (if they haven't already).
    ///
    /// # Arguments
    ///
    /// * `drop_id` - The ID of the drop to be claimed.
    /// * `scavenger_id` - Optional scavenger ID to claim.
    ///
    /// # Panics
    ///
    /// Panics if the drop is not found or if the user is not registered.
    pub fn claim_drop(&mut self, drop_id: String, scavenger_id: Option<String>) {
        let mut drop_data = self.drop_by_id.get(&drop_id).expect("Drop not found");
    
        let receiver_id = self.caller_id_by_signing_pk();
        let old_account_details = self
            .account_details_by_id
            .get(&receiver_id)
            .expect("Receiver not registered...");
        let mut claimed_drops = old_account_details.drops_claimed;
    
        // Match on the DropData enum to handle token and NFT drops
        match drop_data {
            DropData::token(ref mut data) => {
                // Handle claiming a token drop
                self.handle_claim_token_drop(
                    data,
                    &drop_id,
                    &receiver_id,
                    scavenger_id,
                    &mut claimed_drops,
                );
                // Increment the number of claims for this drop
                data.base.num_claimed += 1;
            }
            DropData::nft(ref mut data) => {
                // Handle claiming an NFT drop
                self.handle_claim_nft_drop(
                    data,
                    &drop_id,
                    &receiver_id,
                    scavenger_id,
                    &mut claimed_drops,
                );
                // Increment the number of claims for this drop
                data.base.num_claimed += 1;
            }
        }
    
        // Save the updated drop_data back into the drop_by_id map
        self.drop_by_id.insert(&drop_id, &drop_data);
    
        let mut new_account_details = self
            .account_details_by_id
            .get(&receiver_id)
            .expect("Receiver not registered...");
        new_account_details.drops_claimed = claimed_drops;
        self.account_details_by_id
            .insert(&receiver_id, &new_account_details);
    }

    /// Handles the claim process for a token drop.
    ///
    /// # Arguments
    ///
    /// * `data` - The internal token drop data.
    /// * `receiver_id` - The ID of the receiver claiming the drop.
    /// * `scavenger_id` - Optional scavenger ID to claim.
    /// * `claimed_drops` - The map of claimed drops for the receiver.
    fn handle_claim_token_drop(
        &mut self,
        data: &TokenDropData,
        drop_id: &DropId,
        receiver_id: &AccountId,
        found_scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<DropId, ClaimedDropData>,
    ) {
        match data.base.scavenger_hunt.clone() {
            Some(required_scavenger_ids) => {
                near_sdk::log!("Required Scavenger IDs: {:?}", &required_scavenger_ids);
                let found_scavenger_id =
                    found_scavenger_id.expect("This drop requires a scavenger ID");

                let mut claimed_drop = claimed_drops
                    .get(drop_id)
                    .unwrap_or(ClaimedDropData::token(Some(Vec::new())));
                claimed_drop.add_scavenger_id(found_scavenger_id);

                let found_scavenger_ids =
                    claimed_drop.get_found_scavenger_ids().unwrap_or_default();

                if found_scavenger_ids.len() == required_scavenger_ids.len() {
                    // All scavenger items claimed, now claim the main reward
                    self.internal_deposit_ft_transfer(data, drop_id, receiver_id);
                }

                claimed_drops.insert(drop_id, &claimed_drop);
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(claimed_drops.get(drop_id).is_none(), "Drop already claimed");
                claimed_drops.insert(drop_id, &ClaimedDropData::token(None));
                self.internal_deposit_ft_transfer(data, drop_id, receiver_id);
            }
        }
    }

    /// Handles the claim process for an NFT drop.
    ///
    /// # Arguments
    ///
    /// * `data` - The internal NFT drop data.
    /// * `receiver_id` - The ID of the receiver claiming the drop.
    /// * `scavenger_id` - Optional scavenger ID to claim.
    /// * `claimed_drops` - The map of claimed drops for the receiver.
    fn handle_claim_nft_drop(
        &mut self,
        data: &NFTDropData,
        drop_id: &DropId,
        receiver_id: &AccountId,
        found_scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<DropId, ClaimedDropData>,
    ) {
        match data.base.scavenger_hunt.clone() {
            Some(required_scavenger_ids) => {
                near_sdk::log!("Required Scavenger IDs: {:?}", &required_scavenger_ids);
                let found_scavenger_id =
                    found_scavenger_id.expect("This drop requires a scavenger ID");

                let mut claimed_drop = claimed_drops
                    .get(drop_id)
                    .unwrap_or(ClaimedDropData::token(Some(Vec::new())));
                claimed_drop.add_scavenger_id(found_scavenger_id);

                let found_scavenger_ids =
                    claimed_drop.get_found_scavenger_ids().unwrap_or_default();

                if found_scavenger_ids.len() == required_scavenger_ids.len() {
                    self.internal_nft_mint(data.series_id, receiver_id.clone())
                }

                claimed_drops.insert(drop_id, &claimed_drop);
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(claimed_drops.get(drop_id).is_none(), "Drop already claimed");
                claimed_drops.insert(drop_id, &ClaimedDropData::token(None));
                self.internal_nft_mint(data.series_id, receiver_id.clone())
            }
        }
    }

    /// Decrements the tokens from the sponsor's balance for a token drop.
    ///
    /// If the drop creator is an admin, the tokens are minted internally.
    /// Otherwise, an FT transfer is performed from the drop creator to the user.
    ///
    /// # Panics
    ///
    /// Panics if the sponsor doesn't have enough tokens to cover the amount to be claimed.
    ///
    /// # Arguments
    ///
    /// * `drop` - The internal token drop data containing the creator ID and the amount to be claimed.
    fn internal_deposit_ft_transfer(
        &mut self,
        drop: &TokenDropData,
        drop_id: &DropId,
        receiver_id: &AccountId,
    ) {
        let drop_creator = parse_drop_id(drop_id);
        env::log_str(format!("Internal deposit ft transfer drop creator: {:?}", drop_creator).as_str());
        let mut account_details = self
            .account_details_by_id
            .get(&drop_creator)
            .expect("Drop creator not found in map");
        let creator_status = account_details
            .account_status
            .as_ref()
            .expect("Drop creator not found");

        let amount_to_claim = drop.amount.0;

        if creator_status.is_admin() {
            env::log_str(format!("Creator is admin: {}", drop_creator).as_str());
            // Mint tokens internally if the creator is an admin
            self.internal_deposit_ft_mint(receiver_id, amount_to_claim);
        } else if creator_status.is_sponsor() {
            env::log_str(format!("Creator is sponsor {:?}", drop_creator).as_str());
            // Get the current token balance of the creator
            let mut cur_creator_tokens = account_details.ft_balance;

            // Check if the creator has enough tokens to cover the amount to be claimed
            require!(
                cur_creator_tokens >= amount_to_claim,
                "The creator does not have enough tokens to cover the amount to be claimed."
            );

            // Perform FT transfer from the drop creator to the receiver
            self.internal_ft_transfer(&drop_creator, receiver_id, amount_to_claim);
        }
    }
}
