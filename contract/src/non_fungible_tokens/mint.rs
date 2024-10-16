use crate::*;

#[near]
impl Contract {
    pub(crate) fn internal_delete_series(&mut self, series_id: SeriesId) {
        let mut series = self
            .series_by_id
            .remove(&series_id)
            .expect("No series found with given ID");
        series.tokens.clear();
    }

    pub(crate) fn internal_nft_mint(&mut self, series_id: SeriesId, receiver_id: AccountId) {
        let series = self
            .series_by_id
            .get_mut(&series_id)
            .expect("No series found with given ID");

        let cur_len = series.tokens.len();
        // Ensure we haven't overflowed on the number of copies minted
        if let Some(copies) = series.metadata.copies {
            require!(
                u64::from(cur_len) < copies,
                "cannot mint anymore NFTs for the given series. Limit reached"
            );
        }

        let token_id = format!("{}:{}", series_id, cur_len + 1);
        series.tokens.insert(token_id.clone()); // Clone token_id since it's used later

        //specify the token struct that contains the owner ID
        let token = Token {
            // Series ID that the token belongs to
            series_id,
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id.clone(), // Clone receiver_id since it's used later
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        require!(
            self.nft_tokens_by_id
                .insert(token_id.clone(), token)
                .is_none(), // Clone token_id since it's used later
            "Token already exists"
        );

        //call the internal method for adding the token to the owner
        let num_tokens = self.internal_add_token_to_owner(&receiver_id, &token_id); // Use cloned receiver_id and token_id
                                                                                    // Update leaderboard
        self.update_poap_leaderboard(&receiver_id, num_tokens); // Use cloned receiver_id

        self.total_transactions += 1;

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: receiver_id.to_string(), // Use cloned receiver_id
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()], // Use cloned token_id
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());
    }
}
