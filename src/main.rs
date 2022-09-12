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
use model::{access_maybes, next_iteration_of_upcoming_tasks, setup_required_keys};

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

use cosmos_rust_interface::utils::entry::db::load_sled_db;
use cosmos_rust_interface::utils::entry::db::notification::socket::client_send_request;
use cosmos_rust_interface::utils::entry::db::query::query_sled_db;
use cosmos_rust_interface::utils::entry::db::query::socket::spawn_socket_query_server;
use cosmos_rust_interface::utils::entry::postproc::blockchain::cosmos::gov::governance_proposal_notifications;
use cosmos_rust_interface::utils::entry::postproc::meta_data::debug::debug;
use cosmos_rust_interface::utils::entry::postproc::meta_data::errors::errors;
use cosmos_rust_interface::utils::entry::postproc::meta_data::logs::logs;
use cosmos_rust_interface::utils::entry::*;
use cosmos_rust_interface::utils::response::ResponseResult;
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
    watcher
        .watch("./cosmos-rust-bot.json", RecursiveMode::Recursive)
        .unwrap();

    let tree = load_sled_db("cosmos-rust-bot-sled-db");
    spawn_socket_query_server(&tree);

    let tree_2 = tree.clone();
    let thread = tokio::spawn(async move {
        let mut subscriber = tree_2.watch_prefix(Entry::get_prefix());
        while let Some(event) = (&mut subscriber).await {
            match event {
                sled::Event::Insert { key, .. } | sled::Event::Remove { key } => {
                    // iterate over each subscription
                    for sub in tree_2.scan_prefix(Subscription::get_prefix()) {
                        match sub {
                            Ok((k, v)) => {
                                let entry = CosmosRustBotValue::from(v.to_vec());
                                match entry {
                                    CosmosRustBotValue::Subscription(s) => {
                                        if s.list.contains(&key.to_vec()) {
                                            // if it contains the key
                                            let mut query = s.get_query();
                                            query.as_object_mut().map(|x| {
                                                x.insert(
                                                    "update_subscription".to_string(),
                                                    serde_json::json!(true),
                                                )
                                            });
                                            let notification = Notification {
                                                query: s.query,
                                                entries: query_sled_db(&tree_2, query),
                                                user_list: s.user_list,
                                            };
                                            // notify
                                            client_send_request(
                                                CosmosRustServerValue::Notification(notification),
                                            );
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Err(e) => {}
                        }
                    }
                }
            }
        }
    });

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
            let mut task_meta_data: Vec<CosmosRustBotValue> = next_iteration_of_upcoming_tasks(
                &mut join_set,
                &mut maybes,
                &user_settings,
                &wallet_acc_address,
            )
            .await;
            entries.append(&mut task_meta_data);

            // ensures the display tasks operates on the same snapshot
            // since the processing of the ResponseResults is blazing fast, it makes no sense to hope for a value to be refreshed
            // so potentially reduces function calls
            // also post processing does not need to deal with Arc<Mutex>
            let snapshot_of_maybes = access_maybes(&maybes).await;

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

            let entry_keys = entries
                .iter()
                .map(|x| sled::IVec::from(x.key()))
                .collect::<Vec<sled::IVec>>();

            let mut batch = sled::Batch::default();
            // remove all, those that are not part of the current batch
            // except subscriptions, they are left untouched.
            let sub_prefix = Subscription::get_prefix();

            for k in tree.iter().keys() {
                let key = k?;
                if !entry_keys.contains(&key) && key.subslice(0, sub_prefix.len()) != &sub_prefix {
                    // remove outdated entries / indices
                    batch.remove(key);
                }
            }
            // keep all, those that are already inserted
            // insert new
            for x in 0..entries.len() {
                if !tree.contains_key(&entry_keys[x]).unwrap() {
                    // no-op in case key exists
                    batch.insert(&entry_keys[x], entries[x].value());
                }
            }
            // watch_prefix(..) allows to recieve events when the entry gets updated.
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
                    channels::get_supported_blockchains()
                        .get("terra")
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
