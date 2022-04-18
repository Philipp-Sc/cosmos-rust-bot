use terra_rust_bot_essentials::output::*;


use terra_rust_bot_essentials::shared::Entry;
use crate::state::control::model::{Maybe};
use crate::state::control::model::{MaybeOrPromise,await_function};

use crate::view::*;
use crate::view::interface::*;

//use crate::action::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;   
   
/**
 * Anchor Auto Farm requires that the account balance has sufficient funds.
 * Info: It will not replenish the account balance. 
 * */
pub async fn lazy_anchor_account_auto_farm_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, state: &Arc<RwLock<Vec<Option<Entry>>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> {


    let mut anchor_view: Vec<(Entry,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> = Vec::new();

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "anc_rewards".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Farm][Rewards]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_in_ust_to_string(tasks.clone(),  2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "anc_rewards".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        group: Some("[Anchor Protocol][Auto Farm][Rewards]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "claim_rewards_tx_fee_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Farm][Claim Tx]".to_string()),
    },*offset)); 
  
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee_claim(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "spec_provide_rewards_tx_fee_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Farm][SPEC Provide Tx]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_spec_tx_fee_provide(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "anc_to_keep".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        group: Some("[Anchor Protocol][Auto Farm]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_farm_plan(tasks.clone(),"anc_to_keep",false,2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "anc_to_swap".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        group: Some("[Anchor Protocol][Auto Farm]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(calculate_farm_plan(tasks.clone(),"anc_to_swap",false,2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "belief_price".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Farm]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_exchange_rate_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "max_spread".to_string(),
        prefix: None,
        value: "0.001".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Farm]".to_string()),
    },*offset)); 
    *offset += 1;
    
    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "target".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Farm][Rewards]".to_string()),
    },*offset)); 
   
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"farming","loan_amount","value_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "next".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Farm]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"farming","loan_amount","date_next",2)));
    anchor_tasks.push(t);
    *offset += 1;
     
    let mut field = "auto_farm_tx_result";
    if is_test {
        field = "auto_farm_tx_estimate";
    }
 
    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: field.to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Farm][Tx]".to_string()),
    },*offset)); 
  
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_farm".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;

    if is_first_run {
        add_view_to_state(&state, anchor_view).await; 
    }     

    return anchor_tasks;
}