use secstr::*;

use std::env;


use rust_decimal::Decimal;
use core::str::FromStr;
use std::convert::TryFrom;

 
use serde::Deserialize;
use serde::Serialize;

mod control;

use control::view::model::{UserSettings,MaybeOrPromise,requirements,get_keys_of_running_tasks,await_running_tasks,get_timestamps_of_resolved_tasks};

use control::view::model::smart_contracts::{ResponseResult,get_block_txs_deposit_stable,get_block_txs_deposit_stable_apy,get_block_txs_fee_data};
use control::view::model::smart_contracts::meta::api::{fetch_gas_price, QueryResponse,query_core_block_at_height,query_core_latest_block};
use control::view::model::smart_contracts::meta::api::data::{GasPrices};
use control::view::model::smart_contracts::meta::api::data::endpoints::get_terra_fcd;

use control::view::*;
use control::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use anyhow::anyhow;
use enum_as_inner::EnumAsInner;
   
use num_format::{Locale, ToFormattedString}; 

use std::{thread, time};
use std::time::{Duration, Instant};



use std::sync::Arc; 
use tokio::sync::RwLock; 
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio::time::Timeout;


use colored::*;
 
use simple_user_input::get_input; 

use rand::Rng;


use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

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
 
// TODO: Hardening the requests, what happens when once request fails repeatedly. 
// for each FCD have a LCD backup option.

// TODO: Error handling. Every Unwrapp needs to be inspected. 

// TODO: view.rs order functions and put them in modules
// TODO: optimise view.rs reused code.

// TODO: Add auto repay functionality.

// TODO: Remove all compiler warnings.

 

 #[tokio::main]
