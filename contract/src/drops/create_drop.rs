use crate::*;

#[near]
impl Contract {
    /// Allows a sponsor or admin to create a token drop so people can scan a QR code and get the amount of tokens.
    ///
    /// # Arguments
    ///
    /// * `drop_data` - The base drop data such as scavenger hunt IDs, name, image
    /// * `token_amount` - The amount of tokens that this drop contains
    ///
    /// # Panics
    ///
    /// Panics if the sponsor is not authorized.
    pub fn create_token_drop(
        &mut self,
        image: String,
        name: String,
        scavenger_hunt: Option<Vec<ScavengerHuntData>>,
        key: PublicKey,
        token_amount: U128,
    ) -> String {
        self.assert_no_freeze();
        let drop_creator = self.assert_sponsor();

        let account_details = self
            .account_details_by_id
            .entry(drop_creator.clone())
            .or_insert_with(|| AccountDetails::new(&drop_creator));

        // The drop ID will be a concatenation of the creator, delimiter, and the drop number
        let drop_id = format!(
            "{}{}{}",
            drop_creator,
            DROP_DELIMITER,
            account_details.drops_created.len()
        );
        require!(
            self.drop_by_id
                .insert(
                    drop_id.clone(),
                    DropData::Token(TokenDropData {
                        key,
                        name,
                        image,
                        num_claimed: 0,
                        scavenger_hunt: scavenger_hunt.clone(),
                        id: drop_id.clone(),
                        token_amount
                    })
                )
                .is_none(),
            "Drop ID already exists"
        );

        // Add the drop ID to the creator's list of drop IDs
        account_details.drops_created.insert(drop_id.clone());

        let drop_creation_log: EventLog = EventLog {
            standard: KEYPOM_STANDARD_NAME.to_string(),
            version: KEYPOM_CONFERENCE_METADATA_SPEC.to_string(),
            event: EventLogVariant::KeypomDropCreation(KeypomDropCreationLog {
                creator_id: drop_creator.to_string(),
                drop_reward: DropClaimReward::Token(token_amount),
                num_scavengers: scavenger_hunt.map(|scavenger_hunt| scavenger_hunt.len() as u16),
            }),
        };
        env::log_str(&drop_creation_log.to_string());

        self.total_transactions += 1;
        drop_id
    }

    /// Allows a sponsor or admin to create an NFT drop so people can scan a QR code and mint that NFT
    ///
    /// # Arguments
    ///
    /// * `drop_data` - The base drop data such as scavenger hunt IDs, name, image
    /// * `nft_metadata` - The metadata for the NFTs that will be minted as part of this drop
    ///
    /// # Panics
    ///
    /// Panics if the sponsor is not authorized.
    pub fn create_nft_drop(
        &mut self,
        image: String,
        name: String,
        key: PublicKey,
        scavenger_hunt: Option<Vec<ScavengerHuntData>>,
        nft_metadata: TokenMetadata,
    ) -> String {
        let drop_creator = self.assert_sponsor();

        let account_details = self
            .account_details_by_id
            .entry(drop_creator.clone())
            .or_insert_with(|| AccountDetails::new(&drop_creator));

        // The drop ID will be a concatenation of the creator, delimiter, and the drop number
        let drop_id = format!(
            "{}{}{}",
            drop_creator,
            DROP_DELIMITER,
            account_details.drops_created.len()
        );

        // Create the series
        let series_id = self.series_by_id.len();
        let tokens = IterableSet::new(StorageKeys::SeriesByIdInner {
            account_id_hash: hash_string(&format!("{}{}", drop_creator, series_id)),
        });
        let series = Series {
            metadata: nft_metadata.clone(),
            royalty: None,
            tokens,
        };
        require!(
            self.series_by_id.insert(series_id, series).is_none(),
            "Series ID already exists on the contract"
        );

        require!(
            self.drop_by_id
                .insert(
                    drop_id.clone(),
                    DropData::Nft(NFTDropData {
                        name,
                        key,
                        image,
                        num_claimed: 0,
                        scavenger_hunt: scavenger_hunt.clone(),
                        id: drop_id.clone(),
                        nft_metadata,
                        nft_series_id: series_id
                    })
                )
                .is_none(),
            "Drop ID already exists"
        );

        // Add the drop ID to the creator's list of drop IDs
        account_details.drops_created.insert(drop_id.clone());

        let drop_creation_log: EventLog = EventLog {
            standard: KEYPOM_STANDARD_NAME.to_string(),
            version: KEYPOM_CONFERENCE_METADATA_SPEC.to_string(),
            event: EventLogVariant::KeypomDropCreation(KeypomDropCreationLog {
                creator_id: drop_creator.to_string(),
                drop_reward: DropClaimReward::Nft,
                num_scavengers: scavenger_hunt.map(|scavs| scavs.len() as u16),
            }),
        };
        env::log_str(&drop_creation_log.to_string());

        self.total_transactions += 1;
        drop_id
    }

