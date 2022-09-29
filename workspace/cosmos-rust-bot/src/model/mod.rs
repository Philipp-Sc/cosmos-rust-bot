// includes functions from action/mod.rs

// one list of only joinhandles/keys
// each task gets access to an ARC/Mutex list to place their results
// this way joinhandles can save the results themself.
// https://aeshirey.github.io/code/2020/12/23/arc-mutex-in-rust.html

pub mod requirements;

use requirements::{get_requirements, Feature, TaskSpec, TaskType, UserSettings};
use secstr::*;

use std::collections::HashMap;

use anyhow::anyhow;

use heck::ToTitleCase;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

use std::time::Duration;
use tokio::time::timeout;

use chrono::{TimeZone, Utc};

use core::future::Future;
use core::pin::Pin;
use std::iter;

use cosmos_rust_interface::utils::entry::*;

use cosmos_rust_interface::blockchain::cosmos::gov::get_proposals;
use cosmos_rust_interface::utils::response::ResponseResult;
use cosmos_rust_package::api::core::cosmos::channels;
use cosmos_rust_package::api::core::cosmos::channels::SupportedBlockchain;
use cosmos_rust_package::api::custom::query::gov::ProposalStatus;
use serde_json::json;
use std::string::ToString;
use strum::IntoEnumIterator;
use strum_macros;
use strum_macros::EnumIter;
use cosmos_rust_interface::blockchain::cosmos::chain_registry::get_supported_blockchains_from_chain_registry;

#[derive(strum_macros::ToString, Debug, EnumIter, PartialEq)]
pub enum TaskState {
    Unknown,
    Pending,
    Reserved,
    Failed,
    Resolved,
    Upcoming,
}

pub struct TaskItem {
    pub name: String,
    pub state: TaskState,
    pub timestamp: i64,
}

