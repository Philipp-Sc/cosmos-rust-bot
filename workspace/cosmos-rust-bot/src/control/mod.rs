/*use std::collections::HashMap;
use std::sync::Arc;
use cosmos_rust_interface::cosmos_rust_package::tokio::sync::{Mutex};
use cosmos_rust_interface::cosmos_rust_package::tokio::task::JoinSet;
use core::pin::Pin;
use core::future::Future;
use cosmos_rust_interface::utils::response::ResponseResult;

use crate::model::get_timestamps_of_tasks;
//use crate::model::try_register_function;
use cosmos_rust_interface::utils::entry::{Maybe};
use crate::model::access_maybe;
*/
/*
pub async fn data_is_outdated(memory: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, req: &[&str]) -> bool {
    match access_maybe(&memory, "latest_transaction").await {
        Maybe { data: Ok(_), timestamp } => {
            let mut timestamps = get_timestamps_of_tasks(&memory).await.iter().filter(|x| req.contains(&x.0.as_str())).map(|x| x.1).collect::<Vec<i64>>();
            timestamps.sort();
            if timestamps.len() > 0 && timestamps[0] + 10 > timestamp {
                return true;
            }
            return false;
        }
        Maybe { data: Err(_), .. } => {
            return false;
        }
    }
}*/
/*
pub async fn try_run_function(join_set: &mut JoinSet<()>, memory: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, task: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>, key: &str, is_test: bool) {
    let timeout_duration = 120u64;
    /* if task hangs for some reason (awaiting data, performing estimate, broadcasting transaction) then timeout */

    let mut block_duration_after_resolve = 1i64;
    /* a small duration is optimal, since the data is already there */
    /* only issue is if there just was a transaction, this is handled by ensuring that the relevant data is recent enough.*/

    if is_test {
        // each call executes an estimate, therefore have higher delay to not spam estimates.
        // since test mode does not perform transactions, there is no downside by doing this.
        block_duration_after_resolve = 30i64;
    }
    try_register_function(join_set, memory, key.to_owned(), task, timeout_duration, block_duration_after_resolve).await;
    /* will register and run task if conditions are right */
}*/




/*
	register_value(memory,"anchor_auto_stake_airdrops".to_string(),msg.to_owned()).await;
		        		register_value(memory,"latest_transaction".to_string(),msg.to_owned()).await;

*/