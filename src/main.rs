use secstr::*;

use std::env;

use rust_decimal::Decimal;
use core::str::FromStr;
//use std::convert::TryFrom;

mod control;

use control::view::interface::model::{UserSettings,MaybeOrPromise,requirements,try_register_function,await_function,get_keys_of_running_tasks,get_keys_of_failed_tasks,await_running_tasks,get_timestamps_of_resolved_tasks};
 
use control::view::interface::model::services::blockchain::smart_contracts::objects::meta::api::{get_from_account};

use control::view::*;
use control::view::interface::*;
use control::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

//use anyhow::anyhow;
//use enum_as_inner::EnumAsInner; 
//use num_format::{Locale, ToFormattedString}; 

use std::{thread, time};
use std::time::{Duration/*, Instant*/};



use std::sync::Arc; 
use tokio::sync::RwLock; 
use tokio::task::JoinHandle;
use tokio::time::timeout;
//use tokio::time::Timeout;


use colored::*;
 
use simple_user_input::get_input; 

//use rand::Rng;


use chrono::{Utc};
use std::fs;

extern crate num_cpus;

//use std::collections::HashSet; 

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}  

// TODO: Error handling. Every Unwrapp needs to be inspected.
// Some unwraps panic if the request fail or return an error.
// To be fixed, but not urgent, since only the task panics, which has no bad side effect.  

// TODO: Add UST peg stat.
// TODO: Add config for usersettings
  

// TODO: Auto Replenish: Always get the account balance a good bit above the limit.
// TODO: Anchor Liquidation Bot