    /// Allows an admin to create a multichain drop so people can scan a QR code and mint an NFT on
    /// the destination chain
    ///
    /// # Arguments
    ///
    /// * `series_id` - The series ID of the NFTs that will be minted on the external chain. You
    /// need to have previously called the create series method on the external chain.
    /// * `contract_id` - The NFT contract ID that is deployed on the external chain.
    /// * `chain_id` - The ID of the external chain.
    ///
    /// # Panics
    ///
    /// Panics if the account is not an admin.
    pub fn create_multichain_drop(
        &mut self,
        image: String,
        name: String,
        key: PublicKey,
        scavenger_hunt: Option<Vec<ScavengerHuntData>>,
        multichain_metadata: MultichainMetadata,
        nft_metadata: TokenMetadata,
    ) -> String {
        let drop_creator = self.assert_admin();

        let account_details = self
            .account_details_by_id
            .entry(drop_creator.clone())
            .or_insert_with(|| AccountDetails::new(&drop_creator));

        // The drop ID will be a concatenation of the creator, delimiter, and the drop number
        let drop_id = format!(
            "{}{}{}",
            drop_creator,
            DROP_DELIMITER,
            account_details.drops_created.len()
        );

        require!(
            self.drop_by_id
                .insert(
                    drop_id.clone(),
                    DropData::Multichain(MultichainDropData {
                        name,
                        key,
                        image,
                        num_claimed: 0,
                        nft_metadata,
                        scavenger_hunt: scavenger_hunt.clone(),
                        id: drop_id.clone(),
                        mc_metadata: multichain_metadata
                    })
                )
                .is_none(),
            "Drop ID already exists"
        );

        // Add the drop ID to the creator's list of drop IDs
        account_details.drops_created.insert(drop_id.clone());

        let drop_creation_log: EventLog = EventLog {
            standard: KEYPOM_STANDARD_NAME.to_string(),
            version: KEYPOM_CONFERENCE_METADATA_SPEC.to_string(),
            event: EventLogVariant::KeypomDropCreation(KeypomDropCreationLog {
                creator_id: drop_creator.to_string(),
                drop_reward: DropClaimReward::Multichain,
                num_scavengers: scavenger_hunt.map(|scavs| scavs.len() as u16),
            }),
        };
        env::log_str(&drop_creation_log.to_string());

        self.total_transactions += 1;
        drop_id
    }

    /// Deletes a drop if the requestor is the creator or an admin.
    ///
    /// # Arguments
    ///
    /// * `drop_id` - The ID of the drop to be deleted.
    ///
    /// # Panics
    ///
    /// Panics if the drop is not found or if the requestor is not authorized.
    pub fn delete_drop(&mut self, drop_id: String) {
        let caller_id = self.assert_sponsor();
        let drop_creator = parse_drop_id(&drop_id);

        // Ensure that only the creator of the drop can delete it
        require!(
            drop_creator == caller_id,
            "Only the drop creator can delete this drop"
        );

        // If the drop is an NFT drop and the series doesn't have any claims, delete the series
        if let Some(DropData::Nft(nft_drop)) = self.drop_by_id.remove(&drop_id) {
            self.internal_delete_series(nft_drop.nft_series_id);
        }

        // Access and update the creator's drop IDs using the `entry` API
        let account_details = self
            .account_details_by_id
            .entry(drop_creator.clone())
            .or_insert_with(|| AccountDetails::new(&drop_creator));

        // Remove the drop ID from the creator's list of drop IDs
        account_details.drops_created.remove(&drop_id);

        // Increment the total number of transactions
        self.total_transactions += 1;
    }
}
