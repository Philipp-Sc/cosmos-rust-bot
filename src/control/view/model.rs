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

use tokio::task::JoinHandle;

use std::sync::Arc; 
use tokio::sync::RwLock; 
 

use std::{thread, time};
 
use std::time::{Duration, Instant};

use tokio::time::timeout;
use tokio::time::Timeout;

use tokio::time::sleep;


use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};


pub enum MaybeOrPromise { 
    Data(QueryData),
    MetaData(MetaData),
}

pub enum QueryData { 
    Maybe(Maybe), // add timestamp here.
    Task(JoinHandle<anyhow::Result<ResponseResult>>), 
}

pub enum MetaData { 
    Maybe(anyhow::Result<String>),
    Task(JoinHandle<anyhow::Result<String>>), // not used
}

pub struct Maybe {
    pub data: anyhow::Result<ResponseResult>,   
    pub timestamp: i64,
} 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSettings {
    pub wallet_acc_address: String,  
    pub trigger_percentage: Decimal, 
    pub max_gas_adjustment: Decimal, 
    pub min_ust_balance: Decimal, 
    pub gas_adjustment_preference: Decimal, 
} 
 

/*
 * returns the value for the given key, if the enum is of the type Maybe.
 * will not await the future if it is not yet resolved.
 * in that case it returns an error.
 *
 */
