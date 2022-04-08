use terra_rust_bot_output::output::*;


use terra_rust_bot_output::output::pretty::Entry; 
use crate::state::control::model::{Maybe};
//use crate::state::control::model::{MaybeOrPromise,try_register_function,await_function};

use crate::state::control::model::{MaybeOrPromise,await_function};

use crate::view::*;
use crate::view::interface::*;

//use crate::action::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;     

 pub async fn lazy_anchor_account_auto_borrow(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, state: &Arc<RwLock<Vec<Option<Entry>>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)>  {

    let mut anchor_view: Vec<(Entry,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> = Vec::new();

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "loan_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Borrow]".to_string()),
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
        group: Some("[Anchor Protocol][Auto Borrow]".to_string()),
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
        group: Some("[Anchor Protocol][Auto Borrow]".to_string()),
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
        group: Some("[Anchor Protocol][Auto Borrow]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(check_anchor_loan_status(tasks.clone(),"borrow",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "to_borrow".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Borrow][Borrow]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_amount(tasks.clone(),"borrow",false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
            timestamp: 0i64, 
            key: "borrow_stable_tx_gas_estimate".to_string(),
            prefix: None,
            value: "--".to_string(),
            suffix: None,
            group: Some("[Anchor Protocol][Auto Borrow][Borrow]".to_string()),
        },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_borrow_stable","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;
    /*
     // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   est. stability fee:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"stability_tax_borrow",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    */

    anchor_view.push((Entry {
            timestamp: 0i64, 
            key: "to_deposit".to_string(),
            prefix: None,
            value: "--".to_string(),
            suffix: Some("UST".to_string()),
            group: Some("[Anchor Protocol][Auto Borrow][Deposit]".to_string()),
        },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"to_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "deposit_stable_tx_gas_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Borrow][Deposit]".to_string()),
    },*offset)); 
  
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_deposit_stable","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    /*
    // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   est. stability fee:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"stability_tax_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
    */

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "auto_borrow_tx_fee_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Borrow][Transaction]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_auto_borrow_tx_fee(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;
 
    let mut field = "auto_borrow_tx_result";
    if is_test {
        field = "auto_borrow_tx_estimate";
    }
 
    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: field.to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Borrow][Transaction]".to_string()),
    },*offset)); 
         
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_borrow".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;
 
    if is_first_run {
        add_view_to_state(&state, anchor_view).await; 
    }
 
    return anchor_tasks;
 
}
 