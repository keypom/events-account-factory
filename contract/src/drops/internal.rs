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