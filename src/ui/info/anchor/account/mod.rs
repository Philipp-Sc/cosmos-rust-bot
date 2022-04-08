
use terra_rust_bot_output::output::*;


use terra_rust_bot_output::output::pretty::Entry; 
use crate::state::control::model::{Maybe};

use crate::state::control::model::{MaybeOrPromise};
  
use crate::view::*;
use crate::view::interface::*; 

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;     


pub async fn display_anchor_account(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  state: &Arc<RwLock<Vec<Option<Entry>>>> ,offset: &mut usize,is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> {

    let mut anchor_view: Vec<(Entry,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> = Vec::new();


    // AIRDROP TEST
/*
    anchor_view.push((format!("{}{}","\n   [Airdrops]".truecolor(75,219,75),"  luna staking airdrops:   ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(anchor_airdrops_to_string(tasks.clone())));
    anchor_tasks.push(t);
    *offset += 1;

    
    println!("{}",anchor_claim_and_stake_airdrops(tasks.clone(),"--").await);

*/

    //anchor_view.push((format!("{}{}","\n   [Liquidation Queue]".truecolor(75,219,75),"    withdrawals:             ".purple().to_string()),*offset));
    //*offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "loan_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Account][Borrow]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrower_loan_amount_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "borrow_limit".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Account][Borrow]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrow_limit_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "loan_to_borrow_limit".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol Account][Borrow]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrower_ltv_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "anc_rewards".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Account][Borrow]".to_string()),
    },*offset)); 
  
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_in_ust_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "anc_rewards".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        group: Some("[Anchor Protocol Account][Borrow]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "fee_to_claim_and_stake".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol Account][Borrow]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(anchor_claim_and_stake_transaction_gas_fees_ratio_to_string(tasks.clone(),3)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "fee_to_claim_and_stake".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Account][Borrow]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),3)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "deposit".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Account][Earn]".to_string()),
    },*offset)); 
   
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrower_ust_deposited_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "deposit".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("aUST".to_string()),
        group: Some("[Anchor Protocol Account][Earn]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrower_balance_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1; 

    //anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    fee to claim & stake:    ".to_string(),*offset));
   /*
    let available_liquidity_from_ust_deposit = borrower_deposit_liquidity_to_string(tasks.clone(),2).await;
    display_add(format!("   [Earn]    deposit liquidity:    {}",available_liquidity_from_ust_deposit), 23 as usize,2 as usize); 
   */  
    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "anc_balance".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        group: Some("[Anchor Protocol Account][Gov]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrower_anc_deposited_to_string(tasks.clone(),false,4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "anc_staked".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Account][Gov]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(anc_staked_balance_in_ust_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "anc_staked".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        group: Some("[Anchor Protocol Account][Gov]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(anc_staked_balance_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;
  
    //   add -> (=absolute returns) UST or ANC FOR DISTRIBUTION APR AND AUTO STAKING
  
    /*

    anchor_view.push((format!("    {}",display_add("   _    _    Net APY    Borrow APY    Distribution APY    Earn APY    Auto Staking APY (not included in Net APY)".purple().to_string(), 23 as usize,2 as usize)),*offset));
    *offset += 1;
   
    anchor_view.push((display_add("   [Anchor]    loan_amount:    --".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    loan_amount:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","loan_amount","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","loan_amount","total_returns",2));
    let f = Box::pin(add_format_to_result(" (=".to_string()," UST)".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","loan_amount","date_next",2));
    let f = Box::pin(add_format_to_result(" Next Auto Stake: ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","loan_amount","duration_next",2));
    let f = Box::pin(add_format_to_result(" (every ".to_string(),")".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,1 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("   [Anchor]    target_ltv:    --".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    target_ltv:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","target_ltv","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","target_ltv","total_returns",2));
    let f = Box::pin(add_format_to_result(" (=".to_string()," UST)".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","target_ltv","date_next",2));
    let f = Box::pin(add_format_to_result(" Next Auto Stake: ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","target_ltv","duration_next",2));
    let f = Box::pin(add_format_to_result(" (every ".to_string(),")".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,1 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("   [Anchor]    deposit_amount:    --".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    deposit_amount:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","loan_amount","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,1 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    */
       
    // ADD ANC scenario
    // ANC -50%, -25%, 0%, + 25%, +50%, + 100%

    if is_first_run {
        add_view_to_state(&state, anchor_view).await; 
    }

    return anchor_tasks;
  
}