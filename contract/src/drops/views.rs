use crate::*;

#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum ExtClaimedDrop {
    token(ExtClaimedDropData),
    nft(ExtClaimedDropData),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtClaimedDropData {
    pub found_scavenger_ids: ScavengerHuntIds,
    pub drop_id: DropId
}

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
    pub fn get_drop_information(&self, drop_id: String) -> Option<DropData> {
        self.drop_by_id.get(&drop_id)
    }

    /// Retrieves any drops that have some number of claimed scavenger items for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account to retrieve claimed scavengers for.
    ///
    /// # Returns
    ///
    /// A vector of `DropData` containing the drops that have at least one scavenger item found.
    pub fn get_claimed_scavengers_for_account(
        &self,
        account_id: AccountId,
    ) -> Vec<ExtClaimedDrop> {
        let mut result_scavs = Vec::new();
        if let Some(claimed_drops) = self.claims_by_account.get(&account_id) {
            for (drop_id, claimed_drop) in claimed_drops.iter() {
                match claimed_drop {
                    ClaimedDropData::nft(found_scavenger_ids) => {
                        if found_scavenger_ids.is_some() {
                            result_scavs.push(ExtClaimedDrop::nft(ExtClaimedDropData { found_scavenger_ids, drop_id }));
                        }
                    }
                    ClaimedDropData::token(found_scavenger_ids) => {
                        if found_scavenger_ids.is_some() {
                            result_scavs.push(ExtClaimedDrop::token(ExtClaimedDropData { found_scavenger_ids, drop_id }));
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
    /// A vector of `DropData` containing the claimed NFT drops for the account.
    pub fn get_claimed_nfts_for_account(&self, account_id: AccountId) -> Vec<ExtClaimedDrop> {
        let mut result_nfts = Vec::new();
        if let Some(claimed_drops) = self.claims_by_account.get(&account_id) {
            for (drop_id, claimed_drop) in claimed_drops.iter() {

                if let ClaimedDropData::nft(found_scavenger_ids) = claimed_drop {
                    result_nfts.push(ExtClaimedDrop::nft(ExtClaimedDropData { found_scavenger_ids, drop_id }));
                }
            }
        }
        result_nfts
    }
}
