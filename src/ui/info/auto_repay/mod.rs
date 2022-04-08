use terra_rust_bot_output::output::*;


use terra_rust_bot_output::output::pretty::Entry; 
use crate::state::control::model::{Maybe}; 
use crate::state::control::model::{MaybeOrPromise,await_function};

use crate::view::*;
use crate::view::interface::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;     

  
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

 pub async fn lazy_anchor_account_auto_repay(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, state: &Arc<RwLock<Vec<Option<Entry>>>> ,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)>  {

    let mut anchor_view: Vec<(Entry,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> = Vec::new();

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "loan_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
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
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
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
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrower_ltv_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "left_to_trigger".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(check_anchor_loan_status(tasks.clone(),"repay",2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "ideal_repay_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_amount(tasks.clone(),"repay",false,2)));
    anchor_tasks.push(t);
    *offset += 1;

/*
    let available_to_repay = min_ust_balance_to_string(tasks.clone(),false,2).await;
    add_string_to_display(new_display,*offset,format!("{}{}","\n\n\n   [Auto Repay UST]                account limit:                   ".truecolor(75,219,75), format!("{} UST",available_to_repay).yellow())).await.ok(); 
    *offset += 1;
 
    let available_to_repay = calculate_repay_plan(tasks.clone(),"ust_available_to_repay",2).await;
    add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Repay UST]                available UST:                   ".truecolor(75,219,75), format!("{} UST",available_to_repay).yellow())).await.ok(); 
    *offset += 1;
*/
    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "to_withdraw_from_account".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"to_withdraw_from_account",2)));
    anchor_tasks.push(t);
    *offset += 1;

/*
    let available_in_deposit = calculate_repay_plan(tasks.clone(),"available_in_deposit",2).await;
    add_string_to_display(new_display,*offset,format!("{}{}","\n\n   [Auto Repay Redeem]         max amount:                      ".truecolor(75,219,75), format!("{} UST",available_in_deposit).yellow())).await.ok(); 
    *offset += 1;
*/

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "redeem_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Repay][Redeem]".to_string()),
    },*offset)); 
  
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"to_withdraw_from_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
            timestamp: 0i64, 
            key: "redeem_stable_tx_gas_estimate".to_string(),
            prefix: None,
            value: "--".to_string(),
            suffix: None,
            group: Some("[Anchor Protocol][Auto Repay][Redeem]".to_string()),
        },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_redeem_stable","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
            timestamp: 0i64, 
            key: "to_repay".to_string(),
            prefix: None,
            value: "--".to_string(),
            suffix: Some("UST".to_string()),
            group: Some("[Anchor Protocol][Auto Repay][Repay]".to_string()),
        },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"to_repay",2)));
    anchor_tasks.push(t);
    *offset += 1;   

    anchor_view.push((Entry {
            timestamp: 0i64, 
            key: "repay_stable_tx_gas_estimate".to_string(),
            prefix: None,
            value: "--".to_string(),
            suffix: None,
            group: Some("[Anchor Protocol][Auto Repay][Repay]".to_string()),
        },*offset)); 
  
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_repay_stable","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;
 
    /*
    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   est. stability fee:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"stability_tax",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;*/

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "total_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Repay][Transaction]".to_string()),
    },*offset)); 
 
  
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"total_amount",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "auto_repay_tx_fee_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Repay][Transaction]".to_string()),
    },*offset)); 
  
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_auto_repay_tx_fee(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    let mut field = "auto_repay_tx_result";
    if is_test {
        field = "auto_repay_tx_estimate";
    }
 
    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: field.to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Repay][Transaction]".to_string()),
    },*offset)); 
  
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_repay".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;

    if is_first_run {
        add_view_to_state(&state, anchor_view).await; 
    }

    return anchor_tasks;
 
}