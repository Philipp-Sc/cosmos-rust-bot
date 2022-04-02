

use terra_rust_bot_output::output::*;
use terra_rust_bot_output::output::pretty::Entry;

use crate::state::control::model::{MaybeOrPromise};
  
use crate::view::*;  

use std::collections::HashMap;  

use std::sync::Arc; 
use tokio::sync::RwLock;    
use chrono::Utc;


pub async fn display_all_errors(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &[&str], state: &Arc<RwLock<Vec<Option<Entry>>>> ,offset: &mut usize) {
   
    clear_after_index(state,*offset).await;

    let mut error_view: Vec<(Entry,usize)> = Vec::new();

    let mut error_count = 0;
    for key in req {
        match anything_to_err(tasks.clone(),key).await.as_ref() {
            "--" => {
            },
            e => {
                if !e.contains("Info: Key '"){
                    error_count = error_count +1;
                    error_view.push((Entry {
                        timestamp: Utc::now().timestamp(), 
                        key: key.to_string(),
                        prefix: None,
                        value: e.to_string(),
                        suffix: None,
                        group: Some("[Errors]".to_string()),
                    },*offset));
                    *offset += 1; 
                }
            }
        } 
    } 
    add_view_to_state(&state, error_view).await; 
} 
