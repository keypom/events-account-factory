use crate::*;

/// Used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub fn hash_string(string: &String) -> CryptoHash {
    env::sha256_array(string.as_bytes())
}