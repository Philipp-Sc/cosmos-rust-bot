pub mod meta;

use serde::Deserialize;
use serde::Serialize;
use cosmwasm_std_deprecated::{Uint128, Decimal};
use enum_as_inner::EnumAsInner;


use anchor_token::airdrop::IsClaimedResponse;

//use moneymarket::market::ConfigResponse as MarketConfigResponse;
use moneymarket::market::EpochStateResponse as MarketEpochStateResponse;
use moneymarket::market::StateResponse as MarketStateResponse;
use basset::hub::StateResponse as BassetHubStateResponse;
use moneymarket::market::BorrowerInfoResponse;
use moneymarket::overseer::BorrowLimitResponse;

use anchor_token::collector::ConfigResponse as CollectorConfigResponse;
use moneymarket::interest_model::ConfigResponse as InterestModelConfigResponse;


use cw20::BalanceResponse;
use anchor_token::gov::StakerResponse;

use moneymarket::overseer::WhitelistResponse;
use mirror_protocol::oracle::PriceResponse;

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum ResponseResult {
    Text(String),
    State(StateResponse),
    EpochState(EpochStateResponse),
    Config(ConfigResponse),
    Simulation(Response<SimulationResponse>),
    Pairs(Response<PairsResponse>),
    Pair(Response<PairResponse>),
    TokenInfo(Response<cw20::TokenInfoResponse>),
    CoreSwap(Response<CoreSwapResponse>),
    Price(Response<PriceResponse>),
    BorrowLimit(Response<BorrowLimitResponse>),
    BorrowInfo(Response<BorrowerInfoResponse>),
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
    AnchorWhitelistResponse(Response<WhitelistResponse>),
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct APY {
    pub apy: rust_decimal::Decimal,
    pub result: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BlocksPerYearResponse {
    pub blocks_per_year: f64,
    pub blocks_per_millis: f64,
    pub latest_block: String,
    pub historic_block: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Response<T> {
    pub height: String,
    pub result: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum SimulationResponse {
    #[allow(non_camel_case_types)]
    terraswap(terraswap::pair::SimulationResponse),
    #[allow(non_camel_case_types)]
    astroport(astroport::pair::SimulationResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum PairsResponse {
    #[allow(non_camel_case_types)]
    terraswap(terraswap::factory::PairsResponse),
    #[allow(non_camel_case_types)]
    astroport(astroport::factory::PairsResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum PairResponse {
    #[allow(non_camel_case_types)]
    terraswap(terraswap::asset::PairInfo),
    #[allow(non_camel_case_types)]
    astroport(astroport::asset::PairInfo),
}


#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum StateResponse {
    #[allow(non_camel_case_types)]
    bLunaHub(Response<BassetHubStateResponse>),
    #[allow(non_camel_case_types)]
    mmMarket(Response<MarketStateResponse>),
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum EpochStateResponse {
    #[allow(non_camel_case_types)]
    mmMarket(Response<MarketEpochStateResponse>),
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum ConfigResponse {
    #[allow(non_camel_case_types)]
    mmInterestModel(Response<InterestModelConfigResponse>),
    Collector(Response<CollectorConfigResponse>),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSwapResponse {
    pub amount: Uint128,
    pub denom: String,
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
    pub timestamp: i64,
    //chrono::datetime::DateTime<chrono::offset::utc::Utc> .timestamp()
    pub mint_amount: rust_decimal::Decimal,
    pub deposit_amount: rust_decimal::Decimal,
    pub exchange_rate: rust_decimal::Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TXLog {
    pub height: u64,
    pub timestamp: i64,
    //chrono::datetime::DateTime<chrono::offset::utc::Utc> .timestamp()
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
 