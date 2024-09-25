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

use std::convert::TryInto;

pub(crate) fn verify_signature(
    signature: Base64VecU8,
    caller_id: AccountId,
    expected_key: PublicKey,
) -> bool {
    // Extract the key bytes without the curve type prefix
    let key_bytes = expected_key.as_bytes();
    let key_bytes_without_prefix = &key_bytes[1..]; // Skip the first byte

    // Convert the key bytes slice to a reference to a 32-byte array
    let key_bytes_array: &[u8; 32] = key_bytes_without_prefix
        .try_into()
        .expect("Invalid key length");

    // Serialize the public key to base58
    let expected_key_base58 = bs58::encode(key_bytes_without_prefix).into_string();

    // The message that should have been signed
    let expected_message = format!("{},{}", caller_id, expected_key_base58);

    // Convert the signature into a 64-byte array
    let sig_bytes =
        vec_to_64_byte_array(signature.clone().into()).expect("Invalid signature length");

    // Verify the signature using the key bytes array
    let is_valid = env::ed25519_verify(&sig_bytes, expected_message.as_bytes(), key_bytes_array);

    if !is_valid {
        env::log_str(
            format!(
                "Invalid signature. Expected message: {}, signature: {:?}, public_key: {}",
                expected_message, signature, expected_key_base58,
            )
            .as_str(),
        );
    }

    is_valid
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
