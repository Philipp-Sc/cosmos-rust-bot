// https://lcd.terra.dev/swagger/#/
/*
 * Queries that get information directly from smart contracts.
 *
 */

pub mod objects;

use objects::*;
use objects::meta::api::data::terra_contracts::{contracts, pairs_dex, tokens};
use objects::meta::api::{
    get_fcd_else_lcd_query,
    query_core_market_swap_rate,
    query_core_bank_balances};
use anyhow::anyhow;

use moneymarket::market::QueryMsg as MarketQueryMsg;
use basset::hub::QueryMsg as BassetHubQueryMsg;

use moneymarket::interest_model::QueryMsg as InterestModelQueryMsg;
use moneymarket::overseer::QueryMsg as OverseerQueryMsg;
use anchor_token::collector::QueryMsg as CollectorQueryMsg;
use anchor_token::gov::QueryMsg as GovQueryMsg;

use anchor_token::airdrop::QueryMsg as AirdropQueryMsg;
use anchor_token::airdrop::IsClaimedResponse;

use mirror_protocol::oracle::QueryMsg as MirrorOracleQueryMsg;
use mirror_protocol::oracle::PriceResponse;

use cw20::Cw20QueryMsg;
use cw20::BalanceResponse;

use cosmwasm_std_deprecated::{Uint128};
use std::str::FromStr;

use moneymarket::market::StateResponse as MarketStateResponse;

use basset::hub::StateResponse as BassetHubStateResponse;

use moneymarket::market::EpochStateResponse as MarketEpochStateResponse;

use anchor_token::collector::ConfigResponse as CollectorConfigResponse;
use moneymarket::interest_model::ConfigResponse as InterestModelConfigResponse;

use moneymarket::market::BorrowerInfoResponse;
use moneymarket::overseer::BorrowLimitResponse;

use anchor_token::gov::StakerResponse;
use moneymarket::overseer::WhitelistResponse;
use serde_json::Value;
use objects::meta::api::data::terra_contracts::AssetWhitelist;
use std::sync::Arc;
use secstr::*;

// https://fcd.terra.dev/wasm/contracts/terra146ahqn6d3qgdvmj8cj96hh03dzmeedhsf0kxqm/store?query_msg={%22latest_stage%22:{}}


// get token swap info

// https://lcd-osmosis.blockapsis.com/swagger/

// https://docs.osmosis.zone/developing/modules/spec-gamm.html#overview
// https://lcd-osmosis.keplr.app/osmosis/gamm/v1beta1/pools/560

// https://github.com/confio/osmosis-bindings/blob/main/packages/bindings/src/query.rs

// {}/wasm/contracts/{}/store?query_msg={} for osmosis? where