async fn main() -> anyhow::Result<()> {

        let num_cpus = num_cpus::get();



        let args: Vec<String> = env::args().collect();

        //./target/debug/terra-rust-bot -i market anchor -a anchor_account -b anchor_auto_stake -d test 
        //println!("{:?}", args);

        let mut args_i: Vec<&str> = Vec::new();
        let mut args_a: Vec<&str> = Vec::new();
        let mut args_b: Vec<&str> = Vec::new();
        let mut args_d: Vec<&str> = Vec::new();

        let mut last_item = 0;
        let mut is_test = false;

        for x in 1..args.len() {
            if &args[x] == "-i" || &args[x] == "-a" || &args[x] == "-b" ||  &args[x] == "-d" {
                last_item = x;
            }else{
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
                    args_d.push(&args[x]);

                }
            }
        }
        let is_test = *&is_test;
        //println!("{:?}",(args_i,args_a,args_b,args_d));

        println!("{esc}c", esc = 27 as char); 

        let mut wallet_seed_phrase = SecUtf8::from("".to_string());
        if args_b.len() > 0 {
            wallet_seed_phrase = SecUtf8::from(get_input("Enter your seed phrase (press Enter to skip):").to_string());
            // https://github.com/unrelentingtech/secstr
            println!("{esc}c", esc = 27 as char); 
        } 

        let mut wallet_acc_address = "".to_string();  
        if args_a.len() > 0 {
            wallet_acc_address = get_input("Enter your wallet address (press Enter to skip):").to_string();
        } 

        // todo: read user settings from file. JSON.
        let user_settings: UserSettings = UserSettings {
            trigger_percentage: Decimal::from_str("0.85").unwrap(),
            max_gas_adjustment: Decimal::from_str("1.67").unwrap(),
            gas_adjustment_preference: Decimal::from_str("1.2").unwrap(),
            min_ust_balance: Decimal::from_str("10").unwrap(), 
            wallet_acc_address: wallet_acc_address,  
        };

        println!("{esc}c", esc = 27 as char); 

        let tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>> = Arc::new(RwLock::new(HashMap::new())); 

        let mut display: Vec<String> = vec!["".to_string(); 99];

        let new_display: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec!["".to_string(); 1000]));

        display[0] = format!("{esc}c", esc = 27 as char);
       
        let every_block: i32 = 6;  // around every 6s a new block is generated. fastest setting.

        let fast: i32 = 10;    // for requests short lived information
        let medium: i32 = 60;    // for requests short lived information
        let slow: i32 = 60*10; // for requests that have relative constant results.
        let instant: i32 = 0;  // for settings, where there is no downside/delay of updating the values.

        // (key, target_refresh_time, dependency_tag)

        let mut req = vec![
        ("terra_balances", every_block, vec!["anchor_auto_stake"]),
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
        ("simulation_cw20 anchorprotocol ANC terraswapAncUstPair", fast, vec!["market","anchor_account"]),
        ("epoch_state anchorprotocol mmMarket", fast, vec!["anchor","market","anchor_account"]),
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
        ("borrow_limit", every_block, vec!["anchor_account","anchor_auto_stake"]),
        ("borrow_info", every_block, vec!["anchor_account","anchor_auto_stake"]),
        ("balance", every_block, vec!["anchor_account"]),
        ("anc_balance", every_block, vec!["anchor_account"]),
        ("staker", every_block, vec!["anchor_account"]),
        ("blocks_per_year", slow, vec!["market","anchor","anchor_account"]), 
        ("earn_apy", slow, vec!["anchor","anchor_account"]),
        /* <meta data> */ 
        /* <from settings> */ 
        ("anchor_protocol_txs_claim_rewards", slow, vec!["anchor","anchor_account","anchor_auto_stake"]), 
        ("anchor_protocol_txs_staking", slow, vec!["anchor","anchor_account","anchor_auto_stake"]), 
        ("trigger_percentage", fast, vec!["anchor_account"]),
        ("max_gas_adjustment", fast, vec!["anchor_account"]),
        ("gas_adjustment_preference",fast, vec!["anchor_account"]),
        ("min_ust_balance", fast, vec!["anchor_account"]),
        /* <from gas_prices>*/
        ("gas_fees_uusd", medium, vec!["market","anchor","anchor_account","anchor_auto_stake"]),
        ]; 

        let req_clone = req.clone();

        let mut req_keys: Vec<&str> = Vec::new();
        for i in 0..req.len() {
               req_keys.push(req[i].0);
        } 
        let req_keys = &*req_keys;

        let mut is_first_run: bool = true;

        let mut req_to_check: Vec<&str> = Vec::new(); 
        for i in 0..req.len() {
            for x in &args {
                if req[i].2.contains(&x.as_str()) {
                    req_to_check.push(req[i].0);
                    break;
                }
            }
        }

        add_string_to_display(&new_display, 0, format!("{esc}c", esc = 27 as char)).await;

        let display_loop = print_to_terminal(&new_display,false); 

        loop {
            //let start = Instant::now();  
            let mut req_to_update: Vec<&str> = Vec::new(); 


            let req_unresolved = get_keys_of_running_tasks(&tasks,&req_keys).await;

            // waiting for unresolved tasks to catch up
            // delays the next refresh by max 10s.
            if is_first_run { 
                timeout(Duration::from_secs(60*2), await_running_tasks(&tasks, &req_keys)).await;
            } else if req_unresolved.len() >= num_cpus { 
                timeout(Duration::from_secs(30), await_running_tasks(&tasks, &req_keys)).await;
            } 

            let req_resolved_timestamps = get_timestamps_of_resolved_tasks(&tasks,&req_keys).await;

            let now = Utc::now().timestamp();

            for i in 0..req.len() {
                let mut contains = false;
                for x in &args {
                    if req[i].2.contains(&x.as_str()) {
                        contains = true;
                        break;
                    }
                }  
                if contains && !req_unresolved.contains(&req[i].0) && (req_resolved_timestamps[i]==0i64 || ((now - req_resolved_timestamps[i]) > req[i].1 as i64 )) { // unresolved requirements will not be refreshed.
                    req_to_update.push(req[i].0); 
                }
                if req_to_update.len()>num_cpus {
                    break;
                }
            } 
           add_string_to_display(&new_display,1,format!(
                "{}{}{}{}{}{}{}\n{}",
                timestamp_now_to_string().yellow(),
                " -  unresolved requirements ".purple(),
                req_unresolved.len().to_string().red(),
                " of ".purple(),
                req_to_update.len().to_string().yellow(),
                "/",
                req_to_check.len().to_string().truecolor(75,219,75),
                format!("{:?}\n{:?}",req_unresolved,req_to_update).to_string().purple()
                )).await; 

            requirements(&tasks,&user_settings,&req_to_update).await;  
             
            let mut offset: usize = 2;

            // can also limit by timestamp, so that we do not have infinite display tasks.
            let mut open_tasks: Vec<JoinHandle<anyhow::Result<()>>> = Vec::new();
            if args_i.contains(&"market") {        
                for t in display_market_info(&tasks, &new_display, &mut offset, is_first_run).await {
                    open_tasks.push(t);
                }
            }
            if args_i.contains(&"anchor") {        
                for t in display_anchor_info(&tasks, &new_display, &mut offset, is_first_run).await {
                    open_tasks.push(t);
                }
            }

            if args_a.contains(&"anchor_account") {
                for t in display_anchor_account(&tasks, &new_display, &mut offset, is_first_run).await {
                    open_tasks.push(t);
                }
            }

            if args_b.contains(&"anchor_auto_stake") {
                lazy_anchor_account_auto_stake_rewards(&tasks, &user_settings, &wallet_seed_phrase, &new_display, &mut offset, is_test).await;
            }
            
            // waiting for all open **display** tasks.
            // if one task is slow, because the requirement is not yet resolved, it slows down the whole loop, 
            // therefore it will timeout after 1s, so the loop can continue.  
             
            for t in open_tasks {
                t.await;
            } 

            // den fall verhindern, wenn ein task durch einen neuen task ersetzt wird, dadurch das ergebnis nie ankommt, oder selten ankommt.
            // req um ein feld resolved erweitern: 
            // setze durch get_keys_of_running_tasks auf resolved.
            // timestamp, if to long unresolved then ersetze durch neuen task.
            // reduziert die überabdekung und unnötige doppel queries.

            // todo: might get the dependencies on requirements as well, with the tasks, then could filter, to reduce workload.
            // see which req's are resolved and which are not.
            // only override req if timeout reached. (1min, 3min?)

            display_all_errors(&tasks, &*req_unresolved ,&new_display, &mut offset).await;
            // todo: only write logs when special event (Errors, or TX).
            // todo: can write display to a log file.  

            is_first_run = false;
 
         
        }
 
        Ok(())
  
} 

