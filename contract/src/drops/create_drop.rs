use crate::*;

#[near_bindgen]
impl Contract {
    /// Allows a sponsor or admin to create a drop so people can scan a QR code and get the amount of tokens.
    ///
    /// # Arguments
    ///
    /// * `drop` - The external drop data to be created.
    ///
    /// # Panics
    ///
    /// Panics if the sponsor is not authorized.
    pub fn create_drop(&mut self, drop: ExtClaimedDropData) {
        let drop_creator = self.assert_sponsor();

        let drop_id = drop.get_id();
        let internal_drop = drop.convert_to_internal(env::predecessor_account_id());
        self.drop_by_id.insert(&drop_id, &internal_drop);

        // Add the drop ID to the creator's list of drop IDs
        let mut creator_drop_ids = self.drop_ids_by_creator.get(&drop_creator).unwrap_or(UnorderedSet::new(StorageKeys::DropIdsByCreatorInner {
            account_id_hash: env::sha256_array(drop_creator.as_bytes()),
        }));
        creator_drop_ids.insert(&drop_id);
        self.drop_ids_by_creator.insert(&drop_creator, &creator_drop_ids);
    }

    /// Allows a sponsor or admin to create multiple drops in a batch.
    ///
    /// # Arguments
    ///
    /// * `drops` - A vector of external drop data to be created.
    ///
    /// # Panics
    ///
    /// Panics if the sponsor is not authorized.
    pub fn create_drop_batch(&mut self, drops: Vec<ExtClaimedDropData>) {
        for drop in drops {
            self.create_drop(drop);
        }
    }
}