// TODO: Optimize TX Fee estimate query functions. !! (will reduce query time)


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
            wallet_acc_address: "".to_string(),  
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
        // println!("{:?}", args);
        //./target/debug/terra-rust-bot -i market anchor -a anchor_account -b anchor_auto_stake anchor_auto_repay -d test dev
        
        let mut args_i: Vec<&str> = Vec::new();
        let mut args_a: Vec<&str> = Vec::new();
        let mut args_b: Vec<&str> = Vec::new();
        let mut args_d: Vec<&str> = Vec::new();

        let mut is_test = false;
        let mut is_debug = false;

        let mut last_item = 0;
        for x in 1..args.len() {
            if &args[x] == "-i" || &args[x] == "-a" || &args[x] == "-b" ||  &args[x] == "-d" || &args[x] == "-w" {
                last_item = x;
            }else{
                if &args[last_item] == "-w" {
                    user_settings.wallet_acc_address = format!("{}",&args[x]);
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

        //println!("{:?}",(args_i,args_a,args_b,args_d));
        let is_test = *&is_test;
        let is_debug = *&is_debug;

        /* Get wallet details */

        let mut wallet_seed_phrase = SecUtf8::from("".to_string());

        if args_b.len() > 0 { // ** seed phrase needed **
            wallet_seed_phrase = SecUtf8::from(get_input("Enter your seed phrase (press Enter to skip):").to_string());
            // https://github.com/unrelentingtech/secstr
            println!("{esc}c", esc = 27 as char);  
            if wallet_seed_phrase.unsecure().len()>1 {
                user_settings.wallet_acc_address = get_from_account(wallet_seed_phrase.unsecure()).unwrap_or("".to_string());
            }
        }else if user_settings.wallet_acc_address.len()==0 { /* ask for wallet address */
            if args_a.len() > 0 || args_b.len() > 0 { // if wallet address is needed.
                    user_settings.wallet_acc_address = get_input("Enter your wallet address (press Enter to skip):").to_string();
                    println!("{esc}c", esc = 27 as char); 
            } 
        }

        // Arc allows multiple references to the same object,
        // to potentially spawn multiple tasks with access to the seed phrase, while not revealing the string.
        let wallet_seed_phrase = Arc::new(wallet_seed_phrase);
 
        // note: around every 6s a new block is generated. 
        let fast: i32 = 10;      // 10s for short lived information
        let medium: i32 = 60;    // 1m  for short lived information
        let slow: i32 = 60*10;   // 10m for relative constant information. 

        // (key, target_refresh_time, dependency_tag)
        let req = vec![
        ("terra_balances", fast, vec!["anchor_auto_stake","anchor_auto_repay","anchor_auto_borrow"]),
        /* <market_info> */
        /* core_tokens */
        ("core_swap uusd usdr", fast, vec!["market"]),
        ("core_swap usdr uluna", fast, vec!["market"]),
        ("core_swap uluna uusd", fast, vec!["market"]),
        // "simulation terraswap usdr usdr_uluna_pair_contract",
        // "simulation terraswap uluna uusd_uluna_pair_contract",
        /* anchor_tokens */
        ("simulation anchorprotocol uluna terraswapblunaLunaPair",fast, vec!["market","anchor_account"]),
        ("state anchorprotocol bLunaHub", fast, vec!["market","anchor_account"]),
        ("simulation_cw20 anchorprotocol ANC terraswapAncUstPair", fast, vec!["market","anchor_account","anchor_auto_stake"]),
        ("epoch_state anchorprotocol mmMarket", fast, vec!["anchor","market","anchor_account","anchor_auto_repay"]),
        /* nexus_tokens */
        ("simulation_cw20 nexusprotocol nLunaToken Psi-nLuna_Pair", fast, vec!["market"]),
        ("simulation_cw20 nexusprotocol PsiToken Psi-UST_Pair", fast, vec!["market"]),
        /* mirror_tokens */
        ("simulation_cw20 uusd mir", fast, vec!["market"]),
        ("simulation_cw20 uusd m_tsla", fast, vec!["market"]),
        ("simulation_cw20 uusd m_btc", fast, vec!["market"]),
        ("simulation_cw20 uusd m_eth", fast, vec!["market"]),
        ("simulation_cw20 uusd m_spy", fast, vec!["market"]),
        /* <other> */
        /* <anchor_protocol> */
        ("state anchorprotocol mmMarket", fast, vec!["anchor","anchor_account"]),
        ("api/v2/distribution-apy", fast, vec!["anchor","anchor_account","anchor_auto_stake"]),
        ("api/v2/gov-reward", fast, vec!["anchor","anchor_account","anchor_auto_stake"]),
        ("config anchorprotocol mmInterestModel", fast, vec!["anchor","anchor_account"]),
        //("config anchorprotocol collector",every_minute),
        /* <anchor_protocol account> */ 
        ("borrow_limit", fast, vec!["anchor_account","anchor_auto_stake","anchor_auto_repay","anchor_auto_borrow"]),
        ("borrow_info", fast, vec!["anchor_account","anchor_auto_stake","anchor_auto_repay","anchor_auto_borrow"]),
        ("balance", fast, vec!["anchor_account","anchor_auto_repay","anchor_auto_borrow"]),
        ("anc_balance", fast, vec!["anchor_account","anchor_auto_stake"]),
        ("staker", fast, vec!["anchor_account","anchor_auto_stake"]),
        ("blocks_per_year", slow, vec!["market","anchor","anchor_account"]), 
        ("earn_apy", slow, vec!["anchor","anchor_account"]),
        /* <meta data> */ 
        ("anchor_protocol_txs_claim_rewards", slow, vec!["anchor","anchor_account","anchor_auto_stake"]), 
        ("anchor_protocol_txs_staking", slow, vec!["anchor","anchor_account","anchor_auto_stake"]), 
        ("anchor_protocol_txs_redeem_stable", slow, vec!["anchor_auto_repay"]), 
        ("anchor_protocol_txs_deposit_stable", slow, vec!["anchor_auto_borrow"]), 
        ("anchor_protocol_txs_borrow_stable", slow, vec!["anchor_auto_borrow"]), 
        ("anchor_protocol_txs_repay_stable", slow, vec!["anchor_auto_repay"]), 
        ("trigger_percentage", fast, vec!["anchor_account","anchor_auto_repay"]),
        /* <from settings> */ 
        ("target_percentage", fast, vec!["anchor_auto_repay","anchor_auto_borrow"]),
        ("borrow_percentage", fast, vec!["anchor_auto_borrow"]),
        ("max_gas_adjustment", fast, vec!["anchor_account","anchor_auto_stake","anchor_auto_repay","anchor_auto_borrow"]),
        ("gas_adjustment_preference",fast, vec!["anchor_account","anchor_auto_stake","anchor_auto_repay","anchor_auto_borrow"]),
        ("min_ust_balance", fast, vec!["anchor_account","anchor_auto_stake","anchor_auto_repay","anchor_auto_borrow"]),
        ("max_tx_fee", fast, vec!["anchor_auto_stake","anchor_auto_repay","anchor_auto_borrow"]),
        /* <from gas_prices>*/
        ("gas_fees_uusd", medium, vec!["market","anchor","anchor_account","anchor_auto_stake","anchor_auto_repay","anchor_auto_borrow"]),
        ("tax_rate", medium, vec!["anchor_auto_repay","anchor_auto_borrow"]),
        ("tax_caps", medium, vec!["anchor_auto_repay","anchor_auto_borrow"]),
        ]; 
        let mut req_new = Vec::new();
        let mut req_keys: Vec<&str> = Vec::new();  
        for i in 0..req.len() {
            for x in &args {
                if req[i].2.contains(&x.as_str()) {
                    req_keys.push(req[i].0);
                    req_new.push(&req[i]);
                    break;
                }
            }
        }
        let mut req_keys_status = req_keys.clone();
        for bot_tasks in &args_b {
            req_keys_status.push(bot_tasks);
        }

        let req_keys_status = &*req_keys_status;
        let req_keys = &*req_keys; 
        let req = &*req_new;


        /* Display */
        // object that stores the terminal output
        let display_slots = 1000;
        let new_display: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec!["".to_string(); display_slots])); 
        // using timestamps to update each slot with a short delay.
        let mut timestamps_display: Vec<i64> = vec![0i64; display_slots];

        add_string_to_display(&new_display, 0, format!("{}\n\n",terra_rust_bot_json_loaded.truecolor(77, 77, 237))).await.ok();
        
        let num_cpus = num_cpus::get();

        /* Tasks */
        // stores all requirements either as task or the resolved value.
        let tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>> = Arc::new(RwLock::new(HashMap::new())); 

        let mut is_first_run: bool = true;

        /**
         * This loop has three major blocking elements.
         * 1) Awaiting running tasks if thread limit is reached. No harm of waiting here.
         * 2) Multiple calls of try_add_to_display, checks if results are available. 
         *    Timeout after 0.1s, worst possible delay (unlikely to happen): req.len() * 0.1s. (~10s).
         * 3) Writing the display to disk. Acceptable delay, less than 1s.
         * 
         * */
        loop { 
            let time_yay = Utc::now().timestamp();

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

            requirements(&tasks,&user_settings,&req_to_update).await;  
             
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
                let anchor_auto_stake = lazy_anchor_account_auto_stake_rewards(&tasks, &wallet_seed_phrase, &new_display, &mut offset, is_test, is_first_run).await;
                for t in anchor_auto_stake {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 { 
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }  
                }                    
            }  

            if args_b.contains(&"anchor_auto_repay") {
                let anchor_auto_repay = lazy_anchor_account_auto_repay(&tasks, &wallet_seed_phrase, &new_display, &mut offset, is_test, is_first_run).await;
                for t in anchor_auto_repay {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 { 
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }  
                }  
                
            }   
            if args_b.contains(&"anchor_auto_borrow") {
                let anchor_auto_borrow = lazy_anchor_account_auto_borrow(&tasks, &wallet_seed_phrase, &new_display, &mut offset, is_test, is_first_run).await;
                for t in anchor_auto_borrow {
                    if timestamps_display[t.0] == 0i64 || now - timestamps_display[t.0] > 1i64 { 
                        try_add_to_display(&new_display,t.0,Box::pin(t.1)).await.ok();
                        timestamps_display[t.0] = now;
                    }  
                }  
                
            }   
            display_all_logs(&tasks ,&new_display, &mut offset, &args_b).await;
            
            display_all_errors(&tasks, &*req_unresolved ,&new_display, &mut offset).await;
            
            // todo: can write display to a log file.  

            if is_first_run {
                is_first_run = false;
            }
            
            // writing display to file.
            let new_line = format!("{esc}c", esc = 27 as char);
            let line = format!("{}{}",new_line,new_display.read().await.join(""));
            fs::write("./terra-rust-bot-display.txt", &line).ok(); 
                 

            let time_end_yay = Utc::now().timestamp();
            println!("{}", time_end_yay-time_yay);
         
        }
 
        //Ok(())
  
} 

