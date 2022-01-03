use secstr::*;

use std::env;


use rust_decimal::Decimal;
use core::str::FromStr;
use std::convert::TryFrom;

 
use serde::Deserialize;
use serde::Serialize;

mod control;

use control::view::model::{UserSettings,MaybeOrPromise,requirements};

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
 
use simple_user_input::get_input; 

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
 

 #[tokio::main]
async fn main() -> anyhow::Result<()> {

        //  ./target/debug/terra-rust-bot -i market anchor -a anchor_account -b anchor_auto_stake -d 
         
        let args: Vec<String> = env::args().collect();
        //println!("{:?}", args);

        let mut args_i: Vec<&str> = Vec::new();
        let mut args_a: Vec<&str> = Vec::new();
        let mut args_b: Vec<&str> = Vec::new();
        let mut args_d: Vec<&str> = Vec::new();

        let mut last_item = 0;
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
                    args_d.push(&args[x]);
                }
            }
        }
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
            //sell_anc_to_maintain_min_ust_balance: true,
            wallet_acc_address: wallet_acc_address,  
        };
        println!("{esc}c", esc = 27 as char); 

        let tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>> = Arc::new(RwLock::new(HashMap::new())); 

        let mut display: Vec<String> = vec!["".to_string(); 99];

        display[0] = format!("{esc}c", esc = 27 as char);
    
      
        let hourly: i32 = 60*60;
        let every_minute: i32 = 60; 
        let every_block: i32 = 6;

        let mut req = vec![
        ("terra_balances",every_minute),
        /* <market_info> */
        /* core_tokens */
        ("core_swap uusd usdr",every_minute),
        ("core_swap usdr uluna",every_minute),
        ("core_swap uluna uusd",every_minute),
        // "simulation terraswap usdr usdr_uluna_pair_contract",
        // "simulation terraswap uluna uusd_uluna_pair_contract",
        /* anchor_tokens */
        ("simulation anchorprotocol uluna terraswapblunaLunaPair",every_minute),
        ("state anchorprotocol bLunaHub",every_minute),
        ("simulation_cw20 anchorprotocol ANC terraswapAncUstPair",every_minute),
        ("epoch_state anchorprotocol mmMarket",every_minute),
        /* nexus_tokens */
        ("simulation_cw20 nexusprotocol nLunaToken Psi-nLuna_Pair",every_minute),
        ("simulation_cw20 nexusprotocol PsiToken Psi-UST_Pair",every_minute),
        /* mirror_tokens */
        ("simulation_cw20 uusd mir",every_minute),
        ("simulation_cw20 uusd m_tsla",every_minute),
        ("simulation_cw20 uusd m_btc",every_minute),
        ("simulation_cw20 uusd m_eth",every_minute),
        ("simulation_cw20 uusd m_spy",every_minute),
        /* <other> */
        /* <anchor_protocol> */
        ("state anchorprotocol mmMarket",every_block),
        ("api/v2/distribution-apy",every_minute),
        ("api/v2/gov-reward",every_minute),
        ("config anchorprotocol mmInterestModel",every_minute),
        //("config anchorprotocol collector",every_minute),
        /* <anchor_protocol account> */ 
        ("borrow_limit",every_block),
        ("borrow_info",every_block),
        ("balance",every_block),
        ("anc_balance",every_block),
        ("staker",every_block),
        ("blocks_per_year",hourly), 
        ("earn_apy",hourly),
        /* <meta data> */ 
        /* <from settings> */ 
        ("anchor_protocol_txs_claim_rewards",hourly), 
        ("anchor_protocol_txs_staking",hourly), 
        ("trigger_percentage",0),
        ("max_gas_adjustment",0),
        ("gas_adjustment_preference",0),
        ("min_ust_balance",0),
        /* <from gas_prices>*/
        ("gas_fees_uusd",0),
        ]; 

        let req_clone = req.clone();

        let mut req_keys: Vec<&str> = Vec::new();
        for i in 0..req.len() {
               req_keys.push(req[i].0);
        } 
        let req_keys = &*req_keys;

        let mut is_first_run: bool = true;


        loop {
            let start = Instant::now();  

            let mut req_to_update: Vec<&str> = Vec::new(); 

            for i in 0..req.len() {
                if req[i].1 <= 0 || is_first_run {
                    req_to_update.push(req[i].0);
                    req[i].1 = req_clone[i].1.to_owned();
                }
            } 
            
            requirements(&tasks,/* &mut */&user_settings,&req_to_update).await; 

            /*
            for key in &req {
                println!("\n{:?}", get_data_maybe_or_resolve_promise(&mut key).await);  
            }*/
            let mut offset: usize = 1;

            display_now(&mut display, &mut offset);
            
            if args_i.contains(&"market") {        
                display_market_info(&tasks, &mut display, &mut offset).await;
            }
            if args_i.contains(&"anchor") {        
                display_anchor_info(&tasks, &mut display, &mut offset).await;
            }
            if args_a.contains(&"anchor_account") {
                display_anchor_account(&tasks, &mut display, &mut offset).await;
            }
            if args_b.contains(&"anchor_auto_stake") {
                lazy_anchor_account_auto_stake_rewards(&tasks, &user_settings, &wallet_seed_phrase, &mut display, &mut offset).await;
            }

            display_all_errors(&tasks, & &req_keys ,&mut display, &mut offset).await;
            //println!("\n{:?}", get_data_maybe_or_resolve_promise(&mut "blocks_per_year").await);  
            //println!("\n{:?}", earn_apr_to_string(&mut 4).await);  
            //println!("{:?}",get_block_txs_fee_data().await?.as_transactions().unwrap().result);
           
            /*
            let vec = get_data_maybe_or_resolve_promise(&mut "anchor_protocol_txs_claim_rewards").await; 
            //println!("{:?}",vec); 
            let vec = vec?;
            for entry in &vec.as_transactions().unwrap().result {
                println!("gas_wanted: {}, gas_used: {}, fee_denom: {}, fee_amount: {}, claim_amount: {}",entry.gas_wanted, entry.gas_used, entry.fee_denom, entry.fee_amount, entry.amount);
                println!("---");
            }*/ 

            // todo: only write logs when special event (Errors, or TX).
            // todo: can write display out to a log file.
            // log last x iterations
            // compress logs.
            for i in 0..req.len() {
               req[i].1 = req[i].1 - (start.elapsed().as_secs() as i32); 
            } 
            is_first_run = false;
        }
 
        Ok(())
  
} 

