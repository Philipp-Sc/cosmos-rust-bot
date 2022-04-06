
use terra_rust_bot_output::output::*;
use terra_rust_bot_output::output::pretty::Entry;

use crate::state::control::model::{MaybeOrPromise};
use crate::state::control::model::try_get_resolved;
use crate::state::control::model::Maybe;


use crate::view::*;  

use std::collections::HashMap;  

use std::sync::Arc; 
use tokio::sync::RwLock;    


use chrono::{Utc};


pub async fn display_all_logs(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, state: &Arc<RwLock<Vec<Option<Entry>>>> ,offset: &mut usize, args_b: &Vec<&str>) {
   
    if args_b.len() == 0 {
        return;
    }

    let mut log_view: Vec<(Entry,usize)> = Vec::new();
    
    let vec = vec![
    "anchor_redeem_and_repay_stable",
    "anchor_borrow_and_deposit_stable",
    "anchor_governance_claim_and_stake"
    ];
    for key in vec {

        match try_get_resolved(&tasks,key).await.as_ref() {
            Ok(maybe) => {
                match maybe {
                    Maybe {data: Ok(resolved), timestamp} => { 
                        log_view.push((Entry {
                            timestamp: *timestamp, 
                            key: key.to_string(),
                            prefix: None,
                            value: resolved.as_text().unwrap_or(&"Error: Could not parse value.".to_string()).to_string(),
                            suffix: None,
                            group: Some("[Logs]".to_string()),
                        },*offset));
                        *offset += 1;
                    },
                    Maybe {data: Err(_failed), ..} => {
                    },
                }
            },
            Err(_) => { // not yet resolved
                }
            }
 

    }  

    add_view_to_state(&state, log_view).await; 
}