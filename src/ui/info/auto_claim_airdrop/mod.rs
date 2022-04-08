use terra_rust_bot_output::output::*;


use terra_rust_bot_output::output::pretty::Entry; 
use crate::state::control::model::{Maybe};
use crate::state::control::model::{MaybeOrPromise};

use crate::view::*;
use crate::view::interface::*;

//use crate::action::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;   
use chrono::Utc;
 
pub async fn lazy_anchor_account_auto_claim_airdrop(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_acc_address: &Arc<SecUtf8>, wallet_seed_phrase: &Arc<SecUtf8>, state: &Arc<RwLock<Vec<Option<Entry>>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> {
     
    let mut anchor_view: Vec<(Entry,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> = Vec::new();

    anchor_view.push((Entry {
        timestamp: 0i64, 
        key: "balance".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Claim Airdrops]".to_string()),
    },*offset));
 

    anchor_view.push((format!("{}{}","\n\n   [Auto Stake UST]".truecolor(75,219,75),"         balance:           ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(terra_balance_to_string(tasks.clone(),"uusd",false,2)));
    anchor_tasks.push(t);
    *offset += 1;
 
    /*
    
    let mut field = "result:  ";

    if is_test {
        field = "estimate:";
    }

    anchor_view.push((format!("{}{}","\n\n   [Auto Stake]".truecolor(75,219,75),format!("             {}          ",field.purple())),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    
    // function able to execute auto stake, therefore registering it as task to run concurrently. 
    let important_task: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(anchor_borrow_claim_and_stake_rewards(tasks.clone(), wallet_acc_address.clone(), wallet_seed_phrase.clone(),is_test));
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
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_stake".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;

    */

    
    anchor_view.push(("\n\n".to_string(),*offset));
    *offset += 1;


    if is_first_run {
        add_view_to_state(&state, anchor_view).await; 
    }     

    return anchor_tasks;
}
 