pub fn display_add(display: &mut Vec<String>,offset: &mut usize, item: String, space_len: usize, fixed_len: usize, new_lines: usize){

    let mut split = item.split("    ");
    display[*offset] = "".to_string();

    for s in split {
        if s.len() > 0 {
            if s.len() <= fixed_len {
                let space = fixed_len - s.len();
                display[*offset] = format!("{}{}{}",display[*offset],s," ".repeat(space));
            }else{
                display[*offset] = format!("{}{}", display[*offset],s);
            }
        }
    }

    display[*offset] = format!("{}{}",display[*offset],"\n".repeat(new_lines));
    *offset += 1;
    println!("{}",display.join(""));

}

pub fn display_now(display: &mut Vec<String>,offset: &mut usize) {
    display_add(display,offset,format!("\n{}\n\n",timestamp_now_to_string()),0 as usize, 32 as usize, 0 as usize);
}

pub async fn display_all_errors(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &[&str], display: &mut Vec<String>,offset: &mut usize) {
    display_add(display,offset,"\n  **Errors**\n\n".to_string(),0 as usize, 32 as usize, 0 as usize);
    let mut error_count = 0;
    for key in req {
        match anything_to_err(tasks,  key).await.as_ref() {
            "--" => {
            },
            e => {
                if !e.contains("Info: Key '"){
                    error_count = error_count +1;
                    display_add(display,offset,format!("\n   [Key] '{}'\n   {}",key,e),10 as usize, 23 as usize,1 as usize); 
                }
            }
        } 
    }
    if error_count == 0 {
        display_add(display,offset,format!("\n   None \n"),10 as usize, 23 as usize,1 as usize); 
    }

}