// /osmosis/gamm/v1beta1/{poolId}/estimate/swap_exact_amount_in
/*
pub async fn simulate_swap_ibcs(asset_whitelist: Arc<AssetWhitelist>, dex: String, bid_token_protocol: Option<String>, bid_token: String, ask_token_protocol: Option<String>, ask_token: String) -> anyhow::Result<ResponseResult> {
    osmo_bindings::Os::
    osmo_bindings::OsmosisQuery::estimate_swap("cosmos10885ryvnfvu7hjt8lqvge77uderycqcuu5qtd9", 501, "atom", "osmo", osmo_bindings::SwapAmount::In(cosmwasm_std_latest::Uint128::new(501505)));
    /*
    let coin_a = cosmwasm_std_latest::coin(6_000_000u128, "osmo");
    let coin_b = cosmwasm_std_latest::coin(1_500_000u128, "atom");

    /*let pool_id = 43;
    let pool = osmo_bindings::Pool::new(coin_a.clone(), coin_b.clone());

    // set up with one pool

BasicAppBuilder::<OsmosisMsg, OsmosisQuery>::new_custom()
                .with_custom(OsmosisModule {})
                .build(|_router, _, _storage| {
                    // router.custom.set_owner(storage, &owner).unwrap();
                }),

    let mut app = OsmosisApp::new();
    app.init_modules(|router, _, storage| {
        router.custom.set_pool(storage, pool_id, &pool).unwrap();
    });
    */
    // estimate the price (501505 * 0.997 = 500_000) after fees gone
    let query = osmo_bindings::OsmosisQuery::estimate_swap(
        cosmwasm_std_latest::testing::MOCK_CONTRACT_ADDR,
        500,
        &coin_b.denom,
        &coin_a.denom,
        osmo_bindings::SwapAmount::In(cosmwasm_std_latest::Uint128::new(501505)),
    );
    //let osmo_bindings::SwapResponse { amount } = app.wrap().query(&query.into()).unwrap();
    // 6M * 1.5M = 2M * 4.5M -> output = 1.5M
    let expected = osmo_bindings::SwapAmount::Out(cosmwasm_std_latest::Uint128::new(1_500_000));

    // now try the reverse query. we know what we need to pay to get 1.5M out
    let query = osmo_bindings::OsmosisQuery::estimate_swap(
        cosmwasm_std_latest::testing::MOCK_CONTRACT_ADDR,
        500,
        &coin_b.denom,
        &coin_a.denom,
        osmo_bindings::SwapAmount::Out(cosmwasm_std_latest::Uint128::new(1500000)),
    );
    //let osmo_bindings::SwapResponse { amount } = app.wrap().query(&query.into()).unwrap();
    let expected = osmo_bindings::SwapAmount::In(cosmwasm_std_latest::Uint128::new(501505));*/
    Err(anyhow!("no contract_addr"))
}
*/
pub async fn airdrop_is_claimed(asset_whitelist: Arc<AssetWhitelist>, wallet_acc_address: Arc<SecUtf8>, stage: u64) -> anyhow::Result<ResponseResult> {
    let contract_addr = contracts(&asset_whitelist, "Anchor", "Airdrop").ok_or(anyhow!("no contract_addr"))?;

    let query = AirdropQueryMsg::IsClaimed {
        stage: stage as u8,
        address: wallet_acc_address.unsecure().to_string(),
    };
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let response: Response<IsClaimedResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::IsClaimedResponse(response))
}


// blunaHubState: state, Anchor, bLuna Hub
// anchor_protocol_state: state, Anchor, Market

pub async fn state_query_msg(asset_whitelist: Arc<AssetWhitelist>, protocol: String, contract: String) -> anyhow::Result<ResponseResult> {
    let contract_addr = contracts(&asset_whitelist, &protocol, &contract).ok_or(anyhow!("no contract_addr"))?;

    match contract.as_str() {
        "Market" => {
            let query = MarketQueryMsg::State { block_height: None };
            let query_msg_json = serde_json::to_string(&query)?;

            let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
            let response: Response<MarketStateResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::State(StateResponse::mmMarket(response)));
        }
        "bLuna Hub" => {
            let query = BassetHubQueryMsg::State {};
            let query_msg_json = serde_json::to_string(&query)?;

            let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
            let response: Response<BassetHubStateResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::State(StateResponse::bLunaHub(response)));
        }
        _ => {
            return Err(anyhow!("Unexpected Error: Unknown Contract {:?}",contract));
        }
    }
}

// aust_to_ust: epoch_state, anchorprotocol, mmMarket
pub async fn epoch_state_query_msg(asset_whitelist: Arc<AssetWhitelist>, protocol: String, contract: String) -> anyhow::Result<ResponseResult> {
    let query = MarketQueryMsg::EpochState {
        block_height: None,
        distributed_interest: None,
    };
    let query_msg_json = serde_json::to_string(&query)?;

    let contract_addr = contracts(&asset_whitelist, &protocol, &contract).ok_or(anyhow!("no contract_addr"))?;

    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;

    match contract.as_str() {
        "Market" => {
            let res: Response<MarketEpochStateResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::EpochState(EpochStateResponse::mmMarket(res)));
        }
        _ => {
            return Err(anyhow!("Unexpected Error: Unknown Contract {:?}",contract));
        }
    }
}