pub fn display_add(item: String, fixed_len: usize, new_lines: usize) -> String {

    let split = item.split("    ");
    let mut result = "".to_string();

    for s in split {
        if s.len() > 0 {
            if s.len() <= fixed_len {
                let space = fixed_len - s.len();
                result = format!("{}{}{}",result,s," ".repeat(space));
            }else{
                result = format!("{}{}", result,s);
            }
        }
    }

    result = format!("{}{}",result,"\n".repeat(new_lines));
    result
}

pub async fn add_table_formatting(f: Pin<Box<dyn Future<Output = String> + Send + 'static >>, fixed_len: usize, new_lines: usize) -> String {
    let res = f.await;
    let split = res.split("    ");
    let mut result = "".to_string();

    for s in split {
        if s.len() > 0 {
            if s.len() <= fixed_len {
                let space = fixed_len - s.len();
                result = format!("{}{}{}",result,s," ".repeat(space));
            }else{
                result = format!("{}{}", result,s);
            }
        }
    }

    result = format!("{}{}",result,"\n".repeat(new_lines));
    result
}

pub async fn add_string_to_display(new_display: &Arc<RwLock<Vec<String>>>, index: usize, line: String) -> anyhow::Result<()> {
    
    let mut look = new_display.try_write();
    while look.is_err() {
        thread::sleep(time::Duration::from_millis(10));
        look = new_display.try_write();
    } 
    let mut vector = look.unwrap();
    *vector.get_mut(index).unwrap() = line;
    Ok(())
}