pub async fn lazy_anchor_account_auto_stake_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, user_settings: &UserSettings, wallet_seed_phrase: &SecUtf8, display: &mut Vec<String>,offset: &mut usize) {
    display_add(display,offset,"\n  **Anchor Protocol Auto Stake**\n\n".to_string(),0 as usize, 32 as usize, 0 as usize);

    // lazy check next time to auto stake 
    // laze because data might not be the newest, depends on the refresh rate.
    let date_next_to_auto_claim_and_stake = estimate_anchor_protocol_next_claim_and_stake_tx(tasks,  "loan_amount","date_next",2).await;
    
    if date_next_to_auto_claim_and_stake == "now".to_string() {
        display_add(display,offset,format!("   [Anchor Auto Stake]    initiate auto staking.."),10 as usize, 23 as usize,1 as usize); 
        anchor_account_auto_stake_rewards(tasks,  user_settings,wallet_seed_phrase,display,offset).await;
    }else {  
        display_add(display,offset,format!("   [Anchor Auto Stake]    next:    {}",date_next_to_auto_claim_and_stake),10 as usize, 23 as usize,1 as usize); 
    }

}

async fn anchor_account_auto_stake_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, user_settings: &UserSettings, wallet_seed_phrase: &SecUtf8, display: &mut Vec<String>,offset: &mut usize) {

    // make sure the data is up to date.    
    let req = vec![ 
            "gas_fees_uusd",
            "terra_balances",
            "borrow_limit",
            "borrow_info", 
    //      "anc_balance",  /* only used if claiming & staking in sepearate transactions */
            "anchor_protocol_txs_claim_rewards",
            "anchor_protocol_txs_staking",
            "api/v2/distribution-apy",
            "api/v2/gov-reward"
            ]; 
    requirements(tasks, user_settings,&req).await; 

    // check next time to auto stake
    let date_next_to_auto_claim_and_stake = estimate_anchor_protocol_next_claim_and_stake_tx(tasks, "loan_amount","date_next",2).await;
    
    if date_next_to_auto_claim_and_stake == "now".to_string() {

        // check for sufficient funds
        match terra_balance_to_string(tasks,  "uusd",false,2).await.as_ref() {
            "--" => {
                display_add(display,offset,"   [Anchor Auto Stake]    Error:    Loading UST account balance failed".to_string(),10 as usize, 23 as usize,1 as usize); 
                return;
            },
            "0" => {
                display_add(display,offset,"   [Anchor Auto Stake]    Error:    insufficient funds".to_string(),10 as usize, 23 as usize,1 as usize); 
                return; 
            },
            e => {
                match min_ust_balance_to_string(tasks,  2).await.as_ref() {
                    "--" => {
                        display_add(display,offset,"   [Anchor Auto Stake]    Error:    min_ust_balance undefined".to_string(),10 as usize, 23 as usize,1 as usize); 
                        return; 
                    },
                    m => {
                        let balance = Decimal::from_str(e).unwrap();
                        println!("{}",e);
                        let min_balance = Decimal::from_str(m).unwrap();
                        if balance < min_balance {
                            display_add(display,offset,"   [Anchor Auto Stake]    Error:    insufficient funds: less than min_ust_balance".to_string(),10 as usize, 23 as usize,1 as usize); 
                            return; 
                        }
                    }
                }
            }
        }
        // todo: set to false in production.

        let result = anchor_borrow_claim_and_stake_rewards(tasks,   wallet_seed_phrase,true).await; 
        display_add(display,offset,format!("   [Anchor Auto Stake]    result:    {:?}",result),10 as usize, 23 as usize,1 as usize); 
   
/*
        let result = anchor_borrow_claim_rewards(tasks,   wallet_seed_phrase,true).await;
        display_add(display,offset,format!("   [Anchor Auto Stake]   complete:    {:?}",result),10 as usize, 23 as usize,1 as usize); 
   
        let result = anchor_governance_stake_balance(tasks,   wallet_seed_phrase,true).await;
        display_add(display,offset,format!("   [Anchor Auto Stake]   complete:    {:?}",result),10 as usize, 23 as usize,1 as usize); 
*/ 

    }else {  
        display_add(display,offset,format!("   [Anchor Auto Stake]    next:    {}",date_next_to_auto_claim_and_stake),10 as usize, 23 as usize,1 as usize); 
    }

}

