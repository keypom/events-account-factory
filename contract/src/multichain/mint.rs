use crate::*;

// Represent Token Information
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MultichainMetadata {
    // FOR MPC
    pub chain_id: u64,
    // Receiving NFT contract on external chain
    pub contract_id: String,
    // Arguments that I pass in to the NFT mint function call on external chain
    // **NEEDS TO HAVE BEEN CREATED ON THE NFT CONTRACT BEFORE CALLING CREATE DROP**
    pub series_id: SeriesId,
}

#[near_bindgen]
impl Contract {
    pub fn handle_multichain_mint(&mut self, _data: &MultichainDropData) {
        // Future implemntation: user passes in minting account nonce + gas info or constructed multichain transaction, call MPC, return signature from MPC 
    }
}
