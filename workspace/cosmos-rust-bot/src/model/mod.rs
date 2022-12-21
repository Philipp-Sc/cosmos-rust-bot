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
use std::ops::Deref;

use cosmos_rust_interface::utils::entry::*;

use cosmos_rust_interface::blockchain::cosmos::gov::get_proposals;
use cosmos_rust_interface::utils::response::{ResponseResult, TaskResult};
use cosmos_rust_package::api::core::cosmos::channels;
use cosmos_rust_package::api::core::cosmos::channels::SupportedBlockchain;
use cosmos_rust_package::api::custom::query::gov::ProposalStatus;
use serde_json::json;
use std::string::ToString;
use strum::IntoEnumIterator;
use strum_macros;
use strum_macros::EnumIter;
use cosmos_rust_interface::blockchain::cosmos::chain_registry::get_supported_blockchains_from_chain_registry;
use cosmos_rust_interface::utils::entry::db::{RetrievalMethod, TaskMemoryStore};
use log::{debug, info, trace};
use cosmos_rust_interface::services::fraud_detection::fraud_detection;
use cosmos_rust_interface::services::gpt3::gpt3;

#[derive(strum_macros::Display, Debug, EnumIter, PartialEq, serde::Serialize)]
pub enum TaskState {
    Unknown,
    Pending,
    Reserved,
    Failed,
    Resolved,
    Upcoming,
}

#[derive(Debug, serde::Serialize)]
pub struct TaskItem {
    pub name: String,
    pub state: TaskState,
    pub timestamp: i64,
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

        CosmosRustBotValue::Entry(Entry::Value(Value {
            timestamp: now,
            origin: "task_meta_data".to_string(),
            custom_data: CustomData::MetaData(MetaData{
                index: i as i32,
                kind: v.0.to_owned(),
                state: state.to_owned(),
                value: value.to_owned(),
                summary: info.to_owned(),
            }),
            imperative: ValueImperative::Notify
        }))
    }).collect::<Vec<CosmosRustBotValue>>();

    CosmosRustBotValue::add_index(&mut task_meta_data, "index", "index");
    CosmosRustBotValue::add_membership(&mut task_meta_data, None, "task_meta_data");
    CosmosRustBotValue::add_variants_of_memberships(&mut task_meta_data, vec!["kind", "state"]);
    task_meta_data
}

pub async fn poll_resolved_tasks(join_set: &mut JoinSet<()>) -> usize {

    info!("poll_resolved_tasks");
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
    task_store: &TaskMemoryStore,
    req: &Vec<TaskSpec>,
    user_settings: &UserSettings,
    wallet_acc_address: &Arc<SecUtf8>,
) -> usize {

    let task_list: Vec<TaskItem> = get_task_list(task_store,req).await;

    info!("try_spawn_upcoming_tasks: task_list: {}", serde_json::to_string_pretty(&task_list).unwrap_or("Formatting Error".to_string()));

    let upcoming_task_spec_list: Vec<&TaskSpec> = req
        .iter()
        .filter(|&x| task_list
            .iter()
            .filter(|y| y.state == TaskState::Upcoming)
            .map(|y| &y.name)
            .collect::<Vec<&String>>().contains(&&x.name))
        .collect();

    info!("try_spawn_upcoming_tasks: to_update: {}", serde_json::to_string_pretty(&upcoming_task_spec_list).unwrap_or("Formatting Error".to_string()));

    let number_of_tasks_added =
        spawn_tasks(
            join_set,
            task_store,
            &user_settings,
            &wallet_acc_address,
            upcoming_task_spec_list,
        )
        .await;

    info!("try_spawn_upcoming_tasks: number_of_tasks_added: {}", &number_of_tasks_added);
    number_of_tasks_added
}

pub async fn get_task_meta_data(
                                task_store: &TaskMemoryStore,
                                req: &Vec<TaskSpec>
                                ) -> Vec<CosmosRustBotValue>{

    let task_list = get_task_list(task_store,req).await;
    task_meta_data(task_list)
}

pub async fn get_task_list(
    task_store: &TaskMemoryStore,
    req: &Vec<TaskSpec>,
) ->Vec<TaskItem> {

    let mut task_list: Vec<TaskItem> = Vec::new();

    let now = Utc::now().timestamp();
    for (k, v) in task_store.value_iter::<ResponseResult>(&RetrievalMethod::Get) {
        let state: TaskState;
        let time: i64;
        match v {
            Maybe {
                data: Err(err),
                timestamp,
            } => {
                time = timestamp;
                match err {
                    MaybeError::NotYetResolved(_key) => {
                        state = TaskState::Pending;
                    },
                    MaybeError::EntryReserved(_key) => {
                        state = TaskState::Reserved;
                    },
                    _ => {
                        state = TaskState::Failed;
                    },
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
async fn spawn_tasks(
    join_set: &mut JoinSet<()>,
    task_store: &TaskMemoryStore,
    _user_settings: &UserSettings,
    _wallet_acc_address: &Arc<SecUtf8>,
    to_update: Vec<&TaskSpec>,
) -> usize {

    let supported_blockchains = match task_store.get("internal_chain_registry",&RetrievalMethod::GetOk) {
        Ok(Maybe{data: Ok(ResponseResult::ChainRegistry(chain_registry)), timestamp: t }) => {
            info!("spawn_tasks: chain_registry available");
            Some(chain_registry)
        },
        Err(_) | Ok(Maybe{ .. }) => {
            info!("spawn_tasks: chain_registry unavailable");
            None
        },
    };

    let mut count: usize = 0;
    for req in to_update {

        let mut f: Option<
            Pin<Box<dyn Future<Output=anyhow::Result<TaskResult>> + Send + 'static>>,
        > = None;

        match req.kind {

            TaskType::FraudDetection => {
                f = Some(Box::pin(fraud_detection(task_store.clone(),req.name.clone())));
            }

            TaskType::GPT3 => {
                f = Some(Box::pin(gpt3(task_store.clone(),req.name.clone())));
            }

            TaskType::ChainRegistry => {
                let path = req.args["path"].as_str().unwrap().to_string();
                f = Some(Box::pin(get_supported_blockchains_from_chain_registry(path,task_store.clone(),req.name.clone())));
            }
            TaskType::GovernanceProposals => {
                if let Some(supported_blockchains) = supported_blockchains.as_ref() {
                    let status = ProposalStatus::new(req.args["proposal_status"].as_str().unwrap());
                    let blockchain = supported_blockchains.get(req.args["blockchain"].as_str().unwrap())
                        .unwrap()
                        .clone();
                    f = Some(Box::pin(get_proposals(blockchain, status,task_store.clone(),req.name.clone())));
                }
            }
            _ => {}
        }

        if let Some(m) = f {

            task_store.push(
                &req.name,
                Maybe::<ResponseResult>  {
                    data: Err(MaybeError::NotYetResolved(req.name.clone())),
                    timestamp: Utc::now().timestamp(),
                }
            ).ok();

            let task_store_copy = task_store.clone();
            let key = req.name.clone();
            join_set.spawn(async move {
                {
                    let result = m.await;
                    let result: Maybe<ResponseResult> = Maybe {
                        data: match result {
                            Ok(data) => Ok(ResponseResult::TaskResult(data)),
                            Err(err) => Err(MaybeError::AnyhowError(err.to_string())),
                        },
                        timestamp: Utc::now().timestamp(),
                    };
                    task_store_copy.push(&key,result).ok();
                }
            });
            count += 1;
        }
    }
    count
}
