/*
 * Queries that get information from blockchain data.
 *
 */

pub mod smart_contracts;

use smart_contracts::objects::*; 

use serde_json::Value; 
use core::str::FromStr; 
use smart_contracts::objects::meta::api::data::terra_contracts::{get_contract};
use smart_contracts::objects::meta::api::{    
    query_api,
    query_core_latest_block,
    query_core_block_at_height,
    query_core_block_txs};
 
use anyhow::anyhow; 
use regex::Regex;
use chrono::{DateTime};
use std::time::{Instant}; 


pub async fn get_tax_rate() -> anyhow::Result<ResponseResult> { 
        let res: String = query_api("https://lcd.terra.dev/treasury/tax_rate").await?;
        let res: Response<String> = serde_json::from_str(&res)?;
        return Ok(ResponseResult::TaxRate(res));
}

pub async fn get_tax_caps() -> anyhow::Result<ResponseResult> { 
        let res: String = query_api("https://lcd.terra.dev/treasury/tax_caps").await?;
        let res: Response<Vec<TaxCap>> = serde_json::from_str(&res)?;
        return Ok(ResponseResult::TaxCaps(res));
}

pub async fn blocks_per_year_query() -> anyhow::Result<ResponseResult> {
    let latest_block = query_core_latest_block().await?;      
    let historic_block_height = latest_block.block.header.height - (60*60*24*30)/6;
    let historic_block = query_core_block_at_height(historic_block_height as u64).await?;

    let time_difference = latest_block.block.header.time.timestamp_millis() - historic_block.block.header.time.timestamp_millis();
    let height_difference = latest_block.block.header.height - historic_block.block.header.height;
    let block_per_millis = height_difference as f64 / time_difference as f64;
    let blocks_per_year = block_per_millis *1000f64 * 60f64 * 60f64 * 24f64 * 365f64;

    Ok(ResponseResult::Blocks(Response{
        height: latest_block.block.header.height.to_string(),
        result: BlocksPerYearResponse {
                blocks_per_year: blocks_per_year,
                blocks_per_millis: block_per_millis,
                latest_block: serde_json::to_string_pretty(&latest_block)?,
                historic_block: serde_json::to_string_pretty(&historic_block)?,
            }
        }))
}

pub async fn get_block_txs_fee_data(key: &str) -> anyhow::Result<ResponseResult> { 
 
    let start = Instant::now();  

    let mut tx_data: Vec<TXLog> = Vec::new();
    let mut temp_offset = "0".to_string(); 

    let mut err_count = 0;

    while tx_data.len()<10 && start.elapsed().as_secs() < 60*3 && err_count < 2 {
        let mut next: anyhow::Result<String> = Ok("0".to_string());
        if key == "claim_rewards" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,get_contract("anchorprotocol","mmMarket").as_ref(),"claim_rewards","claim_amount").await;
        }
        if key == "staking" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,get_contract("anchorprotocol","ANC").as_ref(),"staking","amount").await; 
        }
        if key == "redeem_stable" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,get_contract("anchorprotocol","aTerra").as_ref(),"redeem_stable","redeem_amount").await; 
        }
        if key == "deposit_stable" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,get_contract("anchorprotocol","mmMarket").as_ref(),"deposit_stable","deposit_amount").await; 
        }
        if key == "repay_stable" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,get_contract("anchorprotocol","mmMarket").as_ref(),"repay_stable","repay_amount").await; 
        }
        if key == "borrow_stable" {
           next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,get_contract("anchorprotocol","mmMarket").as_ref(),"borrow_stable","borrow_amount").await; 
        }
        if key == "provide_liquidity" {
           next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,get_contract("anchorprotocol","ANC-UST LP").as_ref(),"provide_liquidity","null").await; 
        }
        if key == "staking_lp" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,get_contract("anchorprotocol","ANC-UST LP").as_ref(),"staking_lp","null").await; 
        }

        if next.is_ok() {
            temp_offset = next.unwrap();
            err_count = 0;
        }else{
            err_count = err_count + 1;
        }
    }

    if tx_data.len()<10 && start.elapsed().as_secs() >= 60*3 {
        return Err(anyhow!("Unexpected Error: Timeout!"));
    }

    if tx_data.len() == 0 {
        return Err(anyhow!("Unexpected Error: To many errors!"));
    }

    Ok(ResponseResult::Transactions(Response{
        height: tx_data[0].height.to_string(),
        result: tx_data
        }))

}


