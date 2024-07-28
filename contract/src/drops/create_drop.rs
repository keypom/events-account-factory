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
    pub fn create_token_drop(&mut self, drop_data: DropBase, token_amount: U128) {
        let drop_creator = self.assert_sponsor();

        // The drop ID will be a concatenation of the creator, delimiter, and the drop number
        let mut creator_drop_ids = self.drop_ids_by_creator.get(&drop_creator).unwrap_or(UnorderedSet::new(StorageKeys::DropIdsByCreatorInner {
            account_id_hash: env::sha256_array(drop_creator.as_bytes()),
        }));
        let drop_number = creator_drop_ids.len();
        let drop_id = format!("{}{}{}", drop_creator, DROP_DELIMITER, drop_number);
        require!(self.drop_by_id.insert(&drop_id, &DropData::token(TokenDropData { base: drop_data, amount: token_amount })).is_none(), "Drop ID already exists");

        // Add the drop ID to the creator's list of drop IDs
        creator_drop_ids.insert(&drop_id);
        self.drop_ids_by_creator.insert(&drop_creator, &creator_drop_ids);
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
    pub fn create_nft_drop(&mut self, drop_data: DropBase, nft_metadata: TokenMetadata) {
        let drop_creator = self.assert_sponsor();

        // The drop ID will be a concatenation of the creator, delimiter, and the drop number
        let mut creator_drop_ids = self.drop_ids_by_creator.get(&drop_creator).unwrap_or(UnorderedSet::new(StorageKeys::DropIdsByCreatorInner {
            account_id_hash: env::sha256_array(drop_creator.as_bytes()),
        }));
        let drop_number = creator_drop_ids.len();
        let drop_id = format!("{}{}{}", drop_creator, DROP_DELIMITER, drop_number);
        let series_id = self.internal_create_series(&nft_metadata, &drop_creator);
        require!(self.drop_by_id.insert(&drop_id, &DropData::nft(NFTDropData { base: drop_data, series_id })).is_none(), "Drop ID already exists");

        // Add the drop ID to the creator's list of drop IDs
        creator_drop_ids.insert(&drop_id);
        self.drop_ids_by_creator.insert(&drop_creator, &creator_drop_ids);
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
        require!(drop_creator == caller_id, "Only the drop creator can delete this drop");

        // If the drop is an NFT drop AND the series doesn't have any claims, delete the series
        if let Some(drop) = self.drop_by_id.remove(&drop_id) {
            if let DropData::nft(nft_drop) = drop {
                let series = self.series_by_id.get(&nft_drop.series_id).expect("No series found for drop");

                if series.tokens.len() == 0 {
                    self.internal_delete_series(nft_drop.series_id);
                }
            }
        }

        // Remove the drop ID from the creator's list of drop IDs
        let mut creator_drop_ids = self.drop_ids_by_creator.get(&drop_creator).expect("Creator has no drops");
        creator_drop_ids.remove(&drop_id);
        self.drop_ids_by_creator.insert(&drop_creator, &creator_drop_ids);
    }
}
