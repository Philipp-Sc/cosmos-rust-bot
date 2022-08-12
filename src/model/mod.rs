// includes functions from action/mod.rs

// one list of only joinhandles/keys
// each task gets access to an ARC/Mutex list to place their results
// this way joinhandles can save the results themself.
// https://aeshirey.github.io/code/2020/12/23/arc-mutex-in-rust.html

pub mod requirements;

use requirements::{UserSettings, Feature, TaskSpec, get_requirements, TaskType};
use secstr::*;

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
use std::iter;

use cosmos_rust_interface::utils::postproc::{EntryValue, Maybe as MaybeImported};
use cosmos_rust_interface::utils::postproc::Entry;


use cosmos_rust_interface::ResponseResult;
use cosmos_rust_interface::blockchain::cosmos::gov::get_proposals;
use cosmos_rust_package::api::core::cosmos::channels::SupportedBlockchain;
use cosmos_rust_package::api::custom::query::gov::ProposalStatus;

pub type Maybe<T> = MaybeImported<T>;


pub async fn add_item(pointer: &Arc<Mutex<Vec<Maybe<ResponseResult>>>>, item: Maybe<ResponseResult>) {
    let mut lock = pointer.lock().await;
    lock.insert(0, item);
    if lock.len() > 4 {
        lock.drain(4..);
    }
}

pub async fn get_item(pointer: &Arc<Mutex<Vec<Maybe<ResponseResult>>>>) -> Maybe<ResponseResult> {
    let lock = pointer.lock().await;
    let mut result: Option<Maybe<ResponseResult>> = None;
    for i in 0..lock.len() {
        match &lock[i] {
            Maybe { data: Err(err), .. } => {
                if err.to_string() != "Error: Not yet resolved!".to_string() {
                    result = Some(lock[i].clone());
                    break;
                }
            }
            Maybe { data: Ok(_), .. } => {
                result = Some(lock[i].clone());
                break;
            }
        };
    }
    if let Some(res) = result {
        return res;
    } else {
        return lock[0].clone();
    }
}

/*
 * returns the value for the given key, if the enum is of the type Maybe.
 * will not await the future if it is not yet resolved.
 * in that case it returns an error.
 */
pub async fn access_maybe(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: &str) -> Maybe<ResponseResult> {
    match maybes.get(key.to_string().as_str()) {
        Some(pointer) => {
            get_item(pointer).await
        }
        None => {
            Maybe {
                data: Err(anyhow!("Error: key does not exist")),
                timestamp: Utc::now().timestamp(),
            }
        }
    }
}

pub async fn access_maybes(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>) -> HashMap<String, Maybe<ResponseResult>> {
    let mut only_maybes: HashMap<String, Maybe<ResponseResult>> = HashMap::new();
    for (key, val) in maybes.iter() {
        let value = match maybes.get(key.as_str()) {
            Some(pointer) => {
                get_item(pointer).await
            }
            None => {
                Maybe {
                    data: Err(anyhow!("Error: key does not exist")),
                    timestamp: Utc::now().timestamp(),
                }
            }
        };
        only_maybes.insert(key.to_string(), value);
    }
    only_maybes
}

pub async fn get_timestamps_of_tasks(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>) -> Vec<(String, i64)> {
    let mut keys: Vec<(String, i64)> = Vec::new();

    for key in maybes.keys() {
        let pointer = maybes.get(key.to_string().as_str()).unwrap();
        let timestamp: i64 = get_item(pointer).await.timestamp;
        keys.push((key.to_owned(), timestamp));
    }
    return keys;
}

pub async fn register_value(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: String, value: String) {
    let pointer = maybes.get(key.to_string().as_str()).unwrap();
    let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(value)), timestamp: Utc::now().timestamp() };
    add_item(pointer, result).await;
}

