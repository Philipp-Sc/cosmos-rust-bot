pub mod model;


use std::collections::HashMap; 
use std::sync::Arc; 
use tokio::sync::RwLock; 

use core::pin::Pin;
use core::future::Future;
 
use model::get_meta_data_maybe;
use model::MaybeOrPromise;
use model::get_oldest_timestamps_of_resolved_tasks;
use model::try_register_function;


pub async fn data_is_outdated(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &[&str]) -> bool {
    match get_meta_data_maybe(&tasks, "latest_transaction").await {
    	Ok(maybe) => { 
    		if get_oldest_timestamps_of_resolved_tasks(&tasks,&req).await <= maybe.timestamp + 10 {
    			return true;
    		}
    		return false;
    	},
    	Err(_) => {
    		return false;
    	}
    }
}

pub async fn try_run_function(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, task: Pin<Box<dyn Future<Output = String> + Send + 'static>>, key: &str, is_test: bool) {
	
	let timeout_duration = 120u64;  
	/* if task hangs for some reason (awaiting data, performaing estimate, broadcasting transaction) then timeout */
       
    let mut block_duration_after_resolve = 1i64;
    /* a small duration is optimal, since the data is already there */
    /* only issue is if there just was a transaction, this is handled by ensuring that the relevant data is recent enough.*/

    if is_test {
        // each call executes an estimate, therefore have higher delay to not spam estimates.
        // since test mode does not perform transactions, there is no downside by doing this.
        block_duration_after_resolve = 30i64;
    }
    try_register_function(&tasks,key.to_owned(),task,timeout_duration, block_duration_after_resolve).await; 
    /* will register and run task if conditions are right */ 
 
}




/*
	register_value(&tasks,"anchor_auto_stake_airdrops".to_string(),msg.to_owned()).await;
		        		register_value(&tasks,"latest_transaction".to_string(),msg.to_owned()).await;
		        	
*/