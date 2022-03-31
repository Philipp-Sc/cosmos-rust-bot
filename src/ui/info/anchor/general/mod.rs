
use crate::ui::display::*;

use crate::state::control::model::{MaybeOrPromise};
  
use crate::view::*;
use crate::view::interface::*; 

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;   
use chrono::Utc;

pub async fn display_anchor_info(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, state: &Arc<RwLock<Vec<Option<Entry>>>> ,offset: &mut usize, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {
 
    let mut anchor_view: Vec<(Entry,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();
    
    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "stablecoins_lent".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(total_liabilities_to_string(tasks.clone(),0)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "stablecoins_deposited".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(a_terra_supply_to_string(tasks.clone(),0)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "utilization_ratio".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    },*offset)); 
    //\n  *The utilization ratio quantifies a stablecoin's borrow demand relative to the amount of deposited stablecoins.\n
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(utilization_ratio_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "base_rate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    },*offset)); 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(base_rate_to_string(tasks.clone(),10)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "interest_multiplier".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    },*offset)); 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(interest_multiplier_to_string(tasks.clone(),10)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "borrow_rate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    },*offset)); 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrow_rate_to_string(tasks.clone(),"config anchorprotocol mmInterestModel","state anchorprotocol mmMarket","epoch_state anchorprotocol mmMarket",10)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "net_apr".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol Info][Borrow]".to_string()),
    },*offset)); 
 
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(net_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "borrow_apr".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol Info][Borrow]".to_string()),
    },*offset)); 
    // The borrow rate equation incentivizes markets to have sufficient liquidity at their equilibrium. An increase in borrow demand is met with higher borrow rates, incentivizing repayments, and restoring market liquidity.
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrow_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "distribution_apr".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol Info][Borrow]".to_string()),
    },*offset)); 
    // Borrower incentives: 400M (40%) tokens are linearly released to be used as borrower incentives over a period of 4 years. 
    // TODO: figure out the distribution apy calculation from the smart contracts.
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(distribution_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "fee_to_claim".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Info][Borrow]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_claim_rewards","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "anc_staking_apy".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol Info][Gov]".to_string()),
    },*offset)); 
    // Anchor periodically distributes portion of ANC tokens purchased from protocol fees are distributed to ANC stakers to incentivize governance participation and decrease circulating ANC supply
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(staking_apy_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "fee_to_stake".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Info][Gov]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_staking","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((Entry {
        timestamp: Utc::now().timestamp(), 
        key: "deposit_apy".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol Info][Earn]".to_string()),
    },*offset)); 
    
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(earn_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    if is_first_run {
        add_view_to_state(&state, anchor_view).await;
    }

    return anchor_tasks;

}
 
