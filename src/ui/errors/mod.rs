use terra_rust_bot_essentials::shared::Entry;


use crate::state::control::model::try_get_resolved;
use crate::state::control::model::Maybe;

use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::{Mutex};
use chrono::Utc;
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::ResponseResult;


pub async fn display_all_errors(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>) -> Vec<Entry> {
    let mut view: Vec<Entry> = Vec::new();

    for key in maybes.keys() {
        match try_get_resolved(maybes, key).await {
            Maybe { data: Ok(_resolved), .. } => {},
            Maybe { data: Err(err), timestamp } => {
                if err.to_string() == "Error: Not yet resolved!".to_string() {
                    view.push(Entry {
                        timestamp: Utc::now().timestamp(),
                        key: key.to_string(),
                        prefix: None,
                        value: err.to_string(),
                        suffix: None,
                        index: None,
                        group: Some("[Unresolved]".to_string()),
                    });
                } else if err.to_string() != "Error: Entry reserved!" {
                    view.push(Entry {
                        timestamp,
                        key: key.to_string(),
                        prefix: None,
                        value: err.to_string(),
                        suffix: None,
                        index: None,
                        group: Some("[Errors]".to_string()),
                    });
                }
            },
        }
        }
    view
} 
