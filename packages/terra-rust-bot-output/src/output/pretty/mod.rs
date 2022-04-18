use std::fs;


use chrono::Utc;
use chrono::{NaiveDateTime};

use colored::*;
use colored::control::set_override;

use comfy_table::Table;
use comfy_table::presets::*;

use terra_rust_bot_essentials::shared::{State,Entry};



pub async fn load_state(path: &str) -> Option<State> {
    let mut state: Option<State> = None;
    let mut try_counter = 0;
    while state.is_none() && try_counter<3 {
        match fs::read_to_string(path) {
            Ok(file) => {
                match serde_json::from_str(&file) {
                    Ok(res) => { state = Some(res); },
                    Err(_) => { try_counter = try_counter + 1; },
                };
            },
            Err(_) => {
                try_counter = try_counter + 1;
            }
        }
    }
    state
}

pub async fn terra_rust_bot_state(context: &str, state: &State, is_console: bool) -> String {
    
    set_override(is_console);

    match context { 
  
        "\\market" => {
            return terra_rust_bot_state_default("[Market]",state,is_console).await.unwrap_or("No data".to_string());
        },
        "\\anchor info" => {
            return terra_rust_bot_state_default("[Anchor Protocol Info]",state,is_console).await.unwrap_or("No data".to_string());
        },  
        "\\anchor account" => {
            return terra_rust_bot_state_default("[Anchor Protocol Account]",state,is_console).await.unwrap_or("No data".to_string());
        },   
        "\\auto repay" => { 
            return terra_rust_bot_state_default("[Anchor Protocol][Auto Repay]",state,is_console).await.unwrap_or("No data".to_string());
        },        
        "\\auto borrow" => { 
            return terra_rust_bot_state_default("[Anchor Protocol][Auto Borrow]",state,is_console).await.unwrap_or("No data".to_string());
        },        
        "\\auto stake" => { 
            return terra_rust_bot_state_default("[Anchor Protocol][Auto Stake]",state,is_console).await.unwrap_or("No data".to_string());
        },        
        "\\auto farm" => { 
            return terra_rust_bot_state_default("[Anchor Protocol][Auto Farm]",state,is_console).await.unwrap_or("No data".to_string());
        },  
        "\\errors" => { 
            return terra_rust_bot_state_default("[Errors]",state,is_console).await.unwrap_or("No data".to_string());
        },  
        "\\pending" => { 
            return terra_rust_bot_state_default("[Unresolved]",state,is_console).await.unwrap_or("No data".to_string());
        },     
        "\\logs" => {
            return terra_rust_bot_state_default("[Logs]",state,is_console).await.unwrap_or("No data".to_string());
        },      
        "\\auto" => {
            return format!("{}\n\n\n{}\n\n\n{}\n\n\n{}",
                terra_rust_bot_state_default("[Anchor Protocol][Auto Repay]",state,is_console).await.unwrap_or("No data".to_string()),
                terra_rust_bot_state_default("[Anchor Protocol][Auto Borrow]",state,is_console).await.unwrap_or("No data".to_string()),
                terra_rust_bot_state_default("[Anchor Protocol][Auto Stake]",state,is_console).await.unwrap_or("No data".to_string()),
                terra_rust_bot_state_default("[Anchor Protocol][Auto Farm]",state,is_console).await.unwrap_or("No data".to_string())
                );
        },     
        "\\task count" => { 
            return terra_rust_bot_state_default("[Task][Count]",state,is_console).await.unwrap_or("No data".to_string());
        },           
        "\\task list" => {
            return terra_rust_bot_state_default("[Task][List]",state,false).await.unwrap_or("No data".to_string());
        },      
        "\\help" => {
            return terra_rust_bot_state_help(is_console);
        }, 
        "\\ping" => {
            return terra_rust_bot_state_ping(state,is_console).await;
        },            
        "\\state history" => { 
           return terra_rust_bot_state_history(state,is_console).await;
        },         
        "\\task history" => {
           return terra_rust_bot_task_history(state,is_console).await;
        },
        &_ => {
            return "?".to_string();
        }
    };
 
 
} 

