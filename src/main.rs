#[macro_use]
extern crate litcrypt;
//https://github.com/anvie/litcrypt.rs
use_litcrypt!();

use terra_rust_api_layer::services::blockchain::smart_contracts::objects::meta::api::{get_from_account};

use terra_rust_bot_essentials::output::*;
use terra_rust_bot_essentials::shared::{load_user_settings,get_input,Entry};

mod state;
use crate::state::control::model::{Maybe,MaybeOrPromise,requirements_next,abort_tasks};
use crate::state::control::model::requirements::{UserSettings};
use crate::state::control::model::wallet::{encrypt_text_with_secret,decrypt_text_with_secret};
use crate::state::control::try_run_function;
mod view;
mod bot;  
use bot::action::*;
mod ui;
use ui::info::auto_repay::*;
use ui::info::auto_borrow::*;
use ui::info::auto_stake::*;
use ui::info::auto_farm::*;
use ui::info::anchor::general::*;
use ui::info::anchor::account::*;
use ui::info::market::general::*;
use ui::logs::*;
use ui::errors::*; 

use secstr::*;
use std::collections::HashMap; 
use std::time::{Duration};
use std::sync::Arc; 
use tokio::sync::RwLock;
use chrono::{Utc};
use std::fs;
use core::pin::Pin;
use core::future::Future;

extern crate num_cpus;

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;

 #[tokio::main]
async fn main() -> anyhow::Result<()> {

        let num_cpus = num_cpus::get();

        let state: Arc<RwLock<Vec<Option<Entry>>>> = Arc::new(RwLock::new(vec![None; 1000]));
        // using timestamps to update each slot with a short delay.
        let mut timestamps_display: Vec<i64> = vec![0i64; 1000];
        let mut display_out_timestamp = 0i64;
        // stores all requirements either as task or the resolved value.
        let tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>> = Arc::new(RwLock::new(HashMap::new()));

        let mut user_settings: UserSettings = load_user_settings("./terra-rust-bot.json");

        let (wallet_seed_phrase,wallet_acc_address) = get_wallet_details(&user_settings).await;

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
            /*
             * This loop has three major blocking elements.
             * 1) Awaiting running tasks if thread limit is reached. No harm of waiting here.
             * 2) Multiple calls of try_add_to_state, checks if results are available.
             *    Timeout after 0.1s, worst possible delay (unlikely to happen): req.len() * 0.1s. (~10s).
             * 3) Writing the state to disk. Acceptable delay, less than 1s.
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

                let now = Utc::now().timestamp();

                let mut offset: usize = 0;
                // initiate next batch of parallel tasks
                add_view_to_state(&state,requirements_next(now, num_cpus,&mut offset, &tasks, &user_settings, &wallet_acc_address).await).await;


                // trying to calculate whatever the bot needs to calculate based on the task (query) results,
                // if one task is slow, because the requirement is not yet resolved, it will timeout after 0.1s, so the loop can continue.

                if user_settings.terra_market_info {
                    try_calculate_promises(&state,&mut timestamps_display, now,display_market_info(&tasks, &state, &mut offset, is_first_run).await).await;
                }
                if user_settings.anchor_general_info {
                    try_calculate_promises(&state,&mut timestamps_display, now,display_anchor_info(&tasks, &state, &mut offset, is_first_run).await).await;
                }
                if user_settings.anchor_account_info {
                    try_calculate_promises(&state,&mut timestamps_display, now,display_anchor_account(&tasks, &state, &mut offset, is_first_run).await).await;
                }

                if user_settings.anchor_protocol_auto_stake {
                    // starts the agent specific function as task.
                    // (only if previous task of the same key has finished)
                    let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_stake_rewards(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                    try_run_function(&tasks, task, "anchor_auto_stake", user_settings.test).await;
                    // if resolved the task will be transformed into a maybe result at the end of the lazy_* method.
                    try_calculate_promises(&state,&mut timestamps_display, now,lazy_anchor_account_auto_stake_rewards(&tasks, &state, &mut offset, user_settings.test, is_first_run).await).await;
                    // also tries to calculate all state updates for terra-rust-bot-state.json.
                }
                if user_settings.anchor_protocol_auto_farm {
                    let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_farm_rewards(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                    try_run_function(&tasks, task, "anchor_auto_farm", user_settings.test).await;
                    try_calculate_promises(&state,&mut timestamps_display, now,lazy_anchor_account_auto_farm_rewards(&tasks, &state, &mut offset, user_settings.test, is_first_run).await).await;
                }
                if user_settings.anchor_protocol_auto_repay {
                    let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_redeem_and_repay_stable(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                    try_run_function(&tasks, task, "anchor_auto_repay", user_settings.test).await;
                    try_calculate_promises(&state,&mut timestamps_display, now,lazy_anchor_account_auto_repay(&tasks, &state, &mut offset, user_settings.test, is_first_run).await).await;
                }
                if user_settings.anchor_protocol_auto_borrow {
                    let task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anchor_borrow_and_deposit_stable(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(), user_settings.test));
                    try_run_function(&tasks, task, "anchor_auto_borrow", user_settings.test).await;
                    try_calculate_promises(&state,&mut timestamps_display, now,lazy_anchor_account_auto_borrow(&tasks, &state, &mut offset, user_settings.test, is_first_run).await).await;
                }

                display_all_logs(&tasks, &state, &mut offset).await;
                display_all_errors(&tasks, &state, &mut offset).await;

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
            abort_tasks(&tasks).await.ok();
            if user_settings.hot_reload {
                user_settings = load_user_settings("./terra-rust-bot.json");
            }
        }
}

async fn get_wallet_details(user_settings: &UserSettings) -> (Arc<SecUtf8>,Arc<SecUtf8>) {
    /* Get wallet details */
    let mut wallet_seed_phrase = SecUtf8::from("".to_string());
    let mut wallet_acc_address = SecUtf8::from(user_settings.terra_wallet_address.as_ref().unwrap_or(&"".to_string()));
    //  /^terra1[a-z0-9]{38}$/]

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
    (wallet_seed_phrase,wallet_acc_address)
}

async fn try_calculate_promises(state: &Arc<RwLock<Vec<Option<Entry>>>>,timestamps_display: &mut Vec<i64>,now: i64,maybe_futures:Vec<(usize,Pin<Box<dyn Future<Output=Maybe<String>>+Send>>)>) {
    for t in maybe_futures {
        if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
            try_add_to_state(&state, t.0, Box::pin(t.1)).await.ok();
            timestamps_display[t.0] = now;
        }
    }
}