// anchor_protocol_interest_model_config: Anchor, Interest Model
// anchor_protocol_collector_config: Anchor, Fee Collector
pub async fn config_query_msg(asset_whitelist: Arc<AssetWhitelist>, protocol: String, contract: String) -> anyhow::Result<ResponseResult> {
    let contract_addr = contracts(&asset_whitelist, &protocol, &contract).ok_or(anyhow!("no contract_addr"))?;

    match contract.as_str() {
        "Interest Model" => {
            let query = InterestModelQueryMsg::Config {};
            let query_msg_json = serde_json::to_string(&query)?;

            let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
            let response: Response<InterestModelConfigResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::Config(ConfigResponse::mmInterestModel(response)));
        }
        "Fee Collector" => {
            let query = CollectorQueryMsg::Config {};
            let query_msg_json = serde_json::to_string(&query)?;

            let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
            let response: Response<CollectorConfigResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::Config(ConfigResponse::Collector(response)));
        }
        _ => {
            return Err(anyhow!("Unexpected Error: Unknown Contract {:?}",contract));
        }
    }
}

// core_swap usdr uusd
pub async fn native_token_core_swap(from_native_token: String, to_native_token: String) -> anyhow::Result<ResponseResult> {
    let res: String = query_core_market_swap_rate(&from_native_token, &to_native_token).await?;
    let res: Response<CoreSwapResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::CoreSwap(res))
}


// deprecated because usdr is not pard of pairs.dex.json
// sdt_to_uluna: usdr, terraswap, usdr_uluna_pair_contract
// sdt_to_uluna: usdr, native, usdr (is in contracts, not in pairs_dex)


// luna_to_bluna: terraswap, None, uluna, Some(Anchor), bLuna
// luna_to_ust: terraswap, None, uluna, None, uusd
// ust_to_luna: terraswap, None, uusd, None, uluna
// ust_to_psi: terraswap, None, uusd, Some(Nexus), Psi
// ust_to_anc: terraswap, None, uusd, Some(Anchor), ANC

// bluna_to_luna: terraswap, Some(Anchor), bLuna, None, uluna
// nluna_to_psi: terraswap, Some(Nexus), nLuna, Some(Nexus), Psi
// psi_to_nluna:  terraswap, Some(Nexus), Psi, Some(Nexus), nLuna
// psi_to_ust:  terraswap, Some(Nexus), nLuna, None, uusd
// anc_to_ust: terraswap, Some(Anchor), ANC, None, uusd

// terraswap, Some(Mirror), mTSLA, None, uusd

pub async fn simulate_swap(asset_whitelist: Arc<AssetWhitelist>, dex: String, bid_token_protocol: Option<String>, bid_token: String, ask_token_protocol: Option<String>, ask_token: String) -> anyhow::Result<ResponseResult> {
    let bid_token_addr = match bid_token_protocol.as_ref() {
        Some(p) => {
            tokens(&asset_whitelist, &p, &bid_token)
        }
        None => {
            None
        }
    };
    let ask_token_addr = match ask_token_protocol.as_ref() {
        Some(p) => {
            tokens(&asset_whitelist, &p, &ask_token)
        }
        None => {
            None
        }
    };

    let contract_addr = match pairs_dex(&asset_whitelist, [ask_token_addr.as_ref().unwrap_or(&ask_token).as_str(), bid_token_addr.as_ref().unwrap_or(&bid_token).as_str()], &dex).as_ref() {
        Some(addr) => {
            addr.to_string()
        }
        None => {
            return Err(anyhow!("no contract_addr"));
        }
    };

    let query_msg_json = match dex.as_str() {
        "terraswap" => {
            let info = match bid_token_addr {
                None => {
                    terraswap::asset::AssetInfo::NativeToken {
                        denom: bid_token,
                    }
                }
                Some(addr) => {
                    terraswap::asset::AssetInfo::Token {
                        contract_addr: addr,
                    }
                }
            };
            let query = terraswap::pair::QueryMsg::Simulation {
                offer_asset: terraswap::asset::Asset {
                    info: info,
                    amount: cosmwasm_std_latest::Uint128::from_str("1000000").unwrap(),
                },
            };
            serde_json::to_string(&query)?
        }
        "astroport" => {
            let info = match bid_token_addr {
                None => {
                    astroport::asset::AssetInfo::NativeToken {
                        denom: bid_token,
                    }
                }
                Some(addr) => {
                    astroport::asset::AssetInfo::Token {
                        contract_addr: cosmwasm_std_deprecated::Addr::unchecked(addr),
                    }
                }
            };
            let query = astroport::pair::QueryMsg::Simulation {
                offer_asset: astroport::asset::Asset {
                    info: info,
                    amount: Uint128::from_str("1000000").unwrap(),
                },
            };
            serde_json::to_string(&query)?
        }
        _ => {
            return Err(anyhow!("Error: Unknown DEX!"));
        }
    };
    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res = match dex.as_str() {
        "terraswap" => {
            let res: Response<terraswap::pair::SimulationResponse> = serde_json::from_str(&res)?;
            ResponseResult::Simulation(Response { height: res.height, result: SimulationResponse::terraswap(res.result) })
        }
        "astroport" => {
            let res: Response<astroport::pair::SimulationResponse> = serde_json::from_str(&res)?;
            ResponseResult::Simulation(Response { height: res.height, result: SimulationResponse::astroport(res.result) })
        }
        _ => {
            return Err(anyhow!("Error: Unknown DEX!"));
        }
    };
    Ok(res)
}

