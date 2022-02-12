pub mod lazy_info;

use secstr::*;
use lazy_info::lazy_logs::display::*; 
use lazy_info::lazy_logs::control::view::interface::model::{MaybeOrPromise,try_register_function,await_function};
  
use lazy_info::lazy_logs::control::view::*;
use lazy_info::lazy_logs::control::view::interface::*;
use lazy_info::lazy_logs::control::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;   
use colored::*;
  
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

 pub async fn lazy_anchor_account_auto_repay(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_acc_address: &Arc<SecUtf8>, wallet_seed_phrase: &Arc<SecUtf8>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)>  {

    let mut anchor_view: Vec<(String,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol Auto Repay**\n\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Repay]".truecolor(75,219,75),"                    loan amount:              ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_loan_amount_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST (borrow limit: ".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrow_limit_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST)".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Auto Repay]".truecolor(75,219,75),"                    loan to borrow limit:     ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_ltv_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((format!("{}{}","\n\n   [Auto Repay]".truecolor(75,219,75),"                    left to trigger:          ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(check_anchor_loan_status(tasks.clone(),"repay",2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((format!("{}{}","\n   [Auto Repay]".truecolor(75,219,75),"                    to repay:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_amount(tasks.clone(),"repay",false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

/*
    let available_to_repay = min_ust_balance_to_string(tasks.clone(),false,2).await;
    add_string_to_display(new_display,*offset,format!("{}{}","\n\n\n   [Auto Repay UST]                account limit:                   ".truecolor(75,219,75), format!("{} UST",available_to_repay).yellow())).await.ok(); 
    *offset += 1;
 
    let available_to_repay = calculate_repay_plan(tasks.clone(),"ust_available_to_repay",2).await;
    add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Repay UST]                available UST:                   ".truecolor(75,219,75), format!("{} UST",available_to_repay).yellow())).await.ok(); 
    *offset += 1;
*/
    anchor_view.push((format!("{}{}","\n\n   [Auto Repay UST]".truecolor(75,219,75),"                amount:                   ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"to_withdraw_from_account",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

/*
    let available_in_deposit = calculate_repay_plan(tasks.clone(),"available_in_deposit",2).await;
    add_string_to_display(new_display,*offset,format!("{}{}","\n\n   [Auto Repay Redeem]         max amount:                      ".truecolor(75,219,75), format!("{} UST",available_in_deposit).yellow())).await.ok(); 
    *offset += 1;
*/
    anchor_view.push((format!("{}{}","\n   [Auto Repay Redeem]".truecolor(75,219,75),"             amount:                   ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"to_withdraw_from_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // does include gas_adjustment 
    anchor_view.push((format!("{}{}","\n   [Auto Repay Redeem]".truecolor(75,219,75),"             est. gas:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_redeem_stable","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Repay]".truecolor(75,219,75),"                    repay:                    ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"to_repay",2)));
    anchor_tasks.push(t);
    *offset += 1;   

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Auto Repay]".truecolor(75,219,75),"                    est. gas:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_repay_stable","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n   [Auto Repay]".truecolor(75,219,75),"                    est. stability fee:       ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"stability_tax",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Repay Transaction]".truecolor(75,219,75),"        amount:                   ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_repay_plan(tasks.clone(),"total_amount",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // total fee
    anchor_view.push((format!("{}{}","\n   [Auto Repay Transaction]".truecolor(75,219,75),"        est. fee:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_auto_repay_tx_fee(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    let mut field = "result:  ";

    if is_test {
        field = "estimate:";
    }

    anchor_view.push((format!("{}{}","\n\n   [Auto Repay]".truecolor(75,219,75),format!("                    {}                 ",field.purple())),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    
    // function able to execute auto repay, therefore registering it as task to run concurrently. 
    let important_task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_redeem_and_repay_stable(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(),is_test));
    let timeout_duration = 120u64;  /* if task hangs for some reason (awaiting data, performaing estimate, broadcasting transaction) then timeout */
    
    let mut block_duration_after_resolve = 1i64;
    /* a small duration is optimal, since the data is already there */
    /* only issue is if there just was a transaction, this is handled by ensuring that the relevant data is recent enough.*/

    if is_test {
        // each call executes an estimate, therefore have higher delay to not spam estimates.
        // since test mode does not perform transactions, there is no downside by doing this.
        block_duration_after_resolve = 30i64;
    }
    try_register_function(&tasks,"anchor_auto_repay".to_owned(),important_task,timeout_duration, block_duration_after_resolve).await;  
          
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_repay".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push(("\n\n".to_string(),*offset));
    *offset += 1;

    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }



    return anchor_tasks;
 
}


 pub async fn lazy_anchor_account_auto_borrow(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_acc_address: &Arc<SecUtf8>, wallet_seed_phrase: &Arc<SecUtf8>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)>  {

    let mut anchor_view: Vec<(String,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol Auto Borrow**\n\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

     anchor_view.push((format!("{}{}","\n\n   [Auto Borrow]".truecolor(75,219,75),"                    loan amount:              ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_loan_amount_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST (borrow limit: ".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrow_limit_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST)".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Auto Borrow]".truecolor(75,219,75),"                    loan to borrow limit:     ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_ltv_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;



    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow]".truecolor(75,219,75),"                    left to trigger:          ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(check_anchor_loan_status(tasks.clone(),"borrow",2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((format!("{}{}","\n   [Auto Borrow]".truecolor(75,219,75),"                    to borrow:                ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_amount(tasks.clone(),"borrow",false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // does include gas_adjustment
    anchor_view.push((format!("{}{}","\n   [Auto Borrow]".truecolor(75,219,75),"                    est. gas:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_borrow_stable","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;
 
     // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n   [Auto Borrow]".truecolor(75,219,75),"                    est. stability fee:       ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"stability_tax_borrow",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow]".truecolor(75,219,75),"                    to deposit:               ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"to_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Auto Borrow Deposit]".truecolor(75,219,75),"            est. gas:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_deposit_stable","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n   [Auto Borrow Deposit]".truecolor(75,219,75),"            est. stability fee:       ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"stability_tax_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // total fee
    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow Transaction]".truecolor(75,219,75),"        est. fee:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_auto_borrow_tx_fee(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

 
  
    let mut field = "result:  ";

    if is_test {
        field = "estimate:";
    }

    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow]".truecolor(75,219,75),format!("                    {}                 ",field.purple())),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    
    // function able to execute auto repay, therefore registering it as task to run concurrently. 
    let important_task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_borrow_and_deposit_stable(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(),is_test));
    let timeout_duration = 120u64;  /* if task hangs for some reason (awaiting data, performaing estimate, broadcasting transaction) then timeout */
    
    let mut block_duration_after_resolve = 1i64;
    /* a small duration is optimal, since the data is already there */
    /* only issue is if there just was a transaction, this is handled by ensuring that the relevant data is recent enough.*/

    if is_test {
        // each call executes an estimate, therefore have higher delay to not spam estimates.
        // since test mode does not perform transactions, there is no downside by doing this.
        block_duration_after_resolve = 30i64;
    }
    try_register_function(&tasks,"anchor_auto_borrow".to_owned(),important_task,timeout_duration, block_duration_after_resolve).await;  
          
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_borrow".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;
   

    anchor_view.push(("\n\n".to_string(),*offset));
    *offset += 1;


    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }


    return anchor_tasks;
 
}

/**
 * Anchor Auto Stake requires that the account balance has sufficient funds.
 * Info: It will not replenish the account balance. 
 * */
pub async fn lazy_anchor_account_auto_stake_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_acc_address: &Arc<SecUtf8>, wallet_seed_phrase: &Arc<SecUtf8>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {
     
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
    
    // function able to execute auto stake, therefore registering it as task to run concurrently. 
    let important_task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_stake_rewards(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(),is_test));
    let timeout_duration = 120u64;
    let mut block_duration_after_resolve = 10i64;
    /* a small duration is optimal, since the data is already there */
    /* only issue is if there just was a transaction, this is handled by ensuring that the relevant data is recent enough.*/

    if is_test {
        // each call executes an estimate, therefore have higher delay to not spam estimates.
        // since test mode does not perform transactions, there is no downside by doing this.
        block_duration_after_resolve = 30i64;
    }
    try_register_function(&tasks,"anchor_auto_stake".to_owned(),important_task,timeout_duration, block_duration_after_resolve).await;  
          
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

/**
 * Anchor Auto Farm requires that the account balance has sufficient funds.
 * Info: It will not replenish the account balance. 
 * */
pub async fn lazy_anchor_account_auto_farm_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_acc_address: &Arc<SecUtf8>, wallet_seed_phrase: &Arc<SecUtf8>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {


    let mut anchor_view: Vec<(String,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();


    anchor_view.push(("\n  **Anchor Protocol Auto Farm**\n\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

     // Pending ANC Rewards
    
    anchor_view.push((format!("{}{}","\n   [Auto Borrow Rewards]".truecolor(75,219,75),"   amount:            ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_in_ust_to_string(tasks.clone(),  2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST (=".purple().to_string(),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" ANC)".purple().to_string(),*offset));
    *offset += 1;

    // Fee to claim ANC rewards

    anchor_view.push((format!("{}{}","\n\n   [Anchor Claim]".truecolor(75,219,75),"          est. fee:          ".purple()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee_claim(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1; 

    // Fee to provide ANC rewards to spec

    anchor_view.push((format!("{}{}","\n   [SPEC Provide]".truecolor(75,219,75),"          est. fee:          ".purple()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_spec_tx_fee_provide(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((format!("{}{}","\n\n   [Anchor]".truecolor(75,219,75),"                anc_to_keep:       ".purple()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_farm_plan(tasks.clone(),"anc_to_keep",false,2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((format!("{}{}","\n   [Anchor]".truecolor(75,219,75),"                anc_to_swap:       ".purple()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_farm_plan(tasks.clone(),"anc_to_swap",false,2)));
    anchor_tasks.push(t);
    *offset += 1; 


    anchor_view.push((format!("{}{}","\n\n   [Anchor]".truecolor(75,219,75),"                belief_price:      ".purple()),*offset));
    *offset += 1; 

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_exchange_rate_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((format!("{}{}","\n   [Anchor]".truecolor(75,219,75),"                max_spread:        0.001".purple()),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n\n   [Auto Farm Rewards]".truecolor(75,219,75),"     target:            ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"farming","loan_amount","value_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Auto Farm]".truecolor(75,219,75),"             next:              ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"farming","loan_amount","date_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
    
    let mut field = "result:  ";

    if is_test {
        field = "estimate:";
    }

    anchor_view.push((format!("{}{}","\n\n   [Auto Farm]".truecolor(75,219,75),format!("             {}          ",field.purple())),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    
    // function able to execute auto stake, therefore registering it as task to run concurrently. 
    let important_task: Pin<Box<dyn Future<Output = String> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_farm_rewards(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(),is_test));
    let timeout_duration = 120u64;
    let mut block_duration_after_resolve = 10i64;
    /* a small duration is optimal, since the data is already there */
    /* only issue is if there just was a transaction, this is handled by ensuring that the relevant data is recent enough.*/

    if is_test {
        // each call executes an estimate, therefore have higher delay to not spam estimates.
        // since test mode does not perform transactions, there is no downside by doing this.
        block_duration_after_resolve = 30i64;
    }
    try_register_function(&tasks,"anchor_auto_farm".to_owned(),important_task,timeout_duration, block_duration_after_resolve).await;  
          
    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_farm".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;

    
    anchor_view.push(("\n\n".to_string(),*offset));
    *offset += 1;


    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }     

    return anchor_tasks;
}