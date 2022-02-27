pub mod meta; 

use serde::Deserialize;
use serde::Serialize; 
use cosmwasm_std::{Uint128,Uint256,Decimal256,Decimal}; 
use enum_as_inner::EnumAsInner;   


use anchor_token::airdrop::IsClaimedResponse;

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]  
pub enum ResponseResult {
    Text(String),
    State(StateResponse),
    EpochState(EpochStateResponse),
    Config(ConfigResponse),
    Simulation(Response<SimulationResponse>),
    CoreSwap(Response<CoreSwapResponse>),
    Price(Response<PriceResponse>),
    BorrowLimit(Response<BorrowLimitResponse>),
    BorrowInfo(Response<BorrowInfoResponse>),
    Balance(Response<BalanceResponse>),
    Balances(Response<Vec<Coin>>),
    Staker(Response<StakerResponse>), 
    DistributionApy(DistributionApyResponse),
    GovReward(GovRewardResponse),
    LPReward(LPRewardResponse),
    SpecAstroVault(SpecAstroVaultResponse),
    Blocks(Response<BlocksPerYearResponse>),
    StablecoinDeposits(Response<Vec<DepositStableLog>>),
    Transactions(Response<Vec<TXLog>>),
    EarnAPY(Response<APY>),
    TaxRate(Response<String>),
    TaxCaps(Response<Vec<TaxCap>>),
    AirdropResponse(AnchorAirdrops),
    IsClaimedResponse(Response<IsClaimedResponse>),
    AnchorWhitelistResponse(Response<AnchorWhitelistResult>)
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize,Clone, Debug,PartialEq)]
pub struct APY {
    pub apy: rust_decimal::Decimal,
    pub result: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BlocksPerYearResponse {
    pub blocks_per_year: f64,
    pub blocks_per_millis: f64,
    pub latest_block: String, 
    pub historic_block: String,  
}

#[derive(Serialize, Deserialize,Clone, Debug,PartialEq)]
pub struct Response<T> {
    pub height: String,
    pub result: T
}  

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BLunaStateResponse {
    pub bluna_exchange_rate: Decimal,
    pub stluna_exchange_rate: Decimal,
    pub total_bond_bluna_amount: Uint128,
    pub total_bond_stluna_amount: Uint128,
    pub last_index_modification: u64,
    pub prev_hub_balance: Uint128,
    pub last_unbonded_time: u64,
    pub last_processed_batch: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MarketStateResponse {
    // https://docs.anchorprotocol.com/smart-contracts/money-market/market#stateresponse
    pub total_liabilities: Decimal256, 
    pub total_reserves: Decimal256, 
    pub last_interest_updated: u64, 
    pub last_reward_updated: u64, 
    pub global_interest_index: Decimal256, 
    pub global_reward_index: Decimal256, 
    pub anc_emission_rate: Decimal256, 
    pub prev_aterra_supply: Uint256, 
    pub prev_exchange_rate: Decimal256, 
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]  
pub enum StateResponse {
    #[allow(non_camel_case_types)]
    bLunaHub(Response<BLunaStateResponse>),
    #[allow(non_camel_case_types)]
    mmMarket(Response<MarketStateResponse>), 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MarketEpochStateResponse {
    pub exchange_rate: Decimal256, 
    pub aterra_supply: Uint256, 
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)] 
pub enum EpochStateResponse { 
    #[allow(non_camel_case_types)]
    mmMarket(Response<MarketEpochStateResponse>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InterestModelConfigResponse {
    pub owner: String, 
    pub base_rate: Decimal256, 
    pub interest_multiplier: Decimal256, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CollectorConfigResponse {
    pub gov_contract: String, 
    pub terraswap_factory: String,
    pub anchor_token: String,
    pub distributor_contract: String,
    pub reward_factor: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)] 
pub enum ConfigResponse { 
    #[allow(non_camel_case_types)]
    mmInterestModel(Response<InterestModelConfigResponse>),
    Collector(Response<CollectorConfigResponse>),
}

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SimulationResponse {
    pub return_amount: Uint128,
    pub spread_amount: Uint128,
    pub commission_amount: Uint128,
}

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CoreSwapResponse {
    pub amount: Uint128,
    pub denom: String, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PriceResponse {
    pub rate: Decimal,
    pub last_updated_base: u64,
    pub last_updated_quote: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BorrowLimitResponse {
    pub borrower: String, 
    pub borrow_limit: Uint128, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BorrowInfoResponse {
    pub borrower: String, 
    pub interest_index: Decimal256, 
    pub reward_index: Decimal256, 
    pub loan_amount: Uint256, 
    pub pending_rewards: Decimal256, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StakerResponse {
    pub balance: Uint128,
    pub share: Uint128,
    pub locked_balance: Vec<(u64, VoterInfo)>, // (Voted Poll's ID, VoterInfo)
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VoterInfo {
    pub vote: VoteOption,
    pub balance: Uint128,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VoteOption {
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DistributionApyResponse {
    pub anc_price: Decimal,
    pub height: u64,
    pub timestamp: u64,
    pub anc_emission_rate: Decimal,
    pub total_liabilities: Decimal,
    pub distribution_apy: Decimal, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GovRewardResponse {
    pub height: u64,
    pub timestamp: u64,
    pub gov_share_index: Decimal,
    pub current_apy: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LPRewardResponse {
    pub anc_price: Decimal,
    pub height: u64,
    pub timestamp: u64,
    pub apy: Decimal, 
    pub total_pool: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpecAstroVaultResponse {
    pub pool_apr: f64,
    pub pool_apy: f64,
    pub pool_astro_apr: f64,
    pub farm_apr: f64,
    pub tvl: String,
    pub multiplier: i64,
    pub vault_fee: f64,
    pub spec_apr: f64,
    pub dpr: f64,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxCap {
    pub denom: String,
    #[serde(rename = "tax_cap")]
    pub tax_cap: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logs {
    pub logs: Vec<Log>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    #[serde(rename = "msg_index")]
    pub msg_index: i64,
    pub log: String,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DepositStableLog { 
    pub height: u64,
    pub timestamp: i64, //chrono::datetime::DateTime<chrono::offset::utc::Utc> .timestamp()
    pub mint_amount: rust_decimal::Decimal,
    pub deposit_amount: rust_decimal::Decimal,
    pub exchange_rate: rust_decimal::Decimal, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TXLog { 
    pub height: u64,
    pub timestamp: i64, //chrono::datetime::DateTime<chrono::offset::utc::Utc> .timestamp()
    pub gas_wanted: rust_decimal::Decimal,
    pub gas_used: rust_decimal::Decimal,
    pub fee_denom: String,
    pub fee_amount: rust_decimal::Decimal,
    pub amount: rust_decimal::Decimal,
    pub raw_log: String, 
} 

pub type AnchorAirdrops = Vec<AnchorAirdrop>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnchorAirdrop {
    pub chain_id: String,
    pub merkle_root: String,
    pub rate: String,
    pub stage: u64,
    pub claimable: bool,
    pub proof: String,
    pub staked: String,
    pub amount: String,
    pub sk: i64,
    pub address: String,
    pub pk: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimAirdropMsg {
    pub claim: Claim,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub proof: String,
    pub stage: u64,
    pub amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnchorWhitelistResult {
    pub elems: Vec<AnchorWhitelist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnchorWhitelist {
    pub name: String,
    pub symbol: String,
    #[serde(rename = "max_ltv")]
    pub max_ltv: String,
    #[serde(rename = "custody_contract")]
    pub custody_contract: String,
    #[serde(rename = "collateral_token")]
    pub collateral_token: String,
}