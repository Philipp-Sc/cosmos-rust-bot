use secstr::*;
use display_utils::display::*; 
use terra_rust_bot_backend::control::view::interface::model::{MaybeOrPromise,try_register_function,await_function};
  
use terra_rust_bot_backend::control::view::*;
use terra_rust_bot_backend::control::view::interface::*;
use terra_rust_bot_backend::control::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;   
use colored::*;
   

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
 