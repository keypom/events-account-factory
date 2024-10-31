use crate::*;

#[allow(non_camel_case_types)]
#[derive(Clone)]
#[near(serializers = [json, borsh])]
#[serde(tag = "type")]
pub enum ExtDropData {
    Token(ExtTokenDropData),
    Multichain(MultichainDropData),
    Nft(NFTDropData),
}

#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub struct ExtTokenDropData {
    pub id: String,
    pub key: PublicKey,
    pub name: String,
    pub image: String,
    pub scavenger_hunt: Option<Vec<ScavengerHuntData>>,
    pub num_claimed: u64,
    pub creator_has_funds: bool,

    pub token_amount: U128,
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
#[near(serializers = [json, borsh])]
#[serde(tag = "type")]
pub enum ExtClaimedDrop {
    token(ExtClaimedTokenDropData),
    nft(ExtClaimedNFTDropData),
    multichain(ExtClaimedMultichainDropData),
}

#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub struct ExtClaimedNFTDropData {
    pub found_scavenger_ids: ScavengerKeys,
    pub needed_scavenger_ids: Option<Vec<ScavengerHuntData>>,
    pub name: String,
    pub image: String,
    pub nft_metadata: TokenMetadata,
    pub drop_id: DropId,
    pub key: PublicKey,
}

#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub struct ExtClaimedMultichainDropData {
    pub found_scavenger_ids: ScavengerKeys,
    pub needed_scavenger_ids: Option<Vec<ScavengerHuntData>>,
    pub name: String,
    pub image: String,
    pub drop_id: DropId,
    pub mc_metadata: MultichainMetadata,
    pub nft_metadata: TokenMetadata,
    pub key: PublicKey,
}

#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub struct ExtClaimedTokenDropData {
    pub found_scavenger_ids: ScavengerKeys,
    pub needed_scavenger_ids: Option<Vec<ScavengerHuntData>>,
    pub creator_has_funds: bool,
    pub name: String,
    pub image: String,
    pub token_amount: U128,
    pub drop_id: DropId,
    pub key: PublicKey,
}

