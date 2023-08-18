export type ExtNFTKey = {
    owner_id: string;
    token_id: string;
    metadata: TokenMetadata;
    approved_account_ids: Record<string, number>;
    royalty: Record<string, number>;
}

export type ExtKeyInfo = {
   required_gas: string;
   yoctonear: string;
   ft_list: Array<FTListData>;
   nft_list: Array<NFTListData>;
   drop_id: string;
   pub_key: string;
   token_id: string;
   owner_id: string;
   fc_list: Array<FCData>;
   uses_remaining: number
}

export type FCData = {
    methods: Array<MethodData>
}

export type NFTListData = {
    token_id: string;
    contract_id: string;
}

export type FTListData = {
    amount: string;
    contract_id: string;
}

export type ExtDrop = {
    drop_id: string;
    funder_id: string;
    max_key_uses: number;
    asset_data: Array<ExtAssetDataForUses>,
    nft_asset_data: Array<InternalNFTData>,
    ft_asset_data: Array<InternalFTData>,
    drop_config?: DropConfig,
    next_key_id: number
}

export type InternalNFTData = {
    contract_id: string;
    token_ids: Array<string>;
}

export type InternalFTData = {
    contract_id: string;
    registration_cost: string;
    balance_avail: string
}


export type ExtAssetDataForUses = {
    uses: number;
    assets: Array<ExtAsset | null>;
    config?: UseConfig
}

export type ExtAsset = ExtFTData | ExtNFTData | ExtNEARData | Array<MethodData>;

export type ExtFTData = {
    ft_contract_id: string;
    registration_cost: string;
    ft_amount: string
}

export type ExtNFTData = {
    nft_contract_id: string;
}

export type ExtNEARData = {
    yoctonear: string
}

export interface PasswordPerUse {
    /** The password for this given use */
    pw: string;
    /** Which use does the password belong to? These uses are *NOT* zero-indexed so the first use corresponds to `1` not `0`. */
    key_use: number;
}

export type MethodData = {
    receiver_id: string,
    method_name: string,
    args: string,
    attached_deposit: string,
    attached_gas: string,
    keypom_args?: KeypomInjectedArgs,
    receiver_to_claimer?: boolean,
    user_args_rule?: UserArgsRule,
}

export type UserArgsRule = "AllUser" | "FunderPreferred" | "UserPreferred"

export type KeypomInjectedArgs = {
    account_id_field: string,
    drop_id_field: string,
    key_id_field: string,
    funder_id_field: string,
}


export type TimeConfig = {
    start?: number;
    end?: number;
    throttle?: number;
    interval?: number;
}

export type UseConfig = {
    time?: TimeConfig;
    permissions?: "claim" | "create_account_and_claim";
    account_creation_keypom_args?: KeypomInjectedArgs;
    root_account_id?: string;
}

export type DropConfig = {
    metadata?: string;
    nft_keys_config?: {
        token_metadata?: TokenMetadata;
        royalties?: Record<string, number>;
    };
    add_key_allowlist?: Array<string>;
    delete_empty_drop?: boolean;
    extra_allowance_per_key?: string;
}

export type NFTTokenObject = {
    //token ID
    token_id: string,
    //owner of the token
    owner_id: string,
    //token metadata
    metadata: TokenMetadata,
    
    approved_account_ids: Record<string, number>,
    //keep track of the royalty percentages for the token in a hash map
    royalty: Record<string, number>,
}

export type TokenMetadata = {
    title: string | null;
    description: string | null;
    media: string | null;
    media_hash: string | null;
    copies: number | null;
    issued_at: number | null;
    expires_at: number | null;
    starts_at: number | null;
    updated_at: number | null;
    extra: string | null;
    reference: string | null;
    reference_hash: string | null;
}

export interface ListingJson {
    nft_token_id: string,
    nft_approval_id: number,
    nft_owner_id: string,
    nft_contract_id: string,
    price: string,
    currency: string,
    created_at: string,
    current_offer?: OfferJson,
}

export interface OfferJson {
    offerer_id: string,
    amount: string,
    referrer_id?: string,
    referral_cut?: number
}

export interface TrialRules {
    amounts: string, 
    contracts: string, 
    floor: string, 
    funder: string, 
    methods: string, 
    repay: string, 
    current_floor: string 
}

export type UserProvidedFCArgs = Array<AssetSpecificFCArgs>;
export type AssetSpecificFCArgs = Array<string | undefined> | undefined;