pub async fn display_anchor_account(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  display: &mut Vec<String>,offset: &mut usize) {

    display_add(display,offset,"\n  **Anchor Protocol Account**\n\n".to_string(),0 as usize, 32 as usize, 0 as usize);

    let borrow_limit = borrow_limit_to_string(tasks,  2).await; 
    let borrow_limit = format!("borrow limit: {} UST",borrow_limit);
 
    let loan_amount = borrower_loan_amount_to_string(tasks,  2).await;
    display_add(display,offset,format!("   [Anchor Borrow]    loan amount:    {} UST ({})",loan_amount,borrow_limit),10 as usize, 23 as usize,1 as usize);  
 
    let ltv = borrower_ltv_to_string(tasks,  2).await;
    display_add(display,offset,format!("   [Anchor Borrow]    loan to value:    {}",ltv),10 as usize, 23 as usize,1 as usize); 

    let pending_anc_rewards = borrower_rewards_to_string(tasks,  false,2).await; 
    let pending_anc_rewards_in_ust = borrower_rewards_in_ust_to_string(tasks,  2).await;
    display_add(display,offset,format!("   [Anchor Borrow]    pending rewards:    {} UST (={} ANC)",pending_anc_rewards_in_ust,pending_anc_rewards),10 as usize, 23 as usize,1 as usize); 

    let claim_and_stake_fees = estimate_anchor_protocol_tx_fee_claim_and_stake(tasks,  2).await;
    let claim_and_stake_fees_ratio = anchor_claim_and_stake_transaction_gas_fees_ratio_to_string(tasks,  2).await;
    display_add(display,offset,format!("   [Anchor Borrow]    fee to claim & stake:    {} (={} UST)",claim_and_stake_fees_ratio, claim_and_stake_fees),10 as usize, 23 as usize,2 as usize); 

    let total_deposited_amount = borrower_balance_to_string(tasks,  "balance",2).await;
    let ust_deposited = borrower_ust_deposited_to_string(tasks,  2).await;
    display_add(display,offset,format!("   [Anchor Earn]    balance:    {} UST (={} aUST)",ust_deposited,total_deposited_amount),10 as usize, 23 as usize,2 as usize); 
   /*
    let available_liquidity_from_ust_deposit = borrower_deposit_liquidity_to_string(tasks,  2).await;
    display_add(display,offset,format!("   [Anchor Earn]    deposit liquidity:    {}",available_liquidity_from_ust_deposit),10 as usize, 23 as usize,2 as usize); 
   */

    let anc_balance = borrower_anc_deposited_to_string(tasks,  false,4).await; 
    display_add(display,offset,format!("   [Anchor Gov]    balance:    {} ANC",anc_balance),10 as usize, 23 as usize,1 as usize); 


    let anc_staked = anc_staked_balance_to_string(tasks,  4).await;
    let anc_staked_in_ust = anc_staked_balance_in_ust_to_string(tasks,  4).await;
    display_add(display,offset,format!("   [Anchor Gov]    balance staked:    {} UST (={} ANC)",anc_staked_in_ust,anc_staked),10 as usize, 23 as usize,2 as usize); 

 
    //   add -> (=absolute returns) UST or ANC FOR DISTRIBUTION APR AND AUTO STAKING
  
    display_add(display,offset,"   _    _    Net APY    Borrow APY    Distribution APY    Earn APY    Auto Staking APY (not included in Net APY)".to_string(),10 as usize, 23 as usize,2 as usize);  


    let apy_current_deposits = apy_on_collateral_by(tasks,  "loan_amount","net_apr",2).await; 

    let borrow_apr_deposits = apy_on_collateral_by(tasks,  "loan_amount","borrow_apr",2).await; 

    let distribution_apr_deposits = apy_on_collateral_by(tasks,  "loan_amount","distribution_apr",2).await; 

    let earn_apr_deposits = apy_on_collateral_by(tasks,  "loan_amount","earn_apr",2).await;  

    let date_next_to_auto_claim_and_stake = estimate_anchor_protocol_next_claim_and_stake_tx(tasks,  "loan_amount","date_next",2).await;
    let value_next_to_auto_claim_and_stake = estimate_anchor_protocol_next_claim_and_stake_tx(tasks,  "loan_amount","value_next",2).await;
    let duration_auto_claim_and_stake = estimate_anchor_protocol_next_claim_and_stake_tx(tasks,  "loan_amount","duration_next",2).await;
    let total_returns = estimate_anchor_protocol_next_claim_and_stake_tx(tasks,  "loan_amount","total_returns",2).await; 
    let apr_auto_staking = estimate_anchor_protocol_next_claim_and_stake_tx(tasks,  "loan_amount","apr",2).await; 


    display_add(display,offset,format!("   [Anchor]    loan_amount:    {}    -{}    +{}    +{}    +{} (={} UST) Next Auto Stake: {} (every {})",apy_current_deposits,borrow_apr_deposits,distribution_apr_deposits,earn_apr_deposits,apr_auto_staking,total_returns,date_next_to_auto_claim_and_stake,duration_auto_claim_and_stake),10 as usize, 23 as usize,1 as usize);  

    let apy_current_deposits = apy_on_collateral_by(tasks,  "target_ltv","net_apr",2).await; 

    let borrow_apr_deposits = apy_on_collateral_by(tasks,  "target_ltv","borrow_apr",2).await; 

    let distribution_apr_deposits = apy_on_collateral_by(tasks,  "target_ltv","distribution_apr",2).await; 

    let earn_apr_deposits = apy_on_collateral_by(tasks,  "target_ltv","earn_apr",2).await;  

    let total_returns = estimate_anchor_protocol_next_claim_and_stake_tx(tasks,  "target_ltv","total_returns",2).await; 
    let apr_auto_staking_at_ltv = estimate_anchor_protocol_next_claim_and_stake_tx(tasks,  "target_ltv","apr",2).await; 


    display_add(display,offset,format!("   [Anchor]    target_ltv:    {}    -{}    +{}    +{}    +{} (={} UST)",apy_current_deposits,borrow_apr_deposits,distribution_apr_deposits,earn_apr_deposits,apr_auto_staking_at_ltv,total_returns),10 as usize, 23 as usize,1 as usize);  

    let apy_current_deposits = apy_on_collateral_by(tasks,  "deposit_amount","net_apr",2).await; 

    let borrow_apr_deposits = apy_on_collateral_by(tasks,  "deposit_amount","borrow_apr",2).await; 

    let distribution_apr_deposits = apy_on_collateral_by(tasks,  "deposit_amount","distribution_apr",2).await; 

    let earn_apr_deposits = apy_on_collateral_by(tasks,  "deposit_amount","earn_apr",2).await;  

    display_add(display,offset,format!("   [Anchor]    deposit_amount:    {}    -{}    +{}    +{}    +{}",apy_current_deposits,borrow_apr_deposits,distribution_apr_deposits,earn_apr_deposits,apr_auto_staking),10 as usize, 23 as usize,1 as usize);  
  

    // ADD ANC scenario
    // ANC -50%, -25%, 0%, + 25%, +50%, + 100%
  
}