fn terra_rust_bot_state_help(_is_console: bool) -> String {   

return r#"[Available Commands]
SYSTEM TIME AND LATEST TIMESTAMP 
    \ping      
MARKET INFO    
    \market         
ANCHOR INFO    
    \anchor info    
ANCHOR ACCOUNT INFO    
    \anchor account  
EVERY AUTOMATION
    \auto        
AUTO REPAY INFO
    \auto repay  
AUTO BORROW INFO 
    \auto borrow  
AUTO STAKE INFO
    \auto stake   
AUTO FARM INFO 
    \auto farm   
SHOW ALL ERRORS
    \errors  
SHOW ALL PENDING TASKS
    \pending  
SHOW LOGS OF RECENT TRANSACTIONS
    \logs  
TASK COUNT (failed,pending,upcoming,all)
    \task count         
TASK LIST (failed,pending,upcoming,all)
    \task list          
TIMESTAMPS WHEN TASKS WERE RESOLVED
    \task history     
TIMESTAMPS WHEN ENTRIES WERE WRITTEN TO STATE
    \state history         
"#.to_string();
}



async fn terra_rust_bot_task_history(state: &State, is_console: bool) -> String {

 
    let mut t = Table::new();
    t.set_header(&[""]);

    let mut timestamp = -1i64;
    let mut display = "".to_string();
    let empty = "".to_string();
    let mut state: Vec<Entry> = state.clone().into_iter().filter_map(|x| x).filter(|x| x.group.as_ref().unwrap_or(&empty).contains("[Task][History]")).collect();
    state.sort_unstable_by(|a, b| a.value.parse::<i64>().unwrap().cmp(&b.value.parse::<i64>().unwrap()));
    for x in 0..state.len() {
        let entry = &state[x];
        let entry_timestamp = entry.value.parse::<i64>().unwrap();
        if timestamp != entry_timestamp {
            timestamp = entry_timestamp;
            if timestamp==0i64 {
                display = "[never executed]".to_string();
                t.add_row(&[&"[never executed]".truecolor(75,219,75).to_string()]);
            }else{
                display = format!("{}\n\n[{}]",display,NaiveDateTime::from_timestamp(timestamp,0).format("%d/%m/%y %H:%M:%S"));
                t.add_row(&[&NaiveDateTime::from_timestamp(timestamp,0).format("%d/%m/%y %H:%M:%S").to_string().truecolor(75,219,75).to_string()]);
            }
            display = format!("{}\n{}",display,&entry.key);
            t.add_row(&[&entry.key.truecolor(77, 77, 237).to_string()]);
        }else if timestamp == entry_timestamp {
            display = format!("{}\n{}",display,&entry.key);
            t.add_row(&[&entry.key.truecolor(77, 77, 237).to_string()]);
        }
    }
    if is_console {
        t.load_preset(UTF8_NO_BORDERS); 
    }else {
        return display;
        //./t.load_preset(NOTHING);
    }
    return format!("{}",t);

}
  

async fn terra_rust_bot_state_history(state: &State, is_console: bool) -> String {
    
    let mut t = Table::new();
    t.set_header(&[""]);
            
    let mut timestamp = 0i64;
    let mut display = "".to_string();

    let empty = "".to_string();
    let mut state: Vec<Entry> = state.clone().into_iter().filter_map(|x| x).collect();
    state.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));
    for x in 0..state.len() {
        let entry = &state[x];
        let group = &entry.group.as_ref().unwrap_or(&empty);
        if group.contains("[Anchor Protocol]") {
            if timestamp != entry.timestamp {
                timestamp = entry.timestamp;
                display = format!("{}\n\n{}",display,NaiveDateTime::from_timestamp(timestamp,0).format("%d/%m/%y %H:%M:%S"));
                t.add_row(&[&NaiveDateTime::from_timestamp(timestamp,0).format("%d/%m/%y %H:%M:%S").to_string().truecolor(75,219,75).to_string()]);
            }else if timestamp == entry.timestamp {
                display = format!("{}\n{}",display,&entry.key);
                t.add_row(&[&entry.key.truecolor(77, 77, 237).to_string()]);
            }
        }
    }

    if is_console {
        t.load_preset(UTF8_NO_BORDERS); 
    }else {
        //t.load_preset(NOTHING);
        return display;
    }
    return format!("{}",t);
}

