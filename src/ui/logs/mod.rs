use terra_rust_bot_essentials::shared::Entry;


use crate::state::control::model::try_get_resolved;
use crate::state::control::model::Maybe;


use std::collections::HashMap;

use std::sync::Arc;
use cosmos_rust_interface::services::blockchain::smart_contracts::objects::ResponseResult;
use tokio::sync::{Mutex};


pub async fn display_all_logs(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>) -> Vec<Entry> {
    let mut view: Vec<Entry> = Vec::new();

    let vec = vec![
        "anchor_redeem_and_repay_stable",
        "anchor_borrow_and_deposit_stable",
        "anchor_governance_claim_and_stake",
    ];
    for key in vec {
        match try_get_resolved(maybes, key).await {
            Maybe { data: Ok(resolved), timestamp } => {
                view.push(Entry {
                    timestamp,
                    key: key.to_string(),
                    prefix: None,
                    value: resolved.as_text().unwrap_or(&"Error: Could not parse value.".to_string()).to_string(),
                    suffix: None,
                    index: None,
                    group: Some("[Logs]".to_string()),
                });
            },
            Maybe { data: Err(_failed), .. } => {},
        }
    }

    view
}