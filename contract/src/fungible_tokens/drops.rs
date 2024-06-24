use crate::*;

#[near_bindgen]
impl Contract {
    /// Allows an admin to create a drop so people can scan a QR code and get the amount of tokens
    pub fn create_drop(&mut self, drop: InternalDropData) {
        self.assert_admin();
        let drop_id = drop.get_id();
        self.drop_by_id.insert(&drop_id, &drop);
    }

    /// Allows an admin to create a drop so people can scan a QR code and get the amount of tokens
    pub fn create_drop_batch(&mut self, drops: Vec<InternalDropData>) {
        self.assert_admin();
        for drop in drops {
            let drop_id = drop.get_id();
            self.drop_by_id.insert(&drop_id, &drop);
        }
    }

    /// Allows a user to claim an existing drop (if they haven't already)
    pub fn claim_drop(&mut self, drop_id: String, scavenger_id: Option<String>) {
        let drop_data = self.drop_by_id.get(&drop_id).expect("Drop not found");

        let receiver_id = env::predecessor_account_id();
        let mut claimed_drops = self
            .drops_claimed_by_account
            .get(&receiver_id)
            .expect("User not registered");

        match drop_data {
            InternalDropData::token(data) => {
                self.handle_token_drop(data, &receiver_id, scavenger_id, &mut claimed_drops);
            }
            InternalDropData::nft(data) => {
                self.handle_nft_drop(data, &receiver_id, scavenger_id, &mut claimed_drops);
            }
        }
        self.drops_claimed_by_account
            .insert(&receiver_id, &claimed_drops);
    }

    fn handle_token_drop(
        &mut self,
        data: TokenDropData,
        receiver_id: &AccountId,
        scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<String, Vec<String>>,
    ) {
        match data.scavenger_ids {
            Some(scavenger_ids) => {
                near_sdk::log!("Scavenger IDs: {:?}", &scavenger_ids);
                let mut claimed_scavenger_ids = claimed_drops.get(&data.id).unwrap_or(Vec::new());
                near_sdk::log!("Claimed scavenger IDs: {:?}", &claimed_scavenger_ids);
                near_sdk::log!("Scavenger ID: {:?}", &scavenger_id);

                let scav_id = scavenger_id.expect("Scavenger ID is required");
                require!(
                    !claimed_scavenger_ids.contains(&scav_id),
                    "Scavenger item already claimed"
                );
                claimed_scavenger_ids.push(scav_id);
                claimed_drops.insert(&data.id, &claimed_scavenger_ids);
                if scavenger_ids.len() == claimed_scavenger_ids.len() {
                    // All scavenger items claimed, now claim the main reward
                    self.internal_deposit_mint(receiver_id, data.amount.0);
                }
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(
                    claimed_drops.get(&data.id).is_none(),
                    "Drop already claimed"
                );
                claimed_drops.insert(&data.id, &vec![data.id.clone()]);
                self.internal_deposit_mint(receiver_id, data.amount.0);
            }
        }
    }

    fn handle_nft_drop(
        &mut self,
        data: NFTDropData,
        receiver_id: &AccountId,
        scavenger_id: Option<String>,
        claimed_drops: &mut UnorderedMap<String, Vec<String>>,
    ) {
        match data.scavenger_ids {
            Some(scavenger_ids) => {
                let mut claimed_scavenger_ids = claimed_drops.get(&data.id).unwrap_or(Vec::new());

                let scav_id = scavenger_id.expect("Scavenger ID is required");
                require!(
                    !claimed_scavenger_ids.contains(&scav_id),
                    "Scavenger item already claimed"
                );
                claimed_scavenger_ids.push(scav_id);
                claimed_drops.insert(&data.id, &claimed_scavenger_ids);
                if scavenger_ids.len() == claimed_scavenger_ids.len() {
                    // TODO: actually mint NFT
                }
            }
            None => {
                // Directly claim if no scavenger IDs
                require!(
                    claimed_drops.get(&data.id).is_none(),
                    "Drop already claimed"
                );
                claimed_drops.insert(&data.id, &vec![data.id.clone()]);
                // TODO: actually mint NFT
            }
        }
    }

    /// Query for the total amount of tokens currently circulating.
    pub fn get_drop_information(&self, drop_id: String) -> Option<InternalDropData> {
        self.drop_by_id.get(&drop_id)
    }

    pub fn claims_for_account(&self, account_id: AccountId, drop_id: String) -> Vec<String> {
        self.drops_claimed_by_account
            .get(&account_id)
            .expect("Account not registered")
            .get(&drop_id)
            .unwrap_or(Vec::new())
    }

    pub fn get_scavengers_for_account(
        &self,
        account_id: AccountId,
    ) -> Vec<ScavengerHuntWithOwnership> {
        let mut result_scavs = Vec::new();
        if let Some(claimed_drops) = self.drops_claimed_by_account.get(&account_id) {
            for (_, drop_data) in self.drop_by_id.iter() {
                if let Some(scav_ids) = drop_data.get_scavenger_ids() {
                    let id = drop_data.get_id();
                    let name = drop_data.get_name();
                    let image = drop_data.get_image();
                    let claimed = claimed_drops.get(&id).unwrap_or(Vec::new());

                    result_scavs.push(ScavengerHuntWithOwnership {
                        id,
                        scavenger_ids: scav_ids,
                        found: claimed,
                        name,
                        image,
                    });
                }
            }
        }
        result_scavs
    }

    pub fn get_nfts_for_account(&self, account_id: AccountId) -> Vec<NFTWithOwnership> {
        let mut result_nfts = Vec::new();
        let claimed_drops = self.drops_claimed_by_account.get(&account_id);

        for (_, drop_data) in self.drop_by_id.iter() {
            if let InternalDropData::nft(nft_data) = drop_data {
                near_sdk::log!("Found NFT: {:?}", &nft_data);
                let scavenger_ids_len = nft_data.scavenger_ids.as_ref().map_or(0, Vec::len);
                near_sdk::log!("Scavenger IDs length: {:?}", &scavenger_ids_len);
                let claimed_len = claimed_drops
                    .as_ref()
                    .and_then(|drops| drops.get(&nft_data.id))
                    .map_or(0, |claimed_ids| claimed_ids.len());
                near_sdk::log!("Claimed length: {:?}", &claimed_len);

                let is_owned = if scavenger_ids_len == 0 {
                    claimed_len > 0
                } else {
                    claimed_len == scavenger_ids_len
                };
                result_nfts.push(NFTWithOwnership {
                    nft: nft_data.clone(),
                    is_owned,
                });
            }
        }
        result_nfts
    }
}
