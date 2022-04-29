
// includes functions from action/mod.rs

// one list of only joinhandles/keys
// each task gets access to an ARC/Mutex list to place their results
// this way joinhandles can save the results themself.
// https://aeshirey.github.io/code/2020/12/23/arc-mutex-in-rust.html

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

use std::sync::Arc; 
use tokio::sync::{Mutex};
use tokio::task::JoinSet;
 
 
use std::time::{Duration};
use tokio::time::timeout; 
 

use chrono::{Utc};

use core::pin::Pin;
use core::future::Future;


use terra_rust_bot_essentials::shared::Maybe as MaybeImported;
use terra_rust_bot_essentials::shared::Entry;

pub type Maybe<T> = MaybeImported<T>;



/*
 * returns the value for the given key, if the enum is of the type Maybe.
 * will not await the future if it is not yet resolved.
 * in that case it returns an error.
 */
pub async fn try_get_resolved(maybes: &HashMap<String, Arc<Mutex<Maybe<ResponseResult>>>>,key: &str) -> Maybe<ResponseResult> {

    match maybes.get(key.to_string().as_str()) {
        Some(pointer) => {
            let lock = pointer.lock().await;
            lock.clone()
        },
        None => {
            Maybe{
                data:Err(anyhow!("Error: key does not exist")),
                timestamp:Utc::now().timestamp()
            }
        }
    }

}

pub async fn get_timestamps_of_tasks(maybes: &HashMap<String, Arc<Mutex<Maybe<ResponseResult>>>>) -> Vec<(String, i64)> {

    let mut keys: Vec<(String,i64)> = Vec::new();

    for key in maybes.keys() {
        let pointer = maybes.get(key.to_string().as_str()).unwrap();
        let lock = pointer.lock().await;
        let timestamp: i64 = lock.timestamp;
        keys.push((key.to_owned(),timestamp));
    }
    return keys;
}

pub async fn register_value(maybes: &HashMap<String, Arc<Mutex<Maybe<ResponseResult>>>>, key: String, value: String) {
    let pointer = maybes.get(key.to_string().as_str()).unwrap();
    let result: Maybe<ResponseResult> = Maybe {data: Ok(ResponseResult::Text(value)), timestamp: Utc::now().timestamp()};
    let mut lock = pointer.lock().await;
    *lock = result;
}
   
pub async fn try_register_function(join_set: &mut JoinSet<()>, maybes: &HashMap<String, Arc<Mutex<Maybe<ResponseResult>>>>, key: String, f: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static >>, timeout_duration: u64, block_duration_after_resolve: i64) {

    let timestamp;
    let state;

    match maybes.get(&key) {
        Some(val) => {
            match &*val.lock().await {
                Maybe{data: Err(err),timestamp:t} => {
                    timestamp = *t;
                    if err.to_string() == "Error: Not yet resolved!".to_string() {
                        state = "pending";
                    }else if err.to_string() == "Error: Entry reserved!".to_string(){
                        state = "reserved";
                    }else{
                        state = "failed";
                    }
                },
                Maybe{data: Ok(_),timestamp:t} => {
                    state = "resolved";
                    timestamp = *t;
                }
            }
        },
        None => {
            state = "unknown";
            timestamp = 0i64;
        }
    }
    let now = Utc::now().timestamp();

    if state=="unknown" || state=="reserved" || state=="failed" || (state=="resolved" && now - timestamp >= block_duration_after_resolve) {
        let pointer = maybes.get(key.to_string().as_str()).unwrap().clone();
        join_set.spawn(async move {
                let result = timeout(Duration::from_secs(timeout_duration), f).await.unwrap_or(Maybe{data:Ok("timeout".to_string()),timestamp: Utc::now().timestamp()});
                let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(result.data.unwrap_or("--".to_string()))), timestamp: Utc::now().timestamp() };
                let mut lock = pointer.lock().await;
                *lock = result;
        });
    }
}

pub async fn await_function(maybes: HashMap<String, Arc<Mutex<Maybe<ResponseResult>>>>, key: String) -> Maybe<String> {
   match try_get_resolved(&maybes,&key).await {
        Maybe{data: Ok(succ),timestamp:t} => {
            Maybe{data: Ok(succ.as_text().unwrap().to_string()),timestamp:t}
        },
        Maybe{data: Err(err),timestamp:t} => {
            Maybe{data: Ok(err.to_string()),timestamp:t}
        }
     }
}

