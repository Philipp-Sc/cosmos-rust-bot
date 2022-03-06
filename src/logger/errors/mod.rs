
use display_utils::display::*; 
use terra_rust_bot_backend::control::view::interface::model::{MaybeOrPromise};
  
use terra_rust_bot_backend::control::view::*;  

use std::collections::HashMap;  

use std::sync::Arc; 
use tokio::sync::RwLock;   
use colored::*;

pub async fn display_all_errors(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, req: &[&str], new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize) {
   
    let mut error_view: Vec<(String,usize)> = Vec::new();

    error_view.push(("\n\n  **Errors**\n\n".red().to_string(),*offset));
    *offset += 1;
  
    // clear the previous error messages. 
    for x in *offset..new_display.read().await.len(){
        error_view.push(("".to_string(),x));
    }

    let mut error_count = 0;
    for key in req {
        match anything_to_err(tasks.clone(),key).await.as_ref() {
            "--" => {
            },
            e => {
                if !e.contains("Info: Key '"){
                    error_count = error_count +1;
                    error_view.push((format!("\n   [Key] '{}'\n   {}\n",key,e).yellow().to_string(),*offset));
                    *offset += 1; 
                }
            }
        } 
    }
    if error_count == 0 {
        error_view.push(("\n   None \n\n".red().to_string(),*offset)); 
        *offset += 1; 
    }

    add_view_to_display(&new_display, error_view).await; 
} 
