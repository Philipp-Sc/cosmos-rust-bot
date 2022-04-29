use core::pin::Pin;
use core::future::Future;

use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::shared::Entry;
use crate::shared::Maybe;

pub async fn insert_to_state(state: &Arc<RwLock<HashMap<i64, Entry>>>, hash: u64, entry: &Entry) {
    let mut vector = state.write().await;
    vector.insert(hash as i64 * (-1), entry.clone());
}

pub async fn push_to_state<'a>(state: &'a Arc<RwLock<HashMap<i64, Entry>>>, hash: i64, entry: &'a Entry, f: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>) -> anyhow::Result<()> {
    let mut e = entry.clone();
    let result = f.await;
    e.value = result.data.unwrap_or("--".to_string());
    e.timestamp = result.timestamp;

    let mut vector = state.write().await;
    vector.insert(hash, e);
    Ok(())
}
