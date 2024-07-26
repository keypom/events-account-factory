use crate::*;
use near_sdk::CryptoHash;

pub type DropId = String;
pub type ScavengerId = String;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKeys {
    DataByVendor,
    AccountIdByPubKey,
    VendorItems { vendor_id_hash: CryptoHash },
    DropsClaimedByAccount,
    DropsClaimedByAccountInner { account_id_hash: CryptoHash },
    DropById,
    AccontStatusById,
    BalanceByAccount,
    DropIdsByCreator,
    TicketDataById,
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum AccountStatus {
    Basic,
    Vendor,
    Sponsor,
    Admin
}

impl AccountStatus {
    pub fn is_admin(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Vendor => false,
            AccountStatus::Sponsor => false,
            AccountStatus::Admin => true,
        }
    }

    pub fn is_sponsor(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Vendor => false,
            AccountStatus::Sponsor => true,
            AccountStatus::Admin => true,
        }
    }

    pub fn is_vendor(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Vendor => true,
            AccountStatus::Sponsor => false,
            AccountStatus::Admin => true,
        }
    }
}

/// Data for each ticket such as the account status, starting balances, etc...
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketType {
    pub starting_near_balance: U128,
    pub starting_token_balance: U128,
    pub account_type: AccountStatus
}