#[macro_use]
extern crate litcrypt;
//https://github.com/anvie/litcrypt.rs
use_litcrypt!();

use bot_library::shared::{get_input, load_user_settings};

mod account;
//mod control;
mod model;

use account::wallet::{decrypt_text_with_secret, encrypt_text_with_secret};
//use control::try_run_function;
use model::requirements::UserSettings;

use cosmos_rust_interface::cosmos_rust_package::chrono::Utc;
use core::future::Future;
use core::pin::Pin;
use secstr::*;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use cosmos_rust_interface::cosmos_rust_package::tokio as tokio;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinSet;

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

use cosmos_rust_interface::blockchain::account_from_seed_phrase;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{thread, time};

use cosmos_rust_interface::utils::entry::db::load_sled_db;
use cosmos_rust_interface::utils::entry::db::query::socket::spawn_socket_query_server;
use cosmos_rust_interface::utils::entry::postproc::blockchain::cosmos::gov::governance_proposal_notifications;
use cosmos_rust_interface::utils::entry::postproc::meta_data::debug::debug;
use cosmos_rust_interface::utils::entry::postproc::meta_data::errors::errors;
//use cosmos_rust_interface::utils::entry::postproc::meta_data::logs::logs;
use cosmos_rust_interface::utils::entry::*;
use cosmos_rust_interface::utils::response::ResponseResult;
use cosmos_rust_interface::cosmos_rust_package::api::core::cosmos::channels;

use cosmos_rust_interface::cosmos_rust_package::api::core::cosmos::channels::SupportedBlockchain;
use crate::model::{get_task_meta_data, poll_resolved_tasks, try_spawn_upcoming_tasks};
use crate::model::requirements::get_requirements;

use cosmos_rust_interface::utils::entry::db::*;

const QUERY_SOCKET: &str = "./tmp/cosmos_rust_bot_query_socket";
const SETTINGS_PATH: &str = "./tmp/cosmos-rust-bot.json";
const CRB_SLED_DB: &str = "./tmp/cosmos_rust_bot_sled_db";
const CRB_SUBSCRIPTION_STORE_SLED_DB: &str = "./tmp/cosmos_rust_bot_subscriptions_sled_db";
const CRB_SUBSCRIPTION_STORE_JSON: &str = "./tmp/cosmos_rust_bot_subscriptions.json";
const CRB_REGISTRATION_STORE_JSON: &str = "./tmp/cosmos_rust_bot_registrations.json";


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    env_logger::init();

    // stores all requirements either as task or the resolved value.
    let mut join_set: JoinSet<()> = JoinSet::new();
    let task_store: TaskMemoryStore = TaskMemoryStore::new().unwrap();

    let mut user_settings: UserSettings = load_user_settings(SETTINGS_PATH);
    //println!("{}", serde_json::to_string_pretty(&user_settings)?);

    let (wallet_seed_phrase, wallet_acc_address) = get_wallet_details(&user_settings).await;

    // Create a channel to receive the events.
    let (tx, rx) = channel();
    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();
    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(SETTINGS_PATH, RecursiveMode::Recursive)
        .unwrap();

    let entry_index_db = load_sled_db(CRB_SLED_DB);
    let subscription_db = load_sled_db(CRB_SUBSCRIPTION_STORE_SLED_DB);
    let subscription_store = SubscriptionStore::new(&subscription_db);
    subscription_store.import_subscriptions(CRB_SUBSCRIPTION_STORE_JSON);
    subscription_store.import_registrations(CRB_REGISTRATION_STORE_JSON);

    let mut cosmos_rust_bot_store = CosmosRustBotStore::new(entry_index_db,subscription_store);

    spawn_socket_query_server(QUERY_SOCKET,&cosmos_rust_bot_store);

    let _thread = cosmos_rust_bot_store.spawn_notify_on_subscription_update_thread();

        loop {
            let req = get_requirements(&user_settings);

            while !user_settings.pause_requested {
                if user_settings.hot_reload {
                    match rx.recv_timeout(Duration::from_millis(10)) {
                        Ok(event) => {
                            println!("{:?}", event);
                            break;
                        }
                        Err(_e) => {
                            //println!("watch error: {:?}", e),
                        }
                    }
                }

                let number_of_tasks_resolved = poll_resolved_tasks(&mut join_set).await;

                let _number_of_tasks_added = try_spawn_upcoming_tasks(
                    &mut join_set,
                    &task_store,
                    &req,
                    &user_settings,
                    &wallet_acc_address
                ).await;

                if number_of_tasks_resolved > 0 {

                    // TODO: use SledDb listener/events to reduce the number of get function calls.
                    // TODO: have dependencies and update trigger key lists. if trigger key was updated, then get dependencies and update.

                    let mut entries: Vec<CosmosRustBotValue> = Vec::new();

                    let mut task_meta_data: Vec<CosmosRustBotValue> = get_task_meta_data(&task_store, &req).await;
                    entries.append(&mut task_meta_data);

                    if user_settings.governance_proposal_notifications {
                        entries.append(&mut governance_proposal_notifications(&task_store));
                    }

                    let mut task_meta_data: Vec<CosmosRustBotValue> = Vec::new();
                    //let mut debug: Vec<CosmosRustBotValue> = debug(&mut internal_snapshot_of_memory);
                    //let mut logs: Vec<CosmosRustBotValue> = logs(&snapshot_of_memory);
                    let mut errors: Vec<CosmosRustBotValue> = errors(&task_store); // TODO: make debug, errors, logs one.
                    //task_meta_data.append(&mut debug);
                    //task_meta_data.append(&mut logs);
                    task_meta_data.append(&mut errors);
                    entries.append(&mut task_meta_data);

                    CosmosRustBotValue::add_index(&mut entries, "timestamp", "timestamp");

                    cosmos_rust_bot_store.update_items(entries);
                } else {
                    let millis = time::Duration::from_millis(500);
                    thread::sleep(millis);
                }
            }
            join_set.shutdown().await;
            if user_settings.hot_reload {
                user_settings = load_user_settings(SETTINGS_PATH);
            }
        }
}