pub async fn add_view_to_display(new_display: &Arc<RwLock<Vec<String>>>, view: Vec<(String,usize)>) {
    let mut vector = new_display.write().await;
    for entry in view {
        *vector.get_mut(entry.1).unwrap() = entry.0;
    }
}

pub async fn add_format_to_result(prefix: String,suffix: String, f: Pin<Box<dyn Future<Output = String> + Send + 'static >>) -> String {
    return format!("{}{}{}",prefix,f.await,suffix);
}

pub async fn add_to_display(new_display: &Arc<RwLock<Vec<String>>>, index: usize, result: Option<String>) -> anyhow::Result<()> {
    
    if let Some(succ) = result {
        let mut vector =  new_display.write().await;
        *vector.get_mut(index).unwrap() = format!("{}",succ.truecolor(77, 77, 237));
    }
    Ok(())
}

pub async fn try_add_to_display(new_display: &Arc<RwLock<Vec<String>>>, index: usize, f: Pin<Box<dyn Future<Output = String> + Send + 'static >>) -> anyhow::Result<()> {
    
    let result = timeout(Duration::from_millis(100), f).await;   
    add_to_display(new_display,index,result.ok()).await
}

pub async fn display_all_logs(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize, args_b: &Vec<&str>) {
   
    if args_b.len() == 0 {
        return;
    }

    let mut log_view: Vec<(String,usize)> = Vec::new();

    log_view.push(("\n\n  **Logs**\n\n".yellow().to_string(),*offset));
    *offset += 1;
   
    
    if args_b.contains(&"anchor_auto_repay") {

        let auto_repay = get_past_transaction_logs(tasks.clone(),"anchor_redeem_and_repay_stable").await;

        log_view.push((format!("{}{}","\n   [Auto Repay]            ".yellow(), auto_repay.yellow()),*offset));
        *offset += 1;

    }
    if args_b.contains(&"anchor_auto_borrow") {

        let auto_borrow = get_past_transaction_logs(tasks.clone(),"anchor_borrow_and_deposit_stable").await;

        log_view.push((format!("{}{}","\n   [Auto Borrow]           ".yellow(), auto_borrow.yellow()),*offset));
        *offset += 1;

    }

    if args_b.contains(&"anchor_auto_stake") {

        let auto_stake = get_past_transaction_logs(tasks.clone(),"anchor_governance_claim_and_stake").await;

        log_view.push((format!("{}{}","\n   [Auto Stake]            ".yellow(), auto_stake.yellow()),*offset)); 
        *offset += 1;

    }
    log_view.push(("\n".to_string(),*offset));
    *offset += 1;

    add_view_to_display(&new_display, log_view).await; 
}


