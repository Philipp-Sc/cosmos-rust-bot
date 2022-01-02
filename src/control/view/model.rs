use rust_decimal::Decimal;
use core::str::FromStr;
  
 
use serde::Deserialize;
use serde::Serialize;

pub mod smart_contracts;

use smart_contracts::{ResponseResult};
use smart_contracts::meta::api::{fetch_gas_price, QueryResponse};
use smart_contracts::meta::api::data::{GasPrices};
use smart_contracts::meta::api::data::endpoints::get_terra_fcd;

use smart_contracts::{
    state_query_msg,
    epoch_state_query_msg,
    config_query_msg,
    native_token_core_swap,
    native_token_to_swap_pair,
    cw20_to_swap_pair,
    masset_to_ust,
    anchor_protocol_borrower_limit,
    anchor_protocol_borrower_info,
    anchor_protocol_balance,
    anchor_protocol_staker,
    query_api_distribution_apy,
    query_api_gov_reward,
    blocks_per_year_query,
    get_block_txs_deposit_stable_apy,
    get_block_txs_fee_data,
    anchor_protocol_anc_balance,
    terra_balances};

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use anyhow::anyhow;
use enum_as_inner::EnumAsInner;

 
pub enum MaybeOrPromise { 
    Data(QueryData),
    MetaData(MetaData),
}

pub enum QueryData { 
    Maybe(anyhow::Result<ResponseResult>),
    Promise(Pin<Box<dyn Future<Output = anyhow::Result<ResponseResult>>>>),
}

pub enum MetaData { 
    Maybe(anyhow::Result<String>),
    Promise(Pin<Box<dyn Future<Output = anyhow::Result<String>>>>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSettings {
    pub wallet_acc_address: String,  
    pub trigger_percentage: Decimal, 
    pub max_gas_adjustment: Decimal, 
    pub min_ust_balance: Decimal, 
    pub gas_adjustment_preference: Decimal, 
} 

/*
 * returns all futures (MaybeOrPromise) specified, if they are not yet resolved.
 *
 */
 /*
pub async fn get_data_promise_or_meta_data_promise(data: &HashMap<String, MaybeOrPromise>,req: Vec<String>) -> String { 
          for key in req {
            let entry = data.get_mut(&key).unwrap();
            if let MaybeOrPromise::Data(QueryData::Promise(p)) = entry { 
                 tokio::spawn(async move { 
                    p.await
                });
            }/*else if let MaybeOrPromise::MetaData(MetaData::Promise(p)) = entry { 
                 req_promises.push(entry);
            }*/
        }
        return "".to_string()
        // instead of getting them, join and resolve them.
        //return tokio::join!(req_promises);
} */

/*
 * returns the value for the given key, if the enum is of the type Maybe.
 * will not await the future if it is not yet resolved.
 * in that case it returns an error.
 *
 */
pub fn get_data_maybe_or_meta_data_maybe(data: &HashMap<String, MaybeOrPromise>,key: &str) -> anyhow::Result<ResponseResult> { 
    let entry = data.get(key).unwrap();
    if let MaybeOrPromise::Data(QueryData::Maybe(m)) = entry {
        match m {
            Ok(n) => {
                return Ok(n.clone());
            },
            Err(e) => {
                return Err(anyhow!("Error: {:?}",e));
            }
        } 
    }
    if let MaybeOrPromise::MetaData(MetaData::Maybe(m)) = entry {
        match m {
            Ok(n) => {
                return Ok(ResponseResult::Text(n.clone()));
            },
            Err(e) => {
                return Err(anyhow!("Error: {:?}",e));
            }
        } 
    }
    return Err(anyhow!("Info: Key '{}' is not yet resolved. ",key));
}
/*
 * await promise and save result, then return result
 * or access saved result and return result.
 *
 * always returns a copy (clone)
 */
 // will panic when no value for key exists!
