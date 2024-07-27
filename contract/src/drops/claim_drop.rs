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
        self.claims_by_account.insert(&receiver_id, &claimed_drops);
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
        data: InternalTokenDropData,
        receiver_id: &AccountId,
        scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<DropId, InternalClaimedDropData>,
    ) {
        match data.base.base.scavenger_ids.clone() {
            Some(scavenger_ids) => {
                near_sdk::log!("Scavenger IDs: {:?}", &scavenger_ids);
                let mut claimed_drop = claimed_drops.get(&data.base.base.id).unwrap_or(InternalClaimedDropData {
                    id: data.base.base.id.clone(),
                    scavenger_ids: None,
                });
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
                    self.internal_deposit_transfer(&data, receiver_id);
                }

                claimed_drop.scavenger_ids = Some(claimed_scavenger_ids);
                claimed_drops.insert(&data.base.base.id, &claimed_drop);
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(
                    claimed_drops.get(&data.base.base.id).is_none(),
                    "Drop already claimed"
                );
                claimed_drops.insert(&data.base.base.id, &InternalClaimedDropData {
                    id: data.base.base.id.clone(),
                    scavenger_ids: None,
                });
                self.internal_deposit_transfer(&data, receiver_id);
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
        data: InternalNFTDropData,
        receiver_id: &AccountId,
        scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<DropId, InternalClaimedDropData>,
    ) {
        match data.base.base.scavenger_ids.clone() {
            Some(scavenger_ids) => {
                near_sdk::log!("Scavenger IDs: {:?}", &scavenger_ids);
                let mut claimed_drop = claimed_drops.get(&data.base.base.id).unwrap_or(InternalClaimedDropData {
                    id: data.base.base.id.clone(),
                    scavenger_ids: None,
                });
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
                claimed_drops.insert(&data.base.base.id, &claimed_drop);
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(
                    claimed_drops.get(&data.base.base.id).is_none(),
                    "Drop already claimed"
                );
                claimed_drops.insert(&data.base.base.id, &InternalClaimedDropData {
                    id: data.base.base.id.clone(),
                    scavenger_ids: None,
                });
                // TODO: actually mint NFT
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
    fn internal_deposit_transfer(
        &mut self,
        drop: &InternalTokenDropData,
        receiver_id: &AccountId,
    ) {
        let drop_creator = drop.base.creator_id.clone();
        let creator_status = self.account_status_by_id.get(&drop_creator).expect("Unknown drop creator status");

        let amount_to_claim = drop.amount.0;

        if creator_status.is_admin() {
            // Mint tokens internally if the creator is an admin
            self.internal_deposit_mint(&receiver_id, amount_to_claim);
        } else if creator_status.is_sponsor() {
            // Get the current token balance of the creator
            let mut cur_creator_tokens = self.ft_balance_of(drop_creator.clone()).0;

            // Check if the creator has enough tokens to cover the amount to be claimed
            require!(
                cur_creator_tokens >= amount_to_claim,
                "The creator does not have enough tokens to cover the amount to be claimed."
            );

            // Decrement the tokens from the creator's balance
            cur_creator_tokens -= amount_to_claim;

            // Update the creator's balance in the contract
            self.balance_by_account.insert(&drop_creator, &cur_creator_tokens.into());

            // Perform FT transfer from the drop creator to the receiver
            self.internal_transfer(&drop_creator, &receiver_id, amount_to_claim);
        }
    }
}