pub fn display_add(item: String, space_len: usize, fixed_len: usize, new_lines: usize) -> String {

    let mut split = item.split("    ");
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

pub async fn add_table_formatting(f: Pin<Box<dyn Future<Output = String> + Send + 'static >>, space_len: usize, fixed_len: usize, new_lines: usize) -> String {
    let res = f.await;
    let mut split = res.split("    ");
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

/*
pub fn refresh_display(new_display: &Arc<RwLock<Vec<String>>>, tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req_unresolved: &[&str], user_settings:&UserSettings, wallet_seed_phrase: &SecUtf8, is_first_run: bool, is_test:bool) ->  JoinHandle<anyhow::Result<()>> {
// spawns a task that runs the loop.
}*/


pub fn print_to_terminal(new_display: &Arc<RwLock<Vec<String>>>, once: bool) ->  JoinHandle<anyhow::Result<()>> {

    let display_clone = new_display.clone();

    return tokio::spawn(async move {     
            if once {
                println!("{}",display_clone.read().await.join("")); 
            }else{ 
                loop {
                    println!("{}",display_clone.read().await.join("")); 
                    thread::sleep(time::Duration::from_millis(300));
                }
            }
            Ok(())
    });  
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

pub fn add_view_to_display(new_display: &Arc<RwLock<Vec<String>>>, view: Vec<(String,usize)>) -> JoinHandle<anyhow::Result<()>> {
     
    let display_clone = new_display.clone();

    return tokio::spawn(async move {      
            let mut look = display_clone.try_write();
            while look.is_err() {
                thread::sleep(time::Duration::from_millis(10));
                look = display_clone.try_write();
            } 
            let mut vector = look.unwrap();
            for entry in view {
                *vector.get_mut(entry.1).unwrap() = entry.0;
            }
            Ok(())
    });  
}

pub async fn add_format_to_result(prefix: String,suffix: String, f: Pin<Box<dyn Future<Output = String> + Send + 'static >>) -> String {
    return format!("{}{}{}",prefix,f.await,suffix);
}

pub fn add_to_display(new_display: &Arc<RwLock<Vec<String>>>, index: usize, f: Pin<Box<dyn Future<Output = String> + Send + 'static >>) -> JoinHandle<anyhow::Result<()>> {
    
    let display_clone = new_display.clone();

    return tokio::spawn(async move { 
            let rg = rand::thread_rng().gen_range(55..100);
            let b = rand::thread_rng().gen_range(225..255); 
            let result = timeout(Duration::from_secs(1), f).await?;     
            {
                let mut look = display_clone.try_write();
                while look.is_err() {
                    thread::sleep(time::Duration::from_millis(10));
                    look = display_clone.try_write();
                } 
                let mut vector = look.unwrap();
                *vector.get_mut(index).unwrap() = format!("{}",result.truecolor(rg, rg, b));
            }  
            Ok(())
    });  
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

pub async fn lazy_anchor_account_auto_stake_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, user_settings: &UserSettings, wallet_seed_phrase: &SecUtf8,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool) {
     
    add_string_to_display(new_display,*offset,"\n  **Anchor Protocol Auto Stake**\n\n".truecolor(75,219,75).to_string()).await; 
    *offset += 1;
  
    add_string_to_display(new_display,*offset,format!("{}{}","   [Auto Stake]    next:        ".truecolor(75,219,75),"--".to_string().purple())).await; 
    // initial resolve may take some time.
    // therefore timeout after 1s.
    let date_next_to_auto_claim_and_stake = timeout(Duration::from_secs(1),estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","date_next",2)).await;
    if date_next_to_auto_claim_and_stake.is_err() { 
        *offset += 1;
        return;
    }
    let date_next_to_auto_claim_and_stake = date_next_to_auto_claim_and_stake.unwrap();
 
    add_string_to_display(new_display,*offset,format!("{}{}","   [Auto Stake]    next:        ".truecolor(75,219,75),date_next_to_auto_claim_and_stake.to_string().yellow())).await; 
    *offset += 1;

    if date_next_to_auto_claim_and_stake == "now".to_string() {   
        anchor_account_auto_stake_rewards(&tasks,  user_settings,wallet_seed_phrase,new_display,offset,is_test).await;
    }
 

}

async fn anchor_account_auto_stake_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, user_settings: &UserSettings, wallet_seed_phrase: &SecUtf8, new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool) {

    // check next time to auto stake
    let date_next_to_auto_claim_and_stake = estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(), "loan_amount","date_next",2).await;
    
    if date_next_to_auto_claim_and_stake == "now".to_string() {

        // check for sufficient funds
        match terra_balance_to_string(tasks.clone(),"uusd",false,2).await.as_ref() {
            "--" => {
                add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Stake]".truecolor(75,219,75),"        Error:    Loading UST account balance failed".red())).await; 
                *offset += 1;
                return;
            },
            "0" => {
                add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Stake]".truecolor(75,219,75),"        Error:    insufficient funds".red())).await; 
                *offset += 1;
                return; 
            },
            e => {
                match min_ust_balance_to_string(tasks.clone(),2).await.as_ref() {
                    "--" => {
                        add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Stake]".truecolor(75,219,75),"        Error:    min_ust_balance undefined".red())).await; 
                        *offset += 1;
                        return; 
                    },
                    m => {
                        let balance = Decimal::from_str(e).unwrap();
                        println!("{}",e);
                        let min_balance = Decimal::from_str(m).unwrap();
                        if balance < min_balance {
                            add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Stake]".truecolor(75,219,75),"        Error:    insufficient funds: less than min_ust_balance".red())).await; 
                            *offset += 1;
                            return; 
                        }
                    }
                }
            }
        } 

        add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Stake]    estimate:    ".truecolor(75,219,75),"--".to_string().yellow())).await; 
        let result = anchor_borrow_claim_and_stake_rewards(tasks.clone(), wallet_seed_phrase,true).await; 
        add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Stake]    estimate:    ".truecolor(75,219,75),result.to_string().yellow())).await; 
        *offset += 1;

        if !is_test {
            add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Stake]    result:      ".truecolor(75,219,75),"--".to_string().yellow())).await; 
            let result = anchor_borrow_claim_and_stake_rewards(tasks.clone(), wallet_seed_phrase,false).await; 
            add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Stake]    result:      ".truecolor(75,219,75),result.to_string().yellow())).await; 
            *offset += 1;

        }
