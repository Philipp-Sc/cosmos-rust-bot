use terra_rust_bot_essentials::shared::Entry;


use crate::state::control::model::try_get_resolved;
use crate::state::control::model::Maybe;

use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::{Mutex};
use chrono::Utc;
use cosmos_rust_interface::ResponseResult;


pub async fn all_errors(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>) -> Vec<Entry> {
    let mut view: Vec<Entry> = Vec::new();

    for key in maybes.keys() {
        match try_get_resolved(maybes, key).await {
            Maybe { data: Ok(_resolved), .. } => {}
            Maybe { data: Err(err), timestamp } => {
                let mut group: Option<String> = None;
                if err.to_string() == "Error: Not yet resolved!".to_string() {
                    group = Some("[Unresolved]".to_string());
                } else if err.to_string() != "Error: Entry reserved!" {
                    group = Some("[Errors]".to_string());
                }
                view.push(Entry {
                    timestamp,
                    key: key.to_string(),
                    prefix: None,
                    value: err.to_string(),
                    suffix: None,
                    index: None,
                    group,
                });
            }
        }
    }
    view
} 