pub async fn requirements_next(num_cpus: usize, join_set: &mut JoinSet<()>, maybes: &mut HashMap<String, Arc<Mutex<Maybe<ResponseResult>>>>, user_settings: &UserSettings, wallet_acc_address: &Arc<SecUtf8>) ->  Vec<Entry> {

    let start = Utc::now().timestamp()+1;
    while Utc::now().timestamp()<start && !join_set.is_empty() {
        timeout(Duration::from_millis(100), join_set.join_one()).await.ok();
    }

    let req = my_requirement_list(&user_settings);

    let mut task_list: Vec<(String,String,i64)> = Vec::new();


    let now = Utc::now().timestamp();
    for (k, v) in maybes.iter() {
        let state: &str;
        let time: i64;
        match &*v.lock().await {
            Maybe{data: Err(err),timestamp} => {
                time = *timestamp;
                if err.to_string() == "Error: Not yet resolved!".to_string() {
                    state = "pending";
                }else if err.to_string() == "Error: Entry reserved!".to_string(){
                    state = "reserved";
                }else{
                    state = "failed";
                }
            },
            Maybe{data: Ok(_),timestamp} => {
                time = *timestamp;
                state = "resolved";
            }
        }
        task_list.push((k.to_string(),state.to_string(),time));
    }
    for i in 0..req.len() {
        if task_list.iter().filter(|x| x.0==req[i].0.to_string()).count() == 0 {
            task_list.push((req[i].0.to_string(),"unknown".to_string(),0i64));
        }
    }

    for i in 0..task_list.len() {
        let mut update = false;
        if task_list[i].1 == "reserved".to_string() {
        }else if task_list[i].1 == "unknown".to_string() {
            update = true;
        }else if task_list[i].1 == "failed".to_string(){
            if req.iter().filter(|x| x.0.to_string()==task_list[i].0).count() == 1 {
                update = true;
            }
        }else if task_list[i].1 == "resolved" {
            let period: Vec<i64> = req.iter().filter(|x| x.0.to_string()==task_list[i].0).map(|x| x.1 as i64).collect();
            if period.len() == 1 {
                if (now - task_list[i].2) > period[0] {
                    update = true;
                }
            }
        }
        if update {
            task_list.push((task_list[i].0.to_owned(),"upcoming".to_string(),task_list[i].2));
        }
    }
    task_list.sort_by_key(|k| k.2);

    let mut n = 0;
    if task_list.iter().filter(|x| x.1 == "pending".to_string()).count() < 32 {
        n = num_cpus;
    }
    let req_to_update: Vec<String>  = task_list.iter().filter(|x| x.1 == "upcoming".to_string()).map(|x| x.0.to_string()).take(n).collect();


    let mut entries: Vec<Entry> = Vec::new();
    for x in 0..task_list.len() {
        if task_list[x].1 == "resolved".to_string() {
            let entry = Entry {
                timestamp: now,
                key: task_list[x].0.to_string(),
                prefix: None,
                value: task_list[x].2.to_string(),
                suffix: None,
                group: Some("[Task][History]".to_string()),
            };
            entries.push(entry);
        }
    }
    let entry = Entry {
        timestamp: now,
        key: "failed".to_string(),
        prefix: None,
        value:  task_list.iter().filter(|x| x.1 == "failed".to_string()).count().to_string(),
        suffix: None,
        group: Some("[Task][Count]".to_string()),
    };
    entries.push(entry);
    let entry = Entry {
        timestamp: now,
        key: "pending".to_string(),
        prefix: None,
        value: task_list.iter().filter(|x| x.1 == "pending".to_string()).count().to_string(),
        suffix: None,
        group: Some("[Task][Count]".to_string()),
    };
    entries.push(entry);
    let entry = Entry {
        timestamp: now,
        key: "upcoming".to_string(),
        prefix: None,
        value: task_list.iter().filter(|x| x.1 == "upcoming".to_string()).count().to_string(),
        suffix: None,
        group: Some("[Task][Count]".to_string()),
    };
    entries.push(entry);
    let entry = Entry {
        timestamp: now,
        key: "resolved".to_string(),
        prefix: None,
        value: task_list.iter().filter(|x| x.1 == "resolved".to_string()).count().to_string(),
        suffix: None,
        group: Some("[Task][Count]".to_string()),
    };
    entries.push(entry);
    let entry = Entry {
        timestamp: now,
        key: "all".to_string(),
        prefix: None,
        value: req.len().to_string(),
        suffix: None,
        group: Some("[Task][Count]".to_string()),
    };
    entries.push(entry);
    let entry = Entry {
        timestamp: now,
        key: "failed".to_string(),
        prefix: None,
        value: format!("{:?}", task_list.iter().filter(|x| x.1 == "failed".to_string()).map(|x| x.0.to_string()).collect::<Vec<String>>()),
        suffix: None,
        group: Some("[Task][List]".to_string()),
    };
    entries.push(entry);
    let entry = Entry {
        timestamp: now,
        key: "pending".to_string(),
        prefix: None,
        value: format!("{:?}", task_list.iter().filter(|x| x.1 == "pending".to_string()).map(|x| x.0.to_string()).collect::<Vec<String>>()),
        suffix: None,
        group: Some("[Task][List]".to_string()),
    };
    entries.push(entry);
    let entry = Entry {
        timestamp: now,
        key: "upcoming".to_string(),
        prefix: None,
        value: format!("{:?}", req_to_update),
        suffix: None,
        group: Some("[Task][List]".to_string()),
    };
    entries.push(entry);
    let entry = Entry {
        timestamp: now,
        key: "resolved".to_string(),
        prefix: None,
        value: format!("{:?}", task_list.iter().filter(|x| x.1 == "resolved".to_string()).map(|x| x.0.to_string()).collect::<Vec<String>>()),
        suffix: None,
        group: Some("[Task][List]".to_string()),
    };
    entries.push(entry);
    let entry = Entry {
        timestamp: now,
        key: "all".to_string(),
        prefix: None,
        value: format!("{:?}", req.iter().map(|x| x.0.to_string()).collect::<Vec<String>>()),
        suffix: None,
        group: Some("[Task][List]".to_string()),
    };
    entries.push(entry);

    requirements(join_set,maybes, &user_settings, &wallet_acc_address, req_to_update).await;
    entries
}