pub async fn get_txs_fee_data(offset: &str, tx_data: &mut Vec<TXLog>,account: &str, query_msg: &str, amount_field: &str) -> anyhow::Result<String> {

        let query = format!("https://fcd.terra.dev/v1/txs?offset={}&limit=100&account={}",offset, account); 
        let res: String = query_api(query.as_str()).await?;
        let res: Value = serde_json::from_str(&res)?;

        let entries = res.get("txs").ok_or(anyhow!("no txs"))?.as_array().ok_or(anyhow!("no array"))?;
 
        for entry in entries {
             match get_tx_log(entry, account, query_msg, amount_field) {
                Ok(txlog) => {
                    tx_data.push(txlog);
                },
                Err(_) => {
                }
             };
        } 
        let re = Regex::new(r"[^0-9]").unwrap();
        Ok(re.replace_all(res.get("next").ok_or(anyhow!("no next"))?.to_string().as_str(),"").to_string())
}


fn get_tx_log(entry: &Value, account: &str, query_msg: &str, amount_field: &str) -> anyhow::Result<TXLog> {

    let msg = entry.get("tx").ok_or(anyhow!("no tx"))?.get("value").ok_or(anyhow!("no value"))?.get("msg").ok_or(anyhow!("no msg"))?.as_array().ok_or(anyhow!("no array"))?; 
               
    if  (msg.len() == 2 && 
            (
            query_msg=="provide_liquidity" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("contract").ok_or(anyhow!("no contract"))? == "terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("increase_allowance").ok_or(anyhow!("no increase_allowance"))?.get("spender").ok_or(anyhow!("no spender"))? == "terra1qr2k6yjjd5p2kaewqvg93ag74k6gyjr7re37fs" &&
            msg[1].get("value").ok_or(anyhow!("no value"))?.get("contract").ok_or(anyhow!("no contract"))? == "terra1qr2k6yjjd5p2kaewqvg93ag74k6gyjr7re37fs" &&
            msg[1].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("provide_liquidity").ok_or(anyhow!("no provide_liquidity"))?.get("assets").ok_or(anyhow!("no assets"))?.as_array().ok_or(anyhow!("no array"))?.len() == 2 &&
            msg[1].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("provide_liquidity").ok_or(anyhow!("no provide_liquidity"))?.get("assets").ok_or(anyhow!("no assets"))?.as_array().ok_or(anyhow!("no array"))?[0].get("info").ok_or(anyhow!("no info"))?.get("token").ok_or(anyhow!("no token"))?.get("contract_addr").ok_or(anyhow!("no contract_addr"))? == "terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76" &&
            msg[1].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("provide_liquidity").ok_or(anyhow!("no provide_liquidity"))?.get("assets").ok_or(anyhow!("no assets"))?.as_array().ok_or(anyhow!("no array"))?[1].get("info").ok_or(anyhow!("no info"))?.get("native_token").ok_or(anyhow!("no native_token"))?.get("denom").ok_or(anyhow!("no denom"))? == "uusd"
            )
        )
        ||  (msg.len() == 1 &&
            ( 
            (query_msg=="staking_lp" && 
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("send").ok_or(anyhow!("no send"))?.get("msg").ok_or(anyhow!("no msg"))?.to_string().contains("eyJkZXBvc2l0Ijp7fX0=") &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("send").ok_or(anyhow!("no send"))?.get("contract").ok_or(anyhow!("no contract"))? == "terra1zgrx9jjqrfye8swykfgmd6hpde60j0nszzupp9"
            )   
        ||  (query_msg=="staking" && 
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("send").ok_or(anyhow!("no send"))?.get("msg").ok_or(anyhow!("no msg"))?.to_string().contains("eyJzdGFrZV92b3RpbmdfdG9rZW5zIjp7fX0=") &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("send").ok_or(anyhow!("no send"))?.get("contract").ok_or(anyhow!("no contract"))? == "terra1f32xyep306hhcxxxf7mlyh0ucggc00rm2s9da5"
            ) 
        || (query_msg=="claim_rewards" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("claim_rewards") != None
            )
        || (query_msg=="redeem_stable" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("send").ok_or(anyhow!("no send"))?.get("msg").ok_or(anyhow!("no msg"))?.to_string().contains("eyJyZWRlZW1fc3RhYmxlIjp7fX0=")
            )
        || (query_msg=="deposit_stable" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("deposit_stable") != None
            )
        || (query_msg=="repay_stable" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("repay_stable") != None
            )
        || (query_msg=="borrow_stable" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("borrow_stable") != None
            )
        ) && msg[0].get("value").ok_or(anyhow!("no value"))?.get("contract").ok_or(anyhow!("no contract"))? == account)
    {
            let gas_wanted = entry.get("gas_wanted").ok_or(anyhow!("no gas_wanted"))?;  // gas_limit // gas requested
            let gas_used = entry.get("gas_used").ok_or(anyhow!("no gas_used"))?;        // used


            let events = entry.get("logs").ok_or(anyhow!("no logs"))?.as_array().ok_or(anyhow!("no array"))?;
            if events.len() > 0 {

                let events = events[0].get("events").ok_or(anyhow!("no events"))?.as_array().ok_or(anyhow!("no array"))?;


                let mut amount = "0".to_string();
                for i in 0..events.len() {
                    if events[i].get("type").ok_or(anyhow!("no type"))? == "wasm" {
                        for ii in 0..events[i].get("attributes").ok_or(anyhow!("no attributes"))?.as_array().ok_or(anyhow!("no array"))?.len() {
                            if events[i].get("attributes").unwrap().as_array().unwrap()[ii].get("key").ok_or(anyhow!("no key"))?.to_string().contains(amount_field) {
                                amount = events[i].get("attributes").unwrap().as_array().unwrap()[ii].get("value").ok_or(anyhow!("no value"))?.to_string();
                            } 
                        }
                    }
                }


                let fee = entry.get("tx").unwrap().get("value").unwrap().get("fee").ok_or(anyhow!("no fee"))?;
                //let gas_limit = fee.get("gas").ok_or(anyhow!("no gas"))?; // same as gas_wanted
                
                let fee = fee.get("amount").ok_or(anyhow!("no amount"))?.as_array().ok_or(anyhow!("no array"))?;

                if fee.len() == 1 {

                    if fee[0].get("denom").ok_or(anyhow!("no denom"))?.to_string().contains("uusd") {

                       
                        let re = Regex::new(r"[^0-9]").unwrap();

                        let transaction_fee = re.replace_all(fee[0].get("amount").ok_or(anyhow!("no amount"))?.to_string().as_str(), "").to_string();
                        let tx_height = re.replace_all(entry.get("height").ok_or(anyhow!("no height"))?.to_string().as_str(), "").to_string();

                        let gas_used = re.replace_all(gas_used.to_string().as_str(), "").to_string();
                        let gas_wanted =  re.replace_all(gas_wanted.to_string().as_str(), "").to_string();

                        let amount = re.replace_all(amount.as_str(), "").to_string();


                        return Ok(TXLog { 
                            height: tx_height.parse::<u64>()?, 
                            timestamp: DateTime::parse_from_rfc3339(entry.get("timestamp").ok_or(anyhow!("no timestamp"))?.to_string().replace(r#"""#, "").as_str())?.timestamp(), 
                            gas_wanted: rust_decimal::Decimal::from_str(gas_wanted.as_str())?, 
                            gas_used: rust_decimal::Decimal::from_str(gas_used.as_str())?, 
                            amount: rust_decimal::Decimal::from_str(amount.as_str())?, 
                            fee_denom: "uusd".to_string(),
                            fee_amount: rust_decimal::Decimal::from_str(transaction_fee.as_str())?,
                            // gas_adjustment = fee_amount / (gas_wanted * gas_price)
                            raw_log: entry.get("raw_log").ok_or(anyhow!("no raw_log"))?.to_string()
                        });
                    }
                }
            }
    }
    Err(anyhow!("Error: Invalid format."))
}


// TODO: timeout this function to prevent blocking in case of chaos
pub async fn get_block_txs_deposit_stable_apy() -> anyhow::Result<ResponseResult> { 

    let latest_block = query_core_latest_block().await?;      
    let historic_block_height = latest_block.block.header.height - (60*60*24*30)/6;
    let historic_block = query_core_block_at_height(historic_block_height as u64).await?;

    let latest_stablecoin_deposits = get_block_txs_deposit_stable(latest_block.block.header.height).await?;
    let historic_stablecoin_deposits = get_block_txs_deposit_stable(historic_block.block.header.height).await?;

    let latest_result = &latest_stablecoin_deposits.as_stablecoin_deposits().unwrap().result[0];
    let historic_result = &historic_stablecoin_deposits.as_stablecoin_deposits().unwrap().result[0];

    let exchange_rate_difference = latest_result.exchange_rate.checked_sub(historic_result.exchange_rate).unwrap();
    let time_difference: i64 = latest_result.timestamp - historic_result.timestamp;
    let unit: f64 = (365*24*60*60) as f64 / time_difference as f64;

    let exchange_rate_difference_anual = rust_decimal::Decimal::from_str(unit.to_string().as_str()).unwrap().checked_mul(exchange_rate_difference).unwrap();

     Ok(ResponseResult::EarnAPY(Response{
        height: latest_block.block.header.height.to_string(), 
        result: APY {
            apy: exchange_rate_difference_anual.checked_div(latest_result.exchange_rate).unwrap(),
            result: format!("{:?}\n\n{:?}",latest_stablecoin_deposits.as_stablecoin_deposits().unwrap().result,historic_stablecoin_deposits.as_stablecoin_deposits().unwrap().result)
        }
        }))
}


pub async fn get_block_txs_deposit_stable(height: u64) -> anyhow::Result<ResponseResult> { 


    let start = Instant::now();  

    let mut stablecoin_deposits: Vec<DepositStableLog> = Vec::new();
    let mut count = 0;

    while stablecoin_deposits.len()<1 && start.elapsed().as_secs()<60*3 {

        let transactions = query_core_block_txs(height-count,None, Some(100)).await?; // terra_rust_api::client::tx_types::V1TXSResult
        let tx_responses = transactions.tx_responses;

        for index in 0..tx_responses.len() {
            if tx_responses[index].raw_log.contains("deposit_stable") 
                && tx_responses[index].raw_log.contains("mint_amount") 
                && tx_responses[index].raw_log.contains("deposit_amount") {

                let mut mint_amount = "0".to_string();
                let mut deposit_amount = "0".to_string();

                let fist_tx_log: Log = serde_json::from_str(tx_responses[index].logs[0].to_string().as_str())?;
                
                for i in 0..fist_tx_log.events.len() {
                    if fist_tx_log.events[i].type_field == "wasm" {
                        for ii in 0..fist_tx_log.events[i].attributes.len() {
                            if fist_tx_log.events[i].attributes[ii].key == "mint_amount" {
                                mint_amount = fist_tx_log.events[i].attributes[ii].value.to_owned();
                            }
                            if fist_tx_log.events[i].attributes[ii].key == "deposit_amount" { 
                                deposit_amount = fist_tx_log.events[i].attributes[ii].value.to_owned();   
                            }
                        }
                    }
                }  
                let mint_amount = rust_decimal::Decimal::from_str(mint_amount.as_str());
                let deposit_amount = rust_decimal::Decimal::from_str(deposit_amount.as_str());

                match (mint_amount,deposit_amount) {
                    (Ok(mint),Ok(deposit)) => {
                        let exchange_rate = deposit.checked_div(mint);
                        if let Some(exchange) = exchange_rate {
                             stablecoin_deposits.push( DepositStableLog { height: height-count, timestamp: tx_responses[index].timestamp.timestamp(), mint_amount: mint, deposit_amount: deposit, exchange_rate: exchange});
                        }
                    },
                    _ => {

                    }
                }  
            }
        }
        count = count +1;
    }

    if stablecoin_deposits.len()<1 && start.elapsed().as_secs() >= 60*3 {
        return Err(anyhow!("Unexpected Error: Timeout!"));
    }

    Ok(ResponseResult::StablecoinDeposits(Response{
        height: height.to_string(),
        result: stablecoin_deposits
        }))
}