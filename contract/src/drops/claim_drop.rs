use crate::*;

#[near]
impl Contract {
    pub(crate) fn assert_valid_signature(
        &self,
        drop_id: &DropId,
        receiver_id: &AccountId,
        signature: &Base64VecU8,
        scavenger_id: Option<PublicKey>,
    ) {
        // Determine the expected public key
        let drop_data = self.drop_by_id.get(drop_id).expect("Drop not found");
        let expected_key = if let Some(scavenger_pk) = scavenger_id.clone() {
            scavenger_pk
        } else {
            // Get the drop key from drop_data
            match &drop_data {
                DropData::Token(data) => data.key.clone(),
                DropData::Nft(data) => data.key.clone(),
                DropData::Multichain(data) => data.key.clone(),
            }
        };
        // Verify the signature
        let is_valid_signature =
            verify_signature(signature.clone(), receiver_id.clone(), expected_key.clone());
        require!(is_valid_signature, "Invalid signature");
    }
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
        let receiver_id = self.caller_id_by_signing_pk();
        self.assert_valid_signature(&drop_id, &receiver_id, &signature, scavenger_id.clone());

        // Handle the claim logic based on the drop type
        let claim_log = self.handle_claim_drop(&drop_id, &receiver_id, scavenger_id);

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

        let event_log = EventLog {
            standard: KEYPOM_STANDARD_NAME.to_string(),
            version: KEYPOM_CONFERENCE_METADATA_SPEC.to_string(),
            event: EventLogVariant::KeypomDropClaim(claim_log),
        };
        env::log_str(&event_log.to_string());
        self.drop_by_id
            .get(&drop_id)
            .expect("Drop not found")
            .clone()
    }

    /// Handles the claim process for any drop type.
    ///
    /// # Arguments
    ///
    /// * `drop_data` - The mutable reference to the drop data.
    /// * `drop_id` - The ID of the drop.
    /// * `receiver_id` - The ID of the receiver claiming the drop.
    /// * `found_scavenger_id` - Optional scavenger ID to claim.
    /// * `claimed_drops` - The map of claimed drops for the receiver.
    fn handle_claim_drop(
        &mut self,
        drop_id: &DropId,
        receiver_id: &AccountId,
        found_scavenger_id: Option<PublicKey>,
    ) -> KeypomDropClaimLog {
        let mut drop_data = self
            .drop_by_id
            .get(drop_id)
            .expect("Drop not found")
            .clone();

        let mut event_log = KeypomDropClaimLog {
            claimer_id: receiver_id.to_string(),
            reward: None,
            pieces_found: None,
            pieces_required: None,
        };

        // Increment the number of claims for the drop
        match &mut drop_data {
            DropData::Token(data) => data.num_claimed += 1,
            DropData::Nft(data) => data.num_claimed += 1,
            DropData::Multichain(data) => data.num_claimed += 1,
        }

        // Extract scavenger hunt information if available
        let scavenger_hunt = match &drop_data {
            DropData::Token(data) => data.scavenger_hunt.as_ref(),
            DropData::Nft(data) => data.scavenger_hunt.as_ref(),
            DropData::Multichain(data) => data.scavenger_hunt.as_ref(),
        };

        if let Some(required_scavenger_ids) = scavenger_hunt {
            let found_scavenger_id = found_scavenger_id.expect("This drop requires a scavenger ID");

            // Handle scavenger hunt logic
            let hunt_complete = self.handle_scavenger_hunt(
                receiver_id,
                required_scavenger_ids,
                found_scavenger_id,
                drop_id,
                &mut event_log,
            );

            if hunt_complete {
                // Process the reward based on drop type
                let reward = match drop_data {
                    DropData::Token(ref data) => {
                        self.internal_deposit_ft_transfer(data, drop_id, receiver_id);
                        DropClaimReward::Token(data.amount)
                    }
                    DropData::Nft(ref data) => {
                        self.internal_nft_mint(data.series_id, receiver_id.clone());
                        DropClaimReward::Nft
                    }
                    DropData::Multichain(ref data) => {
                        self.handle_multichain_mint(data);
                        DropClaimReward::Multichain
                    }
                };
                event_log.reward = Some(reward);
            }
        } else {
            // Use entry API to access and modify account details
            let account_details = self
                .account_details_by_id
                .entry(receiver_id.clone())
                .or_insert_with(|| AccountDetails::new(receiver_id));

            // Ensure the drop hasn't been claimed already
            require!(
                account_details
                    .drops_claimed
                    .insert(drop_id.to_string(), None)
                    .is_none(),
                "Drop already claimed"
            );

            // Process the reward based on drop type
            let reward = match drop_data {
                DropData::Token(ref data) => {
                    self.internal_deposit_ft_transfer(data, drop_id, receiver_id);
                    DropClaimReward::Token(data.amount)
                }
                DropData::Nft(ref data) => {
                    self.internal_nft_mint(data.series_id, receiver_id.clone());
                    DropClaimReward::Nft
                }
                DropData::Multichain(ref data) => {
                    self.handle_multichain_mint(data);
                    DropClaimReward::Multichain
                }
            };
            event_log.reward = Some(reward);
        }

        self.drop_by_id.insert(drop_id.to_string(), drop_data);

        event_log
    }

    /// Handles the scavenger hunt logic common to all drop types.
    ///
    /// # Arguments
    ///
    /// * `required_scavenger_ids` - The list of required scavenger IDs.
    /// * `found_scavenger_id` - The scavenger ID found by the user.
    /// * `claimed_drops` - The map of claimed drops for the receiver.
    /// * `drop_id` - The ID of the drop.
    /// * `event_log` - The event log to update with scavenger hunt info.
    ///
    /// # Returns
    ///
    /// * `bool` - Indicates whether the scavenger hunt is complete.
    fn handle_scavenger_hunt(
        &mut self,
        receiver_id: &AccountId,
        required_scavenger_ids: &[ScavengerHuntData],
        found_scavenger_id: PublicKey,
        drop_id: &DropId,
        event_log: &mut KeypomDropClaimLog,
    ) -> bool {
        // Check if the found_scavenger_id is valid and hasn't been claimed yet
        let is_valid_scavenger_id = required_scavenger_ids
            .iter()
            .any(|scavenger| scavenger.key == found_scavenger_id);
        require!(is_valid_scavenger_id, "Incorrect scavenger piece passed in");

        // Use entry API to access and modify account details
        let account_details = self
            .account_details_by_id
            .entry(receiver_id.clone())
            .or_insert_with(|| AccountDetails::new(receiver_id));

        let mut claimed_drop = account_details
            .drops_claimed
            .get(drop_id)
            .cloned()
            .unwrap_or(Some(Vec::new()));

        let already_claimed = claimed_drop
            .clone()
            .unwrap_or_default()
            .contains(&found_scavenger_id);
        require!(!already_claimed, "Scavenger piece already claimed");

        // Add the valid scavenger_id to the claimed_drop
        claimed_drop
            .as_mut()
            .unwrap_or(&mut Vec::new())
            .push(found_scavenger_id);

        let found_scavenger_ids = claimed_drop.clone().unwrap_or_default().len() as u16;
        let required_scavenger_ids_count = required_scavenger_ids.len() as u16;

        event_log.pieces_found = Some(found_scavenger_ids);
        event_log.pieces_required = Some(required_scavenger_ids_count);

        account_details
            .drops_claimed
            .insert(drop_id.to_string(), claimed_drop);

        found_scavenger_ids == required_scavenger_ids_count
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
                NearToken::from_yoctonear(amount_to_claim),
                Some(drop_id.clone()),
                true,
            );
        } else if creator_status.is_sponsor() {
            env::log_str(format!("Creator is sponsor {:?}", drop_creator).as_str());
            // Get the current token balance of the creator
            let cur_creator_tokens = account_details.ft_balance;

            // Check if the creator has enough tokens to cover the amount to be claimed
            require!(
                cur_creator_tokens.ge(&NearToken::from_yoctonear(amount_to_claim)),
                "The creator does not have enough tokens to cover the amount to be claimed."
            );

            // Perform FT transfer from the drop creator to the receiver
            self.internal_ft_transfer(
                &drop_creator,
                receiver_id,
                NearToken::from_yoctonear(amount_to_claim),
                true,
            );
        }
    }
}
