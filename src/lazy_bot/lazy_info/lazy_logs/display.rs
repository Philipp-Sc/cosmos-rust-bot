use core::pin::Pin;
use core::future::Future;

use std::{thread, time};
use std::time::{Duration};

use std::sync::Arc; 
use tokio::sync::RwLock;  
use tokio::time::timeout; 
use colored::*;


pub fn display_add(item: String, fixed_len: usize, new_lines: usize) -> String {

    let split = item.split("    ");
    let mut result = "".to_string();

    for s in split {
        if s.len() > 0 {
            if s.len() <= fixed_len {
                let space = fixed_len - s.len();
                result = format!("{}{}{}",result,s," ".repeat(space));
            }else{
                result = format!("{}{}", result,s);
            }
        }
    }

    result = format!("{}{}",result,"\n".repeat(new_lines));
    result
}

pub async fn add_table_formatting(f: Pin<Box<dyn Future<Output = String> + Send + 'static >>, fixed_len: usize, new_lines: usize) -> String {
    let res = f.await;
    let split = res.split("    ");
    let mut result = "".to_string();

    for s in split {
        if s.len() > 0 {
            if s.len() <= fixed_len {
                let space = fixed_len - s.len();
                result = format!("{}{}{}",result,s," ".repeat(space));
            }else{
                result = format!("{}{}", result,s);
            }
        }
    }

    result = format!("{}{}",result,"\n".repeat(new_lines));
    result
}

pub async fn add_string_to_display(new_display: &Arc<RwLock<Vec<String>>>, index: usize, line: String) -> anyhow::Result<()> {
    
    let mut look = new_display.try_write();
    while look.is_err() {
        thread::sleep(time::Duration::from_millis(10));
        look = new_display.try_write();
    } 
    let mut vector = look.unwrap();
    *vector.get_mut(index).unwrap() = line;
    Ok(())
}

pub async fn add_view_to_display(new_display: &Arc<RwLock<Vec<String>>>, view: Vec<(String,usize)>) {
    let mut vector = new_display.write().await;
    for entry in view {
        *vector.get_mut(entry.1).unwrap() = entry.0;
    }
}

pub async fn add_format_to_result(prefix: String,suffix: String, f: Pin<Box<dyn Future<Output = String> + Send + 'static >>) -> String {
    return format!("{}{}{}",prefix,f.await,suffix);
}

pub async fn add_to_display(new_display: &Arc<RwLock<Vec<String>>>, index: usize, result: Option<String>) -> anyhow::Result<()> {
    
    if let Some(succ) = result {
        let mut vector =  new_display.write().await;
        *vector.get_mut(index).unwrap() = format!("{}",succ.truecolor(77, 77, 237));
    }
    Ok(())
}

pub async fn try_add_to_display(new_display: &Arc<RwLock<Vec<String>>>, index: usize, f: Pin<Box<dyn Future<Output = String> + Send + 'static >>) -> anyhow::Result<()> {
    
    let result = timeout(Duration::from_millis(100), f).await;   
    add_to_display(new_display,index,result.ok()).await
}