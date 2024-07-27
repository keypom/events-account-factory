use crate::*;

#[near_bindgen]
impl Contract {
    /// Query for the information of a specific drop.
    ///
    /// # Arguments
    ///
    /// * `drop_id` - The ID of the drop to retrieve information for.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `InternalDropData` if the drop is found, otherwise `None`.
    pub fn get_drop_information(&self, drop_id: String) -> Option<InternalDropData> {
        self.drop_by_id.get(&drop_id)
    }

    /// Retrieves the claimed scavenger items for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account to retrieve claimed scavengers for.
    ///
    /// # Returns
    ///
    /// A vector of `ExtClaimedDropData` containing the claimed scavenger items for the account.
    ///
    /// # Panics
    ///
    /// Panics if a claimed drop does not have a corresponding drop ID in the internal storage.
    pub fn get_claimed_scavengers_for_account(
        &self,
        account_id: AccountId,
    ) -> Vec<ExtClaimedDropData> {
        let mut result_scavs = Vec::new();
        if let Some(claimed_drops) = self.claims_by_account.get(&account_id) {
            for claimed_drop in claimed_drops.values() {
                if let Some(scavenger_ids) = &claimed_drop.scavenger_ids {
                    let internal_drop_data = self.drop_by_id.get(&claimed_drop.id).expect("No drop with corresponding ID found");

                    match internal_drop_data {
                        InternalDropData::token(internal_token_drop) => {
                            result_scavs.push(ExtClaimedDropData::token(ExtClaimedTokenDropData {
                                base: ExtClaimedDropBase {
                                    id: internal_token_drop.base.base.id.clone(),
                                    scavenger_ids: Some(scavenger_ids.clone()),
                                    name: internal_token_drop.base.base.name.clone(),
                                    image: internal_token_drop.base.base.image.clone(),
                                },
                                amount: internal_token_drop.amount,
                            }));
                        }
                        InternalDropData::nft(internal_nft_drop) => {
                            result_scavs.push(ExtClaimedDropData::nft(ExtClaimedNFTDropData {
                                base: ExtClaimedDropBase {
                                    id: internal_nft_drop.base.base.id.clone(),
                                    scavenger_ids: Some(scavenger_ids.clone()),
                                    name: internal_nft_drop.base.base.name.clone(),
                                    image: internal_nft_drop.base.base.image.clone(),
                                },
                                series_id: internal_nft_drop.series_id.clone(),
                            }));
                        }
                    }
                }
            }
        }
        result_scavs
    }

    /// Retrieves the claimed NFTs for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account to retrieve claimed NFTs for.
    ///
    /// # Returns
    ///
    /// A vector of `ExtClaimedDropData` containing the claimed NFTs for the account.
    ///
    /// # Panics
    ///
    /// Panics if a claimed drop does not have a corresponding drop ID in the internal storage.
    pub fn get_claimed_nfts_for_account(&self, account_id: AccountId) -> Vec<ExtClaimedDropData> {
        let mut result_nfts = Vec::new();
        if let Some(claimed_drops) = self.claims_by_account.get(&account_id) {
            for claimed_drop in claimed_drops.values() {
                let internal_drop_data = self.drop_by_id.get(&claimed_drop.id).expect("No drop with corresponding ID found");

                if let InternalDropData::nft(internal_nft_drop) = internal_drop_data {
                    result_nfts.push(ExtClaimedDropData::nft(ExtClaimedNFTDropData {
                        base: ExtClaimedDropBase {
                            id: internal_nft_drop.base.base.id.clone(),
                            scavenger_ids: claimed_drop.scavenger_ids.clone(),
                            name: internal_nft_drop.base.base.name.clone(),
                            image: internal_nft_drop.base.base.image.clone(),
                        },
                        series_id: internal_nft_drop.series_id.clone(),
                    }));
                }
            }
        }
        result_nfts
    }
}
