#[macro_use]
extern crate litcrypt;
//https://github.com/anvie/litcrypt.rs
use_litcrypt!();
 

use terra_rust_api_layer::services::blockchain::smart_contracts::objects::meta::api::{get_from_account};

// Model
mod state;

use crate::state::control::model::{Maybe,MaybeOrPromise,requirements,get_keys_of_running_tasks,get_keys_of_failed_tasks,await_running_tasks,abort_tasks,get_timestamps_of_resolved_tasks};

use crate::state::control::model::requirements::{UserSettings,my_requirement_keys, my_requirement_list,my_bot_keys};

use crate::state::control::model::wallet::{encrypt_text_with_secret,decrypt_text_with_secret}; 

use crate::state::control::try_run_function;

// View -> Model (Read Data)
mod view; 

// Action -> (View, Model)
mod bot;  
use bot::action::*;

mod ui;  // -> View

use ui::user_input::get_input; 

use ui::info::auto_repay::*;
use ui::info::auto_borrow::*;
use ui::info::auto_stake::*;
use ui::info::auto_farm::*; 

use ui::info::anchor::general::*;
use ui::info::anchor::account::*;
use ui::info::market::general::*;

use ui::logs::*;
use ui::errors::*; 
 

use terra_rust_bot_output::output::*;
use terra_rust_bot_output::output::pretty::Entry;

use secstr::*;
  

use std::collections::HashMap; 
use std::time::{Duration};

use std::sync::Arc; 
use tokio::sync::RwLock;  
use tokio::time::timeout; 

use chrono::{Utc};
use std::fs;


use core::pin::Pin;
use core::future::Future;

extern crate num_cpus;

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;

 #[tokio::main]