pub async fn masset_oracle_price(asset_whitelist: Arc<AssetWhitelist>, masset: String) -> anyhow::Result<ResponseResult> {
    // https://docs.mirror.finance/contracts/oracle#price
    let contract_addr = contracts(&asset_whitelist, "Mirror", "Oracle").ok_or(anyhow!("no contract_addr"))?;

    let query = MirrorOracleQueryMsg::Price {
        base_asset: tokens(&asset_whitelist, "Mirror", &masset).ok_or(anyhow!("no token_addr"))?,
        quote_asset: "uusd".to_string(),
    };
    let query_msg_json = serde_json::to_string(&query)?;


    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res: Response<PriceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Price(res))
}

pub async fn anchor_protocol_borrower_limit(asset_whitelist: Arc<AssetWhitelist>, wallet_acc_address: Arc<SecUtf8>) -> anyhow::Result<ResponseResult> {
    // https://docs.anchorprotocol.com/smart-contracts/money-market/overseer#borrowlimitresponse
    let contract_addr = contracts(&asset_whitelist, "Anchor", "Overseer").ok_or(anyhow!("no contract_addr"))?;
    let query = OverseerQueryMsg::BorrowLimit {
        borrower: wallet_acc_address.unsecure().to_string(),
        block_time: None,
    };
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res: Response<BorrowLimitResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::BorrowLimit(res))
}

pub async fn anchor_protocol_borrower_info(asset_whitelist: Arc<AssetWhitelist>, wallet_acc_address: Arc<SecUtf8>) -> anyhow::Result<ResponseResult> {
    // https://docs.anchorprotocol.com/smart-contracts/money-market/market#borrowerinforesponse
    /*
     * Gets information for the specified borrower. 
     * Returns an interest-and-reward-accrued value if block_height field is filled. 
     * Returns the stored (no interest / reward accrued) state if not filled. **This seems not to be the case anymore**
     * */
    let contract_addr = contracts(&asset_whitelist, "Anchor", "Market").ok_or(anyhow!("no contract_addr"))?;

    let query = MarketQueryMsg::BorrowerInfo {
        borrower: wallet_acc_address.unsecure().to_string(),
        block_height: Some(1),
    };
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res: Response<BorrowerInfoResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::BorrowInfo(res))
}

pub async fn anchor_protocol_anc_balance(asset_whitelist: Arc<AssetWhitelist>, wallet_acc_address: Arc<SecUtf8>) -> anyhow::Result<ResponseResult> {
    let contract_addr = tokens(&asset_whitelist, "Anchor", "ANC").ok_or(anyhow!("no contract_addr"))?;

    let query = Cw20QueryMsg::Balance {
        address: wallet_acc_address.unsecure().to_string()
    };
    let query_msg_json = serde_json::to_string(&query)?;


    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res: Response<BalanceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Balance(res))
}

pub async fn anchor_protocol_balance(asset_whitelist: Arc<AssetWhitelist>, wallet_acc_address: Arc<SecUtf8>) -> anyhow::Result<ResponseResult> {
    let contract_addr = tokens(&asset_whitelist, "Anchor", "aUST").ok_or(anyhow!("no contract_addr"))?;

    let query = Cw20QueryMsg::Balance {
        address: wallet_acc_address.unsecure().to_string()
    };
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res: Response<BalanceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Balance(res))
}