/*
        let result = anchor_borrow_claim_rewards(tasks.clone(), wallet_seed_phrase,true).await;
        display_add(format!("\n   [Auto Stake]   complete:    {:?}",result),10 as usize, 43 as usize,1 as usize); 
   
        let result = anchor_governance_stake_balance(tasks.clone(), wallet_seed_phrase,true).await;
        display_add(format!("\n   [Auto Stake]   complete:    {:?}",result),10 as usize, 43 as usize,1 as usize); 
*/ 

    }else {  
        add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Stake]    next:        ".truecolor(75,219,75),date_next_to_auto_claim_and_stake.to_string().yellow())).await; 
        *offset += 1;
    }

}


pub async fn display_anchor_account(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize,is_first_run: bool) -> Vec<JoinHandle<anyhow::Result<()>>> {


    let mut anchor_tasks: Vec<JoinHandle<anyhow::Result<()>>> = Vec::new();

    let mut anchor_view: Vec<(String,usize)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol Account**\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    loan amount:             ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(borrower_loan_amount_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST (borrow limit: ".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(borrow_limit_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST)".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    loan to value:           ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(borrower_ltv_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    pending rewards:         ".purple().to_string()),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(borrower_rewards_in_ust_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((" UST (=".purple().to_string(),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(borrower_rewards_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((" ANC)".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    fee to claim & stake:    ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(anchor_claim_and_stake_transaction_gas_fees_ratio_to_string(tasks.clone(),3)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push(("  (=".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),3)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST)".purple().to_string(),*offset));
    *offset += 1;
   
    anchor_view.push((format!("{}{}","\n\n   [Earn]".truecolor(75,219,75),"      deposit:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(borrower_ust_deposited_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST  (=".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(borrower_balance_to_string(tasks.clone(),"balance",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" aUST)".purple().to_string(),*offset));
    *offset += 1;
 

    //anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    fee to claim & stake:    ".to_string(),*offset));
   /*
    let available_liquidity_from_ust_deposit = borrower_deposit_liquidity_to_string(tasks.clone(),2).await;
    display_add(format!("   [Earn]    deposit liquidity:    {}",available_liquidity_from_ust_deposit),10 as usize, 23 as usize,2 as usize); 
   */  
    anchor_view.push((format!("{}{}","\n\n   [Gov]".truecolor(75,219,75),"       balance:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(borrower_anc_deposited_to_string(tasks.clone(),false,4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" ANC".purple().to_string(),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Gov]".truecolor(75,219,75),"       staked:                  ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(anc_staked_balance_in_ust_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST  (=".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(anc_staked_balance_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" ANC)\n\n".purple().to_string(),*offset));
    *offset += 1;
  
    //   add -> (=absolute returns) UST or ANC FOR DISTRIBUTION APR AND AUTO STAKING
  
    anchor_view.push((format!("    {}",display_add("   _    _    Net APY    Borrow APY    Distribution APY    Earn APY    Auto Staking APY (not included in Net APY)".purple().to_string(),10 as usize, 23 as usize,2 as usize)),*offset));
    *offset += 1;
   
    anchor_view.push((display_add("   [Anchor]    loan_amount:    --".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    loan_amount:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","total_returns",2));
    let f = Box::pin(add_format_to_result(" (=".to_string()," UST)".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","date_next",2));
    let f = Box::pin(add_format_to_result(" Next Auto Stake: ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","duration_next",2));
    let f = Box::pin(add_format_to_result(" (every ".to_string(),")".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,1 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("   [Anchor]    target_ltv:    --".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    target_ltv:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","total_returns",2));
    let f = Box::pin(add_format_to_result(" (=".to_string()," UST)".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","date_next",2));
    let f = Box::pin(add_format_to_result(" Next Auto Stake: ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","duration_next",2));
    let f = Box::pin(add_format_to_result(" (every ".to_string(),")".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,1 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("   [Anchor]    deposit_amount:    --".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    deposit_amount:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,0 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(),10 as usize, 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f,10 as usize, 23 as usize,1 as usize));
    let t = add_to_display(&new_display, *offset,f);
    anchor_tasks.push(t);
    *offset += 1;

       
    // ADD ANC scenario
    // ANC -50%, -25%, 0%, + 25%, +50%, + 100%

    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }

    return anchor_tasks;
  
}
 

pub async fn display_anchor_info(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize, is_first_run: bool) -> Vec<JoinHandle<anyhow::Result<()>>> {
 

    let mut anchor_tasks: Vec<JoinHandle<anyhow::Result<()>>> = Vec::new();
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
    let interest_multiplier = interest_multiplier_to_string(tasks.clone(),"config anchorprotocol mmInterestModel",10).await;
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
    let t = add_to_display(&new_display, *offset, Box::pin(net_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    // The borrow rate equation incentivizes markets to have sufficient liquidity at their equilibrium. An increase in borrow demand is met with higher borrow rates, incentivizing repayments, and restoring market liquidity.
    anchor_view.push((" (borrow apr: ".purple().to_string(),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(borrow_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    // Borrower incentives: 400M (40%) tokens are linearly released to be used as borrower incentives over a period of 4 years. 
    anchor_view.push((", distribution apr: ".purple().to_string(),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    // TODO: figure out the distribution apy calculation from the smart contracts.
    let t = add_to_display(&new_display, *offset, Box::pin(distribution_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}{}",")\n","   [Borrow]".truecolor(75,219,75),"    fee to claim:            ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_claim_rewards","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}{}"," UST\n\n".purple().to_string(),"   [Gov]".truecolor(75,219,75), "       ANC staking apy:         ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    // Anchor periodically distributes portion of ANC tokens purchased from protocol fees are distributed to ANC stakers to incentivize governance participation and decrease circulating ANC supply
    let t = add_to_display(&new_display, *offset, Box::pin(staking_apy_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Gov]".truecolor(75,219,75),"       fee to stake:            ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_staking","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}{}"," UST\n".purple().to_string(),"\n   [Earn]".truecolor(75,219,75),"      deposit apy:             ".purple().to_string()),*offset));
    *offset += 1;
    
    anchor_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(earn_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push(("\n".to_string(),*offset));
    *offset += 1;
 
    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await;
    }

    return anchor_tasks;

}
 

pub async fn display_market_info(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize, is_first_run: bool) -> Vec<JoinHandle<anyhow::Result<()>>> {

    let mut market_tasks: Vec<JoinHandle<anyhow::Result<()>>> = Vec::new();

    let mut market_view: Vec<(String,usize)> = Vec::new();

    market_view.push(("\n\n  **Terra**\n\n".truecolor(84, 147, 247).to_string(),*offset)); 
    *offset += 1;


    market_view.push((format!("{}{}","   [Terra]".truecolor(84, 147, 247),"    est. blocks per year:   ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(blocks_per_year_to_string(tasks.clone(),"blocks_per_year",0)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n\n   [Terra]".truecolor(84, 147, 247),"    SDT    -> Luna:         ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(core_swap_amount_to_string(tasks.clone(),"core_swap usdr uluna",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((" (=$".purple().to_string(),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(core_swap_amount_to_string(tasks.clone(), "core_swap uusd usdr",2)));
    market_tasks.push(t);
    *offset += 1;
    market_view.push((")".purple().to_string(),*offset));
    *offset += 1;
 
    market_view.push((format!("{}{}","\n   [Terra]".truecolor(84, 147, 247),"    Luna   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(core_swap_amount_to_string(tasks.clone(), "core_swap uluna uusd",2)));
    market_tasks.push(t);
    *offset += 1;
 
    market_view.push((format!("{}{}","\n\n   [Anchor]".truecolor(75,219,75),"   Luna   -> bLuna:        ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation anchorprotocol uluna terraswapblunaLunaPair",4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Bond]".truecolor(75,219,75),"     Luna   -> bLuna:        ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(b_luna_exchange_rate_to_string(tasks.clone(),"state anchorprotocol bLunaHub",4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n\n   [Anchor]".truecolor(75,219,75),"   ANC    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((format!("{}{}","\n   [Anchor]".truecolor(75,219,75),"   aUST   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(a_terra_exchange_rate_to_string(tasks.clone(), "epoch_state anchorprotocol mmMarket",4)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((format!("{}{}","\n\n   [Nexus]".truecolor(244, 182, 199),"    nLuna  -> PSI:          ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(), "simulation_cw20 nexusprotocol nLunaToken Psi-nLuna_Pair",2)));
    market_tasks.push(t);
    *offset += 1;
    
    market_view.push((format!("{}{}","\n   [Nexus]".truecolor(244, 182, 199),"    PSI    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(), "simulation_cw20 nexusprotocol PsiToken Psi-UST_Pair",4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n\n   [Mirror]".truecolor(228, 228, 231),"   MIR    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd mir",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mTSLA  -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_tsla",2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mSPY   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_spy",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mBTC   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_btc",2)));
    market_tasks.push(t);
    *offset += 1;
     
    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mETH   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t = add_to_display(&new_display, *offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_eth",2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push(("\n".to_string(),*offset));
    *offset += 1;

    if is_first_run {
        add_view_to_display(&new_display, market_view).await;
    }

    return market_tasks;

}


