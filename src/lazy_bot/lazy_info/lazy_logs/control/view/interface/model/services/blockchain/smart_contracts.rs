// https://lcd.terra.dev/swagger/#/
/*
 * Queries that get information directly from smart contracts.
 *
 */

pub mod objects; 
use objects::*;  
use objects::meta::api::data::terra_contracts::{get_contract,get_mirrorprotocol_assets};
use objects::meta::api::{
    get_fcd_or_lcd_query,
    query_core_market_swap_rate,
    query_core_bank_balances};
use anyhow::anyhow;   

use moneymarket::market::QueryMsg as MarketQueryMsg;
use basset::hub::QueryMsg as BassetHubQueryMsg;

use moneymarket::interest_model::QueryMsg as InterestModelQueryMsg;
use moneymarket::overseer::QueryMsg as OverseerQueryMsg;
use anchor_token::collector::QueryMsg as CollectorQueryMsg;
use anchor_token::gov::QueryMsg as GovQueryMsg;

use mirror_protocol::oracle::QueryMsg as MirrorOracleQueryMsg;

use cw20::Cw20QueryMsg;

use terraswap::asset::{Asset,AssetInfo};
use terraswap::pair::QueryMsg as TerraswapQueryMsg;

use cosmwasm_std::{Uint128};
use std::str::FromStr;



//use moneymarket::market::{BorrowerInfoResponse, EpochStateResponse};

// https://fcd.terra.dev/wasm/contracts/terra146ahqn6d3qgdvmj8cj96hh03dzmeedhsf0kxqm/store?query_msg={%22latest_stage%22:{}}


pub async fn airdrop_is_claimed(wallet_acc_address: &str, stage: u64) -> anyhow::Result<ResponseResult> {
    let query = r#"{"is_claimed":{"stage":stage_id,"address":"wallet_acc_address"}}"#
                .replace("stage_id", &stage.to_string()) 
                .replace("wallet_acc_address", &wallet_acc_address);

    let contract_addr = get_contract("anchorprotocol","airdrop");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?; 
    
    let response: Response<IsClaimedResult> = serde_json::from_str(&res)?;
    Ok(ResponseResult::IsClaimedResponse(response))
}


// blunaHubState: state, anchorprotocol, bLunaHub
// anchor_protocol_state: state, anchorprotocol, mmMarket 

pub async fn state_query_msg(protocol: String, contract: String) -> anyhow::Result<ResponseResult> { 
    let contract_addr = get_contract(&protocol,&contract);
    match contract.as_str() {
        "mmMarket" => {
            let query = MarketQueryMsg::State { block_height: None}; 
            let query_msg_json = serde_json::to_string(&query)?;

            let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?; 
            let response: Response<MarketStateResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::State(StateResponse::mmMarket(response)));
        },
        "bLunaHub" => {
            let query = BassetHubQueryMsg::State {};
            let query_msg_json = serde_json::to_string(&query)?;

            let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?; 
            let response: Response<BLunaStateResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::State(StateResponse::bLunaHub(response))); 
        },
        _ => {
            return Err(anyhow!("Unexpected Error: Unknown Contract {:?}",contract));
        }
    }
}

// aust_to_ust: epoch_state, anchorprotocol, mmMarket
pub async fn epoch_state_query_msg(protocol: String, contract: String) -> anyhow::Result<ResponseResult> {
  
    let query = MarketQueryMsg::EpochState {
            block_height: None,
            distributed_interest: None
    };     
    let query_msg_json = serde_json::to_string(&query)?;

    let contract_addr = get_contract(&protocol,&contract);
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?; 
    
    match contract.as_str() {
        "mmMarket" => {
            let res: Response<MarketEpochStateResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::EpochState(EpochStateResponse::mmMarket(res)));
        },
        _ => {
            return Err(anyhow!("Unexpected Error: Unknown Contract {:?}",contract));
        }
    } 
}

// anchor_protocol_interest_model_config: anchorprotocol, mmInterestModel
// anchor_protocol_collector_config: anchorprotocol, collector 
pub async fn config_query_msg(protocol: String, contract: String) -> anyhow::Result<ResponseResult> {
    
    let contract_addr = get_contract(&protocol,&contract);

    match contract.as_str() {
        "mmInterestModel" => { 
            let query = InterestModelQueryMsg::Config {};
            let query_msg_json = serde_json::to_string(&query)?;

            let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?; 
            let response: Response<InterestModelConfigResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::Config(ConfigResponse::mmInterestModel(response)));
        },
        "collector" => {
            let query = CollectorQueryMsg::Config {};
            let query_msg_json = serde_json::to_string(&query)?;

            let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?; 
            let response: Response<CollectorConfigResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::Config(ConfigResponse::Collector(response))); 
        },
        _ => {
            return Err(anyhow!("Unexpected Error: Unknown Contract {:?}",contract));
        }
    }
}

// core_swap usdr uusd
pub async fn native_token_core_swap(from_native_token: String, to_native_token: String) ->  anyhow::Result<ResponseResult> {
    let res: String = query_core_market_swap_rate(&from_native_token,&to_native_token).await?; 
    let res: Response<CoreSwapResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::CoreSwap(res))
}

