use crate::*;

#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum ExtClaimedDrop {
    token(ExtClaimedTokenDropData),
    nft(ExtClaimedNFTDropData),
    multichain(ExtClaimedNFTDropData),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtClaimedNFTDropData {
    pub found_scavenger_ids: Option<Vec<String>>,
    pub name: String,
    pub image: String,
    pub nft_metadata: TokenMetadata,
    pub drop_id: DropId,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtClaimedTokenDropData {
    pub found_scavenger_ids: Option<Vec<String>>,
    pub name: String,
    pub image: String,
    pub amount: U128,
    pub drop_id: DropId,
}

#[allow(non_camel_case_types)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum ExtDropData {
    token(ExtTokenDropData),
    nft(ExtNFTDropData),
    multichain(ExtNFTDropData),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtNFTDropData {
    pub name: String,
    pub image: String,
    pub nft_metadata: TokenMetadata,
    pub scavenger_hunt: Option<Vec<String>>,
    pub drop_id: DropId,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtTokenDropData {
    pub name: String,
    pub image: String,
    pub scavenger_hunt: Option<Vec<String>>,
    pub amount: U128,
    pub drop_id: DropId,
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

    /// Generic function to retrieve claimed drops for a specific account based on a filter.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account to retrieve the claimed drops for.
    /// * `filter` - A closure that determines whether a drop should be included in the result.
    ///
    /// # Returns
    ///
    /// A vector of `ExtClaimedDrop` containing the drops that match the filter criteria.
    fn get_claimed_drops<F>(&self, account_id: AccountId, filter: F) -> Vec<ExtClaimedDrop>
    where
        F: Fn(&DropData, &Option<Vec<String>>) -> bool,
    {
        let mut result_drops = Vec::new();

        // Retrieve the claimed drops for the account
        if let Some(claimed_drops) = self
            .account_details_by_id
            .get(&account_id)
            .map(|d| d.drops_claimed)
        {
            // Iterate over the claimed drops
            for (drop_id, claimed_drop) in claimed_drops.iter() {
                // Get the drop information associated with the drop ID
                let drop_data = self
                    .get_drop_information(drop_id.clone())
                    .expect("No drop ID found");

                // Apply the filter to determine if this drop should be included
                if filter(&drop_data, &claimed_drop.get_found_scavenger_ids()) {
                    // Match the drop data to either NFT or Token and push the result to the vector
                    match claimed_drop {
                        ClaimedDropData::Multichain(found_scavenger_ids) => {
                            if let DropData::Multichain(nft_data) = drop_data {
                                result_drops.push(ExtClaimedDrop::multichain(
                                    ExtClaimedNFTDropData {
                                        found_scavenger_ids: found_scavenger_ids.clone(),
                                        name: nft_data.base.name.clone(),
                                        image: nft_data.base.image.clone(),
                                        nft_metadata: self
                                            .series_by_id
                                            .get(&nft_data.series_id)
                                            .unwrap()
                                            .metadata,
                                        drop_id: drop_id.clone(),
                                    },
                                ));
                            }
                        }
                        ClaimedDropData::Nft(found_scavenger_ids) => {
                            if let DropData::Nft(nft_data) = drop_data {
                                result_drops.push(ExtClaimedDrop::nft(ExtClaimedNFTDropData {
                                    found_scavenger_ids: found_scavenger_ids.clone(),
                                    name: nft_data.base.name.clone(),
                                    image: nft_data.base.image.clone(),
                                    nft_metadata: self
                                        .series_by_id
                                        .get(&nft_data.series_id)
                                        .unwrap()
                                        .metadata,
                                    drop_id: drop_id.clone(),
                                }));
                            }
                        }
                        ClaimedDropData::Token(found_scavenger_ids) => {
                            if let DropData::Token(token_data) = drop_data {
                                result_drops.push(ExtClaimedDrop::token(ExtClaimedTokenDropData {
                                    found_scavenger_ids: found_scavenger_ids.clone(),
                                    name: token_data.base.name.clone(),
                                    image: token_data.base.image.clone(),
                                    amount: token_data.amount,
                                    drop_id: drop_id.clone(),
                                }));
                            }
                        }
                    }
                }
            }
        }

        result_drops
    }

    /// Retrieves all claimed drops for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account to retrieve the claimed drops for.
    ///
    /// # Returns
    ///
    /// A vector of `ExtClaimedDrop` containing all the claimed drops for the account.
    pub fn get_claimed_drops_for_account(&self, account_id: AccountId) -> Vec<ExtClaimedDrop> {
        // No filtering; retrieve all claimed drops
        self.get_claimed_drops(account_id, |_, _| true)
    }

    /// Retrieves claimed drops that have associated scavenger items for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account to retrieve the claimed drops for.
    ///
    /// # Returns
    ///
    /// A vector of `ExtClaimedDrop` containing the drops that have scavenger items found.
    pub fn get_claimed_scavengers_for_account(&self, account_id: AccountId) -> Vec<ExtClaimedDrop> {
        // Filter for drops that have scavenger items found
        self.get_claimed_drops(account_id, |_, scavenger_ids| scavenger_ids.is_some())
    }

    /// Retrieves claimed NFT drops for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account to retrieve claimed NFTs for.
    ///
    /// # Returns
    ///
    /// A vector of `ExtClaimedDrop` containing the claimed NFT drops for the account.
    pub fn get_claimed_nfts_for_account(&self, account_id: AccountId) -> Vec<ExtClaimedDrop> {
        // Filter for NFT drops only
        self.get_claimed_drops(account_id, |drop, _| drop.is_nft_drop())
    }

    /// Retrieves all the drops created by a given account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account that created the drops.
    ///
    /// # Returns
    ///
    /// A vector of `DropData` containing the drops created by the account.
    pub fn get_drops_created_by_account(&self, account_id: AccountId) -> Vec<DropData> {
        let mut drops_created = Vec::new();
        // Retrieve the account details to get the list of drops created
        let account_details = self
            .account_details_by_id
            .get(&account_id)
            .expect("No account found");
        // Iterate over the drops and retrieve the drop data
        for drop in account_details.drops_created.iter() {
            drops_created.push(self.drop_by_id.get(&drop).unwrap());
        }
        drops_created
    }

    /// Retrieves the total number of drops in the contract.
    ///
    /// # Returns
    ///
    /// The total number of drops as a `u64`.
    pub fn get_num_drops(&self) -> u64 {
        self.drop_by_id.len()
    }

    /// Retrieves a paginated list of drops.
    ///
    /// # Arguments
    ///
    /// * `from_index` - The starting index for pagination.
    /// * `limit` - The maximum number of drops to retrieve.
    ///
    /// # Returns
    ///
    /// A vector of `ExtDropData` containing the drops.
    pub fn get_drops(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<ExtDropData> {
        // Where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // Iterate through each drop using an iterator
        self.drop_by_id
            .keys()
            // Skip to the index we specified in the start variable
            .skip(start as usize)
            // Take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            // Filter out the drops that are not NFTs
            .filter_map(|drop_id| {
                self.get_drop_information(drop_id.clone())
                    .map(|drop| match drop {
                        DropData::Multichain(nft_data) => ExtDropData::multichain(ExtNFTDropData {
                            name: nft_data.base.name.clone(),
                            image: nft_data.base.image.clone(),
                            nft_metadata: self
                                .series_by_id
                                .get(&nft_data.series_id)
                                .unwrap()
                                .metadata,
                            drop_id: nft_data.base.id.clone(),
                            scavenger_hunt: nft_data.base.scavenger_hunt.map(|scav_data| {
                                scav_data
                                    .iter()
                                    .map(|s| s.description.clone())
                                    .collect::<Vec<String>>()
                            }),
                        }),
                        DropData::Nft(nft_data) => ExtDropData::nft(ExtNFTDropData {
                            name: nft_data.base.name.clone(),
                            image: nft_data.base.image.clone(),
                            nft_metadata: self
                                .series_by_id
                                .get(&nft_data.series_id)
                                .unwrap()
                                .metadata,
                            drop_id: nft_data.base.id.clone(),
                            scavenger_hunt: nft_data.base.scavenger_hunt.map(|scav_data| {
                                scav_data
                                    .iter()
                                    .map(|s| s.description.clone())
                                    .collect::<Vec<String>>()
                            }),
                        }),
                        DropData::Token(token_data) => ExtDropData::token(ExtTokenDropData {
                            name: token_data.base.name.clone(),
                            image: token_data.base.image.clone(),
                            amount: token_data.amount,
                            drop_id: token_data.base.id.clone(),
                            scavenger_hunt: token_data.base.scavenger_hunt.map(|scav_data| {
                                scav_data
                                    .iter()
                                    .map(|s| s.description.clone())
                                    .collect::<Vec<String>>()
                            }),
                        }),
                    })
            })
            // Since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}
