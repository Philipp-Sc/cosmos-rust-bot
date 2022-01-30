pub mod display; 
pub mod control;

 
use display::*; 
use control::view::interface::model::{MaybeOrPromise};
  
use control::view::*;  

use std::collections::HashMap;  

use std::sync::Arc; 
use tokio::sync::RwLock;   
use colored::*;
  

pub async fn display_all_logs(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize, args_b: &Vec<&str>) {
   
    if args_b.len() == 0 {
        return;
    }

    let mut log_view: Vec<(String,usize)> = Vec::new();

    log_view.push(("\n\n  **Logs**\n\n".yellow().to_string(),*offset));
    *offset += 1;
   
    
    if args_b.contains(&"anchor_auto_repay") {

        let auto_repay = get_past_transaction_logs(tasks.clone(),"anchor_redeem_and_repay_stable").await;

        log_view.push((format!("{}{}","\n   [Auto Repay]            ".yellow(), auto_repay.yellow()),*offset));
        *offset += 1;

    }
    if args_b.contains(&"anchor_auto_borrow") {

        let auto_borrow = get_past_transaction_logs(tasks.clone(),"anchor_borrow_and_deposit_stable").await;

        log_view.push((format!("{}{}","\n   [Auto Borrow]           ".yellow(), auto_borrow.yellow()),*offset));
        *offset += 1;

    }

    if args_b.contains(&"anchor_auto_stake") {

        let auto_stake = get_past_transaction_logs(tasks.clone(),"anchor_governance_claim_and_stake").await;

        log_view.push((format!("{}{}","\n   [Auto Stake]            ".yellow(), auto_stake.yellow()),*offset)); 
        *offset += 1;

    }
    log_view.push(("\n".to_string(),*offset));
    *offset += 1;

    add_view_to_display(&new_display, log_view).await; 
}


pub async fn display_all_errors(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &[&str], new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize) {
   
    let mut error_view: Vec<(String,usize)> = Vec::new();

    error_view.push(("\n\n  **Errors**\n\n".red().to_string(),*offset));
    *offset += 1;
  
    // clear the previous error messages. 
    for x in *offset..new_display.read().await.len(){
        error_view.push(("".to_string(),x));
    }

    let mut error_count = 0;
    for key in req {
        match anything_to_err(tasks.clone(),key).await.as_ref() {
            "--" => {
            },
            e => {
                if !e.contains("Info: Key '"){
                    error_count = error_count +1;
                    error_view.push((format!("\n   [Key] '{}'\n   {}\n",key,e).yellow().to_string(),*offset));
                    *offset += 1; 
                }
            }
        } 
    }
    if error_count == 0 {
        error_view.push(("\n   None \n\n".red().to_string(),*offset)); 
        *offset += 1; 
    }

    add_view_to_display(&new_display, error_view).await; 
} 
