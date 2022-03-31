use crate::ui::display::*; 
use crate::state::control::model::{MaybeOrPromise,await_function};

use crate::view::*;
use crate::view::interface::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;    
use chrono::{Utc};


/**
 * Anchor Auto Stake requires that the account balance has sufficient funds.
 * Info: It will not replenish the account balance. 
 * */
pub async fn lazy_anchor_account_auto_stake_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, state: &Arc<RwLock<Vec<Option<Entry>>>> ,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {
     
    let mut anchor_view: Vec<(Entry,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();
 
    anchor_view.push((Entry {
            timestamp: Utc::now().timestamp(), 
            key: "ust_balance".to_string(),
            prefix: None,
            value: "--".to_string(),
            suffix: Some("UST".to_string()),
            group: Some("[Anchor Protocol][Auto Stake][UST]".to_string()),
        },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(terra_balance_to_string(tasks.clone(),"uusd",false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
            timestamp: Utc::now().timestamp(), 
            key: "anc_balance".to_string(),
            prefix: None,
            value: "--".to_string(),
            suffix: Some("ANC".to_string()),
            group: Some("[Anchor Protocol][Auto Stake][Gov]".to_string()),
        },*offset)); 
  
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_anc_deposited_to_string(tasks.clone(),false,4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "anc_staked".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Stake][Gov]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anc_staked_balance_in_ust_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "anc_staked".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        group: Some("[Anchor Protocol][Auto Stake][Gov]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anc_staked_balance_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;
  
    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "anc_rewards".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Stake][Rewards]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_in_ust_to_string(tasks.clone(),  2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "anc_rewards_target".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Stake][Rewards]".to_string()),
    },*offset)); 
  
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","loan_amount","value_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "anc_rewards_target_date".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Stake][Rewards]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","loan_amount","date_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "claim_rewards_tx_gas_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Stake][Claim Tx]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "stake_rewards_tx_gas_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Stake][Stake Tx]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "auto_stake_tx_fee_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Stake][Tx]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;
    
    let mut field = "auto_stake_tx_result";
    if is_test {
        field = "auto_stake_tx_estimate";
    }
 
    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: field.to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Stake][Tx]".to_string()),
    },*offset)); 
       
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_stake".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;

    if is_first_run {
        add_view_to_state(&state, anchor_view).await; 
    }     

    return anchor_tasks;
}
 