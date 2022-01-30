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
	let query = r#"{"state":{}}"#;   
 	let contract_addr = get_contract(&protocol,&contract);
	
	let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?; 
    //println!("{:?}",&res);
    match contract.as_str() {
        "mmMarket" => {
            let response: Response<MarketStateResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::State(StateResponse::mmMarket(response)));
        },
        "bLunaHub" => {
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
    let query = r#"{"epoch_state":{}}"#;  
    let contract_addr = get_contract(&protocol,&contract);
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?; 
    
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
    let query = r#"{"config":{}}"#;  
    let contract_addr = get_contract(&protocol,&contract);
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?; 

    match contract.as_str() {
        "mmInterestModel" => { 
            let response: Response<InterestModelConfigResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::Config(ConfigResponse::mmInterestModel(response)));
        },
        "collector" => {
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
    let query = r#"{"simulation":{"offer_asset":{"amount":"1000000","info":{"native_token":{"denom":"my_native_token"}}}}}"#.replace("my_native_token", &native_token); 
    let contract_addr = get_contract(&protocol, &pair_contract);
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?; 
    let res: Response<SimulationResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Simulation(res))
}

// bluna_to_luna: anchorprotocol, bLunaToken, terraswapblunaLunaPair
// nluna_to_psi: nexusprotocol, nLuna token, Psi-nLuna pair
// psi_to_nluna: nexusprotocol, Psi token, Psi-nLuna pair
// psi_to_ust: nexusprotocol,  Psi token, Psi-UST pair
// anc_to_ust: anchorprotocol, ANC, terraswapAncUstPair 
pub async fn cw20_to_swap_pair(protocol: String, token_contract: String, pair_contract: String) ->  anyhow::Result<ResponseResult> {
    let query = r#"{"simulation":{"offer_asset":{"amount":"1000000","info":{"token":{"contract_addr":"my_cw20_contract_addr"}}}}}"#.replace("my_cw20_contract_addr", &get_contract(&protocol,&token_contract)); 
    let contract_addr = get_contract(&protocol, &pair_contract);
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?;
    let res: Response<SimulationResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Simulation(res))
}
pub async fn masset_to_ust(masset: String) -> anyhow::Result<ResponseResult> {
    //let query = r#"{"simulation":{"offer_asset":{"amount":"1000000","info":{"native_token":{"denom":"my_native_token"}}}}}"#.replace("my_native_token", "uusd"); 
    let query = r#"{"simulation":{"offer_asset":{"amount":"1000000","info":{"token":{"contract_addr":"my_cw20_contract_addr"}}}}}"#.replace("my_cw20_contract_addr", &get_mirrorprotocol_assets(&masset,"token"));
    let contract_addr = get_mirrorprotocol_assets(&masset,"pair");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?;
    let res: Response<SimulationResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Simulation(res))
}  
pub async fn masset_oracle_price(masset: String) ->  anyhow::Result<ResponseResult> {
    // https://docs.mirror.finance/contracts/oracle#price
    let query = r#"{"price": {"base_asset": "my_cw20_contract_addr","quote_asset": "uusd"}}"#.replace("my_cw20_contract_addr", &get_mirrorprotocol_assets(&masset,"token")); 
    let contract_addr = get_contract("mirrorprotocol","oracle");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?;
    let res: Response<PriceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Price(res))
}
pub async fn anchor_protocol_borrower_limit(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> {
    // https://docs.anchorprotocol.com/smart-contracts/money-market/overseer#borrowlimitresponse
    let query = r#"{"borrow_limit": {"borrower": "wallet_acc_address"}}"#.replace("wallet_acc_address", wallet_acc_address); 
    let contract_addr = get_contract("anchorprotocol","mmOverseer");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?;
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
    let query = r#"{"borrower_info": {"borrower": "wallet_acc_address", "block_height": 1}}"#.replace("wallet_acc_address", wallet_acc_address); 
    let contract_addr = get_contract("anchorprotocol","mmMarket");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?;
    let res: Response<BorrowInfoResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::BorrowInfo(res))
} 
pub async fn anchor_protocol_anc_balance(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> {
    // https://docs.terra.money/Tutorials/Smart-contracts/Manage-CW20-tokens.html#checking-cw20-balance
    let query = r#"{"balance": {"address": "wallet_acc_address"}}"#.replace("wallet_acc_address", wallet_acc_address); 
    let contract_addr = get_contract("anchorprotocol","ANC");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?;
    let res: Response<BalanceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Balance(res))
} 
pub async fn anchor_protocol_balance(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> {
    // https://docs.terra.money/Tutorials/Smart-contracts/Manage-CW20-tokens.html#checking-cw20-balance
    let query = r#"{"balance": {"address": "wallet_acc_address"}}"#.replace("wallet_acc_address", wallet_acc_address); 
    let contract_addr = get_contract("anchorprotocol","aTerra");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?;
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
    let query = r#"{"staker": {"address": "wallet_acc_address"}}"#.replace("wallet_acc_address", wallet_acc_address); 
    let contract_addr = get_contract("anchorprotocol","gov");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?;
    let res: Response<StakerResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Staker(res))
}  

pub async fn anchor_protocol_whitelist() ->  anyhow::Result<ResponseResult> { 
    let query = r#"{"whitelist": {}}"#; 
    let contract_addr = get_contract("anchorprotocol","mmOverseer");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query).await?;
    let res: Response<AnchorWhitelistResult> = serde_json::from_str(&res)?; 
    Ok(ResponseResult::AnchorWhitelistResponse(res))
}  