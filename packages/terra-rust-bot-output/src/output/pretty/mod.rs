use std::fs;

use serde::Deserialize;
use serde::Serialize;  

use chrono::Utc;
use chrono::{NaiveDateTime};

use colored::*;
use colored::control::set_override;

use comfy_table::Table;
use comfy_table::presets::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entry {
    pub timestamp: i64, 
    pub key: String, 
    pub prefix: Option<String>,
    pub value: String,  
    pub suffix: Option<String>,
    pub group: Option<String>,
}

pub type State = Vec<Option<Entry>>;

pub async fn terra_rust_bot_state(context: &str, path: &str, is_console: bool) -> String {
    
    set_override(is_console);

    match context { 
  
        "\\market" => {
            return terra_rust_bot_state_default("[Market]",path,is_console).await;
        },
        "\\anchor info" => {
            return terra_rust_bot_state_default("[Anchor Protocol Info]",path,is_console).await;
        },  
        "\\anchor account" => {
            return terra_rust_bot_state_default("[Anchor Protocol Account]",path,is_console).await;
        },   
        "\\auto repay" => { 
            return terra_rust_bot_state_default("[Anchor Protocol][Auto Repay]",path,is_console).await;
        },        
        "\\auto borrow" => { 
            return terra_rust_bot_state_default("[Anchor Protocol][Auto Borrow]",path,is_console).await;
        },        
        "\\auto stake" => { 
            return terra_rust_bot_state_default("[Anchor Protocol][Auto Stake]",path,is_console).await;
        },        
        "\\auto farm" => { 
            return terra_rust_bot_state_default("[Anchor Protocol][Auto Farm]",path,is_console).await;
        },  
        "\\errors" => { 
            return terra_rust_bot_state_default("[Errors]",path,is_console).await;
        },     
        "\\logs" => {
            return terra_rust_bot_state_default("[Logs]",path,is_console).await;
        },      
        "\\auto" => {
            return format!("{}\n\n\n{}\n\n\n{}\n\n\n{}",
                terra_rust_bot_state_default("[Anchor Protocol][Auto Repay]",path,is_console).await,
                terra_rust_bot_state_default("[Anchor Protocol][Auto Borrow]",path,is_console).await,
                terra_rust_bot_state_default("[Anchor Protocol][Auto Stake]",path,is_console).await,
                terra_rust_bot_state_default("[Anchor Protocol][Auto Farm]",path,is_console).await
                );
        },     
        "\\task count" => { 
            return terra_rust_bot_state_default("[Task][Count]",path,is_console).await;
        },           
        "\\task list" => {
            return terra_rust_bot_state_default("[Task][List]",path,false).await;
        },      
        "\\help" => {
            return terra_rust_bot_state_help(is_console);
        }, 
        "\\ping" => {
            return terra_rust_bot_state_ping(path,is_console).await;
        },            
        "\\state history" => { 
           return terra_rust_bot_state_history(path,is_console).await;
        },         
        "\\task history" => {
           return terra_rust_bot_task_history(path,is_console).await;
        },
        &_ => {
            return "?".to_string();
        }
    };
 
 
} 

