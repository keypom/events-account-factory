use crate::*;

#[near_bindgen]
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
    pub fn create_token_drop(&mut self, drop_data: ExtDropBase, token_amount: U128) -> String {
        self.assert_no_freeze();
        let drop_creator = self.assert_sponsor();

        let mut account_details = self
            .account_details_by_id
            .get(&drop_creator)
            .unwrap_or(AccountDetails::new(&drop_creator));
        let mut creator_drop_ids = account_details.drops_created;

        // The drop ID will be a concatenation of the creator, delimiter, and the drop number
        let drop_number: u64 = creator_drop_ids.len();
        let drop_id = format!("{}{}{}", drop_creator, DROP_DELIMITER, drop_number);
        require!(
            self.drop_by_id
                .insert(
                    &drop_id,
                    &DropData::token(TokenDropData {
                        base: DropBase {
                            name: drop_data.name,
                            num_claimed: 0,
                            image: drop_data.image,
                            scavenger_hunt: drop_data.scavenger_hunt,
                            id: drop_id.clone()
                        },
                        amount: token_amount
                    })
                )
                .is_none(),
            "Drop ID already exists"
        );

        // Add the drop ID to the creator's list of drop IDs
        creator_drop_ids.insert(&drop_id);
        account_details.drops_created = creator_drop_ids;
        self.account_details_by_id
            .insert(&drop_creator, &account_details);
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
        drop_data: ExtDropBase,
        nft_metadata: TokenMetadata,
    ) -> String {
        let drop_creator = self.assert_sponsor();

        let mut account_details = self
            .account_details_by_id
            .get(&drop_creator)
            .unwrap_or(AccountDetails::new(&drop_creator));
        let mut creator_drop_ids = account_details.drops_created;

        // The drop ID will be a concatenation of the creator, delimiter, and the drop number
        let drop_number: u64 = creator_drop_ids.len();
        let drop_id = format!("{}{}{}", drop_creator, DROP_DELIMITER, drop_number);

        let series_id = self.internal_create_series(&nft_metadata, &drop_creator);
        require!(
            self.drop_by_id
                .insert(
                    &drop_id,
                    &DropData::nft(NFTDropData {
                        base: DropBase {
                            name: drop_data.name,
                            num_claimed: 0,
                            image: drop_data.image,
                            scavenger_hunt: drop_data.scavenger_hunt,
                            id: drop_id.clone()
                        },
                        series_id
                    })
                )
                .is_none(),
            "Drop ID already exists"
        );

        // Add the drop ID to the creator's list of drop IDs
        creator_drop_ids.insert(&drop_id);
        account_details.drops_created = creator_drop_ids;
        self.account_details_by_id
            .insert(&drop_creator, &account_details);
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
        require!(
            drop_creator == caller_id,
            "Only the drop creator can delete this drop"
        );

        // If the drop is an NFT drop AND the series doesn't have any claims, delete the series
        if let Some(DropData::nft(nft_drop)) = self.drop_by_id.remove(&drop_id) {
            let series = self
                .series_by_id
                .get(&nft_drop.series_id)
                .expect("No series found for drop");

            if series.tokens.is_empty() {
                self.internal_delete_series(nft_drop.series_id);
            }
        }
        // Remove the drop ID from the creator's list of drop IDs
        let mut account_details = self
            .account_details_by_id
            .get(&drop_creator)
            .unwrap_or(AccountDetails::new(&drop_creator));
        let mut creator_drop_ids = account_details.drops_created;
        creator_drop_ids.remove(&drop_id);
        account_details.drops_created = creator_drop_ids;
        self.account_details_by_id
            .insert(&drop_creator, &account_details);
    }
}
