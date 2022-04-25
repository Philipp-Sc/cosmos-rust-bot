
// includes functions from action/mod.rs

pub mod wallet;
pub mod requirements;

use requirements::{UserSettings,my_requirement_list};
use secstr::*;


use terra_rust_api_layer::services::blockchain::smart_contracts::objects::{ResponseResult};
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::meta::api::{fetch_gas_price, get_gas_price}; 
 
use terra_rust_api_layer::services::{
    query_api_distribution_apy,
    query_api_gov_reward,
    query_anchor_airdrops,
    query_api_anc_ust_lp_reward,
    query_api_spec_anc_ust_lp_reward};

use terra_rust_api_layer::services::blockchain::{ 
    get_tax_rate,
    get_tax_caps,
    blocks_per_year_query,
    get_block_txs_deposit_stable_apy,
    get_block_txs_fee_data};

use terra_rust_api_layer::services::blockchain::smart_contracts::{
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
    anchor_protocol_anc_balance,
    terra_balances,
    anchor_protocol_whitelist};

use std::collections::HashMap; 

use anyhow::anyhow; 

use tokio::task::JoinHandle;

use std::sync::Arc; 
use tokio::sync::RwLock; 
 
 
use std::time::{Duration};
use std::thread::sleep;
use tokio::time::timeout; 
 

use chrono::{Utc};

use core::pin::Pin;
use core::future::Future;


use terra_rust_bot_essentials::shared::Maybe as MaybeImported;
use terra_rust_bot_essentials::shared::Entry;

pub type Maybe<T> = MaybeImported<T>;

pub enum MaybeOrPromise { 
    Data(QueryData),
    MetaData(MetaData),
}

pub enum QueryData { 
    Maybe(Maybe<ResponseResult>), // add timestamp here.
    Task(JoinHandle<anyhow::Result<ResponseResult>>), 
}