pub async fn terra_balances(wallet_acc_address: Arc<SecUtf8>) -> anyhow::Result<ResponseResult> {
    let res: String = query_core_bank_balances(wallet_acc_address.unsecure()).await?;
    let res: Response<Vec<Coin>> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Balances(res))
}

pub async fn anchor_protocol_staker(asset_whitelist: Arc<AssetWhitelist>, wallet_acc_address: Arc<SecUtf8>) -> anyhow::Result<ResponseResult> {
    // https://docs.anchorprotocol.com/smart-contracts/anchor-token/gov#staker 
    let contract_addr = contracts(&asset_whitelist, "Anchor", "Governance").ok_or(anyhow!("no contract_addr"))?;

    let query = GovQueryMsg::Staker {
        address: wallet_acc_address.unsecure().to_string(),
    };
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res: Response<StakerResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Staker(res))
}

pub async fn anchor_protocol_whitelist(asset_whitelist: Arc<AssetWhitelist>) -> anyhow::Result<ResponseResult> {
    let contract_addr = contracts(&asset_whitelist, "Anchor", "Overseer").ok_or(anyhow!("no contract_addr"))?;

    let query = OverseerQueryMsg::Whitelist {
        collateral_token: None,
        start_after: None,
        limit: None,
    };
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res: Response<WhitelistResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::AnchorWhitelistResponse(res))
}

pub async fn get_pair(asset_whitelist: Arc<AssetWhitelist>, dex: String, asset_infos: [Value; 2]) -> anyhow::Result<ResponseResult> {
    let contract_addr = contracts(&asset_whitelist, &dex, "TokenFactory").ok_or(anyhow!("no contract_addr"))?;
    let query_msg_json = match dex.as_str() {
        "terraswap" => {
            let asset_infos: [terraswap::asset::AssetInfo; 2] = [serde_json::from_value(asset_infos[0].clone())?, serde_json::from_value(asset_infos[1].clone())?];
            let query = terraswap::factory::QueryMsg::Pair {
                asset_infos: asset_infos,
            };
            serde_json::to_string(&query)?
        }
        "astroport" => {
            let asset_infos: [astroport::asset::AssetInfo; 2] = [serde_json::from_value(asset_infos[0].clone())?, serde_json::from_value(asset_infos[1].clone())?];
            let query = astroport::factory::QueryMsg::Pair {
                asset_infos: asset_infos,
            };
            serde_json::to_string(&query)?
        }
        _ => {
            return Err(anyhow!("Error: Unknown DEX!"));
        }
    };
    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res: Response<PairResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Pair(res))
}

pub async fn get_pairs(asset_whitelist: Arc<AssetWhitelist>, dex: String, start_after: Option<[Value; 2]>, limit: Option<u32>) -> anyhow::Result<ResponseResult> {
    let contract_addr = contracts(&asset_whitelist, &dex, "TokenFactory").ok_or(anyhow!("no contract_addr"))?;
    let query_msg_json = match dex.as_str() {
        "terraswap" => {
            let start_after: Option<[terraswap::asset::AssetInfo; 2]> = match start_after {
                None => { None }
                Some(j) => { Some([serde_json::from_value(j[0].clone())?, serde_json::from_value(j[1].clone())?]) }
            };
            let query = terraswap::factory::QueryMsg::Pairs {
                start_after: start_after,
                limit: limit,
            };
            serde_json::to_string(&query)?
        }
        "astroport" => {
            let start_after: Option<[astroport::asset::AssetInfo; 2]> = match start_after {
                None => { None }
                Some(j) => { Some([serde_json::from_value(j[0].clone())?, serde_json::from_value(j[1].clone())?]) }
            };
            let query = astroport::factory::QueryMsg::Pairs {
                start_after: start_after,
                limit: limit,
            };
            serde_json::to_string(&query)?
        }
        _ => {
            return Err(anyhow!("Error: Unknown DEX!"));
        }
    };
    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res: Response<PairsResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Pairs(res))
}

pub async fn get_token_info(contract_addr: String) -> anyhow::Result<ResponseResult> {
    let query = cw20::Cw20QueryMsg::TokenInfo {};
    let query_msg_json = serde_json::to_string(&query)?;
    let res: String = get_fcd_else_lcd_query(&contract_addr, &query_msg_json).await?;
    let res: Response<cw20::TokenInfoResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::TokenInfo(res))
}