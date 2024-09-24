use near_sdk::Promise;

use crate::*;

pub(crate) fn on_storage_cleared(transfer_account_id: AccountId, bytes_cleared: u64) {
    let amount = env::storage_byte_cost()
        .checked_mul(bytes_cleared as u128)
        .expect("invalid storage cost");

    near_sdk::log!("Cleared {} bytes. Refunding {}.", bytes_cleared, amount);
    Promise::new(transfer_account_id).transfer(amount);
}