pub enum MetaData { 
    Maybe(Maybe<String>),
//    Task(JoinHandle<anyhow::Result<String>>), // not used
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
        match &m.data {
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
 * returns the value for the given key, if the enum is of the type Maybe.
 * will not await the future if it is not yet resolved.
 * in that case it returns an error.
 * 
 * TODO: deprecated get_data_maybe_or_meta_data_maybe;
 */
pub async fn try_get_resolved(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str) -> anyhow::Result<Maybe<ResponseResult>> { 
    
    let map = tasks.read().await; 
    let res = map.get(key).ok_or(anyhow!("Error: key does not exist"))?;

    if let MaybeOrPromise::Data(QueryData::Maybe(m)) = res {
        match &m.data {
            Ok(n) => {
                return Ok(Maybe{
                    data:Ok(n.clone()),
                    timestamp:m.timestamp
                });
            },
            Err(e) => {
                return Ok(Maybe{
                    data:Err(anyhow!("{}",e.clone())),
                    timestamp:m.timestamp
                }); 
            }
        } 
    }
    if let MaybeOrPromise::MetaData(MetaData::Maybe(m)) = res {
        match &m.data {
            Ok(n) => {
                return Ok(Maybe{
                    data:Ok(ResponseResult::Text(n.clone())),
                    timestamp:m.timestamp
                });
            },
            Err(e) => {
                return Ok(Maybe{
                    data:Err(anyhow!("{}",e.clone())),
                    timestamp:m.timestamp
                }); 
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
        
        let mut map = tasks.write().await; 
        let res = map.get_mut(key).ok_or(anyhow!("Error: key does not exist"))?;
        
        if let MaybeOrPromise::Data(QueryData::Task(task)) = res { 
 
            let maybe: Result<ResponseResult, anyhow::Error>  = match task.await {
                Ok(n) => { n },
                Err(e) => { Err(anyhow!("Error: {:?}",e)) } // JoinError
            };
            let maybe: Maybe<ResponseResult>= Maybe {data: maybe, timestamp: Utc::now().timestamp()};

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

pub async fn await_task(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str) -> anyhow::Result<()> { 
        
        let mut map = tasks.write().await; 
        let res = map.get_mut(key).ok_or(anyhow!("Error: key does not exist"))?;
        
        if let MaybeOrPromise::Data(QueryData::Task(task)) = res { 
 
            let maybe: Result<ResponseResult, anyhow::Error>  = match task.await {
                Ok(n) => { n },
                Err(e) => { Err(anyhow!("Error: {:?}",e)) } // JoinError
            };
            let maybe: Maybe<ResponseResult>= Maybe {data: maybe, timestamp: Utc::now().timestamp()};

            *res = MaybeOrPromise::Data(QueryData::Maybe(maybe));  
        }  
        Ok(())
 }

 pub async fn get_resolved(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str) -> Maybe<ResponseResult> { 
        
        match await_task(tasks,key).await {
            Ok(_) => { // resolved
                return try_get_resolved(tasks,key).await.unwrap(); // always Ok(_) 
            },
            Err(e) =>{ // Error: key does not exit
                return Maybe{
                    data:Err(anyhow!("{}",e)),
                    timestamp: Utc::now().timestamp()
                }; 
            }
        }          
 }

pub async fn abort_tasks(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>) ->  anyhow::Result<()> {

    for (_, value) in tasks.write().await.iter_mut(){
        if let MaybeOrPromise::Data(QueryData::Task(task)) = value {
            task.abort();
        }
    }
    Ok(())
}

pub async fn get_timestamp_or_await_task(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str) -> anyhow::Result<i64> { 
        
        /* this is only efficient with low refresh rates. Otherwise this adds a second delay to this function.
        let maybe = get_data_maybe_or_meta_data_maybe(tasks,key).await;
        if maybe.is_ok() {
            return maybe; 
        }*/

        let mut map = tasks.write().await; 
        let res = map.get_mut(key).ok_or(anyhow!("Error: key does not exist"))?;
        
        if let MaybeOrPromise::Data(QueryData::Task(task)) = res { 
 
            let maybe: Result<ResponseResult, anyhow::Error>  = match task.await {
                Ok(n) => { n },
                Err(e) => { Err(anyhow!("Error: {:?}",e)) } // JoinError
            };
            let maybe: Maybe<ResponseResult>= Maybe {data: maybe, timestamp: Utc::now().timestamp()};

            *res = MaybeOrPromise::Data(QueryData::Maybe(maybe));  

            if let MaybeOrPromise::Data(QueryData::Maybe(m)) = res {
                return Ok(m.timestamp);
            } 
        } else if let MaybeOrPromise::Data(QueryData::Maybe(maybe)) = res {
            return Ok(maybe.timestamp);
        } else if let MaybeOrPromise::MetaData(MetaData::Maybe(maybe)) = res {
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


pub async fn get_meta_data_maybe(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str) -> anyhow::Result<Maybe<String>> { 
 
        let map = tasks.read().await; 
        let res = map.get(key).ok_or(anyhow!("Error: key does not exist"))?;

        if let MaybeOrPromise::MetaData(MetaData::Maybe(maybe)) = res {
            match &maybe.data {
                Ok(n) => {
                    return Ok(Maybe {data: Ok(n.clone()), timestamp: maybe.timestamp});
                },
                Err(e) => {
                    return Ok(Maybe {data: Err(anyhow!(e.to_string())), timestamp: maybe.timestamp});
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

    pub async fn get_oldest_timestamps_of_resolved_tasks(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &[&str]) -> i64 {

    let mut keys: i64 = Utc::now().timestamp();
        
    for k in req {
         // if the functions returns a value in the given time it is considered resolved.
         match timeout(Duration::from_millis(100), get_timestamp_or_await_task(tasks,k)).await {
            Ok(Ok(timestamp)) => { 
                if timestamp < keys {
                    keys = timestamp;
                } 
            },
            Ok(Err(_)) => {
                if 0i64 < keys {
                    keys = 0i64;
                } 
            },
            Err(_) => { 
                if 0i64 < keys {
                    keys = 0i64;
                } 
            }
         }
    }
    return keys;
  } 

  pub async fn get_keys_of_running_tasks<'a>(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>) -> Vec<String> {

    let mut keys: Vec<String> = Vec::new();

    for key in tasks.read().await.keys() {
        match try_get_resolved(tasks, key).await {
            Err(_) => {
                keys.push(key.to_owned());
            },
            Ok(_) => {}
        }
    }
    return keys;
  } 

  pub async fn get_keys_of_failed_tasks<'a>(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>) -> Vec<String> {

    let mut keys: Vec<String> = Vec::new();

    for key in tasks.read().await.keys() {
        // if the functions returns a value in the given time it is considered resolved.
        match try_get_resolved(tasks,key).await {
            Err(msg) => {
                if !msg.to_string().contains("Info"){
                    keys.push(key.to_owned());
                }
            },
            Ok(_) => {}  
        }
    }
    return keys;
  }

pub async fn register_value(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: String, value: String) {
    let mut map = tasks.write().await;
    map.insert(key,MaybeOrPromise::MetaData(MetaData::Maybe(Maybe {data: Ok(value), timestamp: Utc::now().timestamp()})));
}
   
pub async fn try_register_function(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: String, f: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static >>, timeout_duration: u64, block_duration_after_resolve: i64) {

    let mut does_not_exist = false;
 
    match tasks.read().await.get(&key) {
        Some(_) => {}, 
        None => {
            does_not_exist = true;
        }
    }

    let req: [&str; 1] = [&key];
    let timestamp = get_timestamps_of_resolved_tasks(tasks,&req).await[0];
    let now = Utc::now().timestamp();

    if does_not_exist || (timestamp > 0i64 && now - timestamp >= block_duration_after_resolve) {
        let handle = tokio::spawn(async move {   
                let result = timeout(Duration::from_secs(timeout_duration), f).await.unwrap_or(Maybe{data:Ok("timeout".to_string()),timestamp: Utc::now().timestamp()});                   
                Ok(ResponseResult::Text(result.data.unwrap_or("--".to_string())))
        });
        let mut map = tasks.write().await;
        map.insert(key, MaybeOrPromise::Data(QueryData::Task(handle))); 
    }
}

pub async fn await_function(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: String) -> Maybe<String> {
   match get_resolved(&tasks,&key).await {
        Maybe{data: Ok(succ),timestamp:t} => {
            return Maybe{data: Ok(succ.as_text().unwrap().to_string()),timestamp:t};
        },
        Maybe{data: Err(err),timestamp:t} => {
            return Maybe{data: Ok(err.to_string()),timestamp:t};
        }
     }
}
pub async fn requirements_next(now: i64, num_cpus: usize, offset: &mut usize, tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, user_settings: &UserSettings, wallet_acc_address: &Arc<SecUtf8>) ->  Vec<(Entry,usize)> {

    let req = my_requirement_list(&user_settings);

    let mut req_keys= Vec::new();
    for i in 0..req.len() {
        req_keys.push(req[i].0);
    }

    let mut entries: Vec<(Entry,usize)> = Vec::new();


    let mut req_unresolved = get_keys_of_running_tasks(&tasks).await;

    // waiting for unresolved tasks to catch up
    if req_unresolved.len() >= num_cpus {
        // anyway we need to have free threads to spawn more tasks
        // useful to wait here
        sleep(Duration::from_secs(10));
        req_unresolved = get_keys_of_running_tasks(&tasks).await;
    }

    let req_resolved_timestamps = get_timestamps_of_resolved_tasks(&tasks, &req_keys).await;
    let req_failed = get_keys_of_failed_tasks(&tasks).await;


    for x in 0..req_resolved_timestamps.len() {
        let entry = Entry {
            timestamp: now,
            key: req[x].0.to_string(),
            prefix: None,
            value: req_resolved_timestamps[x].to_string(),
            suffix: None,
            group: Some("[Task][History]".to_string()),
        };
        entries.push((entry,*offset));
        *offset += 1;
    }

    let mut req_to_update: Vec<(&str,i64)> = Vec::new();
    for i in 0..req.len() {
        // check req is not pending
        if !req_unresolved.contains(&req[i].0.to_string()) {
            // check req is failed or
            // req new or
            // time passed is greater than req duration
            if req_failed.contains(&req[i].0.to_string()) || req_resolved_timestamps[i] == 0i64 || ((now - req_resolved_timestamps[i]) > req[i].1 as i64) {
                req_to_update.push((&req[i].0,now - req_resolved_timestamps[i] - req[i].1 as i64));
            }
        }
    }
    req_to_update.sort_by_key(|k| k.1);
    let req_to_update: Vec<&str> = req_to_update.iter().map(|x| x.0).rev().take(num_cpus - req_unresolved.len()).collect();


    let entry = Entry {
        timestamp: now,
        key: "failed".to_string(),
        prefix: None,
        value: req_failed.len().to_string(),
        suffix: None,
        group: Some("[Task][Count]".to_string()),
    };
    entries.push((entry,*offset));
    *offset += 1;
    let entry = Entry {
        timestamp: now,
        key: "pending".to_string(),
        prefix: None,
        value: req_unresolved.len().to_string(),
        suffix: None,
        group: Some("[Task][Count]".to_string()),
    };
    entries.push((entry,*offset));
    *offset += 1;
    let entry = Entry {
        timestamp: now,
        key: "upcoming".to_string(),
        prefix: None,
        value: req_to_update.len().to_string(),
        suffix: None,
        group: Some("[Task][Count]".to_string()),
    };
    entries.push((entry,*offset));
    *offset += 1;
    let entry = Entry {
        timestamp: now,
        key: "all".to_string(),
        prefix: None,
        value: req_keys.len().to_string(),
        suffix: None,
        group: Some("[Task][Count]".to_string()),
    };
    entries.push((entry,*offset));
    *offset += 1;
    let entry = Entry {
        timestamp: now,
        key: "failed".to_string(),
        prefix: None,
        value: format!("{:?}", req_failed),
        suffix: None,
        group: Some("[Task][List]".to_string()),
    };
    entries.push((entry,*offset));
    *offset += 1;
    let entry = Entry {
        timestamp: now,
        key: "pending".to_string(),
        prefix: None,
        value: format!("{:?}", req_unresolved),
        suffix: None,
        group: Some("[Task][List]".to_string()),
    };
    entries.push((entry,*offset));
    *offset += 1;
    let entry = Entry {
        timestamp: now,
        key: "upcoming".to_string(),
        prefix: None,
        value: format!("{:?}", req_to_update),
        suffix: None,
        group: Some("[Task][List]".to_string()),
    };
    entries.push((entry,*offset));
    *offset += 1;
    let entry = Entry {
        timestamp: now,
        key: "all".to_string(),
        prefix: None,
        value: format!("{:?}", req_keys),
        suffix: None,
        group: Some("[Task][List]".to_string()),
    };
    entries.push((entry,*offset));
    *offset += 1;

    if req_unresolved.len() >= num_cpus {
        return entries;
    }
    requirements(&tasks, &user_settings, &wallet_acc_address, &req_to_update).await;
    entries
}
  /*
  * all required queries are triggered here in async fashion
  *
  * retrieve the value when it is needed: "data.get_mut(String).unwrap().await"
  * use try_join!, join! or select! macros to optimise retrieval of multiple values.
  */
 async fn requirements(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, user_settings: &UserSettings, wallet_acc_address: &Arc<SecUtf8>, req: &Vec<&str>) {
          
         let mut map = tasks.write().await;

         for cmd in req {
            let vec: Vec<&str> = cmd.split(" ").collect();
            let length = vec.len();
            let mut into_iter = vec.into_iter();
            let wallet = wallet_acc_address.clone();
            if length == 1 {

                let first: String = into_iter.next().unwrap().to_string();
                match first.as_ref() {
                    "anchor_protocol_whitelist" => {
                         let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_whitelist().await;   
                            }
                        }); 
                        map.insert("anchor_protocol_whitelist".to_string(), MaybeOrPromise::Data(QueryData::Task(handle))); 
                  
                    }
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
                    "anchor_airdrops" => {   
                        let handle = tokio::spawn(async move { 
                            {    
                                return query_anchor_airdrops(wallet.unsecure()).await;   
                            }
                        });
                       
                        map.insert("anchor_airdrops".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));

                    },
                    "borrow_limit" => {   
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_borrower_limit(wallet.unsecure()).await;   
                            }
                        });
                       
                        map.insert("borrow_limit".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
 
                    },
                    "borrow_info" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_borrower_info(wallet.unsecure()).await;   
                            }
                        });
                       
                            map.insert("borrow_info".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                        
                    },
                    "balance" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_balance(wallet.unsecure()).await;   
                            }
                        });
                       
                            map.insert("balance".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                        
                    },
                    "terra_balances" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return terra_balances(wallet.unsecure()).await;   
                            }
                        });
                       
                            map.insert("terra_balances".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                        
                    },
                    "anc_balance" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_anc_balance(wallet.unsecure()).await;   
                            }
                        });
                       
                            map.insert("anc_balance".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    },
                    "staker" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return anchor_protocol_staker(wallet.unsecure()).await;   
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
                    "api/data?type=lpVault" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return query_api_spec_anc_ust_lp_reward().await;   
                            }
                        });
                       
                            map.insert("api/data?type=lpVault".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    },  
                    "api/v2/ust-lp-reward" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return query_api_anc_ust_lp_reward().await;   
                            }
                        });
                       
                            map.insert("api/v2/ust-lp-reward".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
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
                    "anchor_protocol_txs_redeem_stable" => {  
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_block_txs_fee_data("redeem_stable").await;   
                            }
                        });
                            map.insert("anchor_protocol_txs_redeem_stable".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    }, 
                    "anchor_protocol_txs_deposit_stable" => {  
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_block_txs_fee_data("deposit_stable").await;   
                            }
                        });
                            map.insert("anchor_protocol_txs_deposit_stable".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    }, 
                    "anchor_protocol_txs_borrow_stable" => {  
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_block_txs_fee_data("borrow_stable").await;   
                            }
                        });
                            map.insert("anchor_protocol_txs_borrow_stable".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    }, 
                    "anchor_protocol_txs_repay_stable" => {  
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_block_txs_fee_data("repay_stable").await;   
                            }
                        });
                            map.insert("anchor_protocol_txs_repay_stable".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    }, 
                    "anchor_protocol_txs_provide_liquidity" => {  
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_block_txs_fee_data("provide_liquidity").await;   
                            }
                        });
                            map.insert("anchor_protocol_txs_provide_liquidity".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    }, 
                    "txs_provide_to_spec_anc_ust_vault" => {  
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_block_txs_fee_data("provide_to_spec_anc_ust_vault").await;   
                            }
                        });
                            map.insert("txs_provide_to_spec_anc_ust_vault".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    }, 
                    "anchor_protocol_txs_staking_lp" => {  
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_block_txs_fee_data("staking_lp").await;   
                            }
                        });
                            map.insert("anchor_protocol_txs_staking_lp".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    },  
                     "tax_rate" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_tax_rate().await;   
                            }
                        });
                       
                            map.insert("tax_rate".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                    },
                     "tax_caps" => { 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return get_tax_caps().await;   
                            }
                        });
                       
                            map.insert("tax_caps".to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                    },
                    "gas_fees_uusd" => {      
                        let mut gas_prices = get_gas_price(); 
                        match fetch_gas_price().await {
                            Ok(res) => {gas_prices = res},
                            Err(err) => {
                                println!("{}",err.to_string());
                                println!("Info: Failed to query gas price. Fallback to static gas prices.");
                            },
                        }; 
                        map.insert("gas_fees_uusd".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Maybe {data: Ok(gas_prices.uusd.to_string().to_owned()), timestamp: Utc::now().timestamp()})));
                    }, 
                    "trigger_percentage" => {     
                        map.insert("trigger_percentage".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Maybe {data: Ok(user_settings.trigger_percentage.to_string().to_owned()), timestamp: Utc::now().timestamp()})));
                    }, 
                    "borrow_percentage" => {     
                        map.insert("borrow_percentage".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Maybe {data: Ok(user_settings.borrow_percentage.to_string().to_owned()), timestamp: Utc::now().timestamp()})));
                    }, 
                    "target_percentage" => {     
                        map.insert("target_percentage".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Maybe {data: Ok(user_settings.target_percentage.to_string().to_owned()), timestamp: Utc::now().timestamp()})));
                    },      
                    "gas_adjustment_preference" => {      
                        map.insert("gas_adjustment_preference".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Maybe {data: Ok(user_settings.gas_adjustment_preference.to_string().to_owned()), timestamp: Utc::now().timestamp()})));
                    },    
                    "min_ust_balance" => {       
                        map.insert("min_ust_balance".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Maybe {data: Ok(user_settings.min_ust_balance.to_string().to_owned()), timestamp: Utc::now().timestamp()})));
                    },    
                    "ust_balance_preference" => {       
                        map.insert("ust_balance_preference".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Maybe {data: Ok(user_settings.ust_balance_preference.to_string().to_owned()), timestamp: Utc::now().timestamp()})));
                    },   
                    "max_tx_fee" => {       
                        map.insert("max_tx_fee".to_string(),MaybeOrPromise::MetaData(MetaData::Maybe(Maybe {data: Ok(user_settings.max_tx_fee.to_string().to_owned()), timestamp: Utc::now().timestamp()})));
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
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return state_query_msg(second_copy,third_copy).await;   
                            }
                        });
                       
                            map.insert(format!("state {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                    },
                     "epoch_state" => {
                        // "epoch_state anchorprotocol mmMarket"
                        let second_copy = second.to_owned();
                        let third_copy = third.to_owned(); 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return epoch_state_query_msg(second_copy,third_copy).await;   
                            }
                        });
                       
                            map.insert(format!("epoch_state {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                    },
                     "config" => {
                        let second_copy = second.to_owned();
                        let third_copy = third.to_owned(); 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return config_query_msg(second_copy,third_copy).await;   
                            }
                        });
                       
                            map.insert(format!("config {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                    },
                    "simulation_cw20" => {
                        let third_copy = third.to_owned(); 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return masset_to_ust(third_copy).await;   
                            }
                        });
                       
                            map.insert(format!("simulation_cw20 {} {}",second, third).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                         
                    },
                    "core_swap" => { 
                        let second_copy = second.to_owned();
                        let third_copy = third.to_owned(); 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return native_token_core_swap(second_copy,third_copy).await;   
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
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return native_token_to_swap_pair(second_copy,third_copy,fourth_copy).await;   
                            }
                        });
                       
                            map.insert(format!("simulation {} {} {}",second, third, fourth).to_string(), MaybeOrPromise::Data(QueryData::Task(handle)));
                       
                    },
                    "simulation_cw20" => {
                        let second_copy = second.to_owned();
                        let third_copy = third.to_owned();
                        let fourth_copy = fourth.to_owned(); 
                        let handle = tokio::spawn(async move { 
                            {    
                                
                                return cw20_to_swap_pair(second_copy,third_copy,fourth_copy).await;   
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
 