pub async fn get_data_maybe_or_meta_data_maybe(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str) -> anyhow::Result<ResponseResult> { 
    
    let map = tasks.read().await; 
    let res = map.get(key).ok_or(anyhow!("Error: key does not exist"))?;

    if let MaybeOrPromise::Data(QueryData::Maybe(m)) = res {
        match &m.data {
            Ok(n) => {
                return Ok(n.clone());
            },
            Err(e) => {
                return Err(anyhow!("Error: {:?}",e));
            }
        } 
    }
    if let MaybeOrPromise::MetaData(MetaData::Maybe(m)) = res {
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

 // specifiy a max timeout //timeout(Duration::from_millis(10), rx) // use tokio::time::timeout;
 // if reached return err.

 // task.await is blocking. // current behaviour leads to one run being as fast as the slowest task.
pub async fn get_data_maybe_or_await_task(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str) -> anyhow::Result<ResponseResult> { 
        
        /* this is only efficient with low refresh rates. Otherwise this adds a second delay to this function.
        let maybe = get_data_maybe_or_meta_data_maybe(tasks,key).await;
        if maybe.is_ok() {
            return maybe;
        }*/

        let mut map = tasks.write().await; 
        let mut res = map.get_mut(key).ok_or(anyhow!("Error: key does not exist"))?;
        
        if let MaybeOrPromise::Data(QueryData::Task(task)) = res { 
 
            let maybe: Result<ResponseResult, anyhow::Error>  = match task.await {
                Ok(n) => { n },
                Err(e) => { Err(anyhow!("Error: {:?}",e)) } // JoinError
            };
            let maybe: Maybe = Maybe {data: maybe, timestamp: Utc::now().timestamp()};

            *res = MaybeOrPromise::Data(QueryData::Maybe(maybe));  

            if let MaybeOrPromise::Data(QueryData::Maybe(m)) = res {
                match &m.data {
                    Ok(n) => {
                        return Ok(n.clone());
                    },
                    Err(e) => {
                        return Err(anyhow!("Error: {:?}",e));
                    }
                } 
            } 
        } else if let MaybeOrPromise::Data(QueryData::Maybe(maybe)) = res {
            match &maybe.data {
                Ok(n) => {
                    return Ok(n.clone());
                },
                Err(e) => {
                    return Err(anyhow!("Error: {:?}",e));
                }
            } 
        } 
        return Err(anyhow!("Unexpected Error: Unreachable point reached."));
        
 }

pub async fn get_timestamp_or_await_task(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str) -> anyhow::Result<i64> { 
        
        /* this is only efficient with low refresh rates. Otherwise this adds a second delay to this function.
        let maybe = get_data_maybe_or_meta_data_maybe(tasks,key).await;
        if maybe.is_ok() {
            return maybe; 
        }*/

        let mut map = tasks.write().await; 
        let mut res = map.get_mut(key).ok_or(anyhow!("Error: key does not exist"))?;
        
        if let MaybeOrPromise::Data(QueryData::Task(task)) = res { 
 
            let maybe: Result<ResponseResult, anyhow::Error>  = match task.await {
                Ok(n) => { n },
                Err(e) => { Err(anyhow!("Error: {:?}",e)) } // JoinError
            };
            let maybe: Maybe = Maybe {data: maybe, timestamp: Utc::now().timestamp()};

            *res = MaybeOrPromise::Data(QueryData::Maybe(maybe));  

            if let MaybeOrPromise::Data(QueryData::Maybe(m)) = res {
                return Ok(m.timestamp);
            } 
        } else if let MaybeOrPromise::Data(QueryData::Maybe(maybe)) = res {
            return Ok(maybe.timestamp);
        } 
        return Err(anyhow!("Unexpected Error: Unreachable point reached."));
        
 }

/*
 * await promise and save result, then return result
 * or access saved result and return result.
 *
 * always returns a copy (clone)
 */
 // will panic when no value for key exists!
pub async fn get_meta_data_maybe_or_await_task(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str) -> anyhow::Result<String> { 
 

        let map = tasks.read().await; 
        let res = map.get(key).ok_or(anyhow!("Error: key does not exist"))?;
        /*
        if let MaybeOrPromise::MetaData(MetaData::Task(task)) = res { 
 
            let maybe: Result<String, anyhow::Error>  = match task.await {
                Ok(n) => { n },
                Err(e) => { Err(anyhow!("Error: {:?}",e)) } // JoinError
            };

            *map.get_mut(key).ok_or(anyhow!("Error: key does not exist"))? = MaybeOrPromise::MetaData(MetaData::Maybe(maybe));  
 
            let res = map.get(key).ok_or(anyhow!("Error: key does not exist"))?;

            if let MaybeOrPromise::MetaData(MetaData::Maybe(m)) = res {
                match m {
                    Ok(n) => {
                        return Ok(n.clone());
                    },
                    Err(e) => {
                        return Err(anyhow!("Error: {:?}",e));
                    }
                } 
            } 
        } else*/
        if let MaybeOrPromise::MetaData(MetaData::Maybe(maybe)) = res {
            match maybe {
                Ok(n) => {
                    return Ok(n.clone());
                },
                Err(e) => {
                    return Err(anyhow!("Error: {:?}",e));
                }
            } 
        } 
        return Err(anyhow!("Unexpected Error: Unreachable point reached."));
        
 }

  pub async fn get_timestamps_of_resolved_tasks(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &[&str]) -> Vec<i64> {

    let mut keys: Vec<i64> = Vec::new(); 
        
    for k in req {
         // if the functions returns a value in the given time it is considered resolved.
         match timeout(Duration::from_millis(100), get_timestamp_or_await_task(tasks,k)).await {
            Ok(Ok(timestamp)) => { 
                keys.push(timestamp);
            },
            Ok(Err(_)) => {
                keys.push(0i64);
            },
            Err(_) => { 
                keys.push(0i64);
            }
         }
    }
    return keys;
  } 

  pub async fn get_keys_of_running_tasks<'a>(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &'a [&str]) -> Vec<&'a str> {

    let mut keys: Vec<&str> = Vec::new(); 
        
    for k in req {
         // if the functions returns a value in the given time it is considered resolved.
         match timeout(Duration::from_millis(100), get_data_maybe_or_await_task(tasks,k)).await {
            Err(_) => { 
                keys.push(k);
            },
            Ok(_) => { 
            }
         }
    }
    return keys;
  } 

