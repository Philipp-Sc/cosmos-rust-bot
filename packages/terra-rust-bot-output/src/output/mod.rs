pub mod pretty;

use core::pin::Pin;
use core::future::Future;

use std::{thread, time};
use std::time::{Duration};

use std::sync::Arc; 
use tokio::sync::RwLock;  
use tokio::time::timeout;  

use chrono::{Utc};
use pretty::Entry;

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


pub async fn try_add_to_state(state: &Arc<RwLock<Vec<Option<Entry>>>>, index: usize, f: Pin<Box<dyn Future<Output = String> + Send + 'static >>) -> anyhow::Result<()> {
    
    let result = timeout(Duration::from_millis(100), f).await;   
    add_to_state(state,index,result.ok()).await
}

pub async fn add_to_state(state: &Arc<RwLock<Vec<Option<Entry>>>>, index: usize, result: Option<String>) -> anyhow::Result<()> {
    
    if let Some(succ) = result {
        let mut vector =  state.write().await;
        let val = vector.get_mut(index).unwrap();
        if let Some(entry) = val { 
            let key = &entry.key;
            let prefix = &entry.prefix;  
            let suffix = &entry.suffix;
            let group = &entry.group;
            *vector.get_mut(index).unwrap() = Some(Entry {
                timestamp: Utc::now().timestamp(), 
                key: key.to_string(),
                prefix: prefix.as_ref().map(|x|x.to_string()),
                value: succ,
                suffix: suffix.as_ref().map(|x|x.to_string()),
                group: group.as_ref().map(|x|x.to_string()),
            });
        }
        
    }
    Ok(())
}