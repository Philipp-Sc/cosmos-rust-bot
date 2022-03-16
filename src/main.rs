mod simple_user_input;
use simple_user_input::get_input; 

use terra_rust_api_layer::services::blockchain::smart_contracts::objects::meta::api::{get_from_account};

use terra_rust_bot_controller::control::view::{timestamp_now_to_string}; 
use terra_rust_bot_controller::control::view::interface::model::{UserSettings,MaybeOrPromise,requirements,get_keys_of_running_tasks,get_keys_of_failed_tasks,await_running_tasks,get_timestamps_of_resolved_tasks};
use terra_rust_bot_controller::control::view::interface::model::requirements::{my_requirement_keys, my_requirement_list};
use terra_rust_bot_controller::control::view::interface::model::wallet::{encrypt_text_with_secret,decrypt_text_with_secret}; 

mod logger;
use logger::logs::*;
use logger::errors::*; 

mod observer;
use observer::anchor::general::*;
use observer::anchor::account::*;
use observer::market::general::*;

mod agent;  
use agent::auto_repay::*;
use agent::auto_borrow::*;
use agent::auto_stake::*;
use agent::auto_farm::*; 

use display_utils::display::{add_string_to_display,try_add_to_display};

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

extern crate num_cpus;



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
                let anchor_auto_stake = lazy_anchor_account_auto_stake_rewards(&tasks, &wallet_acc_address, &wallet_seed_phrase, &new_display, &mut offset, is_test, is_first_run).await;
                for t in anchor_auto_stake {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 { 
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }  
                }                    
            }  
            if args_b.contains(&"anchor_auto_lp") {
                let anchor_auto_lp = lazy_anchor_account_auto_farm_rewards(&tasks, &wallet_acc_address, &wallet_seed_phrase, &new_display, &mut offset, is_test, is_first_run).await;
                for t in anchor_auto_lp {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 { 
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }  
                }                    
            }   

            if args_b.contains(&"anchor_auto_repay") {
                let anchor_auto_repay = lazy_anchor_account_auto_repay(&tasks, &wallet_acc_address, &wallet_seed_phrase, &new_display, &mut offset, is_test, is_first_run).await;
                for t in anchor_auto_repay {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 { 
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }  
                }  
                
            }   
            if args_b.contains(&"anchor_auto_borrow") {
                let anchor_auto_borrow = lazy_anchor_account_auto_borrow(&tasks, &wallet_acc_address, &wallet_seed_phrase, &new_display, &mut offset, is_test, is_first_run).await;
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