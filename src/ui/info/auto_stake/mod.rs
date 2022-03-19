use crate::ui::display::*; 
use crate::state::control::model::{MaybeOrPromise,await_function};

use crate::view::*;
use crate::view::interface::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;   
use colored::*;

/**
 * Anchor Auto Stake requires that the account balance has sufficient funds.
 * Info: It will not replenish the account balance. 
 * */
pub async fn lazy_anchor_account_auto_stake_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {
     
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
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","loan_amount","value_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Auto Stake]".truecolor(75,219,75),"             next:              ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","loan_amount","date_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    // est fees.
    anchor_view.push((format!("{}{}","\n\n   [Auto Stake Claim Tx]".truecolor(75,219,75),"    est. gas:          ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Auto Stake Stake Tx]".truecolor(75,219,75),"    est. gas:          ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
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
       
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_stake".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;


    
    anchor_view.push(("\n\n".to_string(),*offset));
    *offset += 1;


    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }     

    return anchor_tasks;
}
 