use std::convert::TryFrom;

use crate::*;

/// Take a drop ID and return the creator's account ID and the drop ID based on the delimiter.
pub fn parse_drop_id(drop_id: &DropId) -> AccountId {
    let split: Vec<&str> = drop_id.split(DROP_DELIMITER).collect();
    if split.len() != 2 {
        panic!("Invalid drop ID");
    }

    AccountId::try_from(split[0].to_string()).expect("invalid account Id")
}

pub(crate) fn verify_signature(
    signature: Base64VecU8,
    caller_id: AccountId,
    expected_key: PublicKey,
) -> bool {
    // Serialize the public key to base58
    let expected_key_base58 = bs58::encode(expected_key.as_bytes()).into_string();

    // The message that should have been signed
    let expected_message = format!("{},{}", caller_id, expected_key_base58);

    // Convert the public key and signature into byte arrays
    let pk_bytes = pk_to_32_byte_array(&expected_key).expect("Invalid public key length");
    let sig_bytes = vec_to_64_byte_array(signature.into()).expect("Invalid signature length");

    // Verify the signature
    env::ed25519_verify(&sig_bytes, expected_message.as_bytes(), &pk_bytes)
}

pub(crate) fn pk_to_32_byte_array(pk: &PublicKey) -> Option<&[u8; 32]> {
    let len = pk.as_bytes().len();
    // Check if the string is exactly 32 or 33 bytes
    if len != 32 && len != 33 {
        return None;
    }

    // Explicitly import TryInto trait
    use std::convert::TryInto;

    // if the public key has the prefix appended, remove it to ensure it's 32 bytes
    if len == 33 {
        return pk.as_bytes()[1..33].try_into().ok();
    }

    pk.as_bytes()[0..32].try_into().ok()
}

pub(crate) fn vec_to_64_byte_array(vec: Vec<u8>) -> Option<[u8; 64]> {
    // Check if the string is exactly 64 bytes
    if vec.len() != 64 {
        return None;
    }

    // Explicitly import TryInto trait
    use std::convert::TryInto;

    let array: [u8; 64] = vec
        .try_into() // Try to convert the Vec<u8> into a fixed-size array
        .expect("Vec with incorrect length"); // This expect will never panic due to the above length check

    Some(array)
}