pub async fn display_anchor_info(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  display: &mut Vec<String>,offset: &mut usize) {
 
    display[*offset] = "\n  **Anchor Protocol**\n\n".to_string();    
   
    *offset += 1;
    println!("{}",display.join(""));

    /* Expert Parameters
    let total_liabilities = total_liabilities_to_string(tasks,  "state anchorprotocol mmMarket",0).await;
    display[*offset] = format!("   [Anchor] stablecoins lent:        {} UST\n",total_liabilities);
    *offset += 1;
    println!("{}",display.join(""));
    
    let stablecoins_deposited = a_terra_supply_to_string(tasks,   "epoch_state anchorprotocol mmMarket",0).await; 
    display[*offset] = format!("   [Anchor] stablecoins deposited:   {} UST\n", stablecoins_deposited);
    *offset += 1;
    println!("{}",display.join(""));

    let utilization_ratio = utilization_ratio_to_string(tasks,  "state anchorprotocol mmMarket","epoch_state anchorprotocol mmMarket",2).await;
    display[*offset] = format!("   [Anchor] utilization ratio:       {}\n\n",utilization_ratio);
    //\n  *The utilization ratio quantifies a stablecoin's borrow demand relative to the amount of deposited stablecoins.\n
    *offset += 1;
    println!("{}",display.join(""));
    */

    /*  Expert Parameters
    let base_rate = base_rate_to_string(tasks,  "config anchorprotocol mmInterestModel",10).await;
    display[*offset] = format!("   [Anchor] base rate:               {}\n",base_rate);
    *offset += 1;
    println!("{}",display.join(""));
    let interest_multiplier = interest_multiplier_to_string(tasks,  "config anchorprotocol mmInterestModel",10).await;
    display[*offset] = format!("   [Anchor] interest multiplier:     {}\n",interest_multiplier);
    *offset += 1;
    println!("{}",display.join(""));

    let borrow_rate = borrow_rate_to_string(tasks,  "config anchorprotocol mmInterestModel","state anchorprotocol mmMarket","epoch_state anchorprotocol mmMarket",10).await;
    display[*offset] = format!("   [Anchor] borrow rate:             {}\n",borrow_rate);
    *offset += 1;
    println!("{}",display.join(""));
    */

    let borrow_apr = borrow_apr_to_string(tasks,  2).await;
    let borrow_apr = format!("borrow apr: {}",borrow_apr);
    //\n  *The borrow rate equation incentivizes markets to have sufficient liquidity at their equilibrium. An increase in borrow demand is met with higher borrow rates, incentivizing repayments, and restoring market liquidity.\n
   

    // TODO: figure out the distribution apy calculation from the smart contracts.
    let distribution_apr = distribution_apr_to_string(tasks,  2).await;
    let distribution_apr = format!("distribution apr: {}",distribution_apr);
    //\n  *Borrower incentives: 400M (40%) tokens are linearly released to be used as borrower incentives over a period of 4 years. \n
   
    let net_apr = net_apr_to_string(tasks,  2).await;
    display[*offset] = format!("   [Anchor Borrow] net apr:          {} ({}, {})\n",net_apr,borrow_apr,distribution_apr);
    //\n  *Borrower incentives: 400M (40%) tokens are linearly released to be used as borrower incentives over a period of 4 years. \n
    *offset += 1;
    println!("{}",display.join(""));


    let tx_fee_claim_rewards = estimate_anchor_protocol_tx_fee(tasks,  "anchor_protocol_txs_claim_rewards","fee_amount_adjusted".to_owned(),false,2).await;   
    display[*offset] =format!("   [Anchor Borrow] fee to claim:      {} UST\n\n",tx_fee_claim_rewards);
    *offset += 1;
    println!("{}",display.join(""));

    
    let staking_apy = staking_apy_to_string(tasks,  2).await;
    display[*offset] =format!("   [Anchor Gov] ANC staking apy:      {}\n",staking_apy);
    //\n  *Anchor periodically distributes portion of ANC tokens purchased from protocol fees are distributed to ANC stakers to incentivize governance participation and decrease circulating ANC supply
    *offset += 1;
    println!("{}",display.join(""));

    let tx_fee_stake_rewards = estimate_anchor_protocol_tx_fee(tasks,  "anchor_protocol_txs_staking","fee_amount_adjusted".to_owned(),false,2).await;   
    display[*offset] =format!("   [Anchor Gov] fee to stake:         {} UST\n\n",tx_fee_stake_rewards);
    *offset += 1;
    println!("{}",display.join(""));

    let earn_apy = earn_apr_to_string(tasks,  2).await;
    display[*offset] =format!("   [Anchor Earn] deposit apy:         {}\n",earn_apy);
    *offset += 1;
    println!("{}",display.join(""));

}


