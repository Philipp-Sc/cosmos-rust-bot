
use terra_rust_bot_output::output::*;
use terra_rust_bot_output::output::pretty::Entry;

use crate::state::control::model::{MaybeOrPromise};
  
use crate::view::*;  

use std::collections::HashMap;  

use std::sync::Arc; 
use tokio::sync::RwLock;    


use chrono::{Utc};


pub async fn display_all_logs(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, state: &Arc<RwLock<Vec<Option<Entry>>>> ,offset: &mut usize, args_b: &Vec<&str>) {
   
    if args_b.len() == 0 {
        return;
    }

    let mut log_view: Vec<(Entry,usize)> = Vec::new();
    
    if args_b.contains(&"anchor_auto_repay") {

        let auto_repay = get_past_transaction_logs(tasks.clone(),"anchor_redeem_and_repay_stable").await;

        log_view.push((Entry {
            timestamp: Utc::now().timestamp(), 
            key: "anchor_auto_repay".to_string(),
            prefix: None,
            value: auto_repay.to_string(),
            suffix: None,
            group: Some("[Logs]".to_string()),
        },*offset));
 
        *offset += 1;

    }
    if args_b.contains(&"anchor_auto_borrow") {

        let auto_borrow = get_past_transaction_logs(tasks.clone(),"anchor_borrow_and_deposit_stable").await;

        log_view.push((Entry {
            timestamp: Utc::now().timestamp(), 
            key: "anchor_auto_borrow".to_string(),
            prefix: None,
            value: auto_borrow.to_string(),
            suffix: None,
            group: Some("[Logs]".to_string()),
        },*offset));
 
        *offset += 1;

    }

    if args_b.contains(&"anchor_auto_stake") {

        let auto_stake = get_past_transaction_logs(tasks.clone(),"anchor_governance_claim_and_stake").await;

        log_view.push((Entry {
            timestamp: Utc::now().timestamp(), 
            key: "anchor_auto_stake".to_string(),
            prefix: None,
            value: auto_stake.to_string(),
            suffix: None,
            group: Some("[Logs]".to_string()),
        },*offset));
 
        *offset += 1;
    } 

    add_view_to_state(&state, log_view).await; 
}