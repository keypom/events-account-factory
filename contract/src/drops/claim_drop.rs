use std::convert::TryInto;

use crate::*;

#[near]
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
    pub fn claim_drop(
        &mut self,
        drop_id: String,
        scavenger_id: Option<PublicKey>,
        signature: Base64VecU8,
    ) -> DropData {
        self.assert_no_freeze();
        let drop_data = self.drop_by_id.get_mut(&drop_id).expect("Drop not found");

        let receiver_id = self.caller_id_by_signing_pk();

        // Use entry API to access and modify account details
        let account_details = self
            .account_details_by_id
            .entry(receiver_id.clone())
            .or_insert_with(|| AccountDetails::new(&receiver_id));

        let mut claimed_drops = account_details.drops_claimed;

        // Match on the DropData enum to handle token and NFT drops
        let claim_log = match &mut drop_data {
            DropData::Token(data) => {
                data.num_claimed += 1;
                self.handle_claim_token_drop(
                    data,
                    &drop_id,
                    &receiver_id,
                    scavenger_id,
                    &mut claimed_drops,
                )
            }
            DropData::Nft(data) => {
                data.num_claimed += 1;
                self.handle_claim_nft_drop(
                    data,
                    &drop_id,
                    &receiver_id,
                    scavenger_id,
                    &mut claimed_drops,
                )
            }
            DropData::Multichain(data) => {
                data.num_claimed += 1;
                self.handle_claim_multichain_drop(
                    data,
                    &drop_id,
                    &receiver_id,
                    scavenger_id,
                    &mut claimed_drops,
                )
            }
        };

        let reward = match claim_log.reward {
            Some(DropClaimReward::Token(amount)) => format!("{}", amount.0),
            Some(DropClaimReward::Nft) => "NFT".to_string(),
            Some(DropClaimReward::Multichain) => "Multichain POAP".to_string(),
            _ => "Scavenger Piece".to_string(),
        };

        self.add_transaction(TransactionType::Claim {
            account_id: receiver_id.clone(),
            reward,
            timestamp: env::block_timestamp(),
        });

        self.total_transactions += 1;

        // Update account details with claimed drops
        account_details.drops_claimed = claimed_drops;

        let event_log = EventLog {
            standard: KEYPOM_STANDARD_NAME.to_string(),
            version: KEYPOM_CONFERENCE_METADATA_SPEC.to_string(),
            event: EventLogVariant::KeypomDropClaim(claim_log),
        };
        env::log_str(&event_log.to_string());

        drop_data.clone()
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
        claimed_drops: &mut IterableMap<DropId, ClaimedDropData>,
    ) -> KeypomDropClaimLog {
        let mut event_log = KeypomDropClaimLog {
            claimer_id: receiver_id.to_string(),
            reward: None,
            pieces_found: None,
            pieces_required: None,
        };

        if let Some(required_scavenger_ids) = &data.base.scavenger_hunt {
            let found_scavenger_id = found_scavenger_id.expect("This drop requires a scavenger ID");

            // Check if the found_scavenger_id is valid and hasn't been claimed yet
            let is_valid_scavenger_id = required_scavenger_ids
                .iter()
                .any(|scavenger| scavenger.piece == found_scavenger_id);
            require!(is_valid_scavenger_id, "Incorrect scavenger piece passed in");

            let mut claimed_drop = claimed_drops
                .get(drop_id)
                .unwrap_or(ClaimedDropData::Token(Some(Vec::new())));

            let already_claimed = claimed_drop
                .get_found_scavenger_ids()
                .unwrap_or_default()
                .contains(&found_scavenger_id);
            require!(!already_claimed, "Scavenger piece already claimed");

            // Add the valid scavenger_id to the claimed_drop
            claimed_drop.add_scavenger_id(found_scavenger_id.clone());

            let found_scavenger_ids = claimed_drop
                .get_found_scavenger_ids()
                .unwrap_or_default()
                .len();
            let required_scavenger_ids = required_scavenger_ids.len();

            event_log.pieces_found = Some(
                found_scavenger_ids
                    .try_into()
                    .expect("Too many pieces found to convert to u16"),
            );
            event_log.pieces_required = Some(
                required_scavenger_ids
                    .try_into()
                    .expect("Too many pieces required to convert to u16"),
            );
            if found_scavenger_ids == required_scavenger_ids {
                event_log.reward = Some(DropClaimReward::Token(data.amount));
                self.internal_deposit_ft_transfer(data, drop_id, receiver_id);
            }

            claimed_drops.insert(drop_id, &claimed_drop);
            event_log
        } else {
            require!(claimed_drops.get(drop_id).is_none(), "Drop already claimed");
            claimed_drops.insert(drop_id, &ClaimedDropData::Token(None));
            self.internal_deposit_ft_transfer(data, drop_id, receiver_id);
            event_log.reward = Some(DropClaimReward::Token(data.amount));
            event_log
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
        claimed_drops: &mut IterableMap<DropId, ClaimedDropData>,
    ) -> KeypomDropClaimLog {
        let mut event_log = KeypomDropClaimLog {
            claimer_id: receiver_id.to_string(),
            reward: None,
            pieces_found: None,
            pieces_required: None,
        };

        if let Some(required_scavenger_ids) = &data.base.scavenger_hunt {
            let found_scavenger_id = found_scavenger_id.expect("This drop requires a scavenger ID");

            // Check if the found_scavenger_id is valid and hasn't been claimed yet
            let is_valid_scavenger_id = required_scavenger_ids
                .iter()
                .any(|scavenger| scavenger.piece == found_scavenger_id);
            require!(is_valid_scavenger_id, "Incorrect scavenger piece passed in");

            let mut claimed_drop = claimed_drops
                .get(drop_id)
                .unwrap_or(ClaimedDropData::Nft(Some(Vec::new())));

            let already_claimed = claimed_drop
                .get_found_scavenger_ids()
                .unwrap_or_default()
                .contains(&found_scavenger_id);
            require!(!already_claimed, "Scavenger piece already claimed");

            // Add the valid scavenger_id to the claimed_drop
            claimed_drop.add_scavenger_id(found_scavenger_id.clone());

            let found_scavenger_ids = claimed_drop
                .get_found_scavenger_ids()
                .unwrap_or_default()
                .len();
            let required_scavenger_ids = required_scavenger_ids.len();

            event_log.pieces_found = Some(
                found_scavenger_ids
                    .try_into()
                    .expect("Too many pieces found to convert to u16"),
            );
            event_log.pieces_required = Some(
                required_scavenger_ids
                    .try_into()
                    .expect("Too many pieces required to convert to u16"),
            );
            if found_scavenger_ids == required_scavenger_ids {
                self.internal_nft_mint(data.series_id, receiver_id.clone());
                event_log.reward = Some(DropClaimReward::Nft);
            }

            claimed_drops.insert(drop_id, &claimed_drop);
            event_log
        } else {
            require!(claimed_drops.get(drop_id).is_none(), "Drop already claimed");
            claimed_drops.insert(drop_id, &ClaimedDropData::Nft(None));
            self.internal_nft_mint(data.series_id, receiver_id.clone());
            event_log.reward = Some(DropClaimReward::Nft);
            event_log
        }
    }

    /// Handles the claim process for a multichain drop.
    ///
    /// # Arguments
    ///
    /// * `data` - The internal token drop data.
    /// * `receiver_id` - The ID of the receiver claiming the drop.
    /// * `scavenger_id` - Optional scavenger ID to claim.
    /// * `claimed_drops` - The map of claimed drops for the receiver.
    fn handle_claim_multichain_drop(
        &mut self,
        data: &MultichainDropData,
        drop_id: &DropId,
        receiver_id: &AccountId,
        found_scavenger_id: Option<String>,
        claimed_drops: &mut IterableMap<DropId, ClaimedDropData>,
    ) -> KeypomDropClaimLog {
        let mut event_log = KeypomDropClaimLog {
            claimer_id: receiver_id.to_string(),
            reward: None,
            pieces_found: None,
            pieces_required: None,
        };

        if let Some(required_scavenger_ids) = &data.base.scavenger_hunt {
            let found_scavenger_id = found_scavenger_id.expect("This drop requires a scavenger ID");

            // Check if the found_scavenger_id is valid and hasn't been claimed yet
            let is_valid_scavenger_id = required_scavenger_ids
                .iter()
                .any(|scavenger| scavenger.piece == found_scavenger_id);
            require!(is_valid_scavenger_id, "Incorrect scavenger piece passed in");

            let mut claimed_drop = claimed_drops
                .get(drop_id)
                .unwrap_or(ClaimedDropData::Multichain(Some(Vec::new())));

            let already_claimed = claimed_drop
                .get_found_scavenger_ids()
                .unwrap_or_default()
                .contains(&found_scavenger_id);
            require!(!already_claimed, "Scavenger piece already claimed");

            // Add the valid scavenger_id to the claimed_drop
            claimed_drop.add_scavenger_id(found_scavenger_id.clone());

            let found_scavenger_ids = claimed_drop
                .get_found_scavenger_ids()
                .unwrap_or_default()
                .len();
            let required_scavenger_ids = required_scavenger_ids.len();

            event_log.pieces_found = Some(
                found_scavenger_ids
                    .try_into()
                    .expect("Too many pieces found to convert to u16"),
            );
            event_log.pieces_required = Some(
                required_scavenger_ids
                    .try_into()
                    .expect("Too many pieces required to convert to u16"),
            );
            if found_scavenger_ids == required_scavenger_ids {
                event_log.reward = Some(DropClaimReward::Multichain);
                //self.handle_multichain_mint(data);
            }

            claimed_drops.insert(drop_id, &claimed_drop);
            event_log
        } else {
            require!(claimed_drops.get(drop_id).is_none(), "Drop already claimed");
            claimed_drops.insert(drop_id, &ClaimedDropData::Multichain(None));
            self.handle_multichain_mint(data);
            event_log.reward = Some(DropClaimReward::Multichain);
            event_log
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
        env::log_str(
            format!(
                "Internal deposit ft transfer drop creator: {:?}",
                drop_creator
            )
            .as_str(),
        );
        let account_details = self
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
            self.internal_deposit_ft_mint(
                receiver_id,
                amount_to_claim,
                Some(drop_id.clone()),
                true,
            );
        } else if creator_status.is_sponsor() {
            env::log_str(format!("Creator is sponsor {:?}", drop_creator).as_str());
            // Get the current token balance of the creator
            let cur_creator_tokens = account_details.ft_balance;

            // Check if the creator has enough tokens to cover the amount to be claimed
            require!(
                cur_creator_tokens >= amount_to_claim,
                "The creator does not have enough tokens to cover the amount to be claimed."
            );

            // Perform FT transfer from the drop creator to the receiver
            self.internal_ft_transfer(&drop_creator, receiver_id, amount_to_claim, true);
        }
    }
}