pub async fn terra_rust_bot_state_ping_delay(state: &State, max_delay: i64) -> Option<String> {
            
    let signal_bot = Utc::now().timestamp();
    let mut timestamp = 0i64;
    for x in 0..state.len() {
        if let Some(entry) = &state[x] {
            if timestamp < entry.timestamp {
                 timestamp = entry.timestamp;
            }
        }
    }
    let delay = signal_bot - timestamp;
    if delay >= max_delay {
         return Some(format!("Terra-rust-bot is {} seconds behind schedule.\nsignal-bot:\n                        {}\nterra-rust-bot:\n                        {}\nTo check yourself use:\n\\ping",delay,NaiveDateTime::from_timestamp(signal_bot,0).format("%d/%m/%y %H:%M:%S").to_string(),NaiveDateTime::from_timestamp(timestamp,0).format("%d/%m/%y %H:%M:%S").to_string()));
    }
    return None;
}


async fn terra_rust_bot_state_ping(state: &State, is_console: bool) -> String {
    
    let mut t = Table::new();
    t.set_header(&["", ""]);
            
    let signal_bot = Utc::now().format("%d/%m/%y %H:%M:%S").to_string();
    let mut timestamp = 0i64;
    for x in 0..state.len() {
        if let Some(entry) = &state[x] {
            if timestamp < entry.timestamp {
                 timestamp = entry.timestamp;
            }
        }
    }
    let terra = NaiveDateTime::from_timestamp(timestamp,0).format("%d/%m/%y %H:%M:%S").to_string();
    if !is_console {
         return format!("signal-bot:\n                        {}\nterra-rust-bot:\n                        {}",signal_bot,terra);
    }
    t.add_row(&[&"signal-bot:".truecolor(75,219,75).to_string(),&format!("{}",signal_bot.purple()) ]);
    t.add_row(&[&"terra-rust-bot:".truecolor(75,219,75).to_string(),&format!("{}",terra.purple()) ]);

    if is_console {
        t.load_preset(UTF8_NO_BORDERS); 
    }else {
        t.load_preset(NOTHING);
    }
    return format!("{}",t);
}

pub async fn terra_rust_bot_state_get_latest(identifier: &str, state: &State) -> Option<i64> {

    let empty = "".to_string();

    let mut max_timestamp = 0i64;
    for x in 0..state.len() {
        if let Some(entry) = &state[x] {
            let group = &entry.group.as_ref().unwrap_or(&empty);
            if group.contains(identifier) {
                if max_timestamp < entry.timestamp {
                    max_timestamp = entry.timestamp;
                }
            }
        }
    }
    return Some(max_timestamp);
}


pub async fn terra_rust_bot_state_default(identifier: &str, state: &State, is_console: bool) -> Option<String> {

    let mut t = Table::new();
    let mut display = format!("{}",identifier.truecolor(75,219,75));

    t.set_header(&[&identifier.truecolor(75,219,75).to_string()]);
    let mut has_data = false;
    let mut prev_group = "".to_string();
    let empty = "".to_string();

    for x in 0..state.len() {
        if let Some(entry) = &state[x] {
            let group = &entry.group.as_ref().unwrap_or(&empty);
            if group.contains(identifier) {
                has_data = true;
                if prev_group != group.to_string() {
                    prev_group = group.to_string();
                    let header = group.replace(identifier,"");
                    if header != "" {
                        display = format!("{}\n{}",display,&header.truecolor(75,219,75)/*.truecolor(84, 147, 247)*/);
                        t.add_row(&[&format!("{}",&header.truecolor(75,219,75))]);
                    }
                }
                let prefix = entry.prefix.as_ref().unwrap_or(&empty);
                let suffix = entry.suffix.as_ref().unwrap_or(&empty);
                let sec_ago_updated = Utc::now().timestamp()-entry.timestamp;
                display = format!("{}\n{}:\n                        {} {} {}\n                         [{}s. ago updated]",display,entry.key.truecolor(77, 77, 237),prefix.purple(),entry.value.purple(),suffix.purple(),sec_ago_updated);
                t.add_row(&[&format!("{}:",entry.key.truecolor(77, 77, 237)),&format!("{} {} {}",prefix.purple(),entry.value.purple(),suffix.purple()),&format!("[{}s. ago updated]",sec_ago_updated)]);
            }
        }
    }
    if !has_data {
        return None;
    }
    if is_console {
        t.load_preset(ASCII_HORIZONTAL_ONLY); 
        // UTF8_HORIZONTAL_ONLY
    }else {
        return Some(display);
        //t.load_preset(NOTHING);
    }
    return Some(format!("{}",t));

}