pub async fn try_register_function(join_set: &mut JoinSet<()>, maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: String, f: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>, timeout_duration: u64, block_duration_after_resolve: i64) {
    let timestamp;
    let state;

    match maybes.get(&key) {
        Some(val) => {
            match val.lock().await[0].clone() {
                Maybe { data: Err(err), timestamp: t } => {
                    timestamp = t;
                    if err.to_string() == "Error: Not yet resolved!".to_string() {
                        state = "pending";
                    } else if err.to_string() == "Error: Entry reserved!".to_string() {
                        state = "reserved";
                    } else {
                        state = "failed";
                    }
                }
                Maybe { data: Ok(_), timestamp: t } => {
                    state = "resolved";
                    timestamp = t;
                }
            }
        }
        None => {
            state = "unknown";
            timestamp = 0i64;
        }
    }
    let now = Utc::now().timestamp();

    if state == "unknown" || state == "reserved" || state == "failed" || (state == "resolved" && now - timestamp >= block_duration_after_resolve) {
        let pointer = maybes.get(key.to_string().as_str()).unwrap().clone();
        join_set.spawn(async move {
            let result = timeout(Duration::from_secs(timeout_duration), f).await.unwrap_or(Maybe { data: Ok("timeout".to_string()), timestamp: Utc::now().timestamp() });
            let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(result.data.unwrap_or("--".to_string()))), timestamp: Utc::now().timestamp() };
            add_item(&pointer, result).await;
        });
    }
}

pub async fn await_function(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: String) -> Maybe<String> {
    match access_maybe(&maybes, &key).await {
        Maybe { data: Ok(succ), timestamp: t } => {
            Maybe { data: Ok(succ.as_text().unwrap().to_string()), timestamp: t }
        }
        Maybe { data: Err(err), timestamp: t } => {
            Maybe { data: Ok(err.to_string()), timestamp: t }
        }
    }
}

pub fn task_meta_data(task_list: Vec<(String, String, i64)>) -> Vec<Entry> {
    let now = Utc::now().timestamp();
    let mut entries: Vec<Entry> = Vec::new();

    let status_list = vec!["failed".to_string(), "pending".to_string(), "upcoming".to_string(), "resolved".to_string()];
    let data_list: Vec<(String, String, String)> =
        task_list.iter().filter(|x| x.1 == "resolved".to_string()).map(|x|
            (
                "[Task][History]".to_string(), x.0.to_string(), x.2.to_string()))
            .chain(status_list.iter().map(|y| {
                (
                    "[Task][Count]".to_string(), y.to_owned(), task_list.iter().filter(|x| x.1 == y.to_string()).count().to_string())
            }))
            .chain(iter::once(
                (
                    "[Task][Count]".to_string(), "all".to_string(), task_list.len().to_string()))
            )
            .chain(status_list.iter().map(|y| {
                (
                    "[Task][List]".to_string(), y.to_owned(), format!("{:?}", task_list.iter().filter(|x| x.1 == y.to_string()).map(|x| x.0.to_string()).collect::<Vec<String>>()).to_string())
            }))
            .chain(iter::once(
                (
                    "[Task][List]".to_string(), "all".to_string(), format!("{:?}", task_list.iter().map(|x| x.0.to_string()).collect::<Vec<String>>())))
            )
            .collect();

    for i in 0..data_list.len() {
        let entry = Entry {
            timestamp: now,
            key: data_list[i].1.to_owned(),
            value: EntryValue::Json(serde_json::json!({
                        "data": data_list[i].2.to_owned(),
                        "group": Some(data_list[i].0.to_owned()),
                        "index": Some(i as i32),
                    }).to_string()),
        };
        entries.push(entry);
    }
    entries
}

