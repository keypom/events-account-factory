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
    drop_name: String,
    drop_id: DropId,
) -> bool {
    let pk = env::signer_account_pk();

    let expected_message = format!(
        "{},{},{},{}",
        caller_id,
        serde_json::to_string(&pk).unwrap(),
        drop_name,
        drop_id
    );

    // Verify the signature is the valid message and signed by the linkdrop PK
    let pk_bytes = pk_to_32_byte_array(&pk).unwrap();
    let sig_bytes = vec_to_64_byte_array(signature.into()).unwrap();
    let is_valid = env::ed25519_verify(&sig_bytes, expected_message.as_bytes(), pk_bytes);

    is_valid
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