async fn get_wallet_details(user_settings: &UserSettings) -> (Arc<SecUtf8>, Arc<SecUtf8>) {
    /* Get wallet details */
    let mut wallet_seed_phrase = SecUtf8::from("".to_string());
    let mut wallet_acc_address = SecUtf8::from(
        user_settings
            .terra_wallet_address
            .as_ref()
            .unwrap_or(&"".to_string()),
    );
    //  /^terra1[a-z0-9]{38}$/]

    if false
    /*user_settings.anchor_protocol_auto_repay || user_settings.anchor_protocol_auto_borrow || user_settings.anchor_protocol_auto_stake || user_settings.anchor_protocol_auto_farm*/
    {
        // ** seed phrase needed **
        wallet_seed_phrase = encrypt_text_with_secret(
            get_input("Enter your seed phrase (press Enter to skip):").to_string(),
        );
        if wallet_acc_address.unsecure().len() != 44 || !user_settings.test {
            wallet_acc_address = SecUtf8::from(
                account_from_seed_phrase(
                    decrypt_text_with_secret(&wallet_seed_phrase),
                    channels::get_supported_blockchains_from_chain_registry("./chain-registry".to_string(),true,None)
                        .await.get("terra2")
                        .unwrap()
                        .clone(),
                )
                .unwrap_or("".to_string()),
            );
        }
    } else if wallet_acc_address.unsecure().len() == 0 {
        // ** maybe need wallet address **
        if false
        /*user_settings.anchor_account_info || user_settings.anchor_protocol_auto_repay || user_settings.anchor_protocol_auto_borrow || user_settings.anchor_protocol_auto_stake || user_settings.anchor_protocol_auto_farm*/
        {
            wallet_acc_address = SecUtf8::from(
                get_input("Enter your wallet address (press Enter to skip):").to_string(),
            );
        }
    }
    println!("{esc}c", esc = 27 as char);

    // Arc allows multiple references to the same object,
    // to potentially spawn multiple tasks with access to the seed phrase, while not revealing the string.
    let wallet_seed_phrase = Arc::new(wallet_seed_phrase);
    let wallet_acc_address = Arc::new(wallet_acc_address);
    (wallet_seed_phrase, wallet_acc_address)
}