/*
 * Preparing entries so that they can be used without the need to mutate the hashmap later on.
 */
pub async fn requirements_setup(maybes: &mut HashMap<String, Arc<Mutex<Maybe<ResponseResult>>>>) {

    let list = vec![
        "anchor_auto_stake",
        "anchor_auto_farm",
        "anchor_auto_repay",
        "anchor_auto_borrow",
        "latest_transaction",
        "anchor_auto_stake_airdrops",
        "anchor_borrow_and_deposit_stable",
        "anchor_redeem_and_repay_stable",
        "anchor_governance_claim_and_farm",
        "anchor_governance_claim_and_stake"];

    for key in list {
        maybes.insert(key.to_string(),Arc::new(Mutex::new(Maybe{data:Err(anyhow::anyhow!("Error: Entry reserved!")),timestamp:Utc::now().timestamp()})));
    }
}

  /*
  * all required queries are triggered here in async fashion
  *
  * retrieve the value when it is needed: "data.get_mut(String).unwrap().await"
  * use try_join!, join! or select! macros to optimise retrieval of multiple values.
  */
 async fn requirements(join_set: &mut JoinSet<()>, maybes: &mut HashMap<String, Arc<Mutex<Maybe<ResponseResult>>>>, user_settings: &UserSettings, wallet_acc_address: &Arc<SecUtf8>, req: Vec<String>) {

         for cmd in req {
            let vec: Vec<&str> = cmd.split(" ").collect();
            let length = vec.len();
            let mut into_iter = vec.into_iter();

            maybes.insert(cmd.to_string(),Arc::new(Mutex::new(Maybe{data:Err(anyhow::anyhow!("Error: Not yet resolved!")),timestamp:Utc::now().timestamp()})));

            let pointer = maybes.get(cmd.as_str()).unwrap().clone();

            let mut f: Option<Pin<Box<dyn Future<Output = anyhow::Result<ResponseResult>> + Send + 'static>>> = None;

            if length == 1 {

                match into_iter.next().unwrap() {
                    "anchor_protocol_whitelist" => {
                        f= Some(Box::pin(anchor_protocol_whitelist()));                  
                    }
                    "earn_apy" => {
                        f= Some(Box::pin(get_block_txs_deposit_stable_apy()));                        
                    }
                    "blocks_per_year" => {
                        f= Some(Box::pin(blocks_per_year_query()));
                    }
                    "anchor_airdrops" => {
                        f= Some(Box::pin(query_anchor_airdrops(wallet_acc_address.unsecure().to_string())));
                    },
                    "borrow_limit" => {
                        f= Some(Box::pin(anchor_protocol_borrower_limit(wallet_acc_address.unsecure().to_string())));
                    },
                    "borrow_info" => {
                        f= Some(Box::pin(anchor_protocol_borrower_info(wallet_acc_address.unsecure().to_string())));
                    },
                    "balance" => {
                        f= Some(Box::pin(anchor_protocol_balance(wallet_acc_address.unsecure().to_string())));
                    },
                    "terra_balances" => {
                        f= Some(Box::pin(terra_balances(wallet_acc_address.unsecure().to_string())));
                    },
                    "anc_balance" => {
                        f= Some(Box::pin(anchor_protocol_anc_balance(wallet_acc_address.unsecure().to_string())));
                    },
                    "staker" => {
                        f= Some(Box::pin(anchor_protocol_staker(wallet_acc_address.unsecure().to_string())));
                    },
                    "api/v2/distribution-apy" => {
                        f= Some(Box::pin(query_api_distribution_apy()));
                    },
                    "api/data?type=lpVault" => {
                        f= Some(Box::pin(query_api_spec_anc_ust_lp_reward()));                         
                    },  
                    "api/v2/ust-lp-reward" => {
                        f= Some(Box::pin(query_api_anc_ust_lp_reward()));                    
                    }, 
                    "api/v2/gov-reward" => {
                        f= Some(Box::pin(query_api_gov_reward()));
                    },
                    "anchor_protocol_txs_claim_rewards" => {
                        f= Some(Box::pin(get_block_txs_fee_data("claim_rewards")));
                    }, 
                    "anchor_protocol_txs_staking" => {
                        f= Some(Box::pin(get_block_txs_fee_data("staking")));
                    }, 
                    "anchor_protocol_txs_redeem_stable" => {
                        f= Some(Box::pin(get_block_txs_fee_data("redeem_stable")));
                    }, 
                    "anchor_protocol_txs_deposit_stable" => {
                        f= Some(Box::pin(get_block_txs_fee_data("deposit_stable")));
                    }, 
                    "anchor_protocol_txs_borrow_stable" => {
                        f= Some(Box::pin(get_block_txs_fee_data("borrow_stable")));
                    }, 
                    "anchor_protocol_txs_repay_stable" => {
                        f= Some(Box::pin(get_block_txs_fee_data("repay_stable")));
                    }, 
                    "anchor_protocol_txs_provide_liquidity" => {
                        f= Some(Box::pin(get_block_txs_fee_data("provide_liquidity")));
                    }, 
                    "txs_provide_to_spec_anc_ust_vault" => {
                        f= Some(Box::pin(get_block_txs_fee_data("provide_to_spec_anc_ust_vault")));
                    }, 
                    "anchor_protocol_txs_staking_lp" => {
                        f= Some(Box::pin(get_block_txs_fee_data("staking_lp")));
                    },  
                     "tax_rate" => {
                         f= Some(Box::pin(get_tax_rate()));
                    },
                     "tax_caps" => {
                         f= Some(Box::pin(get_tax_caps()));
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
                        let result: Maybe<ResponseResult> = Maybe{data:Ok(ResponseResult::Text(gas_prices.uusd.to_string().to_owned())),timestamp:Utc::now().timestamp()};
                        let mut lock = pointer.lock().await;
                        *lock = result;
                        
                    }, 
                    "trigger_percentage" => {
                        let result: Maybe<ResponseResult> = Maybe{data:Ok(ResponseResult::Text(user_settings.trigger_percentage.to_string().to_owned())),timestamp:Utc::now().timestamp()};
                        let mut lock = pointer.lock().await;
                        *lock = result;
                    }, 
                    "borrow_percentage" => {    
                        let result: Maybe<ResponseResult> = Maybe{data:Ok(ResponseResult::Text(user_settings.borrow_percentage.to_string().to_owned())),timestamp:Utc::now().timestamp()};
                        let mut lock = pointer.lock().await;
                        *lock = result;  
                    }, 
                    "target_percentage" => {
                        let result: Maybe<ResponseResult> = Maybe{data:Ok(ResponseResult::Text(user_settings.target_percentage.to_string().to_owned())),timestamp:Utc::now().timestamp()};
                        let mut lock = pointer.lock().await;
                        *lock = result;
                    },      
                    "gas_adjustment_preference" => {
                        let result: Maybe<ResponseResult> = Maybe{data:Ok(ResponseResult::Text(user_settings.gas_adjustment_preference.to_string().to_owned())),timestamp:Utc::now().timestamp()};
                        let mut lock = pointer.lock().await;
                        *lock = result;
                    },    
                    "min_ust_balance" => {
                        let result: Maybe<ResponseResult> = Maybe{data:Ok(ResponseResult::Text(user_settings.min_ust_balance.to_string().to_owned())),timestamp:Utc::now().timestamp()};
                        let mut lock = pointer.lock().await;
                        *lock = result;    
                    },    
                    "ust_balance_preference" => {
                        let result: Maybe<ResponseResult> = Maybe{data:Ok(ResponseResult::Text(user_settings.ust_balance_preference.to_string().to_owned())),timestamp:Utc::now().timestamp()};
                        let mut lock = pointer.lock().await;
                        *lock = result;    
                    },   
                    "max_tx_fee" => {
                        let result: Maybe<ResponseResult> = Maybe{data:Ok(ResponseResult::Text(user_settings.max_tx_fee.to_string().to_owned())),timestamp:Utc::now().timestamp()};
                        let mut lock = pointer.lock().await;
                        *lock = result;  
                    },     
                    &_ => {

                    }
                };
            }else if length == 3 {

                let first = into_iter.next().unwrap();
                let second = into_iter.next().unwrap();
                let third = into_iter.next().unwrap();

                let second_copy = second.to_owned();
                let third_copy = third.to_owned();

                match first.as_ref() { 
                    "state" => { 
                        // "state anchorprotocol bLunaHub"
                        // "state anchorprotocol mmMarket"
                        f= Some(Box::pin(state_query_msg(second_copy,third_copy)));
                    },
                     "epoch_state" => {
                        // "epoch_state anchorprotocol mmMarket"
                        f= Some(Box::pin(epoch_state_query_msg(second_copy,third_copy)));
                    },
                     "config" => {
                        f= Some(Box::pin(config_query_msg(second_copy,third_copy)));
                    },
                    "simulation_cw20" => {
                        f= Some(Box::pin(masset_to_ust(third_copy)));
                    },
                    "core_swap" => {
                        f= Some(Box::pin(native_token_core_swap(second_copy,third_copy)));
                    },
                    &_ => {

                    }
                }
            }else if length == 4 {

                let first = into_iter.next().unwrap();
                let second = into_iter.next().unwrap();
                let third = into_iter.next().unwrap();
                let fourth = into_iter.next().unwrap();


                let second_copy = second.to_owned();
                let third_copy = third.to_owned();
                let fourth_copy = fourth.to_owned();

                match first.as_ref() { 
                    // luna_to_bluna: simulation uluna anchorprotocol terraswapblunaLunaPair
                    // luna_to_ust: simulation uluna terraswap uusd_uluna_pair_contract
                    // sdt_to_uluna: simulation usdr terraswap usdr_uluna_pair_contract
                    // ust_to_luna: simulation uusd terraswap uusd_uluna_pair_contract
                    // ust_to_psi: simulation uusd nexusprotocol Psi-UST pair
                    // ust_to_anc: simulation uusd anchorprotocol terraswapAncUstPair
                    "simulation" => {
                        f= Some(Box::pin(native_token_to_swap_pair(second_copy,third_copy,fourth_copy)));
                    },
                    "simulation_cw20" => {
                        f= Some(Box::pin(cw20_to_swap_pair(second_copy,third_copy,fourth_copy)));
                    }
                    &_ => {

                    }
                }
            }

             if let Some(m) = f {
                 join_set.spawn(async move {
                     {
                         let result = m.await;
                         let result: Maybe<ResponseResult> = Maybe { data: result, timestamp: Utc::now().timestamp() };
                         let mut lock = pointer.lock().await;
                         *lock = result;
                     }
                 });
             }
        
        }        
}
 