pub async fn await_running_tasks(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &[&str]) -> anyhow::Result<String> {

    for k in req { 
         get_data_maybe_or_await_task(tasks,k).await;
    }
    Ok("finished".to_string())
} 
  
  /*
  * all required queries are triggered here in async fashion
  *
  * retrieve the value when it is needed: "data.get_mut(String).unwrap().await"
  * use try_join!, join! or select! macros to optimise retrieval of multiple values.
  */
 pub async fn requirements(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, user_settings: &UserSettings, req: &Vec<&str>) { 
          
         let gas_prices = fetch_gas_price().await; 

         if !gas_prices.response_status.unwrap().is_ok {
            println!("WARNING: Using static gas_prices.");
         } 

         let mut map = tasks.write().await;

         for cmd in req {
            let vec: Vec<&str> = cmd.split(" ").collect();
            let length = vec.len();
            let mut into_iter = vec.into_iter();
            if length == 1 {

                let first: String = into_iter.next().unwrap().to_string();
                match first.as_ref() {
                    "earn_apy" => {
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_block_txs_deposit_stable_apy().await;   
                            }
                        }); 
                        map.insert("earn_apy".to_string(), MaybeOrPromise::Data(QueryData::Task(handle))); 
                        
                    }
                    "blocks_per_year" => {
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return blocks_per_year_query().await;   
                            }
                        }); 
                        map.insert("blocks_per_year".to_string(), MaybeOrPromise::Data(QueryData::Task(handle))); 
                    }
                    "borrow_limit" => {  
                        let wallet = user_settings.wallet_acc_address.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_borrower_limit(wallet,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert("borrow_limit".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
 
                    },
                    "borrow_info" => {
                        let wallet = user_settings.wallet_acc_address.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_borrower_info(wallet,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert("borrow_info".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                        
                    },
                    "balance" => {
                        let wallet = user_settings.wallet_acc_address.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_balance(wallet,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert("balance".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                        
                    },
                    "terra_balances" => {
                        let wallet = user_settings.wallet_acc_address.to_owned(); 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return terra_balances(wallet).await;   
                            }
                        });
                       
                            map.insert("terra_balances".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                        
                    },
                    "anc_balance" => {
                        let wallet = user_settings.wallet_acc_address.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_anc_balance(wallet,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert("anc_balance".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    },
                    "staker" => {
                        let wallet = user_settings.wallet_acc_address.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_staker(wallet,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert("staker".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                        
                    },
                    "api/v2/distribution-apy" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return query_api_distribution_apy().await;   
                            }
                        });
                       
                            map.insert("api/v2/distribution-apy".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                    },
                    "api/v2/gov-reward" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return query_api_gov_reward().await;   
                            }
                        });
                       
                            map.insert("api/v2/gov-reward".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    },
                    "anchor_protocol_txs_claim_rewards" => {  
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_block_txs_fee_data("claim_rewards").await;   
                            }
                        });
                       
                            map.insert("anchor_protocol_txs_claim_rewards".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    }, 
                    "anchor_protocol_txs_staking" => {  
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_block_txs_fee_data("staking").await;   
                            }
                        });
                       
                            map.insert("anchor_protocol_txs_staking".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                    }, 
                    "gas_fees_uusd" => {                      
                        map.insert("gas_fees_uusd".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Ok(gas_prices.response.as_ref().unwrap().uusd.to_string().to_owned()))));
                    }, 
                    "trigger_percentage" => {     
                        map.insert("trigger_percentage".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Ok(user_settings.trigger_percentage.to_string().to_owned()))));
                    },  
                    "max_gas_adjustment" => {      
                        map.insert("max_gas_adjustment".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Ok(user_settings.max_gas_adjustment.to_string().to_owned()))));
                    },    
                    "gas_adjustment_preference" => {      
                        map.insert("gas_adjustment_preference".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Ok(user_settings.gas_adjustment_preference.to_string().to_owned()))));
                    },    
                    "min_ust_balance" => {       
                        map.insert("min_ust_balance".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Ok(user_settings.min_ust_balance.to_string().to_owned()))));
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
                        let second_copy = second.to_owned();
                        let third_copy = third.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return state_query_msg(second_copy,third_copy,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert(format!("state {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                    },
                     "epoch_state" => {
                        // "epoch_state anchorprotocol mmMarket"
                        let second_copy = second.to_owned();
                        let third_copy = third.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return epoch_state_query_msg(second_copy,third_copy,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert(format!("epoch_state {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                    },
                     "config" => {
                        let second_copy = second.to_owned();
                        let third_copy = third.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return config_query_msg(second_copy,third_copy,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert(format!("config {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                    },
                    "simulation_cw20" => {
                        let third_copy = third.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return masset_to_ust(third_copy,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert(format!("simulation_cw20 {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    },
                    "core_swap" => { 
                        let second_copy = second.to_owned();
                        let third_copy = third.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return native_token_core_swap(second_copy,third_copy,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert(format!("core_swap {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
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
                        let second_copy = second.to_owned();
                        let third_copy = third.to_owned();
                        let fourth_copy = fourth.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return native_token_to_swap_pair(second_copy,third_copy,fourth_copy,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert(format!("simulation {} {} {}",second, third, fourth).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                       
                    },
                    "simulation_cw20" => {
                        let second_copy = second.to_owned();
                        let third_copy = third.to_owned();
                        let fourth_copy = fourth.to_owned();
                        let gas_prices_copy = gas_prices.response.as_ref().unwrap().to_owned();
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return cw20_to_swap_pair(second_copy,third_copy,fourth_copy,gas_prices_copy).await;   
                            }
                        });
                       
                            map.insert(format!("simulation_cw20 {} {} {}",second, third, fourth).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                     
                    }
                    &_ => {

                    }
                }
            }
        
        }        
}
 