// luna_to_bluna: uluna, anchorprotocol,terraswapblunaLunaPair
// luna_to_ust: uluna, terraswap, uusd_uluna_pair_contract
// sdt_to_uluna: usdr, terraswap, usdr_uluna_pair_contract
// ust_to_luna: uusd, terraswap, uusd_uluna_pair_contract
// ust_to_psi: uusd, nexusprotocol, Psi-UST pair
// ust_to_anc: uusd, anchorprotocol, terraswapAncUstPair
pub async fn native_token_to_swap_pair(protocol: String, native_token: String, pair_contract: String) ->  anyhow::Result<ResponseResult> {
    let contract_addr = get_contract(&protocol, &pair_contract);

    let query = TerraswapQueryMsg::Simulation {
        offer_asset: Asset {
                        info: AssetInfo::NativeToken {
                            denom: native_token,
                        },
                        amount: Uint128::from_str("1000000").unwrap(),
                },
    }; 

    let query_msg_json = serde_json::to_string(&query)?;
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?; 
    let res: Response<SimulationResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Simulation(res))
}

// bluna_to_luna: anchorprotocol, bLunaToken, terraswapblunaLunaPair
// nluna_to_psi: nexusprotocol, nLuna token, Psi-nLuna pair
// psi_to_nluna: nexusprotocol, Psi token, Psi-nLuna pair
// psi_to_ust: nexusprotocol,  Psi token, Psi-UST pair
// anc_to_ust: anchorprotocol, ANC, terraswapAncUstPair 
pub async fn cw20_to_swap_pair(protocol: String, token_contract: String, pair_contract: String) ->  anyhow::Result<ResponseResult> {
    let contract_addr = get_contract(&protocol, &pair_contract);
    
    let query = TerraswapQueryMsg::Simulation {
        offer_asset: Asset {
                        info: AssetInfo::Token {
                            contract_addr: get_contract(&protocol,&token_contract),
                        },
                        amount: Uint128::from_str("1000000").unwrap(),
                }
    }; 
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?;
    let res: Response<SimulationResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Simulation(res))
}
pub async fn masset_to_ust(masset: String) -> anyhow::Result<ResponseResult> {
    let contract_addr = get_mirrorprotocol_assets(&masset,"pair");
    
    let query = TerraswapQueryMsg::Simulation {
        offer_asset: Asset {
                        info: AssetInfo::Token {
                            contract_addr: get_mirrorprotocol_assets(&masset,"token"),
                        },
                        amount: Uint128::from_str("1000000").unwrap(),
                }
    }; 
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?;
    let res: Response<SimulationResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Simulation(res))
}  
pub async fn masset_oracle_price(masset: String) ->  anyhow::Result<ResponseResult> {
    // https://docs.mirror.finance/contracts/oracle#price
    let contract_addr = get_contract("mirrorprotocol","oracle");
    
    let query = MirrorOracleQueryMsg::Price {
            base_asset: get_mirrorprotocol_assets(&masset,"token"),
            quote_asset: "uusd".to_string(),
    };
    let query_msg_json = serde_json::to_string(&query)?;


    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?;
    let res: Response<PriceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Price(res))
}
pub async fn anchor_protocol_borrower_limit(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> {
    // https://docs.anchorprotocol.com/smart-contracts/money-market/overseer#borrowlimitresponse
    let contract_addr = get_contract("anchorprotocol","mmOverseer");
    let query = OverseerQueryMsg::BorrowLimit {
            borrower: wallet_acc_address.to_string(),
            block_time: None
    };
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?;
    let res: Response<BorrowLimitResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::BorrowLimit(res))
}
pub async fn anchor_protocol_borrower_info(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> {
    // https://docs.anchorprotocol.com/smart-contracts/money-market/market#borrowerinforesponse
    /*
     * Gets information for the specified borrower. 
     * Returns an interest-and-reward-accrued value if block_height field is filled. 
     * Returns the stored (no interest / reward accrued) state if not filled. **This seems not to be the case anymore**
     * */
    let contract_addr = get_contract("anchorprotocol","mmMarket");

    let query = MarketQueryMsg:: BorrowerInfo {
        borrower: wallet_acc_address.to_string(),
        block_height: Some(1),
    };
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?;
    let res: Response<BorrowInfoResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::BorrowInfo(res))
} 
pub async fn anchor_protocol_anc_balance(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> { 
    let contract_addr = get_contract("anchorprotocol","ANC");

    let query = Cw20QueryMsg::Balance {
        address: wallet_acc_address.to_string()
    };
    let query_msg_json = serde_json::to_string(&query)?;


    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?;
    let res: Response<BalanceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Balance(res))
} 
pub async fn anchor_protocol_balance(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> { 
    let contract_addr = get_contract("anchorprotocol","aTerra");
    
    let query = Cw20QueryMsg::Balance {
        address: wallet_acc_address.to_string()
    };
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?;
    let res: Response<BalanceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Balance(res))
} 
pub async fn terra_balances(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> { 
    let res: String = query_core_bank_balances(wallet_acc_address).await?;
    let res: Response<Vec<Coin>> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Balances(res))
} 
pub async fn anchor_protocol_staker(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> {
    // https://docs.anchorprotocol.com/smart-contracts/anchor-token/gov#staker 
    let contract_addr = get_contract("anchorprotocol","gov");
    
    let query = GovQueryMsg::Staker {
            address: wallet_acc_address.to_string(),
        };
    let query_msg_json = serde_json::to_string(&query)?;
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?;
    let res: Response<StakerResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Staker(res))
}  

pub async fn anchor_protocol_whitelist() ->  anyhow::Result<ResponseResult> { 
    let contract_addr = get_contract("anchorprotocol","mmOverseer");

    let query = OverseerQueryMsg::Whitelist {
                collateral_token: None,
                start_after: None,
                limit: None,
        };
    let query_msg_json = serde_json::to_string(&query)?;

    let res: String = get_fcd_or_lcd_query(&contract_addr,&query_msg_json).await?;
    let res: Response<AnchorWhitelistResult> = serde_json::from_str(&res)?; 
    Ok(ResponseResult::AnchorWhitelistResponse(res))
}  