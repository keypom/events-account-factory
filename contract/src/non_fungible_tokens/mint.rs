use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn internal_create_series(&mut self, nft_metadata: &TokenMetadata, creator_id: &AccountId) -> SeriesId {
        let series_id = self
            .series_by_id
            .len();

        let tokens = UnorderedSet::new(StorageKeys::SeriesByIdInner { account_id_hash: env::sha256_array(format!("{}{}", creator_id, series_id).as_bytes()), });
        let series = Series {
            metadata: nft_metadata.clone(),
            royalty: None,
            tokens
        };

        require!(self.series_by_id.insert(&series_id, &series).is_none(), "Series ID already exists on the contract");
        series_id
    }

    pub(crate) fn internal_delete_series(&mut self, series_id: SeriesId) {
        let mut series = self.series_by_id.remove(&series_id).expect("No series found with given ID");
        series.tokens.clear();
    }

    pub(crate) fn internal_nft_mint(&mut self, series_id: SeriesId, receiver_id: AccountId) {
        let mut series = self.series_by_id.get(&series_id).expect("No series found with given ID");
        let cur_len = series.tokens.len();
        // Ensure we haven't overflowed on the number of copies minted
        if let Some(copies) = series.metadata.copies {
            require!(
                cur_len < copies,
                "cannot mint anymore NFTs for the given series. Limit reached"
            );
        }

        let token_id = format!("{}:{}", series_id, cur_len + 1);
        series.tokens.insert(&token_id);
        self.series_by_id.insert(&series_id, &series);

        //specify the token struct that contains the owner ID
        let token = Token {
            // Series ID that the token belongs to
            series_id,
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        require!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());
    }
}