pub async fn next_iteration_of_upcoming_tasks(join_set: &mut JoinSet<()>, maybes: &mut HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, user_settings: &UserSettings, wallet_acc_address: &Arc<SecUtf8>) -> Vec<Entry> {
    for _ in 0..join_set.len() {
        let result = timeout(Duration::from_millis(0), join_set.join_one()).await;
        match result {
            Ok(_) => {}
            Err(_) => { break; }
        };
    }

    let req = get_requirements(&user_settings);

    let mut task_list: Vec<(String, String, i64)> = Vec::new();

    let now = Utc::now().timestamp();
    for (k, v) in maybes.iter() {
        let state: &str;
        let time: i64;
        match v.lock().await[0].clone() {
            Maybe { data: Err(err), timestamp } => {
                time = timestamp;
                if err.to_string() == "Error: Not yet resolved!".to_string() {
                    state = "pending";
                } else if err.to_string() == "Error: Entry reserved!".to_string() {
                    state = "reserved";
                } else {
                    state = "failed";
                }
            }
            Maybe { data: Ok(_), timestamp } => {
                time = timestamp;
                state = "resolved";
            }
        }
        task_list.push((k.to_string(), state.to_string(), time));
    }
    for i in 0..req.len() {
        if task_list.iter().filter(|x| x.0 == req[i].name).count() == 0 {
            task_list.push(((&req[i].name).to_string(), "unknown".to_string(), 0i64));
        }
    }

    for i in 0..task_list.len() {
        let mut update = false;
        if task_list[i].1 == "reserved".to_string() {} else if task_list[i].1 == "unknown".to_string() {
            update = true;
        } else if task_list[i].1 == "failed".to_string() {
            if req.iter().filter(|x| x.name == task_list[i].0).count() == 1 {
                update = true;
            }
        } else if task_list[i].1 == "resolved" {
            let period: Vec<i64> = req.iter().filter(|x| x.name == task_list[i].0).map(|x| x.refresh_rate as i64).collect();
            if period.len() == 1 {
                if (now - task_list[i].2) > period[0] {
                    update = true;
                }
            }
        }
        if update {
            task_list.push((task_list[i].0.to_owned(), "upcoming".to_string(), task_list[i].2));
        }
    }
    task_list.sort_by_key(|k| k.2);

    let upcoming_task_name_list: Vec<String> = task_list.iter().filter(|x| x.1 == "upcoming".to_string()).map(|x| x.0.to_string()).collect();
    let upcoming_task_spec_list: Vec<TaskSpec> = req.into_iter().filter(|x| upcoming_task_name_list.contains(&x.name)).collect();

    spawn_tasks(join_set, maybes, &user_settings, &wallet_acc_address, upcoming_task_spec_list).await;
    task_meta_data(task_list)
}

/*
 * Preparing entries so that they can be used without the need to mutate the hashmap later on.
 */
pub async fn setup_required_keys(maybes: &mut HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>) {
    let list: Vec<&str> = vec![];/*
        "anchor_auto_stake",
        "anchor_auto_farm",
        "anchor_auto_repay",
        "anchor_auto_borrow",
        "latest_transaction",
        "anchor_auto_stake_airdrops",
        "anchor_borrow_and_deposit_stable",
        "anchor_redeem_and_repay_stable",
        "anchor_governance_claim_and_farm",
        "anchor_governance_claim_and_stake"*/

    for key in list {
        maybes.insert(key.to_string(), Arc::new(Mutex::new(vec![Maybe { data: Err(anyhow::anyhow!("Error: Entry reserved!")), timestamp: Utc::now().timestamp() }])));
    }
}