#[near]
impl Contract {
    /// Query for the information of a specific drop.
    ///
    /// # Arguments
    ///
    /// * `drop_id` - The ID of the drop to retrieve information for.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `ExtDropData` if the drop is found, otherwise `None`.
    pub fn get_drop_information(&self, drop_id: String) -> Option<ExtDropData> {
        // Get the DropData
        if let Some(drop_data) = self.drop_by_id.get(&drop_id) {
            // Parse the creator_id from the drop_id
            let creator_id = parse_drop_id(&drop_id);
            // Get the creator's account details
            if let Some(creator) = self.account_details_by_id.get(&creator_id) {
                // Now, depending on the type of drop, construct the appropriate ExtDropData
                match drop_data {
                    DropData::Token(token_data) => {
                        // Compute creator_has_funds
                        let creator_has_funds = match creator.account_status {
                            Some(AccountStatus::Admin) => true,
                            _ => creator.ft_balance.as_yoctonear() >= token_data.token_amount.0,
                        };
                        // Construct ExtTokenDropData
                        let ext_token_data = ExtTokenDropData {
                            id: token_data.id.clone(),
                            key: token_data.key.clone(),
                            name: token_data.name.clone(),
                            image: token_data.image.clone(),
                            scavenger_hunt: token_data.scavenger_hunt.clone(),
                            num_claimed: token_data.num_claimed,
                            creator_has_funds,
                            token_amount: token_data.token_amount,
                        };
                        Some(ExtDropData::Token(ext_token_data))
                    }
                    DropData::Nft(nft_data) => {
                        // Construct NFTDropData (no creator_has_funds needed)
                        let ext_nft_data = NFTDropData {
                            id: nft_data.id.clone(),
                            key: nft_data.key.clone(),
                            name: nft_data.name.clone(),
                            image: nft_data.image.clone(),
                            scavenger_hunt: nft_data.scavenger_hunt.clone(),
                            num_claimed: nft_data.num_claimed,
                            nft_metadata: nft_data.nft_metadata.clone(),
                            nft_series_id: nft_data.nft_series_id,
                        };
                        Some(ExtDropData::Nft(ext_nft_data))
                    }
                    DropData::Multichain(mc_data) => {
                        // Construct MultichainDropData (no creator_has_funds needed)
                        let ext_mc_data = MultichainDropData {
                            id: mc_data.id.clone(),
                            key: mc_data.key.clone(),
                            name: mc_data.name.clone(),
                            image: mc_data.image.clone(),
                            scavenger_hunt: mc_data.scavenger_hunt.clone(),
                            num_claimed: mc_data.num_claimed,
                            nft_metadata: mc_data.nft_metadata.clone(),
                            mc_metadata: mc_data.mc_metadata.clone(),
                        };
                        Some(ExtDropData::Multichain(ext_mc_data))
                    }
                }
            } else {
                // Creator not found
                None
            }
        } else {
            // DropData not found
            None
        }
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
        F: Fn(&DropData, &ClaimedDropData) -> bool, // Use DropData here
    {
        let mut result_drops = Vec::new();

        // Retrieve the claimed drops for the account
        if let Some(account_details) = self.account_details_by_id.get(&account_id) {
            // Borrow the drops_claimed rather than moving it
            let claimed_drops = &account_details.drops_claimed;

            // Iterate over the claimed drops if they exist
            for (drop_id, found_scavenger_ids) in claimed_drops.iter() {
                // Convert drop_id (which is a &String) to &str
                let drop_id_str: &str = drop_id;

                let creator_id = parse_drop_id(drop_id);
                let creator = self.account_details_by_id.get(&creator_id).unwrap();

                // Get the drop information associated with the drop ID
                if let Some(drop_data) = self.drop_by_id.get(drop_id_str) {
                    // Apply the filter to determine if this drop should be included
                    if filter(drop_data, found_scavenger_ids) {
                        // Now process the data and match it to the claimed drop types
                        match drop_data {
                            DropData::Multichain(_) => {
                                if let DropData::Multichain(multichain_data) = &drop_data {
                                    result_drops.push(ExtClaimedDrop::multichain(
                                        ExtClaimedMultichainDropData {
                                            found_scavenger_ids: found_scavenger_ids.clone(),
                                            needed_scavenger_ids: drop_data.get_scavenger_data(),
                                            name: multichain_data.name.clone(),
                                            image: multichain_data.image.clone(),
                                            mc_metadata: multichain_data.mc_metadata.clone(),
                                            nft_metadata: multichain_data.nft_metadata.clone(),
                                            drop_id: drop_id.to_string(),
                                            key: multichain_data.key.clone(),
                                        },
                                    ));
                                }
                            }
                            DropData::Nft(_) => {
                                if let DropData::Nft(nft_data) = &drop_data {
                                    result_drops.push(ExtClaimedDrop::nft(ExtClaimedNFTDropData {
                                        found_scavenger_ids: found_scavenger_ids.clone(),
                                        needed_scavenger_ids: drop_data.get_scavenger_data(),
                                        name: nft_data.name.clone(),
                                        image: nft_data.image.clone(),
                                        key: nft_data.key.clone(),
                                        nft_metadata: self
                                            .series_by_id
                                            .get(&nft_data.nft_series_id)
                                            .unwrap()
                                            .metadata
                                            .clone(),
                                        drop_id: drop_id.to_string(),
                                    }));
                                }
                            }
                            DropData::Token(_) => {
                                if let DropData::Token(token_data) = &drop_data {
                                    let creator_has_funds = match creator.account_status {
                                        Some(AccountStatus::Admin) => true,
                                        _ => {
                                            creator.ft_balance.as_yoctonear()
                                                >= token_data.token_amount.0
                                        }
                                    };
                                    result_drops.push(ExtClaimedDrop::token(
                                        ExtClaimedTokenDropData {
                                            found_scavenger_ids: found_scavenger_ids.clone(),
                                            image: token_data.image.clone(),
                                            needed_scavenger_ids: drop_data.get_scavenger_data(),
                                            creator_has_funds,
                                            key: token_data.key.clone(),
                                            name: token_data.name.clone(),
                                            token_amount: token_data.token_amount,
                                            drop_id: drop_id.to_string(),
                                        },
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        result_drops
    }

    /// Retrieves a specific claimed drop for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account to retrieve the claimed drops for.
    /// * `drop_id` - The ID of the drop to retrieve.
    ///
    /// # Returns
    ///
    /// A `ExtClaimedDrop` containing the information about the claimed drop for the account.
    pub fn get_claimed_drop_for_account(
        &self,
        account_id: AccountId,
        drop_id: String,
    ) -> ExtClaimedDrop {
        let found_scavenger_ids = self
            .account_details_by_id
            .get(&account_id)
            .expect("Account not scanned in yet")
            .drops_claimed
            .get(&drop_id)
            .expect("Drop not found")
            .clone();

        let drop_data = self
            .drop_by_id
            .get(&drop_id)
            .expect("Drop not found")
            .clone();

        let creator_id = parse_drop_id(&drop_id);
        let creator = self.account_details_by_id.get(&creator_id).unwrap();

        match drop_data {
            DropData::Multichain(_) => {
                if let DropData::Multichain(multichain_data) = &drop_data {
                    ExtClaimedDrop::multichain(ExtClaimedMultichainDropData {
                        found_scavenger_ids: found_scavenger_ids.clone(),
                        image: multichain_data.image.clone(),
                        needed_scavenger_ids: drop_data.get_scavenger_data(),
                        name: multichain_data.name.clone(),
                        mc_metadata: multichain_data.mc_metadata.clone(),
                        nft_metadata: multichain_data.nft_metadata.clone(),
                        key: multichain_data.key.clone(),
                        drop_id: drop_id.clone(),
                    })
                } else {
                    panic!("Drop type mismatch for Multichain");
                }
            }
            DropData::Nft(_) => {
                if let DropData::Nft(nft_data) = &drop_data {
                    ExtClaimedDrop::nft(ExtClaimedNFTDropData {
                        found_scavenger_ids: found_scavenger_ids.clone(),
                        needed_scavenger_ids: drop_data.get_scavenger_data(),
                        image: nft_data.image.clone(),
                        name: nft_data.name.clone(),
                        key: nft_data.key.clone(),
                        nft_metadata: self
                            .series_by_id
                            .get(&nft_data.nft_series_id)
                            .unwrap()
                            .metadata
                            .clone(),
                        drop_id: drop_id.clone(),
                    })
                } else {
                    panic!("Drop type mismatch for NFT");
                }
            }
            DropData::Token(_) => {
                if let DropData::Token(token_data) = &drop_data {
                    let creator_has_funds = match creator.account_status {
                        Some(AccountStatus::Admin) => true,
                        _ => creator.ft_balance.as_yoctonear() >= token_data.token_amount.0,
                    };

                    ExtClaimedDrop::token(ExtClaimedTokenDropData {
                        found_scavenger_ids: found_scavenger_ids.clone(),
                        key: token_data.key.clone(),
                        needed_scavenger_ids: drop_data.get_scavenger_data(),
                        creator_has_funds,
                        image: token_data.image.clone(),
                        name: token_data.name.clone(),
                        token_amount: token_data.token_amount,
                        drop_id: drop_id.clone(),
                    })
                } else {
                    panic!("Drop type mismatch for Token");
                }
            }
        }
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

    /// Retrieves claimed Multichain drops for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The ID of the account to retrieve claimed NFTs for.
    ///
    /// # Returns
    ///
    /// A vector of `ExtClaimedDrop` containing the claimed NFT drops for the account.
    pub fn get_claimed_multichain_nfts_for_account(
        &self,
        account_id: AccountId,
    ) -> Vec<ExtClaimedDrop> {
        // Filter for NFT drops only
        self.get_claimed_drops(account_id, |drop, _| drop.is_multichain_drop())
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
    pub fn get_drops_created_by_account(&self, account_id: AccountId) -> Vec<ExtDropData> {
        let mut drops_created = Vec::new();
        // Retrieve the account details to get the list of drops created
        let account_details = self
            .account_details_by_id
            .get(&account_id)
            .expect("No account found");
        // Iterate over the drops and retrieve the drop data
        for drop in account_details.drops_created.iter() {
            drops_created.push(
                self.get_drop_information(drop.clone())
                    .expect("Drop not found"),
            );
        }
        drops_created
    }

    /// Retrieves the total number of drops in the contract.
    ///
    /// # Returns
    ///
    /// The total number of drops as a `u64`.
    pub fn get_num_drops(&self) -> u32 {
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
            .filter_map(|drop_id| self.get_drop_information(drop_id.clone()))
            // Since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}
