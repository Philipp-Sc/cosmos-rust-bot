use core::pin::Pin;
use core::future::Future;

use std::{thread, time};
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::shared::Entry;
use crate::shared::Maybe;


pub async fn add_entry_to_state(state: &Arc<RwLock<Vec<Option<Entry>>>>, index: usize, entry: Entry) -> anyhow::Result<()> {

    let mut look = state.try_write();
    while look.is_err() {
        thread::sleep(time::Duration::from_millis(10));
        look = state.try_write();
    }
    let mut vector = look.unwrap();
    *vector.get_mut(index).unwrap() = Some(entry);
    Ok(())
}

pub async fn clear_after_index(state: &Arc<RwLock<Vec<Option<Entry>>>>,index: usize) {

    let mut vector = state.write().await;
    for x in index..vector.len(){
        *vector.get_mut(x).unwrap() = None;
    }
}

pub async fn add_view_to_state(state: &Arc<RwLock<Vec<Option<Entry>>>>, view: Vec<(Entry,usize)>) {
    let mut vector = state.write().await;
    for entry in view {
        *vector.get_mut(entry.1).unwrap() = Some(entry.0);
    }
}

pub async fn remove_entries_from_state(state: &Arc<RwLock<HashMap<i64,Entry>>>,order_state: &mut HashMap<i64,usize>,starting_from: u64) {
    let mut vector = state.write().await;
    let key = starting_from as i64 * (-1);
    vector.retain(|k, _| *k > key);
    order_state.retain(|k, _| *k > key);
}

pub async fn insert_to_state(state: &Arc<RwLock<HashMap<i64,Entry>>>,hash: u64, entry: &Entry){
    let mut vector =  state.write().await;
    vector.insert(hash as i64 * (-1),entry.clone());
}

pub async fn push_to_state<'a>(state: &'a Arc<RwLock<HashMap<i64,Entry>>>,hash: i64, entry: &'a Entry, f: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static >>)  -> anyhow::Result<()>  {
    let mut e = entry.clone();
    let result = f.await;
    e.value = result.data.unwrap_or("--".to_string());
    e.timestamp = result.timestamp;

    let mut vector =  state.write().await;
    vector.insert(hash,e);
    Ok(())
}