pub async fn get_data_maybe_or_resolve_promise(data: &mut HashMap<String, MaybeOrPromise>,key: &str) -> anyhow::Result<ResponseResult> { 
        let entry = data.get_mut(key).unwrap();
            /* &mut is needed to get the Future, see:
             *   = help: the trait `Future` is not implemented for `&Pin<Box<dyn Future<Output = Result<ResponseResult, anyhow::Error>>>>`
             *   = note: `Future` is implemented for `&mut std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = std::result::Result<control::view::model::smart_contracts::ResponseResult, anyhow::Error>>>>`, but not for `&std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = std:
             */
        if let MaybeOrPromise::Data(QueryData::Promise(p)) = entry { 
            let res = p.await;
            data.insert(key.to_owned(), MaybeOrPromise::Data(QueryData::Maybe(res))); 

            let entry = data.get(key).unwrap();
            if let MaybeOrPromise::Data(QueryData::Maybe(m)) = entry {
                match m {
                    Ok(n) => {
                        return Ok(n.clone());
                    },
                    Err(e) => {
                        return Err(anyhow!("Error: {:?}",e));
                    }
                } 
            }
        }else{ 
            if let MaybeOrPromise::Data(QueryData::Maybe(m)) = entry {
                match m {
                    Ok(n) => {
                        return Ok(n.clone());
                    },
                    Err(e) => {
                        return Err(anyhow!("Error: {:?}",e));
                    }
                } 
            }
        } 
        return Err(anyhow!("Unexpected Error: "));
} 
pub async fn get_meta_data_maybe_or_resolve_promise(data: &mut HashMap<String, MaybeOrPromise>,key: &str) -> anyhow::Result<String> { 
        let entry = data.get_mut(key).unwrap();
        if let MaybeOrPromise::MetaData(MetaData::Promise(p)) = entry { 
            let res = p.await;
            data.insert(key.to_owned(), MaybeOrPromise::MetaData(MetaData::Maybe(res))); 

            let entry = data.get(key).unwrap();
            if let MaybeOrPromise::MetaData(MetaData::Maybe(m)) = entry {
                match m {
                    Ok(n) => {
                        return Ok(n.clone());
                    },
                    Err(e) => {
                        return Err(anyhow!("Error: {:?}",e));
                    }
                } 
            }
        }else{ 
            if let MaybeOrPromise::MetaData(MetaData::Maybe(m)) = entry {
                match m {
                    Ok(n) => {
                        return Ok(n.clone());
                    },
                    Err(e) => {
                        return Err(anyhow!("Error: {:?}",e));
                    }
                } 
            }
        } 
        return Err(anyhow!("Unexpected Error: "));
} 

  /*
  * all required queries are triggered here in async fashion
  *
  * retrieve the value when it is needed: "data.get_mut(String).unwrap().await"
  * use try_join!, join! or select! macros to optimise retrieval of multiple values.
  */
 
 pub async fn requirements(data: &mut HashMap<String, MaybeOrPromise>, user_settings: &UserSettings, req: &Vec<&str>) { 
         

         let gas_prices = fetch_gas_price().await; 

         if !gas_prices.response_status.unwrap().is_ok {
            println!("WARNING: Using static gas_prices.");
         }
         //println!("{:?}\n",&gas_prices); 

         for cmd in req {
            let vec: Vec<&str> = cmd.split(" ").collect();
            let length = vec.len();
            let mut into_iter = vec.into_iter();
            if length == 1 {

                let first: String = into_iter.next().unwrap().to_string();
                match first.as_ref() {
                    "earn_apy" => {
                        data.insert("earn_apy".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(get_block_txs_deposit_stable_apy()))));
                    }
                    "blocks_per_year" => {
                        data.insert("blocks_per_year".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(blocks_per_year_query()))));
                    }
                    "borrow_limit" => {
                        data.insert("borrow_limit".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(anchor_protocol_borrower_limit(user_settings.wallet_acc_address.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                        //.unwrap().as_borrow_limit().unwrap()))); 
                    },
                    "borrow_info" => {
                        data.insert("borrow_info".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(anchor_protocol_borrower_info(user_settings.wallet_acc_address.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                        //.unwrap().as_borrow_info().unwrap()))); 
                    },
                    "balance" => {
                        data.insert("balance".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(anchor_protocol_balance(user_settings.wallet_acc_address.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                        //.unwrap().as_balance().unwrap()))); 
                    },
                    "terra_balances" => {
                        data.insert("terra_balances".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(terra_balances(user_settings.wallet_acc_address.to_owned())))));
                        //.unwrap().as_balance().unwrap()))); 
                    },
                    "anc_balance" => {
                        data.insert("anc_balance".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(anchor_protocol_anc_balance(user_settings.wallet_acc_address.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                        //.unwrap().as_balance().unwrap()))); 
                    },
                    "staker" => {
                        data.insert("staker".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(anchor_protocol_staker(user_settings.wallet_acc_address.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                        //.unwrap().as_staker().unwrap())));
                    },
                    "api/v2/distribution-apy" => {
                        data.insert("api/v2/distribution-apy".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(query_api_distribution_apy()))));
                    },
                    "api/v2/gov-reward" => {
                        data.insert("api/v2/gov-reward".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(query_api_gov_reward()))));
                    },
                    "anchor_protocol_txs_claim_rewards" => { 
                        data.insert("anchor_protocol_txs_claim_rewards".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(get_block_txs_fee_data("claim_rewards")))));
                    }, 
                    "anchor_protocol_txs_staking" => { 
                        data.insert("anchor_protocol_txs_staking".to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(get_block_txs_fee_data("staking")))));
                    }, 
                    "gas_fees_uusd" => {
                        data.insert("gas_fees_uusd".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Ok(gas_prices.response.as_ref().unwrap().uusd.to_string().to_owned()))));
                    }, 
                    "trigger_percentage" => {
                        data.insert("trigger_percentage".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Ok(user_settings.trigger_percentage.to_string().to_owned()))));
                    },  
                    "max_gas_adjustment" => {
                        data.insert("max_gas_adjustment".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Ok(user_settings.max_gas_adjustment.to_string().to_owned()))));
                    },    
                    "gas_adjustment_preference" => {
                        data.insert("gas_adjustment_preference".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Ok(user_settings.gas_adjustment_preference.to_string().to_owned()))));
                    },    
                    "min_ust_balance" => {
                        data.insert("min_ust_balance".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Ok(user_settings.min_ust_balance.to_string().to_owned()))));
                    },     
                    &_ => {

                    }
                }
 
            }else if length == 3 {

                let first = into_iter.next().unwrap();
                let second = into_iter.next().unwrap();
                let third = into_iter.next().unwrap();
                match first.as_ref() { 
                    "state" => { 
                        // "state anchorprotocol bLunaHub"
                        // "state anchorprotocol mmMarket"
                        data.insert(format!("state {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(state_query_msg(second.to_owned(),third.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                    },
                     "epoch_state" => {
                        // "epoch_state anchorprotocol mmMarket"
                        data.insert(format!("epoch_state {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(epoch_state_query_msg(second.to_owned(),third.to_owned(),gas_prices.response.as_ref().unwrap().to_owned()))))); 
                    },
                     "config" => {
                        data.insert(format!("config {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(config_query_msg(second.to_owned(),third.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                        //.unwrap().as_config().unwrap().as_mm_interest_model().unwrap()); 
                        //.unwrap().as_config().unwrap().as_collector().unwrap()); 
                    },
                    "simulation_cw20" => {
                        data.insert(format!("simulation_cw20 {} {}",second, third), MaybeOrPromise::Data(QueryData::Promise(Box::pin(masset_to_ust(third.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                     //.unwrap().as_simulation().unwrap()))); 
                    },
                    "core_swap" => { 
                        data.insert(format!("core_swap {} {}",second, third), MaybeOrPromise::Data(QueryData::Promise(Box::pin(native_token_core_swap(second.to_owned(),third.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                        //.unwrap().as_simulation().unwrap()))); 
                        
                    
                    },
                    &_ => {

                    }
                }
            }else if length == 4 {

                let first = into_iter.next().unwrap();
                let second = into_iter.next().unwrap();
                let third = into_iter.next().unwrap();
                let fourth = into_iter.next().unwrap();
                match first.as_ref() { 
                    // luna_to_bluna: simulation uluna anchorprotocol terraswapblunaLunaPair
                    // luna_to_ust: simulation uluna terraswap uusd_uluna_pair_contract
                    // sdt_to_uluna: simulation usdr terraswap usdr_uluna_pair_contract
                    // ust_to_luna: simulation uusd terraswap uusd_uluna_pair_contract
                    // ust_to_psi: simulation uusd nexusprotocol Psi-UST pair
                    // ust_to_anc: simulation uusd anchorprotocol terraswapAncUstPair
                    "simulation" => {
                        data.insert(format!("simulation {} {} {}",second, third, fourth).to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(native_token_to_swap_pair(second.to_owned(), third.to_owned(), fourth.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                        //.unwrap().as_simulation().unwrap()); 
                    },
                    "simulation_cw20" => {
                        data.insert(format!("simulation_cw20 {} {} {}",second, third, fourth).to_string(), MaybeOrPromise::Data(QueryData::Promise(Box::pin(cw20_to_swap_pair(second.to_owned(), third.to_owned(), fourth.to_owned(),gas_prices.response.as_ref().unwrap().to_owned())))));
                        
                    }
                    &_ => {

                    }
                }

            }
         }        
}
 