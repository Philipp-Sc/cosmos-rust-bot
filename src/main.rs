// entrypoint of the application
// user data/settings are loaded
// arguments read
// loop: 
//   -  display state
//   -  bot execution

#[macro_use]
extern crate litcrypt;
//https://github.com/anvie/litcrypt.rs
use_litcrypt!();
 

use terra_rust_api_layer::services::blockchain::smart_contracts::objects::meta::api::{get_from_account};

// Model
mod state;

use crate::state::control::model::{UserSettings,MaybeOrPromise,requirements,get_keys_of_running_tasks,get_keys_of_failed_tasks,await_running_tasks,get_timestamps_of_resolved_tasks};

use crate::state::control::model::requirements::{my_requirement_keys, my_requirement_list};

use crate::state::control::model::wallet::{encrypt_text_with_secret,decrypt_text_with_secret}; 

use crate::state::control::try_run_function;

// View -> Model (Read Data)
mod view;
use crate::view::{timestamp_now_to_string}; 

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
 
use ui::display::{add_string_to_display,try_add_to_display};

use std::env;
use secstr::*;

use rust_decimal::Decimal;
use core::str::FromStr; 

use std::collections::HashMap; 
use std::time::{Duration};

use std::sync::Arc; 
use tokio::sync::RwLock;  
use tokio::time::timeout; 

use colored::*;
 
use chrono::{Utc};
use std::fs;


use core::pin::Pin;
use core::future::Future;

extern crate num_cpus;


// TODOS NOW:

// clean up: bot/action (multiple modules, separation into utility functions)

// clean up: view/mod.rs (multiple modules, separation into utility functions)

// clean up info/..  reduce code, 

// find out why auto_farm fails.


// NOTE NEW DEV FEATURE:
// watch all APIs, notfiy if one fails -> EMAIL



// TODO: Error handling. Every Unwrapp needs to be inspected.
// Some unwraps panic if the request fail or return an error.
// To be fixed, but not urgent, since only the task panics, which has no bad side effect.  

// TODO: Add UST peg stat.
// TODO: Add config for usersettings
  

// TODO: Auto Replenish: Always get the account balance a good bit above the limit.
// TODO: Anchor Liquidation Bot


// TODO: Optimize TX Fee estimate query functions. !! (will reduce query time)


// TODO: Instead of current display implementation write out a JSON file.
// TODO: Then to display this file write a different programm.
// TODO: Long term this will be easier to maintain, and make it much easier to write forks or extensions.


 #[tokio::main]
