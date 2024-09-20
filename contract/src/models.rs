use crate::*;
use near_sdk::CryptoHash;

pub type DropId = String;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKeys {
    AttendeeTicketInformation,
    VendorItems { vendor_id_hash: CryptoHash },
    AccountDetailsById,
    DropsClaimedByAccountInner { account_id_hash: CryptoHash },
    DropById,
    TokensForOwner,
    TokensForOwnerInner { account_id_hash: CryptoHash },
    DropIdsByCreatorInner { account_id_hash: CryptoHash },
    TicketDataById,
    SeriesById,
    SeriesByIdInner { account_id_hash: CryptoHash },
    TokensById,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum AccountStatus {
    Basic,
    Vendor,
    Sponsor,
    DataSetter,
    Admin,
}

impl AccountStatus {
    pub fn is_admin(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Vendor => false,
            AccountStatus::Sponsor => false,
            AccountStatus::DataSetter => false,
            AccountStatus::Admin => true,
        }
    }

    pub fn is_sponsor(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Vendor => false,
            AccountStatus::Sponsor => true,
            AccountStatus::DataSetter => false,
            AccountStatus::Admin => true,
        }
    }

    pub fn is_vendor(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Vendor => true,
            AccountStatus::Sponsor => false,
            AccountStatus::DataSetter => false,
            AccountStatus::Admin => true,
        }
    }

    pub fn is_data_sponsor(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Vendor => false,
            AccountStatus::Sponsor => false,
            AccountStatus::DataSetter => true,
            AccountStatus::Admin => true,
        }
    }
}

/// Data for each ticket such as the account status, starting balances, etc...
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtAccountDetails {
    pub account_id: String,
    pub ft_balance: U128,

    // ------------------------ Vendor Information ------------------------- //
    pub vendor_data: Option<VendorMetadata>,
    pub account_status: Option<AccountStatus>,
}

/// Data for each ticket such as the account status, starting balances, etc...
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AttendeeTicketInformation {
    pub has_scanned: bool,
    pub drop_id: Option<DropId>,
    pub account_id: Option<AccountId>,
    pub metadata: Option<String>,
}

/// Data for each ticket such as the account status, starting balances, etc...
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketType {
    pub starting_near_balance: U128,
    pub starting_token_balance: U128,
    pub account_type: AccountStatus,
}

/// Data for each ticket such as the account status, starting balances, etc...
#[derive(BorshSerialize, BorshDeserialize)]
pub struct AccountDetails {
    // ------------------------ Fungible Tokens ---------------------------- //
    pub ft_balance: Balance,

    // ------------------------ Vendor Information ------------------------- //
    pub vendor_data: Option<VendorInformation>,
    pub account_status: Option<AccountStatus>,

    // ------------------------ Drops -------------------------------------- //
    pub drops_created: UnorderedSet<DropId>,
    pub drops_claimed: UnorderedMap<DropId, ClaimedDropData>,
}

impl AccountDetails {
    pub fn new(account_id: &AccountId) -> AccountDetails {
        let drops_created = UnorderedSet::new(StorageKeys::DropIdsByCreatorInner {
            account_id_hash: hash_string(&account_id.to_string()),
        });
        let drops_claimed = UnorderedMap::new(StorageKeys::DropsClaimedByAccountInner {
            account_id_hash: hash_string(&account_id.to_string()),
        });

        AccountDetails {
            ft_balance: 0,
            vendor_data: None,
            account_status: None,
            drops_created,
            drops_claimed,
        }
    }
}