/*
* all required queries are triggered here in async fashion
*
* retrieve the value when it is needed: "data.get_mut(String).unwrap().await"
* use try_join!, join! or select! macros to optimise retrieval of multiple values.
*/
async fn spawn_tasks(join_set: &mut JoinSet<()>, maybes: &mut HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, user_settings: &UserSettings, wallet_acc_address: &Arc<SecUtf8>, to_update: Vec<TaskSpec>) {
    for req in to_update {
        let contains_key = maybes.contains_key(&req.name);
        if !contains_key {
            maybes.insert(req.name.clone(), Arc::new(Mutex::new(vec![Maybe { data: Err(anyhow::anyhow!("Error: Not yet resolved!")), timestamp: Utc::now().timestamp() }])));
        }
        let pointer = maybes.get(&req.name).unwrap().clone();

        if contains_key {
            add_item(&pointer, Maybe { data: Err(anyhow::anyhow!("Error: Not yet resolved!")), timestamp: Utc::now().timestamp() }).await;
        }

        let mut f: Option<Pin<Box<dyn Future<Output=anyhow::Result<ResponseResult>> + Send + 'static>>> = None;

        let wallet_acc_address = wallet_acc_address.clone();

        match req.kind {
            TaskType::GovernanceProposals => {
                let status = ProposalStatus::new(req.args["proposal_status"].as_str().unwrap());
                let blockchain = SupportedBlockchain::new(req.args["blockchain"].as_str().unwrap());
                f = Some(Box::pin(get_proposals(blockchain, status)));
            }
            _ => {}
        }

        /*

        if length == 1 {
            match vec[0] {
                "governance_proposals_terra" => {
                    f = Some(Box::pin(get_proposals("terra")));
                }
                "governance_proposals_osmosis" => {
                    f = Some(Box::pin(get_proposals("osmosis")));
                }
                /*"terra_money_assets_cw20_tokens" => {
                    f = Some(Box::pin(query_terra_money_assets_cw20_tokens()));
                }*/
                "anchor_protocol_whitelist" => {
                    f = Some(Box::pin(anchor_protocol_whitelist(asset_whitelist)));
                }
                "earn_apy" => {
                    f = Some(Box::pin(get_block_txs_deposit_stable_apy()));
                }
                "blocks_per_year" => {
                    f = Some(Box::pin(blocks_per_year_query()));
                }
                "anchor_airdrops" => {
                    f = Some(Box::pin(query_anchor_airdrops(asset_whitelist, wallet_acc_address)));
                }
                "borrow_limit" => {
                    f = Some(Box::pin(anchor_protocol_borrower_limit(asset_whitelist, wallet_acc_address)));
                }
                "borrow_info" => {
                    f = Some(Box::pin(anchor_protocol_borrower_info(asset_whitelist, wallet_acc_address)));
                }
                "balance" => {
                    f = Some(Box::pin(anchor_protocol_balance(asset_whitelist, wallet_acc_address)));
                }
                "terra_balances" => {
                    f = Some(Box::pin(terra_balances(wallet_acc_address)));
                }
                "anc_balance" => {
                    f = Some(Box::pin(anchor_protocol_anc_balance(asset_whitelist, wallet_acc_address)));
                }
                "staker" => {
                    f = Some(Box::pin(anchor_protocol_staker(asset_whitelist, wallet_acc_address)));
                }
                "api/v2/distribution-apy" => {
                    f = Some(Box::pin(query_api_distribution_apy()));
                }
                "api/data?type=lpVault" => {
                    f = Some(Box::pin(query_api_spec_anc_ust_lp_reward()));
                }
                "api/v2/ust-lp-reward" => {
                    f = Some(Box::pin(query_api_anc_ust_lp_reward()));
                }
                "api/v2/gov-reward" => {
                    f = Some(Box::pin(query_api_gov_reward()));
                }
                "anchor_protocol_txs_claim_rewards" => {
                    f = Some(Box::pin(get_block_txs_fee_data("claim_rewards", asset_whitelist)));
                }
                "anchor_protocol_txs_staking" => {
                    f = Some(Box::pin(get_block_txs_fee_data("staking", asset_whitelist)));
                }
                "anchor_protocol_txs_redeem_stable" => {
                    f = Some(Box::pin(get_block_txs_fee_data("redeem_stable", asset_whitelist)));
                }
                "anchor_protocol_txs_deposit_stable" => {
                    f = Some(Box::pin(get_block_txs_fee_data("deposit_stable", asset_whitelist)));
                }
                "anchor_protocol_txs_borrow_stable" => {
                    f = Some(Box::pin(get_block_txs_fee_data("borrow_stable", asset_whitelist)));
                }
                "anchor_protocol_txs_repay_stable" => {
                    f = Some(Box::pin(get_block_txs_fee_data("repay_stable", asset_whitelist)));
                }
                "anchor_protocol_txs_provide_liquidity" => {
                    f = Some(Box::pin(get_block_txs_fee_data("provide_liquidity", asset_whitelist)));
                }
                "txs_provide_to_spec_anc_ust_vault" => {
                    f = Some(Box::pin(get_block_txs_fee_data("provide_to_spec_anc_ust_vault", asset_whitelist)));
                }
                "anchor_protocol_txs_staking_lp" => {
                    f = Some(Box::pin(get_block_txs_fee_data("staking_lp", asset_whitelist)));
                }
                "tax_rate" => {
                    f = Some(Box::pin(get_tax_rate()));
                }
                "tax_caps" => {
                    f = Some(Box::pin(get_tax_caps()));
                }
                "gas_fees_uusd" => {
                    let mut gas_prices = get_gas_price();
                    match fetch_gas_price().await {
                        Ok(res) => { gas_prices = res }
                        Err(err) => {
                            println!("{}", err.to_string());
                            println!("Info: Failed to query gas price. Fallback to static gas prices.");
                        }
                    };
                    let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(gas_prices.uusd.to_string().to_owned())), timestamp: Utc::now().timestamp() };
                    add_item(&pointer, result).await;
                }
                "trigger_percentage" => {
                    let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(user_settings.trigger_percentage.to_string().to_owned())), timestamp: Utc::now().timestamp() };
                    add_item(&pointer, result).await;
                }
                "borrow_percentage" => {
                    let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(user_settings.borrow_percentage.to_string().to_owned())), timestamp: Utc::now().timestamp() };
                    add_item(&pointer, result).await;
                }
                "target_percentage" => {
                    let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(user_settings.target_percentage.to_string().to_owned())), timestamp: Utc::now().timestamp() };
                    add_item(&pointer, result).await;
                }
                "gas_adjustment_preference" => {
                    let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(user_settings.gas_adjustment_preference.to_string().to_owned())), timestamp: Utc::now().timestamp() };
                    add_item(&pointer, result).await;
                }
                "min_ust_balance" => {
                    let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(user_settings.min_ust_balance.to_string().to_owned())), timestamp: Utc::now().timestamp() };
                    add_item(&pointer, result).await;
                }
                "ust_balance_preference" => {
                    let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(user_settings.ust_balance_preference.to_string().to_owned())), timestamp: Utc::now().timestamp() };
                    add_item(&pointer, result).await;
                }
                "max_tx_fee" => {
                    let result: Maybe<ResponseResult> = Maybe { data: Ok(ResponseResult::Text(user_settings.max_tx_fee.to_string().to_owned())), timestamp: Utc::now().timestamp() };
                    add_item(&pointer, result).await;
                }
                &_ => {}
            };
        } else if length == 3 {
            match vec[0] {
                "state" => {
                    f = Some(Box::pin(state_query_msg(asset_whitelist, vec[1].to_owned(), vec[2].to_owned())));
                }
                "epoch_state" => {
                    f = Some(Box::pin(epoch_state_query_msg(asset_whitelist, vec[1].to_owned(), vec[2].to_owned())));
                }
                "config" => {
                    f = Some(Box::pin(config_query_msg(asset_whitelist, vec[1].to_owned(), vec[2].to_owned())));
                }
                "core_swap" => {
                    f = Some(Box::pin(native_token_core_swap(vec[1].to_owned(), vec[2].to_owned())));
                }
                &_ => {}
            }
        } else if length == 6 {
            match vec[0] {
                "simulate_swap" => {
                    f = Some(Box::pin(simulate_swap(asset_whitelist,
                                                    vec[1].to_owned(),
                                                    match vec[2] {
                                                        "none" => { None }
                                                        protocol => { Some(protocol.to_string()) }
                                                    },
                                                    vec[3].to_owned(),
                                                    match vec[4] {
                                                        "none" => { None }
                                                        protocol => { Some(protocol.to_string()) }
                                                    },
                                                    vec[5].to_owned())));
                }
                &_ => {}
            }
        }

        */

        if let Some(m) = f {
            join_set.spawn(async move {
                {
                    let result = m.await;
                    let result: Maybe<ResponseResult> = Maybe { data: result, timestamp: Utc::now().timestamp() };
                    add_item(&pointer, result).await;
                }
            });
        }
    }
}
