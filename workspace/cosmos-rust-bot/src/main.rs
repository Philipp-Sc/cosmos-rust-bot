#[macro_use]
extern crate litcrypt;
//https://github.com/anvie/litcrypt.rs
use_litcrypt!();

use bot_library::shared::{get_input, load_user_settings};

mod account;
mod control;
mod model;

use account::wallet::{decrypt_text_with_secret, encrypt_text_with_secret};
use control::try_run_function;
use model::requirements::UserSettings;
use model::{access_maybes};

use chrono::Utc;
use core::future::Future;
use core::pin::Pin;
use secstr::*;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinSet;

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

use cosmos_rust_interface::blockchain::account_from_seed_phrase;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{thread, time};

use cosmos_rust_interface::utils::entry::db::load_sled_db;
use cosmos_rust_interface::utils::entry::db::notification::socket::client_send_request;
use cosmos_rust_interface::utils::entry::db::query::{handle_query_sled_db, query_entries_sled_db, update_subscription};
use cosmos_rust_interface::utils::entry::db::query::socket::spawn_socket_query_server;
use cosmos_rust_interface::utils::entry::postproc::blockchain::cosmos::gov::governance_proposal_notifications;
use cosmos_rust_interface::utils::entry::postproc::meta_data::debug::debug;
use cosmos_rust_interface::utils::entry::postproc::meta_data::errors::errors;
use cosmos_rust_interface::utils::entry::postproc::meta_data::logs::logs;
use cosmos_rust_interface::utils::entry::*;
use cosmos_rust_interface::utils::response::ResponseResult;
use cosmos_rust_package::api::core::cosmos::channels;

use cosmos_rust_package::api::core::cosmos::channels::SupportedBlockchain;
use crate::model::{get_task_meta_data, poll_resolved_tasks, try_spawn_upcoming_tasks};
use crate::model::requirements::get_requirements;

use cosmos_rust_interface::utils::entry::db::*;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // stores all requirements either as task or the resolved value.
    let mut join_set: JoinSet<()> = JoinSet::new();
    let mut maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>> = HashMap::new();

    let mut user_settings: UserSettings = load_user_settings("./cosmos-rust-bot.json");
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
        .watch("./cosmos-rust-bot.json", RecursiveMode::Recursive)
        .unwrap();

    let tree = load_sled_db("cosmos_rust_bot_sled_db");
    spawn_socket_query_server(&tree);

    let mut cosmos_rust_bot_store = CosmosRustBotStore::new(&tree);
    let _thread = cosmos_rust_bot_store.spawn_thread_notify_on_subscription_update();


    loop {
        /*setup_required_keys(&mut maybes).await;*/
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
                &mut maybes,
                &req,
                &user_settings,
                &wallet_acc_address
            ).await;

            if number_of_tasks_resolved > 0 {

                // ensures the display tasks operates on the same snapshot
                // since the processing of the ResponseResults is blazing fast, it makes no sense to hope for a value to be refreshed
                // so potentially reduces function calls
                // also post processing does not need to deal with Arc<Mutex>
                let snapshot_of_maybes = access_maybes(&maybes).await;

                let mut entries: Vec<CosmosRustBotValue> = Vec::new();

                let mut task_meta_data: Vec<CosmosRustBotValue> = get_task_meta_data(&mut maybes, &req).await;
                entries.append(&mut task_meta_data);

                if user_settings.governance_proposal_notifications {
                    entries.append(&mut governance_proposal_notifications(&snapshot_of_maybes));
                }

                let mut task_meta_data: Vec<CosmosRustBotValue> = Vec::new();
                let mut debug: Vec<CosmosRustBotValue> = debug(&snapshot_of_maybes);
                let mut logs: Vec<CosmosRustBotValue> = logs(&snapshot_of_maybes);
                let mut errors: Vec<CosmosRustBotValue> = errors(&snapshot_of_maybes);
                task_meta_data.append(&mut debug);
                task_meta_data.append(&mut logs);
                task_meta_data.append(&mut errors);
                entries.append(&mut task_meta_data);

                CosmosRustBotValue::add_index(&mut entries, "timestamp", "timestamp");

                cosmos_rust_bot_store.update_items(entries);

            }else{
                let millis = time::Duration::from_millis(500);
                thread::sleep(millis);
            }
        }
        join_set.shutdown().await;
        if user_settings.hot_reload {
            user_settings = load_user_settings("./cosmos-rust-bot.json");
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
                    channels::get_supported_blockchains_from_chain_registry("./packages/chain-registry".to_string(),true,None)
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
