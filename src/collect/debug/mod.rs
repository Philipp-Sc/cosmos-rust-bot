use terra_rust_bot_essentials::shared::Entry;

use crate::state::control::model::try_get_resolved;
use crate::state::control::model::Maybe;

use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::{Mutex};
use chrono::Utc;
use cosmos_rust_interface::ResponseResult;


pub async fn inspect_results(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>) -> Vec<Entry> {
    let mut view: Vec<Entry> = Vec::new();

    for key in maybes.keys() {
        let mut group: Option<String> = Some("[DEBUG]".to_string());
        match try_get_resolved(maybes, key).await {
            Maybe { data: Ok(resolved), timestamp } => {
                view.push(Entry {
                    timestamp,
                    key: key.to_string(),
                    prefix: None,
                    value: format!("{:?}", resolved).to_string(),
                    suffix: None,
                    index: None,
                    group,
                });
            }
            Maybe { data: Err(_), .. } => {}
        }
    }
    view
}
