use crate::*;

#[near_bindgen]
impl Contract {
    /// Query for the total amount of tokens currently circulating.
    pub fn get_drop_information(&self, drop_id: String) -> Option<InternalDropData> {
        self.drop_by_id.get(&drop_id)
    }

    pub fn get_claimed_scavengers_for_account(
        &self,
        account_id: AccountId,
    ) -> Vec<ExtClaimedDropData> {
        let mut result_scavs = Vec::new();
        if let Some(claimed_drops) = self.claims_by_account.get(&account_id) {
            for claimed_drop in claimed_drops.values() {
                if claimed_drop.scavenger_ids.is_some() {
                    let internal_drop_data = self.drop_by_id.get(&claimed_drop.id).expect("No drop with corresponding ID found");

                    match internal_drop_data {
                        InternalDropData::token(internal_token_drop) => {
                            result_scavs.push(ExtClaimedDropData::token(ExtClaimedTokenDropData { id: internal_token_drop.id, scavs_found: claimed_drop.scavenger_ids, amount: internal_token_drop.amount, name: internal_token_drop.name, image: internal_token_drop.image }));
                        }
                        InternalDropData::nft(internal_nft_drop) => {
                            result_scavs.push(ExtClaimedDropData::nft(ExtClaimedNFTDropData { id: internal_nft_drop.id, scavs_found: claimed_drop.scavenger_ids, series_id: internal_nft_drop.series_id, name: internal_nft_drop.name, image: internal_nft_drop.image }));
                        }
                    }
                }
            }
        }
        result_scavs
    }

    pub fn get_claimed_nfts_for_account(&self, account_id: AccountId) -> Vec<ExtClaimedDropData> {
        let mut result_nfts = Vec::new();
        if let Some(claimed_drops) = self.claims_by_account.get(&account_id) {
            for claimed_drop in claimed_drops.values() {
                let internal_drop_data = self.drop_by_id.get(&claimed_drop.id).expect("No drop with corresponding ID found");

                if let InternalDropData::nft(internal_nft_drop) = internal_drop_data {
                    result_nfts.push(ExtClaimedDropData::nft(ExtClaimedNFTDropData { id: internal_nft_drop.id, scavs_found: claimed_drop.scavenger_ids, series_id: internal_nft_drop.series_id, name: internal_nft_drop.name, image: internal_nft_drop.image }));
                }
            }
            
        }
        result_nfts
    }
}