pub async fn add_item(
    pointer: &Arc<Mutex<Vec<Maybe<ResponseResult>>>>,
    item: Maybe<ResponseResult>,
) {
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
pub async fn access_maybe(
    maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
    key: &str,
) -> Maybe<ResponseResult> {
    match maybes.get(key.to_string().as_str()) {
        Some(pointer) => get_item(pointer).await,
        None => Maybe {
            data: Err(anyhow!("Error: key does not exist")),
            timestamp: Utc::now().timestamp(),
        },
    }
}

pub async fn access_maybes(
    maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
) -> HashMap<String, Maybe<ResponseResult>> {
    let mut only_maybes: HashMap<String, Maybe<ResponseResult>> = HashMap::new();
    for (key, val) in maybes.iter() {
        let value = match maybes.get(key.as_str()) {
            Some(pointer) => get_item(pointer).await,
            None => Maybe {
                data: Err(anyhow!("Error: key does not exist")),
                timestamp: Utc::now().timestamp(),
            },
        };
        only_maybes.insert(key.to_string(), value);
    }
    only_maybes
}

pub async fn get_timestamps_of_tasks(
    maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
) -> Vec<(String, i64)> {
    let mut keys: Vec<(String, i64)> = Vec::new();

    for key in maybes.keys() {
        let pointer = maybes.get(key.to_string().as_str()).unwrap();
        let timestamp: i64 = get_item(pointer).await.timestamp;
        keys.push((key.to_owned(), timestamp));
    }
    return keys;
}

pub async fn register_value(
    maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
    key: String,
    value: String,
) {
    let pointer = maybes.get(key.to_string().as_str()).unwrap();
    let result: Maybe<ResponseResult> = Maybe {
        data: Ok(ResponseResult::Text(value)),
        timestamp: Utc::now().timestamp(),
    };
    add_item(pointer, result).await;
}

pub async fn try_register_function(
    join_set: &mut JoinSet<()>,
    maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
    key: String,
    f: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>,
    timeout_duration: u64,
    block_duration_after_resolve: i64,
) {
    let timestamp;
    let state: TaskState;

    match maybes.get(&key) {
        Some(val) => match val.lock().await[0].clone() {
            Maybe {
                data: Err(err),
                timestamp: t,
            } => {
                timestamp = t;
                if err.to_string() == "Error: Not yet resolved!".to_string() {
                    state = TaskState::Pending;
                } else if err.to_string() == "Error: Entry reserved!".to_string() {
                    state = TaskState::Reserved;
                } else {
                    state = TaskState::Failed;
                }
            }
            Maybe {
                data: Ok(_),
                timestamp: t,
            } => {
                state = TaskState::Resolved;
                timestamp = t;
            }
        },
        None => {
            state = TaskState::Unknown;
            timestamp = 0i64;
        }
    }
    let now = Utc::now().timestamp();

    let spawn = match state {
        TaskState::Unknown | TaskState::Reserved | TaskState::Failed => true,
        TaskState::Resolved => {
            if now - timestamp >= block_duration_after_resolve {
                true
            } else {
                false
            }
        }
        _ => false,
    };
    if spawn {
        let pointer = maybes.get(key.to_string().as_str()).unwrap().clone();
        join_set.spawn(async move {
            let result = timeout(Duration::from_secs(timeout_duration), f)
                .await
                .unwrap_or(Maybe {
                    data: Ok("timeout".to_string()),
                    timestamp: Utc::now().timestamp(),
                });
            let result: Maybe<ResponseResult> = Maybe {
                data: Ok(ResponseResult::Text(
                    result.data.unwrap_or("--".to_string()),
                )),
                timestamp: Utc::now().timestamp(),
            };
            add_item(&pointer, result).await;
        });
    }
}

pub async fn await_function(
    maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
    key: String,
) -> Maybe<String> {
    match access_maybe(&maybes, &key).await {
        Maybe {
            data: Ok(succ),
            timestamp: t,
        } => Maybe {
            data: Ok(succ.as_text().unwrap().to_string()),
            timestamp: t,
        },
        Maybe {
            data: Err(err),
            timestamp: t,
        } => Maybe {
            data: Ok(err.to_string()),
            timestamp: t,
        },
    }
}

pub fn task_meta_data(task_list: Vec<TaskItem>) -> Vec<CosmosRustBotValue> {
    let now = Utc::now().timestamp();

    let mut task_meta_data = task_list.iter().map(|x|
        ( "task_history".to_string(), x.state.to_string(), json!({"value": x.name, "timestamp": x.timestamp})))
        .chain(TaskState::iter().map(|y| {
            ("task_count".to_string(), y.to_string(), json!({"value": task_list.iter().filter(|x| x.state == y).count().to_string()}))
        }))
        .chain(iter::once(
            ("task_count".to_string(), "all".to_string(), json!({"value": task_list.len().to_string()})))
        )
        .chain(TaskState::iter().map(|y| {
            ("task_list".to_string(), y.to_string(), json!({"value": format!("{:?}", task_list.iter().filter(|x| x.state == y).map(|x| x.name.to_string()).collect::<Vec<String>>()).to_string()}))
        }))
        .chain(iter::once(
            ("task_list".to_string(), "all".to_string(), json!({"value": format!("{:?}", task_list.iter().map(|x| x.name.to_string()).collect::<Vec<String>>())})))
        ).enumerate().map(|(i, v)| {
        let state = v.1.to_title_case();
        let value = v.2.get("value").unwrap().as_str().unwrap();

        let info = match v.2.get("timestamp") {
            Some(timestamp) => {
                format!("[{}] - {} - {}", Utc.timestamp(timestamp.as_i64().unwrap(), 0), state, value.to_string().to_title_case())
            }
            None => {
                format!("{} Tasks: {}", state, value)
            }
        };

        CosmosRustBotValue::Entry(Entry::MetaData(MetaData {
            index: i as i32,
            timestamp: now,
            origin: "task_meta_data".to_string(),
            kind: v.0.to_owned(),
            state: state.to_owned(),
            value: value.to_owned(),
            summary: info.to_owned(),
        }))
    }).collect::<Vec<CosmosRustBotValue>>();

    CosmosRustBotValue::add_index(&mut task_meta_data, "index", "index");
    CosmosRustBotValue::add_membership(&mut task_meta_data, None, "task_meta_data");
    CosmosRustBotValue::add_variants_of_memberships(&mut task_meta_data, vec!["kind", "state"]);
    task_meta_data
}

pub async fn poll_resolved_tasks(join_set: &mut JoinSet<()>) -> usize {
    let mut counter: usize = 0;
    // The following removes all completed tasks from the set.
    // Unresolved tasks are unaffected.
    for _ in 0..join_set.len() {
        let result = timeout(Duration::from_millis(0), join_set.join_next()).await;
        // join_set.join_next()
        // If this returns `Poll::Ready(Some(_))`, then the task that completed is removed from the set.
        match result {
            Ok(_) => {
                counter+=1;
            }
            Err(_) => {
                // timeout returned an error
                // currently all tasks pending
                break;
            }
        };
    }
    counter
}


pub async fn try_spawn_upcoming_tasks(
    join_set: &mut JoinSet<()>,
    maybes: &mut HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
    req: &Vec<TaskSpec>,
    user_settings: &UserSettings,
    wallet_acc_address: &Arc<SecUtf8>,
) -> usize {

    let task_list: Vec<TaskItem> = get_task_list(maybes,req).await;

    let upcoming_task_spec_list: Vec<&TaskSpec> = req
        .iter()
        .filter(|&x| task_list
            .iter()
            .filter(|y| y.state == TaskState::Upcoming)
            .map(|y| &y.name)
            .collect::<Vec<&String>>().contains(&&x.name))
        .collect();

    let number_of_tasks_added =
        spawn_tasks(
            join_set,
            maybes,
            &user_settings,
            &wallet_acc_address,
            upcoming_task_spec_list,
        )
        .await;
    number_of_tasks_added
}

pub async fn get_task_meta_data(maybes: &mut HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
                                req: &Vec<TaskSpec>
                                ) -> Vec<CosmosRustBotValue>{

    let task_list = get_task_list(maybes,req).await;
    task_meta_data(task_list)
}

pub async fn get_task_list(
    maybes: &mut HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
    req: &Vec<TaskSpec>,
) -> Vec<TaskItem> {

    let mut task_list: Vec<TaskItem> = Vec::new();

    let now = Utc::now().timestamp();
    for (k, v) in maybes.iter() {
        let state: TaskState;
        let time: i64;
        match v.lock().await[0].clone() {
            Maybe {
                data: Err(err),
                timestamp,
            } => {
                time = timestamp;
                if err.to_string() == "Error: Not yet resolved!".to_string() {
                    state = TaskState::Pending;
                } else if err.to_string() == "Error: Entry reserved!".to_string() {
                    state = TaskState::Reserved;
                } else {
                    state = TaskState::Failed;
                }
            }
            Maybe {
                data: Ok(_),
                timestamp,
            } => {
                time = timestamp;
                state = TaskState::Resolved;
            }
        }
        task_list.push(TaskItem {
            name: k.to_string(),
            state,
            timestamp: time,
        });
    }
    for i in 0..req.len() {
        if task_list.iter().filter(|x| x.name == req[i].name).count() == 0 {
            task_list.push(TaskItem {
                name: (&req[i].name).to_string(),
                state: TaskState::Unknown,
                timestamp: 0i64,
            });
        }
    }

    for i in 0..task_list.len() {
        let mut update = false;
        if task_list[i].state == TaskState::Reserved {
        } else if task_list[i].state == TaskState::Unknown {
            if req.iter().filter(|x| x.name == task_list[i].name).count() == 1 {
                update = true;
            }
        } else if task_list[i].state == TaskState::Failed {
            update = true;
        } else if task_list[i].state == TaskState::Resolved {
            let period: Vec<i64> = req
                .iter()
                .filter(|x| x.name == task_list[i].name)
                .map(|x| x.refresh_rate as i64)
                .collect();
            if period.len() == 1 {
                if (now - task_list[i].timestamp) > period[0] {
                    update = true;
                }
            }
        }
        if update {
            task_list.push(TaskItem {
                name: task_list[i].name.to_owned(),
                state: TaskState::Upcoming,
                timestamp: task_list[i].timestamp,
            });
        }
    }
    task_list.sort_by_key(|k| k.timestamp);
    task_list
}

/*
 * Preparing entries so that they can be used without the need to mutate the hashmap later on.
 */
/*
pub async fn setup_required_keys(
    maybes: &mut HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
) {
    let list: Vec<&str> = vec![]; /*
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
        maybes.insert(
            key.to_string(),
            Arc::new(Mutex::new(vec![Maybe {
                data: Err(anyhow::anyhow!("Error: Entry reserved!")),
                timestamp: Utc::now().timestamp(),
            }])),
        );
    }
}*/

/*
* all required queries are triggered here in async fashion
*
* retrieve the value when it is needed: "data.get_mut(String).unwrap().await"
* use try_join!, join! or select! macros to optimise retrieval of multiple values.
*/
async fn spawn_tasks(
    join_set: &mut JoinSet<()>,
    maybes: &mut HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,
    _user_settings: &UserSettings,
    wallet_acc_address: &Arc<SecUtf8>,
    to_update: Vec<&TaskSpec>,
) -> usize {

    let supported_blockchains = match access_maybe(maybes,"chain_registry").await {
        Maybe{data: Ok(ResponseResult::ChainRegistry(chain_registry)), timestamp: t } => {
            Some(chain_registry)
        },
        Maybe{ .. } => {
            None
        },
    };

    let mut count: usize = 0;
    for req in to_update {
        if supported_blockchains.is_some() || req.kind == TaskType::ChainRegistry {
            let contains_key = maybes.contains_key(&req.name);
            if !contains_key {
                maybes.insert(
                    req.name.clone(),
                    Arc::new(Mutex::new(vec![Maybe {
                        data: Err(anyhow::anyhow!("Error: Not yet resolved!")),
                        timestamp: Utc::now().timestamp(),
                    }])),
                );
            }
            let pointer = maybes.get(&req.name).unwrap().clone();

            if contains_key {
                add_item(
                    &pointer,
                    Maybe {
                        data: Err(anyhow::anyhow!("Error: Not yet resolved!")),
                        timestamp: Utc::now().timestamp(),
                    },
                )
                    .await;
            }
            let mut f: Option<
                Pin<Box<dyn Future<Output=anyhow::Result<ResponseResult>> + Send + 'static>>,
            > = None;

            let wallet_acc_address = wallet_acc_address.clone();
            match req.kind {
                TaskType::ChainRegistry => {
                    let path = req.args["path"].as_str().unwrap().to_string();

                    f = Some(Box::pin(get_supported_blockchains_from_chain_registry(path)));
                }
                TaskType::GovernanceProposals => {
                    let status = ProposalStatus::new(req.args["proposal_status"].as_str().unwrap());
                    let blockchain = supported_blockchains.as_ref().unwrap().get(req.args["blockchain"].as_str().unwrap())
                        .unwrap()
                        .clone();

                    f = Some(Box::pin(get_proposals(blockchain, status)));
                }
                _ => {}
            }

            if let Some(m) = f {
                join_set.spawn(async move {
                    {
                        let result = m.await;
                        let result: Maybe<ResponseResult> = Maybe {
                            data: result,
                            timestamp: Utc::now().timestamp(),
                        };
                        add_item(&pointer, result).await;
                    }
                });
                count += 1;
            }
        }
    }
    count
}
