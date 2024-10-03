use crate::*;
use near_sdk::CryptoHash;

pub type DropId = String;
pub type ClaimedDropData = Option<Vec<PublicKey>>;

#[near]
#[derive(BorshStorageKey)]
pub enum StorageKeys {
    AttendeeTicketInformation,
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

#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub enum AccountStatus {
    Basic,
    Sponsor,
    DataSetter,
    Admin,
}

impl AccountStatus {
    pub fn is_admin(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Sponsor => false,
            AccountStatus::DataSetter => false,
            AccountStatus::Admin => true,
        }
    }

    pub fn is_sponsor(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Sponsor => true,
            AccountStatus::DataSetter => false,
            AccountStatus::Admin => true,
        }
    }

    pub fn is_data_sponsor(&self) -> bool {
        match self {
            AccountStatus::Basic => false,
            AccountStatus::Sponsor => false,
            AccountStatus::DataSetter => true,
            AccountStatus::Admin => true,
        }
    }
}

/// Data for each ticket such as the account status, starting balances, etc...
#[near(serializers = [json, borsh])]
pub struct ExtAccountDetails {
    pub account_id: String,
    pub ft_balance: NearToken,
    pub ft_collected: NearToken,

    // ------------------------ Account Information ------------------------- //
    pub account_status: Option<AccountStatus>,
}

/// Data for each ticket such as the account status, starting balances, etc...
#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub struct AttendeeTicketInformation {
    pub has_scanned: bool,
    pub drop_id: Option<DropId>,
    pub account_id: Option<AccountId>,
    pub metadata: Option<String>,
}

/// Data for each ticket such as the account status, starting balances, etc...
#[derive(Clone)]
#[near(serializers = [json, borsh])]
pub struct TicketType {
    pub starting_near_balance: NearToken,
    pub starting_token_balance: NearToken,
    pub account_type: AccountStatus,
}

/// Data for each ticket such as the account status, starting balances, etc...
#[near(serializers = [borsh])]
pub struct AccountDetails {
    pub account_status: Option<AccountStatus>,

    // ------------------------ Fungible Tokens ---------------------------- //
    pub ft_balance: NearToken,

    // ------------------------ Leaderboard -------------------------------- //
    pub tokens_collected: NearToken,

    // ------------------------ Drops -------------------------------------- //
    pub drops_created: IterableSet<DropId>,

    /// Represents what the user has claimed for a specific drop. If scavenger IDs is none, the drop contains no scavengers
    /// If scavengers is Some, the drop needs X amount of scavenger Ids to be found before the reward is allocated
    /// On the frontend, query for drop data to see how many are needed and cross reference with this data structure to see
    /// how many more IDs are left to be found
    /// This is done to optimize the contract and not require duplicate data to be stored
    pub drops_claimed: IterableMap<DropId, ClaimedDropData>,
}

impl AccountDetails {
    pub fn new(account_id: &AccountId) -> AccountDetails {
        let drops_created = IterableSet::new(StorageKeys::DropIdsByCreatorInner {
            account_id_hash: hash_string(&account_id.to_string()),
        });
        let drops_claimed = IterableMap::new(StorageKeys::DropsClaimedByAccountInner {
            account_id_hash: hash_string(&account_id.to_string()),
        });

        AccountDetails {
            ft_balance: NearToken::from_yoctonear(0),
            tokens_collected: NearToken::from_yoctonear(0),
            account_status: None,
            drops_created,
            drops_claimed,
        }
    }
}