pub async fn display_market_info(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  display: &mut Vec<String>,offset: &mut usize) {

    display[*offset] = "\n  **Terra**\n\n".to_string();  
    *offset += 1;
    println!("{}",display.join(""));

    // replace this with gas fee info.
    let uluna_usdr = core_swap_amount_to_string(tasks,   "core_swap usdr uluna",2).await;
    display[*offset] = format!("   [Terra]  SDT    -> Luna:   {}",uluna_usdr); 
    *offset += 1;
    println!("{}",display.join(""));
    let usdr_uusd = core_swap_amount_to_string(tasks,   "core_swap uusd usdr",2).await;
    display[*offset] = format!(" (=${} UST)\n",usdr_uusd); 
    *offset += 1;
    println!("{}",display.join(""));
    let uusd_uluna = core_swap_amount_to_string(tasks,   "core_swap uluna uusd",2).await; 
    display[*offset] = format!("\n   [Terra]  Luna   -> UST:    ${}\n",uusd_uluna);   
    *offset += 1;
    println!("{}",display.join(""));
    let uluna_ubluna = simulation_swap_return_amount_to_string(tasks,  "simulation anchorprotocol uluna terraswapblunaLunaPair",4).await;
    display[*offset] = format!("   [Anchor] Luna   -> bLuna:  {}\n",uluna_ubluna);
    *offset += 1;
    println!("{}",display.join(""));
    let bond_uluna = b_luna_exchange_rate_to_string(tasks,  "state anchorprotocol bLunaHub",4).await;
    display[*offset] = format!("   [Bond]   Luna   -> bLuna:  {}\n\n",bond_uluna);
    *offset += 1;
    println!("{}",display.join(""));
    let uusd_anc = simulation_swap_return_amount_to_string(tasks,  "simulation_cw20 anchorprotocol ANC terraswapAncUstPair",2).await;
    display[*offset] = format!("   [Anchor] ANC    -> UST:    ${}\n",uusd_anc);
    *offset += 1;
    println!("{}",display.join(""));
    let aust_uust = a_terra_exchange_rate_to_string(tasks,   "epoch_state anchorprotocol mmMarket",4).await;
    display[*offset] = format!("   [Anchor] aUST   -> UST:    ${}\n",aust_uust);
    *offset += 1;
    println!("{}",display.join(""));
    let nluna_psi = simulation_swap_return_amount_to_string(tasks,   "simulation_cw20 nexusprotocol nLunaToken Psi-nLuna_Pair",2).await;
    display[*offset] = format!("\n   [Nexus] nLuna   -> PSI:    {}\n",nluna_psi);
    *offset += 1;
    println!("{}",display.join(""));
    let psi_uusd = simulation_swap_return_amount_to_string(tasks,   "simulation_cw20 nexusprotocol PsiToken Psi-UST_Pair",4).await;
    display[*offset] = format!("   [Nexus] PSI     -> UST:    ${}\n",psi_uusd);
    *offset += 1;
    println!("{}",display.join(""));
    let mir_uusd = simulation_swap_return_amount_to_string(tasks,  "simulation_cw20 uusd mir",2).await;
    display[*offset] = format!("\n   [Mirror] MIR    -> UST:    ${}\n",mir_uusd);
    *offset += 1;
    println!("{}",display.join(""));
    let m_tsla = simulation_swap_return_amount_to_string(tasks,  "simulation_cw20 uusd m_tsla",2).await;
    display[*offset] = format!("   [Mirror] mTSLA  -> UST:    ${}\n",m_tsla);
    *offset += 1;
    println!("{}",display.join(""));
    let m_spy = simulation_swap_return_amount_to_string(tasks,  "simulation_cw20 uusd m_spy",2).await;
    display[*offset] = format!("   [Mirror] mSPY   -> UST:    ${}\n",m_spy);
    *offset += 1;
    println!("{}",display.join(""));
    let m_btc = simulation_swap_return_amount_to_string(tasks,  "simulation_cw20 uusd m_btc",2).await;
    display[*offset] = format!("   [Mirror] mBTC   -> UST:    ${}\n",m_btc);
    *offset += 1;
    println!("{}",display.join(""));
    let m_eth = simulation_swap_return_amount_to_string(tasks,  "simulation_cw20 uusd m_eth",2).await;
    display[*offset] = format!("   [Mirror] mETH   -> UST:    ${}\n",m_eth);
    *offset += 1;
    println!("{}",display.join(""));
    let blocks_per_year = blocks_per_year_to_string(tasks,  "blocks_per_year",0).await;
    display[*offset] = format!("\n   [Terra] est. blocks per year:    {}\n",blocks_per_year);
    *offset += 1;
    println!("{}",display.join("")); 
   
}


