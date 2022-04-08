

use terra_rust_bot_output::output::*;
use terra_rust_bot_output::output::pretty::Entry;

use crate::state::control::model::{MaybeOrPromise};
use crate::state::control::model::try_get_resolved;
use crate::state::control::model::Maybe;

use std::collections::HashMap;  

use std::sync::Arc; 
use tokio::sync::RwLock;    
use chrono::Utc;


pub async fn display_all_errors(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &[&str], state: &Arc<RwLock<Vec<Option<Entry>>>> ,offset: &mut usize) {
   
    clear_after_index(state,*offset).await;

    let mut error_view: Vec<(Entry,usize)> = Vec::new();
 
    for key in req {
        match try_get_resolved(&tasks,key).await.as_ref() {
            Ok(maybe) => {
                match maybe {
                    Maybe {data: Ok(_resolved), ..} => {
                    },
                    Maybe {data: Err(failed), timestamp} => { 
                        error_view.push((Entry {
                            timestamp: *timestamp, 
                            key: key.to_string(),
                            prefix: None,
                            value: failed.to_string(),
                            suffix: None,
                            group: Some("[Errors]".to_string()),
                        },*offset));
                        *offset += 1;
                    },
                }
            },
            Err(err) => {  
                error_view.push((Entry {
                    timestamp: Utc::now().timestamp(), 
                    key: key.to_string(),
                    prefix: None,
                    value: err.to_string(),
                    suffix: None,
                    group: Some("[Unresolved]".to_string()),
                },*offset));
                *offset += 1;
                }
            }
        } 
 
    add_view_to_state(&state, error_view).await; 
} 
