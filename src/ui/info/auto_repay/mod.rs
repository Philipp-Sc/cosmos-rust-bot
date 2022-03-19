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

 pub async fn lazy_anchor_account_auto_repay(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)>  {

    let mut anchor_view: Vec<(String,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol Auto Repay**\n\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Repay]".truecolor(75,219,75),"                    loan amount:              ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_loan_amount_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST (borrow limit: ".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrow_limit_to_string(tasks.clone(),false,2)));
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