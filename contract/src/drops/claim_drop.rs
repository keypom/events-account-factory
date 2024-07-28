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

        let receiver_id = self.caller_id_by_signing_pk();
        let mut claimed_drops = self
            .claims_by_account
            .get(&receiver_id)
            .expect("User not registered");

        match drop_data {
            DropData::token(data) => {
                self.handle_claim_token_drop(data, &drop_id, &receiver_id, scavenger_id, &mut claimed_drops);
            }
            DropData::nft(data) => {
                self.handle_claim_nft_drop(data, &drop_id, &receiver_id, scavenger_id, &mut claimed_drops);
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
        data: TokenDropData,
        drop_id: &DropId,
        receiver_id: &AccountId,
        found_scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<DropId, ClaimedDropData>,
    ) {
        match data.base.scavenger_ids.clone() {
            Some(required_scavenger_ids) => {
                near_sdk::log!("Required Scavenger IDs: {:?}", &required_scavenger_ids);
                let found_scavenger_id = found_scavenger_id.expect("This drop requires a scavenger ID");

                let mut claimed_drop = claimed_drops.get(drop_id).unwrap_or(ClaimedDropData::token(Some(Vec::new())));
                claimed_drop.add_scavenger_id(found_scavenger_id);

                let found_scavenger_ids = claimed_drop.get_found_scavenger_ids().unwrap_or(Vec::new());

                if found_scavenger_ids.len() == required_scavenger_ids.len() {
                    // All scavenger items claimed, now claim the main reward
                    self.internal_deposit_ft_transfer(&data, &drop_id, receiver_id);
                }

                claimed_drops.insert(&drop_id, &claimed_drop);
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(
                    claimed_drops.get(&drop_id).is_none(),
                    "Drop already claimed"
                );
                claimed_drops.insert(&drop_id, &ClaimedDropData::token(None));
                self.internal_deposit_ft_transfer(&data, &drop_id, receiver_id);
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
        data: NFTDropData,
        drop_id: &DropId,
        receiver_id: &AccountId,
        found_scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<DropId, ClaimedDropData>,
    ) {
        match data.base.scavenger_ids.clone() {
            Some(required_scavenger_ids) => {
                near_sdk::log!("Required Scavenger IDs: {:?}", &required_scavenger_ids);
                let found_scavenger_id = found_scavenger_id.expect("This drop requires a scavenger ID");

                let mut claimed_drop = claimed_drops.get(drop_id).unwrap_or(ClaimedDropData::token(Some(Vec::new())));
                claimed_drop.add_scavenger_id(found_scavenger_id);

                let found_scavenger_ids = claimed_drop.get_found_scavenger_ids().unwrap_or(Vec::new());

                if found_scavenger_ids.len() == required_scavenger_ids.len() {
                    self.internal_nft_mint(data.series_id, receiver_id.clone())
                }

                claimed_drops.insert(&drop_id, &claimed_drop);
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(
                    claimed_drops.get(&drop_id).is_none(),
                    "Drop already claimed"
                );
                claimed_drops.insert(&drop_id, &ClaimedDropData::token(None));
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
        let creator_status = self.account_status_by_id.get(&drop_creator).expect("Unknown drop creator status");

        let amount_to_claim = drop.amount.0;

        if creator_status.is_admin() {
            // Mint tokens internally if the creator is an admin
            self.internal_deposit_ft_mint(&receiver_id, amount_to_claim);
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
            self.ft_balance_by_account.insert(&drop_creator, &cur_creator_tokens.into());

            // Perform FT transfer from the drop creator to the receiver
            self.internal_ft_transfer(&drop_creator, &receiver_id, amount_to_claim);
        }
    }
}