async fn main() -> anyhow::Result<()> {

        /* Load user settings */
        let mut terra_rust_bot_json_loaded = "terra-rust-bot.json not loaded";

        let mut user_settings: UserSettings = UserSettings {
            trigger_percentage: Decimal::from_str("0.9").unwrap(),  
            target_percentage: Decimal::from_str("0.72").unwrap(),   
            borrow_percentage: Decimal::from_str("0.5").unwrap(),   
            max_tx_fee: Decimal::from_str("5").unwrap(),
            max_gas_adjustment: Decimal::from_str("1.67").unwrap(),
            gas_adjustment_preference: Decimal::from_str("1.2").unwrap(),
            min_ust_balance: Decimal::from_str("10").unwrap(),   
            ust_balance_preference: Decimal::from_str("20").unwrap(),
        };

        match fs::read_to_string("./terra-rust-bot.json") {
            Ok(file) => {
                user_settings = match serde_json::from_str(&file) {
                    Ok(res) => {
                        terra_rust_bot_json_loaded="terra-rust-bot.json loaded.";
                        res
                    },
                    Err(err) => {println!("{:?}",err);user_settings}
                }
            },
            Err(err) => {
                println!("{:?}",err);
                // use hard coded values.
            }
        }
        /* Load arguments */

        let args: Vec<String> = env::args().collect();

        let mut args_i: Vec<&str> = Vec::new();
        let mut args_a: Vec<&str> = Vec::new();
        let mut args_b: Vec<&str> = Vec::new();
        let mut args_d: Vec<&str> = Vec::new();
        let mut arg_w: String = "".to_string();

        let mut is_test = false;
        let mut is_debug = false;

        let mut last_item = 0;
        for x in 1..args.len() {
            if &args[x] == "-i" || &args[x] == "-a" || &args[x] == "-b" ||  &args[x] == "-d" || &args[x] == "-w" {
                last_item = x;
            }else{
                if &args[last_item] == "-w" {
                    arg_w = format!("{}",&args[x]);
                }
                if &args[last_item] == "-i" {
                    args_i.push(&args[x]);
                }
                if &args[last_item] == "-a" {
                    args_a.push(&args[x]);
                }  
                if &args[last_item] == "-b" {
                    args_b.push(&args[x]);
                }  
                if &args[last_item] == "-d" {
                    if &args[x] == "test" {
                        is_test = true;
                    }
                    if &args[x] == "dev" {
                        is_debug = true;
                    }
                    args_d.push(&args[x]);

                }
            }
        } 
        /* Get wallet details */

        let mut wallet_seed_phrase = SecUtf8::from("".to_string());
        let mut wallet_acc_address = SecUtf8::from(arg_w);

        if args_b.len() > 0 {                              
            // ** seed phrase needed **
            wallet_seed_phrase = encrypt_text_with_secret(get_input("Enter your seed phrase (press Enter to skip):").to_string());
            if wallet_acc_address.unsecure().len()!=44 || !is_test {
                wallet_acc_address = SecUtf8::from(get_from_account(&decrypt_text_with_secret(&wallet_seed_phrase)).unwrap_or("".to_string()));
            }
        }else if wallet_acc_address.unsecure().len()==0 { 
            // ** maybe need wallet address **
            if args_a.len() > 0 || args_b.len() > 0 { // yes.
                    wallet_acc_address = SecUtf8::from(get_input("Enter your wallet address (press Enter to skip):").to_string());
            } 
        }
        println!("{esc}c", esc = 27 as char); 

        // Arc allows multiple references to the same object,
        // to potentially spawn multiple tasks with access to the seed phrase, while not revealing the string.
        let wallet_seed_phrase = Arc::new(wallet_seed_phrase);
        let wallet_acc_address = Arc::new(wallet_acc_address);


        let req: Vec<(&'static str, i32, Vec<&'static str>)> = my_requirement_list(&args);
        let req_keys: Vec<&str> = my_requirement_keys(&args);  

        let mut req_keys_status = req_keys.clone();
        for bot_tasks in &args_b {
            req_keys_status.push(bot_tasks);
        } 

        /* Display */
        // object that stores the terminal output
        let display_slots = 1000;
        let new_display: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec!["".to_string(); display_slots])); 
        // using timestamps to update each slot with a short delay.
        let mut timestamps_display: Vec<i64> = vec![0i64; display_slots];
        let mut display_out_timestamp = 0i64;

        add_string_to_display(&new_display, 0, format!("{}\n\n",terra_rust_bot_json_loaded.truecolor(77, 77, 237))).await.ok();
        
        let num_cpus = num_cpus::get();

        /* Tasks */
        // stores all requirements either as task or the resolved value.
        let tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>> = Arc::new(RwLock::new(HashMap::new())); 

        let mut is_first_run: bool = true;

        /*
         * This loop has three major blocking elements.
         * 1) Awaiting running tasks if thread limit is reached. No harm of waiting here.
         * 2) Multiple calls of try_add_to_display, checks if results are available. 
         *    Timeout after 0.1s, worst possible delay (unlikely to happen): req.len() * 0.1s. (~10s).
         * 3) Writing the display to disk. Acceptable delay, less than 1s.
         * 
         * */
        loop {  

            let req_unresolved = get_keys_of_running_tasks(&tasks,&req_keys_status).await;
            let req_failed = get_keys_of_failed_tasks(&tasks, &req_keys_status).await;

            // waiting for unresolved tasks to catch up 
            if req_unresolved.len() >= num_cpus { 
                // anyway we need to have free threads to spawn more tasks
                // useful to wait here
                timeout(Duration::from_secs(30), await_running_tasks(&tasks, &req_keys)).await.ok();
            } 

            let req_resolved_timestamps = get_timestamps_of_resolved_tasks(&tasks,&req_keys).await;
            let now = Utc::now().timestamp();

            let mut req_to_update: Vec<&str> = Vec::new(); 
            for i in 0..req.len() {
                if req_to_update.len()==num_cpus {
                    break;
                }
                let mut contains = false;
                for x in &args {
                    if req[i].2.contains(&x.as_str()) {
                        contains = true;
                        break;
                    }
                }  
                if contains && !req_unresolved.contains(&req[i].0) && (req_failed.contains(&req[i].0) || req_resolved_timestamps[i]==0i64 || ((now - req_resolved_timestamps[i]) > req[i].1 as i64 )) { // unresolved requirements will not be refreshed.
                    req_to_update.push(req[i].0); 
                }
                
            } 

           if is_debug {
               add_string_to_display(&new_display,1,format!(
                    "{}{}{}{}{}{}{}{}{}\n\n{}\n{}\n{}\n\n",
                    timestamp_now_to_string().yellow(),
                    " -  failed: ".purple(), 
                    req_failed.len().to_string().red(),
                    ", pending: ".purple(),
                    req_unresolved.len().to_string().yellow(),
                    ", waiting: ".purple(),
                    req_to_update.len().to_string().purple(),
                    ", total requirements: ".to_string().purple(),
                    req_keys.len().to_string().purple(),
                    format!("{:?}\n",req_failed).to_string().red(),
                    format!("{:?}\n",req_unresolved).to_string().yellow(),
                    format!("{:?}",req_to_update).to_string().purple()
                    )).await.ok(); 
            }else{
                add_string_to_display(&new_display,1,format!(
                    "{}{}{}{}{}{}{}{}{}\n\n",
                    timestamp_now_to_string().yellow(),
                    " -  failed: ".purple(), 
                    req_failed.len().to_string().red(),
                    ", pending: ".purple(),
                    req_unresolved.len().to_string().yellow(),
                    ", waiting: ".purple(),
                    req_to_update.len().to_string().purple(),
                    ", total requirements: ".to_string().purple(),
                    req_keys.len().to_string().purple()
                    )).await.ok(); 
            }

            requirements(&tasks,&user_settings,&wallet_acc_address,&req_to_update).await;  
            // instead of calculating what req should be updated here, it should be part of _memory
            // so here only requirements_next() needs to be called.
             
            let mut offset: usize = 2;

            // waiting for all open **display** updates.
            // if one task is slow, because the requirement is not yet resolved, it slows down the whole loop, 
            // therefore it will timeout after 0.1s, so the loop can continue.  
 
            if args_i.contains(&"market") {        
                for t in display_market_info(&tasks, &new_display, &mut offset, is_first_run).await {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }
                }
            }
            if args_i.contains(&"anchor") {        
                for t in display_anchor_info(&tasks, &new_display, &mut offset, is_first_run).await {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    } 
                }
            }
            if args_a.contains(&"anchor_account") {        
                for t in display_anchor_account(&tasks, &new_display, &mut offset, is_first_run).await {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 {
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    } 
                }
            }

            if args_b.contains(&"anchor_auto_stake") {

                // starts the bot specific function as task.
                // (only if previous task of the same key has finished)
                let task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_stake_rewards(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(),is_test));
                try_run_function(&tasks,task,"anchor_auto_stake",is_test).await;  
   
                // checks if data for the display is available
                let anchor_auto_stake = lazy_anchor_account_auto_stake_rewards(&tasks, &new_display, &mut offset, is_test, is_first_run).await;
                for t in anchor_auto_stake {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 { 
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }  
                }                    
            }  
            if args_b.contains(&"anchor_auto_lp") {

                let task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_farm_rewards(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(),is_test));
                try_run_function(&tasks,task,"anchor_auto_farm",is_test).await;  
      
                let anchor_auto_lp = lazy_anchor_account_auto_farm_rewards(&tasks, &new_display, &mut offset, is_test, is_first_run).await;
                for t in anchor_auto_lp {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 { 
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }  
                }                    
            }   

            if args_b.contains(&"anchor_auto_repay") {

                let task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_redeem_and_repay_stable(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(),is_test));
                try_run_function(&tasks,task,"anchor_auto_repay",is_test).await;  

                let anchor_auto_repay = lazy_anchor_account_auto_repay(&tasks, &new_display, &mut offset, is_test, is_first_run).await;
                for t in anchor_auto_repay {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 { 
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }  
                }  
                
            }   
            if args_b.contains(&"anchor_auto_borrow") {

                let task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_borrow_and_deposit_stable(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(),is_test));
                try_run_function(&tasks,task,"anchor_auto_borrow",is_test).await;  
    
                let anchor_auto_borrow = lazy_anchor_account_auto_borrow(&tasks, &new_display, &mut offset, is_test, is_first_run).await;
                for t in anchor_auto_borrow {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 { 
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }  
                }  
            }

            display_all_logs(&tasks ,&new_display, &mut offset, &args_b).await;
            
            display_all_errors(&tasks, &*req_unresolved ,&new_display, &mut offset).await;

            if is_first_run {
                is_first_run = false;
            }
            
            // ensuring one file write per 100ms, not faster.
            let now = Utc::now().timestamp_millis();

            if display_out_timestamp== 0i64 || now - display_out_timestamp > 100i64 {
                // writing display to file.
                let new_line = format!("{esc}c", esc = 27 as char);
                let line = format!("{}{}",new_line,new_display.read().await.join(""));
                fs::write("./terra-rust-bot-display.txt", &line).ok();  
                display_out_timestamp = now;        
            }
        } 
} 