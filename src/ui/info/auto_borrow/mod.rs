use crate::ui::display::*;
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
use colored::*;
   

 pub async fn lazy_anchor_account_auto_borrow(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)>  {

    let mut anchor_view: Vec<(String,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol Auto Borrow**\n\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow]\n".truecolor(75,219,75),"   loan amount:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_loan_amount_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST \n   borrow limit:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrow_limit_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   loan to borrow limit:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_ltv_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   left to trigger:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(check_anchor_loan_status(tasks.clone(),"borrow",2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   to borrow:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_amount(tasks.clone(),"borrow",false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // does include gas_adjustment
    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   est. gas:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_borrow_stable","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;
 
     // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   est. stability fee:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"stability_tax_borrow",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   to deposit:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"to_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Auto Borrow Deposit]\n".truecolor(75,219,75),"   est. gas:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_deposit_stable","avg_gas_used".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   est. stability fee:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_borrow_plan(tasks.clone(),"stability_tax_deposit",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    // total fee
    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow Transaction]\n".truecolor(75,219,75),"   est. fee:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_auto_borrow_tx_fee(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
 
    let mut field = "   result:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}";

    if is_test {
        field = "   estimate:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}";
    }

    anchor_view.push((format!("{}{}","\n\n   [Auto Borrow]\n".truecolor(75,219,75),format!("{}",field.purple())),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
         
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
 