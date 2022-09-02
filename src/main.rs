#[macro_use]
extern crate litcrypt;
//https://github.com/anvie/litcrypt.rs
use_litcrypt!();

use bot_library::shared::{load_user_settings, get_input};

mod account;
mod model;
mod control;

use model::{next_iteration_of_upcoming_tasks, setup_required_keys, access_maybes};
use model::requirements::{UserSettings};
use account::wallet::{encrypt_text_with_secret, decrypt_text_with_secret};
use control::try_run_function;

use secstr::*;
use std::collections::HashMap;
use std::time::{Duration};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinSet;
use chrono::{Utc};
use std::fs;
use core::pin::Pin;
use core::future::Future;


use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use cosmos_rust_interface::blockchain::account_from_seed_phrase;

use cosmos_rust_interface::utils::response::ResponseResult;
use cosmos_rust_interface::utils::entry::postproc::blockchain::cosmos::gov::governance_proposal_notifications;
use cosmos_rust_interface::utils::entry::*;
use cosmos_rust_interface::utils::entry::db::{load_sled_db, spawn_socket_query_server};
use cosmos_rust_interface::utils::entry::postproc::meta_data::debug::debug;
use cosmos_rust_interface::utils::entry::postproc::meta_data::errors::errors;
use cosmos_rust_interface::utils::entry::postproc::meta_data::logs::logs;
use cosmos_rust_package::api::core::cosmos::channels;

use cosmos_rust_package::api::core::cosmos::channels::SupportedBlockchain;


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
    watcher.watch("./cosmos-rust-bot.json", RecursiveMode::Recursive).unwrap();

    let tree = load_sled_db("cosmos-rust-bot-sled-db");
    spawn_socket_query_server(&tree);

    loop {
        setup_required_keys(&mut maybes).await;

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

            let mut entries: Vec<CosmosRustBotValue> = Vec::new();
            let mut task_meta_data: Vec<CosmosRustBotValue> = next_iteration_of_upcoming_tasks(&mut join_set, &mut maybes, &user_settings, &wallet_acc_address).await;
            entries.append(&mut task_meta_data);

            // ensures the display tasks operates on the same snapshot
            // since the processing of the ResponseResults is blazing fast, it makes no sense to hope for a value to be refreshed
            // so potentially reduces function calls
            // also post processing does not need to deal with Arc<Mutex>
            let snapshot_of_maybes = access_maybes(&maybes).await;

            if user_settings.governance_proposal_notifications {
                entries.append(&mut governance_proposal_notifications(&snapshot_of_maybes));
            }
            /*
            if user_settings.terra_market_info {
                maybe_futures.append(&mut market_info(&copy_of_maybes).await);
            }
            if user_settings.anchor_general_info {
                maybe_futures.append(&mut anchor_info(&copy_of_maybes).await);
            }
            if user_settings.anchor_account_info {
                maybe_futures.append(&mut anchor_account(&copy_of_maybes).await);
            }

            if user_settings.anchor_protocol_auto_stake {
                // starts the agent specific function as task.
                // (only if previous task of the same key has finished)
                let asset_list = asset_whitelist.clone();
                let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_stake_rewards(asset_list, copy_of_maybes.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                try_run_function(&mut join_set, &copy_of_maybes, task, "anchor_auto_stake", user_settings.test).await;
                // if resolved the task will be transformed into a maybe result at the end of the lazy_* method.
                maybe_futures.append(&mut lazy_anchor_account_auto_stake_rewards(&copy_of_maybes, user_settings.test).await);
                // also tries to calculate all state updates for cosmos-rust-bot-state.json.
            }
            if user_settings.anchor_protocol_auto_farm {
                let asset_list = asset_whitelist.clone();
                let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_farm_rewards(asset_list, copy_of_maybes.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                try_run_function(&mut join_set, &copy_of_maybes, task, "anchor_auto_farm", user_settings.test).await;
                maybe_futures.append(&mut lazy_anchor_account_auto_farm_rewards(&copy_of_maybes, user_settings.test).await);
            }
            if user_settings.anchor_protocol_auto_repay {
                let asset_list = asset_whitelist.clone();
                let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_redeem_and_repay_stable(asset_list, copy_of_maybes.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                try_run_function(&mut join_set, &copy_of_maybes, task, "anchor_auto_repay", user_settings.test).await;
                maybe_futures.append(&mut lazy_anchor_account_auto_repay(&copy_of_maybes, user_settings.test).await);
            }
            if user_settings.anchor_protocol_auto_borrow {
                let asset_list = asset_whitelist.clone();
                let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_borrow_and_deposit_stable(asset_list, copy_of_maybes.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                try_run_function(&mut join_set, &copy_of_maybes, task, "anchor_auto_borrow", user_settings.test).await;
                -maybe_futures.append(&mut lazy_anchor_account_auto_borrow(&copy_of_maybes, user_settings.test).await);
            }
            */

            let mut result_meta_data: Vec<CosmosRustBotValue> = Vec::new();
            //let mut debug: Vec<Entry> = debug(&snapshot_of_maybes);
            let mut logs: Vec<CosmosRustBotValue> = logs(&snapshot_of_maybes);
            let mut errors: Vec<CosmosRustBotValue> = errors(&snapshot_of_maybes);
            //meta_data.append(&mut debug);
            result_meta_data.append(&mut logs);
            result_meta_data.append(&mut errors);
            entries.append(&mut result_meta_data);

            CosmosRustBotValue::add_index(&mut entries, "timestamp", "timestamp");

            let mut batch = sled::Batch::default();
            for k in tree.iter().keys() {
                let key = k?;
                batch.remove(key);
            }
            for x in 0..entries.len() {
                batch.insert(entries[x].key(), entries[x].value());
            }
            tree.apply_batch(batch)?;
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
    let mut wallet_acc_address = SecUtf8::from(user_settings.terra_wallet_address.as_ref().unwrap_or(&"".to_string()));
    //  /^terra1[a-z0-9]{38}$/]

    if false /*user_settings.anchor_protocol_auto_repay || user_settings.anchor_protocol_auto_borrow || user_settings.anchor_protocol_auto_stake || user_settings.anchor_protocol_auto_farm*/ {
        // ** seed phrase needed **
        wallet_seed_phrase = encrypt_text_with_secret(get_input("Enter your seed phrase (press Enter to skip):").to_string());
        if wallet_acc_address.unsecure().len() != 44 || !user_settings.test {
            wallet_acc_address = SecUtf8::from(account_from_seed_phrase(decrypt_text_with_secret(&wallet_seed_phrase), channels::get_supported_blockchains().get("terra").unwrap().clone()).unwrap_or("".to_string()));
        }
    } else if wallet_acc_address.unsecure().len() == 0 {
        // ** maybe need wallet address **
        if false /*user_settings.anchor_account_info || user_settings.anchor_protocol_auto_repay || user_settings.anchor_protocol_auto_borrow || user_settings.anchor_protocol_auto_stake || user_settings.anchor_protocol_auto_farm*/ {
            wallet_acc_address = SecUtf8::from(get_input("Enter your wallet address (press Enter to skip):").to_string());
        }
    }
    println!("{esc}c", esc = 27 as char);

    // Arc allows multiple references to the same object,
    // to potentially spawn multiple tasks with access to the seed phrase, while not revealing the string.
    let wallet_seed_phrase = Arc::new(wallet_seed_phrase);
    let wallet_acc_address = Arc::new(wallet_acc_address);
    (wallet_seed_phrase, wallet_acc_address)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}