fn terra_rust_bot_state_help(is_console: bool) -> String {   

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



async fn terra_rust_bot_task_history(path: &str, is_console: bool) -> String { 

 
    let mut t = Table::new();
    t.set_header(&[""]);

    let mut timestamp = -1i64;
    let mut display = "".to_string();
    let empty = "".to_string();
    match fs::read_to_string(path) {
        Ok(file) => { 
            let state: State = match serde_json::from_str(&file) {
                Ok(res) => {res},
                Err(err) => {return format!("{:?}",err);},
            };
            let mut state: Vec<Entry> = state.into_iter().filter_map(|x| x).filter(|x| x.group.as_ref().unwrap_or(&empty).contains("[Task][History]")).collect();
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
        },
        Err(err) => {
            return format!("{:?}",err);
        }
    }; 
    if is_console {
        t.load_preset(UTF8_NO_BORDERS); 
    }else {
        return display;
        //./t.load_preset(NOTHING);
    }
    return format!("{}",t);

}
  

async fn terra_rust_bot_state_history(path: &str, is_console: bool) -> String { 
    
    let mut t = Table::new();
    t.set_header(&[""]);
            
    let mut timestamp = 0i64;
    let mut display = "".to_string();

    let empty = "".to_string();
    match fs::read_to_string(path) {
            Ok(file) => { 
                let state: State = match serde_json::from_str(&file) {
                    Ok(res) => {res},
                    Err(err) => {return format!("{:?}",err).red().to_string();},
                };
                let mut state: Vec<Entry> = state.into_iter().filter_map(|x| x).collect();
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
            },
            Err(err) => {
                return format!("{:?}",err).red().to_string();
            }
    };

    if is_console {
        t.load_preset(UTF8_NO_BORDERS); 
    }else {
        //t.load_preset(NOTHING);
        return display;
    }
    return format!("{}",t);
}


async fn terra_rust_bot_state_ping(path: &str, is_console: bool) -> String { 
    
    let mut t = Table::new();
    t.set_header(&["", ""]);
            
    let signal_bot = Utc::now().format("%d/%m/%y %H:%M:%S").to_string();
    let mut timestamp = 0i64;  
    match fs::read_to_string(path) {
            Ok(file) => { 
                let state: State = match serde_json::from_str(&file) {
                    Ok(res) => {res},
                    Err(err) => {return format!("{:?}",err);},
                };
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
            },
            Err(err) => {
                if !is_console {
                     return format!("signal-bot:\n                        {}\nterra-rust-bot:\n                        {:?}",signal_bot,err);
                }
                t.add_row(&[&"signal-bot:".truecolor(75,219,75).to_string(),&format!("{}",signal_bot.purple()) ]);
                t.add_row(&[&"terra-rust-bot:".truecolor(75,219,75).to_string(),&format!("{}",err.to_string().purple()) ]);
            }
    }; 
    if is_console {
        t.load_preset(UTF8_NO_BORDERS); 
    }else {
        t.load_preset(NOTHING);
    }
    return format!("{}",t);
}



async fn terra_rust_bot_state_default(identifier: &str, path: &str, is_console: bool) -> String {  

    let mut t = Table::new();
    let mut display = format!("{}",identifier.truecolor(75,219,75));
            
 
    match fs::read_to_string(path) {
        Ok(file) => { 
            let state: State = match serde_json::from_str(&file) {
                Ok(res) => {res},
                Err(err) => {return format!("{:?}",err);},
            }; 

            t.set_header(&[&identifier.truecolor(75,219,75).to_string(), ""]);
            let mut prev_group = "".to_string();
            let empty = "".to_string();
            
            for x in 0..state.len() { 
                if let Some(entry) = &state[x] {
                    let group = &entry.group.as_ref().unwrap_or(&empty);
                    if group.contains(identifier) {
                        if prev_group != group.to_string() {
                            display = format!("{}\n{}",display,group.replace(identifier,"").truecolor(75,219,75)/*.truecolor(84, 147, 247)*/);   
                            prev_group = group.to_string();
                            t.add_row(&[&group.replace(identifier,"").truecolor(75,219,75).to_string(), ""]);
                        }
                        let prefix = entry.prefix.as_ref().unwrap_or(&empty);
                        let suffix = entry.suffix.as_ref().unwrap_or(&empty);
                        display = format!("{}\n{}:\n                        {} {} {}",display,entry.key.truecolor(77, 77, 237),prefix.purple(),entry.value.purple(),suffix.purple());
                        t.add_row(&[&format!("{}:",entry.key.truecolor(77, 77, 237)),&format!("{} {} {}",prefix.purple(),entry.value.purple(),suffix.purple())]);
                    }
                }
            } 
        },
        Err(err) => {
            return format!("{:?}",err);
        }
    };
    if is_console {
        t.load_preset(UTF8_NO_BORDERS); 
    }else {
        return display;
        //t.load_preset(NOTHING);
    }
    return format!("{}",t);

}