pub async fn display_all_errors(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &[&str], new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize) {
   
    let mut error_view: Vec<(String,usize)> = Vec::new();

    error_view.push(("\n\n  **Errors**\n\n".red().to_string(),*offset));
    *offset += 1;
  
    // clear the previous error messages. 
    for x in *offset..new_display.read().await.len(){
        error_view.push(("".to_string(),x));
    }

    let mut error_count = 0;
    for key in req {
        match anything_to_err(tasks.clone(),key).await.as_ref() {
            "--" => {
            },
            e => {
                if !e.contains("Info: Key '"){
                    error_count = error_count +1;
                    error_view.push((format!("\n   [Key] '{}'\n   {}\n",key,e).yellow().to_string(),*offset));
                    *offset += 1; 
                }
            }
        } 
    }
    if error_count == 0 {
        error_view.push(("\n   None \n\n".red().to_string(),*offset)); 
        *offset += 1; 
    }

    add_view_to_display(&new_display, error_view).await; 
}
/**
 * Anchor Auto Repay requires that the account balance has sufficient funds.
 * Anchor Auto Repay tries to be net neutral regarding the UST account balance.
 * Info: In some edge cases the account balance still can fall bellow the limit.
 * 
 * All the requirements specified by the tag "anchor_auto_repay" are mostly loaded concurrently.
 * Each requirement has it's own refresh time (fast=10s) 
 * There are no guarantees that they resolve in time (in 10s).
 * Anchor Auto Repay non blocking waits for unresolved requirements to be fulfilled.
 * 
 * */

 pub async fn lazy_anchor_account_auto_repay(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_seed_phrase: &Arc<SecUtf8>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)>  {

    let mut anchor_view: Vec<(String,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol Auto Repay**\n\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Repay]".truecolor(75,219,75),"                    left to trigger:          ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(check_anchor_loan_status(tasks.clone(),"repay",2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((format!("{}{}","\n   [Auto Repay]".truecolor(75,219,75),"                    to repay:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_amount(tasks.clone(),"repay",false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

/*
    let available_to_repay = min_ust_balance_to_string(tasks.clone(),false,2).await;
    add_string_to_display(new_display,*offset,format!("{}{}","\n\n\n   [Auto Repay UST]                account limit:                   ".truecolor(75,219,75), format!("{} UST",available_to_repay).yellow())).await.ok(); 
    *offset += 1;
 
    let available_to_repay = calculate_repay_plan(tasks.clone(),"ust_available_to_repay",2).await;
    add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Repay UST]                available UST:                   ".truecolor(75,219,75), format!("{} UST",available_to_repay).yellow())).await.ok(); 
    *offset += 1;
*/
    anchor_view.push((format!("{}{}","\n\n   [Auto Repay UST]".truecolor(75,219,75),"                amount:                   ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"to_withdraw_from_account",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

/*
    let available_in_deposit = calculate_repay_plan(tasks.clone(),"available_in_deposit",2).await;
    add_string_to_display(new_display,*offset,format!("{}{}","\n\n   [Auto Repay Redeem]         max amount:                      ".truecolor(75,219,75), format!("{} UST",available_in_deposit).yellow())).await.ok(); 
    *offset += 1;
*/
    anchor_view.push((format!("{}{}","\n   [Auto Repay Redeem]".truecolor(75,219,75),"             amount:                   ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"to_withdraw_from_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // does include gas_adjustment 
    anchor_view.push((format!("{}{}","\n   [Auto Repay Redeem]".truecolor(75,219,75),"             est. fee:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_redeem_stable","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n\n   [Auto Repay]".truecolor(75,219,75),"                    repay:                    ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_redeem_stable","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // does include gas_adjustment
    anchor_view.push((format!("{}{}","\n   [Auto Repay]".truecolor(75,219,75),"                    est. fee:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_repay_stable","avg_fee_amount_adjusted_without_stability_fee".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n   [Auto Repay]".truecolor(75,219,75),"                    est. stability fee:       ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"stability_tax",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Repay Transaction]".truecolor(75,219,75),"        amount:                   ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"total_amount",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // total fee
    anchor_view.push((format!("{}{}","\n   [Auto Repay Transaction]".truecolor(75,219,75),"        est. fee:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_auto_repay_tx_fee(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    let mut field = "result:  ";

    if is_test {
        field = "estimate:";
    }

    anchor_view.push((format!("{}{}","\n\n   [Auto Repay]".truecolor(75,219,75),format!("                    {}                 ",field.purple())),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    
    // function able to execute auto repay, therefore registering it as task to run concurrently. 
    let important_task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_redeem_and_repay_stable(tasks.clone(), wallet_seed_phrase.clone(),is_test));
    let timeout_duration = 120u64;  /* if task hangs for some reason (awaiting data, performaing estimate, broadcasting transaction) then timeout */
    
    let mut block_duration_after_resolve = 1i64;
    /* a small duration is optimal, since the data is already there */
    /* only issue is if there just was a transaction, this is handled by ensuring that the relevant data is recent enough.*/

    if is_test {
        // each call executes an estimate, therefore have higher delay to not spam estimates.
        // since test mode does not perform transactions, there is no downside by doing this.
        block_duration_after_resolve = 30i64;
    }
    try_register_function(&tasks,"anchor_auto_repay".to_owned(),important_task,timeout_duration, block_duration_after_resolve).await;  
          
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_repay".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push(("\n\n".to_string(),*offset));
    *offset += 1;

    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }



    return anchor_tasks;
 
}


 pub async fn lazy_anchor_account_auto_borrow(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_seed_phrase: &Arc<SecUtf8>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)>  {

    let mut anchor_view: Vec<(String,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol Auto Borrow**\n\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow]".truecolor(75,219,75),"                    left to trigger:          ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(check_anchor_loan_status(tasks.clone(),"borrow",2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((format!("{}{}","\n   [Auto Borrow]".truecolor(75,219,75),"                    to borrow:                ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_amount(tasks.clone(),"borrow",false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // does include gas_adjustment
    anchor_view.push((format!("{}{}","\n   [Auto Borrow]".truecolor(75,219,75),"                    est. fee:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_borrow_stable","avg_fee_amount_adjusted_without_stability_fee".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
 
     // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n   [Auto Borrow]".truecolor(75,219,75),"                    est. stability fee:       ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"stability_tax_borrow",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow]".truecolor(75,219,75),"                    to deposit:               ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"to_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // does include gas_adjustment
    anchor_view.push((format!("{}{}","\n   [Auto Borrow Deposit]".truecolor(75,219,75),"            est. fee:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_deposit_stable","avg_fee_amount_adjusted_without_stability_fee".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n   [Auto Borrow Deposit]".truecolor(75,219,75),"            est. stability fee:       ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"stability_tax_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // total fee
    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow Transaction]".truecolor(75,219,75),"        est. fee:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_auto_borrow_tx_fee(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

 
  
    let mut field = "result:  ";

    if is_test {
        field = "estimate:";
    }

    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow]".truecolor(75,219,75),format!("                    {}                 ",field.purple())),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    
    // function able to execute auto repay, therefore registering it as task to run concurrently. 
    let important_task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_borrow_and_deposit_stable(tasks.clone(), wallet_seed_phrase.clone(),is_test));
    let timeout_duration = 120u64;  /* if task hangs for some reason (awaiting data, performaing estimate, broadcasting transaction) then timeout */
    
    let mut block_duration_after_resolve = 1i64;
    /* a small duration is optimal, since the data is already there */
    /* only issue is if there just was a transaction, this is handled by ensuring that the relevant data is recent enough.*/

    if is_test {
        // each call executes an estimate, therefore have higher delay to not spam estimates.
        // since test mode does not perform transactions, there is no downside by doing this.
        block_duration_after_resolve = 30i64;
    }
    try_register_function(&tasks,"anchor_auto_borrow".to_owned(),important_task,timeout_duration, block_duration_after_resolve).await;  
          
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_borrow".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;
   

    anchor_view.push(("\n\n".to_string(),*offset));
    *offset += 1;


    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }


    return anchor_tasks;
 
}

/**
 * Anchor Auto Stake requires that the account balance has sufficient funds.
 * Info: It will not replenish the account balance. 
 * */
pub async fn lazy_anchor_account_auto_stake_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_seed_phrase: &Arc<SecUtf8>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {
     
    let mut anchor_view: Vec<(String,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();


    anchor_view.push(("\n  **Anchor Protocol Auto Stake**\n\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Stake UST]".truecolor(75,219,75),"         balance:           ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(terra_balance_to_string(tasks.clone(),"uusd",false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Stake Gov]".truecolor(75,219,75),"         balance:           ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_anc_deposited_to_string(tasks.clone(),false,4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" ANC".purple().to_string(),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Auto Stake Gov]".truecolor(75,219,75),"         staked:            ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anc_staked_balance_in_ust_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST  (=".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anc_staked_balance_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" ANC)\n".purple().to_string(),*offset));
    *offset += 1;
    
    anchor_view.push((format!("{}{}","\n   [Auto Stake Rewards]".truecolor(75,219,75),"     amount:            ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_in_ust_to_string(tasks.clone(),  2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Auto Stake Rewards]".truecolor(75,219,75),"     target:            ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","value_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Auto Stake]".truecolor(75,219,75),"             next:              ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","date_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    // est fees.
    anchor_view.push((format!("{}{}","\n\n   [Auto Stake Claim Tx]".truecolor(75,219,75),"    est. fee:          ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Auto Stake Stake Tx]".truecolor(75,219,75),"    est. fee:          ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Auto Stake Tx]".truecolor(75,219,75),"          est. fee:          ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
    
    let mut field = "result:  ";

    if is_test {
        field = "estimate:";
    }

    anchor_view.push((format!("{}{}","\n\n   [Auto Stake]".truecolor(75,219,75),format!("             {}          ",field.purple())),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    
    // function able to execute auto stake, therefore registering it as task to run concurrently. 
    let important_task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_stake_rewards(tasks.clone(), wallet_seed_phrase.clone(),is_test));
    let timeout_duration = 120u64;
    let mut block_duration_after_resolve = 10i64;
    /* a small duration is optimal, since the data is already there */
    /* only issue is if there just was a transaction, this is handled by ensuring that the relevant data is recent enough.*/

    if is_test {
        // each call executes an estimate, therefore have higher delay to not spam estimates.
        // since test mode does not perform transactions, there is no downside by doing this.
        block_duration_after_resolve = 30i64;
    }
    try_register_function(&tasks,"anchor_auto_stake".to_owned(),important_task,timeout_duration, block_duration_after_resolve).await;  
          
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_stake".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;

    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }     

    return anchor_tasks;
}


pub async fn display_anchor_account(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize,is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {


    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();

    let mut anchor_view: Vec<(String,usize)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol Account**\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

    //anchor_view.push((format!("{}{}","\n   [Liquidation Queue]".truecolor(75,219,75),"    withdrawals:             ".purple().to_string()),*offset));
    //*offset += 1;

    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    loan amount:             ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_loan_amount_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST (borrow limit: ".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrow_limit_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST)".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    loan to value:           ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_ltv_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    pending rewards:         ".purple().to_string()),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_in_ust_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((" UST (=".purple().to_string(),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((" ANC)".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    fee to claim & stake:    ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anchor_claim_and_stake_transaction_gas_fees_ratio_to_string(tasks.clone(),3)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push(("  (=".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),3)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST)".purple().to_string(),*offset));
    *offset += 1;
   
    anchor_view.push((format!("{}{}","\n\n   [Earn]".truecolor(75,219,75),"      deposit:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_ust_deposited_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST  (=".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_balance_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" aUST)".purple().to_string(),*offset));
    *offset += 1;
 

    //anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    fee to claim & stake:    ".to_string(),*offset));
   /*
    let available_liquidity_from_ust_deposit = borrower_deposit_liquidity_to_string(tasks.clone(),2).await;
    display_add(format!("   [Earn]    deposit liquidity:    {}",available_liquidity_from_ust_deposit), 23 as usize,2 as usize); 
   */  
    anchor_view.push((format!("{}{}","\n\n   [Gov]".truecolor(75,219,75),"       balance:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_anc_deposited_to_string(tasks.clone(),false,4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" ANC".purple().to_string(),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Gov]".truecolor(75,219,75),"       staked:                  ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anc_staked_balance_in_ust_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST  (=".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anc_staked_balance_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" ANC)\n\n".purple().to_string(),*offset));
    *offset += 1;
  
    //   add -> (=absolute returns) UST or ANC FOR DISTRIBUTION APR AND AUTO STAKING
  
    anchor_view.push((format!("    {}",display_add("   _    _    Net APY    Borrow APY    Distribution APY    Earn APY    Auto Staking APY (not included in Net APY)".purple().to_string(), 23 as usize,2 as usize)),*offset));
    *offset += 1;
   
    anchor_view.push((display_add("   [Anchor]    loan_amount:    --".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    loan_amount:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","total_returns",2));
    let f = Box::pin(add_format_to_result(" (=".to_string()," UST)".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","date_next",2));
    let f = Box::pin(add_format_to_result(" Next Auto Stake: ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","duration_next",2));
    let f = Box::pin(add_format_to_result(" (every ".to_string(),")".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,1 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("   [Anchor]    target_ltv:    --".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    target_ltv:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","total_returns",2));
    let f = Box::pin(add_format_to_result(" (=".to_string()," UST)".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","date_next",2));
    let f = Box::pin(add_format_to_result(" Next Auto Stake: ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","duration_next",2));
    let f = Box::pin(add_format_to_result(" (every ".to_string(),")".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,1 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("   [Anchor]    deposit_amount:    --".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    deposit_amount:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,1 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

       
    // ADD ANC scenario
    // ANC -50%, -25%, 0%, + 25%, +50%, + 100%

    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }

    return anchor_tasks;
  
}
 

pub async fn display_anchor_info(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {
 

    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();
    let mut anchor_view: Vec<(String,usize)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol**\n\n".truecolor(75,219,75).to_string(),*offset));
    *offset += 1;
 

    /* Expert Parameters
    let total_liabilities = total_liabilities_to_string(tasks.clone(),"state anchorprotocol mmMarket",0).await;
    display[*offset] = format!("   [Anchor] stablecoins lent:        {} UST\n",total_liabilities);
    *offset += 1;
    println!("{}",display.join(""));
    
    let stablecoins_deposited = a_terra_supply_to_string(tasks.clone(), "epoch_state anchorprotocol mmMarket",0).await; 
    display[*offset] = format!("   [Anchor] stablecoins deposited:   {} UST\n", stablecoins_deposited);
    *offset += 1;
    println!("{}",display.join(""));

    let utilization_ratio = utilization_ratio_to_string(tasks.clone(),"state anchorprotocol mmMarket","epoch_state anchorprotocol mmMarket",2).await;
    display[*offset] = format!("   [Anchor] utilization ratio:       {}\n\n",utilization_ratio);
    //\n  *The utilization ratio quantifies a stablecoin's borrow demand relative to the amount of deposited stablecoins.\n
    *offset += 1;
    println!("{}",display.join(""));
    */

    /*  Expert Parameters
    let base_rate = base_rate_to_string(tasks.clone(),"config anchorprotocol mmInterestModel",10).await;
    display[*offset] = format!("   [Anchor] base rate:               {}\n",base_rate);
    *offset += 1;
    println!("{}",display.join(""));
    let interest_multiplier = interest_multiplier_to_string(tasks.clone(),10).await;
    display[*offset] = format!("   [Anchor] interest multiplier:     {}\n",interest_multiplier);
    *offset += 1;
    println!("{}",display.join(""));

    let borrow_rate = borrow_rate_to_string(tasks.clone(),"config anchorprotocol mmInterestModel","state anchorprotocol mmMarket","epoch_state anchorprotocol mmMarket",10).await;
    display[*offset] = format!("   [Anchor] borrow rate:             {}\n",borrow_rate);
    *offset += 1;
    println!("{}",display.join(""));
    */

    anchor_view.push((format!("{}{}","   [Borrow]".truecolor(75,219,75),"    net apr:                 ".purple().to_string()),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(net_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    // The borrow rate equation incentivizes markets to have sufficient liquidity at their equilibrium. An increase in borrow demand is met with higher borrow rates, incentivizing repayments, and restoring market liquidity.
    anchor_view.push((" (borrow apr: ".purple().to_string(),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrow_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    // Borrower incentives: 400M (40%) tokens are linearly released to be used as borrower incentives over a period of 4 years. 
    anchor_view.push((", distribution apr: ".purple().to_string(),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    // TODO: figure out the distribution apy calculation from the smart contracts.
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(distribution_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}{}",")\n","   [Borrow]".truecolor(75,219,75),"    fee to claim:            ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_claim_rewards","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}{}"," UST\n\n".purple().to_string(),"   [Gov]".truecolor(75,219,75), "       ANC staking apy:         ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    // Anchor periodically distributes portion of ANC tokens purchased from protocol fees are distributed to ANC stakers to incentivize governance participation and decrease circulating ANC supply
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(staking_apy_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Gov]".truecolor(75,219,75),"       fee to stake:            ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_staking","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}{}"," UST\n".purple().to_string(),"\n   [Earn]".truecolor(75,219,75),"      deposit apy:             ".purple().to_string()),*offset));
    *offset += 1;
    
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(earn_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push(("\n".to_string(),*offset));
    *offset += 1;
 
    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await;
    }

    return anchor_tasks;

}
 

pub async fn display_market_info(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {

    let mut market_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = vec![];

    let mut market_view: Vec<(String,usize)> = Vec::new();

    market_view.push(("\n\n  **Terra**\n\n".truecolor(84, 147, 247).to_string(),*offset)); 
    *offset += 1;


    market_view.push((format!("{}{}","   [Terra]".truecolor(84, 147, 247),"    est. blocks per year:   ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(blocks_per_year_to_string(tasks.clone(),0)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Terra]".truecolor(84, 147, 247),"    gas price:              ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(gas_price_to_string(tasks.clone(),4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n\n   [Terra]".truecolor(84, 147, 247),"    SDT    -> Luna:         ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(core_swap_amount_to_string(tasks.clone(),"core_swap usdr uluna",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((" (=$".purple().to_string(),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(core_swap_amount_to_string(tasks.clone(), "core_swap uusd usdr",2)));
    market_tasks.push(t);
    *offset += 1;
    market_view.push((")".purple().to_string(),*offset));
    *offset += 1;
 
    market_view.push((format!("{}{}","\n   [Terra]".truecolor(84, 147, 247),"    Luna   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(core_swap_amount_to_string(tasks.clone(), "core_swap uluna uusd",2)));
    market_tasks.push(t);
    *offset += 1;
 
    market_view.push((format!("{}{}","\n\n   [Anchor]".truecolor(75,219,75),"   Luna   -> bLuna:        ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation anchorprotocol uluna terraswapblunaLunaPair",4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Bond]".truecolor(75,219,75),"     Luna   -> bLuna:        ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(b_luna_exchange_rate_to_string(tasks.clone(),4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n\n   [Anchor]".truecolor(75,219,75),"   ANC    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((format!("{}{}","\n   [Anchor]".truecolor(75,219,75),"   aUST   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(a_terra_exchange_rate_to_string(tasks.clone(),4)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((format!("{}{}","\n\n   [Nexus]".truecolor(244, 182, 199),"    nLuna  -> PSI:          ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(), "simulation_cw20 nexusprotocol nLunaToken Psi-nLuna_Pair",2)));
    market_tasks.push(t);
    *offset += 1;
    
    market_view.push((format!("{}{}","\n   [Nexus]".truecolor(244, 182, 199),"    PSI    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(), "simulation_cw20 nexusprotocol PsiToken Psi-UST_Pair",4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n\n   [Mirror]".truecolor(228, 228, 231),"   MIR    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd mir",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mTSLA  -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_tsla",2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mSPY   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_spy",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mBTC   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_btc",2)));
    market_tasks.push(t);
    *offset += 1;
     
    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mETH   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_eth",2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push(("\n".to_string(),*offset));
    *offset += 1;

    if is_first_run {
        add_view_to_display(&new_display, market_view).await;
    }

    return market_tasks;

}