async fn main() -> anyhow::Result<()> {

        /* Internal variables */
        let num_cpus = num_cpus::get();

        let state: Arc<RwLock<Vec<Option<Entry>>>> = Arc::new(RwLock::new(vec![None; 1000]));
        // using timestamps to update each slot with a short delay.
        let mut timestamps_display: Vec<i64> = vec![0i64; 1000];
        let mut display_out_timestamp = 0i64;
        // stores all requirements either as task or the resolved value.
        let tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>> = Arc::new(RwLock::new(HashMap::new()));


        /* Load user settings */
        let mut user_settings: UserSettings = load_user_settings();

        if user_settings.remove {
            let res = fs::remove_file("./terra-rust-bot.json");
            println!("{:?}",res);
        }

        /* Get wallet details */
        let mut wallet_seed_phrase = SecUtf8::from("".to_string());
        let mut wallet_acc_address = SecUtf8::from(user_settings.terra_wallet_address.as_ref().unwrap_or(&"".to_string()));

        if user_settings.anchor_protocol_auto_repay || user_settings.anchor_protocol_auto_borrow || user_settings.anchor_protocol_auto_stake || user_settings.anchor_protocol_auto_farm {
         // ** seed phrase needed **
         wallet_seed_phrase = encrypt_text_with_secret(get_input("Enter your seed phrase (press Enter to skip):").to_string());
         if wallet_acc_address.unsecure().len()!=44 || !user_settings.test {
             wallet_acc_address = SecUtf8::from(get_from_account(&decrypt_text_with_secret(&wallet_seed_phrase)).unwrap_or("".to_string()));
         }
        }else if wallet_acc_address.unsecure().len()==0 {
         // ** maybe need wallet address **
         if user_settings.anchor_account_info || user_settings.anchor_protocol_auto_repay || user_settings.anchor_protocol_auto_borrow || user_settings.anchor_protocol_auto_stake || user_settings.anchor_protocol_auto_farm {
             wallet_acc_address = SecUtf8::from(get_input("Enter your wallet address (press Enter to skip):").to_string());
         }
        }
        println!("{esc}c", esc = 27 as char);

        // Arc allows multiple references to the same object,
        // to potentially spawn multiple tasks with access to the seed phrase, while not revealing the string.
        let wallet_seed_phrase = Arc::new(wallet_seed_phrase);
        let wallet_acc_address = Arc::new(wallet_acc_address);


        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Create a watcher object, delivering debounced events.
        // The notification back-end is selected based on the platform.
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch("./terra-rust-bot.json", RecursiveMode::Recursive).unwrap();

        loop {
            let mut is_first_run: bool = true;
            let (req, req_keys, bot_args, req_keys_status) = process_user_settings(&user_settings);
            /*
             * This loop has three major blocking elements.
             * 1) Awaiting running tasks if thread limit is reached. No harm of waiting here.
             * 2) Multiple calls of try_add_to_display, checks if results are available.
             *    Timeout after 0.1s, worst possible delay (unlikely to happen): req.len() * 0.1s. (~10s).
             * 3) Writing the display to disk. Acceptable delay, less than 1s.
             *
             * */
            while !user_settings.pause_requested {
                if user_settings.hot_reload {
                    match rx.recv_timeout(Duration::from_millis(10)) {
                        Ok(event) => {
                            println!("{:?}", event);
                            break;
                        },
                        Err(_e) => {
                            //println!("watch error: {:?}", e),
                        },
                    }
                }

                let mut offset: usize = 0;

                let req_unresolved = get_keys_of_running_tasks(&tasks, &req_keys_status).await;
                let req_failed = get_keys_of_failed_tasks(&tasks, &req_keys_status).await;

                // waiting for unresolved tasks to catch up
                if req_unresolved.len() >= num_cpus {
                    // anyway we need to have free threads to spawn more tasks
                    // useful to wait here
                    timeout(Duration::from_secs(30), await_running_tasks(&tasks, &req_keys)).await.ok();
                }

                let req_resolved_timestamps = get_timestamps_of_resolved_tasks(&tasks, &req_keys).await;

                let now = Utc::now().timestamp();

                for x in 0..req_resolved_timestamps.len() {
                    let entry = Entry {
                        timestamp: now,
                        key: req_keys[x].to_string(),
                        prefix: None,
                        value: req_resolved_timestamps[x].to_string(),
                        suffix: None,
                        group: Some("[Task][History]".to_string()),
                    };
                    add_entry_to_state(&state, offset, entry).await.ok();
                    offset += 1;
                }

                let mut req_to_update: Vec<&str> = Vec::new();
                for i in 0..req.len() {
                    if req_to_update.len() >= num_cpus {
                        break;
                    }
                    if !req_unresolved.contains(&req[i].0) && (req_failed.contains(&req[i].0) || req_resolved_timestamps[i] == 0i64 || ((now - req_resolved_timestamps[i]) > req[i].1 as i64)) { // unresolved requirements will not be refreshed.
                        req_to_update.push(req[i].0);
                    }
                }

                let entry = Entry {
                    timestamp: now,
                    key: "failed".to_string(),
                    prefix: None,
                    value: req_failed.len().to_string(),
                    suffix: None,
                    group: Some("[Task][Count]".to_string()),
                };
                add_entry_to_state(&state, offset, entry).await.ok();
                offset += 1;

                let entry = Entry {
                    timestamp: now,
                    key: "pending".to_string(),
                    prefix: None,
                    value: req_unresolved.len().to_string(),
                    suffix: None,
                    group: Some("[Task][Count]".to_string()),
                };
                add_entry_to_state(&state, offset, entry).await.ok();
                offset += 1;
                let entry = Entry {
                    timestamp: now,
                    key: "upcoming".to_string(),
                    prefix: None,
                    value: req_to_update.len().to_string(),
                    suffix: None,
                    group: Some("[Task][Count]".to_string()),
                };
                add_entry_to_state(&state, offset, entry).await.ok();
                offset += 1;

                let entry = Entry {
                    timestamp: now,
                    key: "all".to_string(),
                    prefix: None,
                    value: req_keys.len().to_string(),
                    suffix: None,
                    group: Some("[Task][Count]".to_string()),
                };
                add_entry_to_state(&state, offset, entry).await.ok();
                offset += 1;

                let entry = Entry {
                    timestamp: now,
                    key: "failed".to_string(),
                    prefix: None,
                    value: format!("{:?}", req_failed),
                    suffix: None,
                    group: Some("[Task][List]".to_string()),
                };
                add_entry_to_state(&state, offset, entry).await.ok();
                offset += 1;

                let entry = Entry {
                    timestamp: now,
                    key: "pending".to_string(),
                    prefix: None,
                    value: format!("{:?}", req_unresolved),
                    suffix: None,
                    group: Some("[Task][List]".to_string()),
                };
                add_entry_to_state(&state, offset, entry).await.ok();
                offset += 1;

                let entry = Entry {
                    timestamp: now,
                    key: "upcoming".to_string(),
                    prefix: None,
                    value: format!("{:?}", req_to_update),
                    suffix: None,
                    group: Some("[Task][List]".to_string()),
                };
                add_entry_to_state(&state, offset, entry).await.ok();
                offset += 1;

                let entry = Entry {
                    timestamp: now,
                    key: "all".to_string(),
                    prefix: None,
                    value: format!("{:?}", req_keys),
                    suffix: None,
                    group: Some("[Task][List]".to_string()),
                };
                add_entry_to_state(&state, offset, entry).await.ok();
                offset += 1;

                requirements(&tasks, &user_settings, &wallet_acc_address, &req_to_update).await;
                // instead of calculating what req should be updated here, it should be part of _memory
                // so here only requirements_next() needs to be called.


                // waiting for all open **display** updates.
                // if one task is slow, because the requirement is not yet resolved, it slows down the whole loop,
                // therefore it will timeout after 0.1s, so the loop can continue.

                if user_settings.terra_market_info {
                    for t in display_market_info(&tasks, &state, &mut offset, is_first_run).await {
                        if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
                            try_add_to_state(&state, t.0, Box::pin(t.1)).await.ok();
                            timestamps_display[t.0] = now;
                        }
                    }
                }
                if user_settings.anchor_general_info {
                    for t in display_anchor_info(&tasks, &state, &mut offset, is_first_run).await {
                        if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
                            try_add_to_state(&state, t.0, Box::pin(t.1)).await.ok();
                            timestamps_display[t.0] = now;
                        }
                    }
                }
                if user_settings.anchor_account_info {
                    for t in display_anchor_account(&tasks, &state, &mut offset, is_first_run).await {
                        if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
                            try_add_to_state(&state, t.0, Box::pin(t.1)).await.ok();
                            timestamps_display[t.0] = now;
                        }
                    }
                }

                if user_settings.anchor_protocol_auto_stake {

                    // starts the bot specific function as task.
                    // (only if previous task of the same key has finished)
                    let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_stake_rewards(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                    try_run_function(&tasks, task, "anchor_auto_stake", user_settings.test).await;

                    // checks if data for the display is available
                    let anchor_auto_stake = lazy_anchor_account_auto_stake_rewards(&tasks, &state, &mut offset, user_settings.test, is_first_run).await;
                    for t in anchor_auto_stake {
                        if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
                            try_add_to_state(&state, t.0, Box::pin(t.1)).await.ok();
                            timestamps_display[t.0] = now;
                        }
                    }
                }
                if user_settings.anchor_protocol_auto_farm {
                    let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_farm_rewards(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                    try_run_function(&tasks, task, "anchor_auto_farm", user_settings.test).await;

                    let anchor_auto_lp = lazy_anchor_account_auto_farm_rewards(&tasks, &state, &mut offset, user_settings.test, is_first_run).await;
                    for t in anchor_auto_lp {
                        if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
                            try_add_to_state(&state, t.0, Box::pin(t.1)).await.ok();
                            timestamps_display[t.0] = now;
                        }
                    }
                }

                if user_settings.anchor_protocol_auto_repay {
                    let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_redeem_and_repay_stable(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                    try_run_function(&tasks, task, "anchor_auto_repay", user_settings.test).await;

                    let anchor_auto_repay = lazy_anchor_account_auto_repay(&tasks, &state, &mut offset, user_settings.test, is_first_run).await;
                    for t in anchor_auto_repay {
                        if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
                            try_add_to_state(&state, t.0, Box::pin(t.1)).await.ok();
                            timestamps_display[t.0] = now;
                        }
                    }
                }
                if user_settings.anchor_protocol_auto_borrow {
                    let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_borrow_and_deposit_stable(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                    try_run_function(&tasks, task, "anchor_auto_borrow", user_settings.test).await;

                    let anchor_auto_borrow = lazy_anchor_account_auto_borrow(&tasks, &state, &mut offset, user_settings.test, is_first_run).await;
                    for t in anchor_auto_borrow {
                        if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
                            try_add_to_state(&state, t.0, Box::pin(t.1)).await.ok();
                            timestamps_display[t.0] = now;
                        }
                    }
                }

                display_all_logs(&tasks, &state, &mut offset, &bot_args).await;

                display_all_errors(&tasks, &*req_unresolved, &state, &mut offset).await;

                if is_first_run {
                    is_first_run = false;
                }

                // ensuring one file write per 300ms, not faster.
                let now = Utc::now().timestamp_millis();

                if display_out_timestamp == 0i64 || now - display_out_timestamp > 300i64 {
                    // writing display to file.
                    // let new_line = format!("{esc}c", esc = 27 as char);
                    let vec: Vec<Option<Entry>> = state.read().await.to_vec();
                    let vec: Vec<Entry> = vec.into_iter().filter_map(|x| x).collect();
                    let line = format!("{}", serde_json::to_string(&*vec).unwrap());
                    fs::write("./packages/terra-rust-bot-output/terra-rust-bot-state.json", &line).ok();
                    display_out_timestamp = now;
                }
            }
            abort_tasks(&tasks,&req_keys_status).await.ok();
            if user_settings.hot_reload {
                user_settings = load_user_settings();
            }
        }
}


fn load_user_settings() -> UserSettings {

    let user_settings: UserSettings = match fs::read_to_string("./terra-rust-bot.json") {
        Ok(file) => {
            match serde_json::from_str(&file) {
                Ok(res) => {
                    res
                },
                Err(err) => {
                    println!("{:?}",err);
                    Default::default()
                }
            }
        },
        Err(err) => {
            println!("{:?}",err);
            Default::default()
        }
    };
    user_settings
}

fn process_user_settings(user_settings: &UserSettings) -> (Vec<(&'static str, i32, Vec<&'static str>)>,Vec<&str>,Vec<&str>,Vec<&str>) {
    let req: Vec<(&'static str, i32, Vec<&'static str>)> = my_requirement_list(&user_settings);
    let req_keys: Vec<&str> = my_requirement_keys(&user_settings);

    let bot_args: Vec<&str> = my_bot_keys(&user_settings);
    let mut req_keys_status = req_keys.clone();
    for bot_tasks in &bot_args {
        req_keys_status.push(bot_tasks);
    }
    (req,req_keys,bot_args,req_keys_status)
}