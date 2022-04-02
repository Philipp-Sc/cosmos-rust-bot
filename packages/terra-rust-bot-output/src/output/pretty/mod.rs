use std::fs;

use serde::Deserialize;
use serde::Serialize;  

use chrono::Utc;
use chrono::{NaiveDateTime};

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

pub async fn terra_rust_bot_state(context: &str, path: &str) -> String {
    
    let identifier = match context { 
        "\\debug" => {
            ""
        },
        "\\market" => {
            "[Market]"
        },
        "\\anchor info" => {
            "[Anchor Protocol Info]"
        },  
        "\\anchor account" => {
            "[Anchor Protocol Account]"
        },  
        "\\auto" => {
            "[Anchor Protocol]"
        },
        "\\auto repay" => {
            "[Anchor Protocol][Auto Repay]"
        },        
        "\\auto borrow" => {
            "[Anchor Protocol][Auto Borrow]"
        },        
        "\\auto stake" => {
            "[Anchor Protocol][Auto Stake]"
        },        
        "\\auto farm" => {
            "[Anchor Protocol][Auto Farm]"
        },  
        "\\errors" => {
            "[Errors]"
        },     
        "\\logs" => {
            "[Logs]"
        },    
        "\\task count" => {
            "[Task][Count]"
        },           
        "\\task list" => {
            "[Task][List]"
        },      
        "\\help" => {
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
DEBUG (SHOWS ABSOLUTLY EVERYTHING)
    \debug    
"#.to_string();
        }           
        "\\state history" => { 
            let mut timestamp = 0i64;
            let mut display = "".to_string();
            let empty = "".to_string();
            match fs::read_to_string(path) {
                    Ok(file) => { 
                        let state: State = match serde_json::from_str(&file) {
                            Ok(res) => {res},
                            Err(err) => {return format!("{:?}",err);},
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
                                }else if timestamp == entry.timestamp {
                                     display = format!("{}\n{}",display,&entry.key);
                                }
                            } 
                        } 
                        return display;
                    },
                    Err(err) => {
                        return format!("{:?}",err);
                    }
            };

        }, 
        "\\ping" => {
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
                        let t = NaiveDateTime::from_timestamp(timestamp,0).format("%d/%m/%y %H:%M:%S").to_string();
                        return format!("signal-bot:\n                        {}\nterra-rust-bot:\n                        {}",signal_bot,t);
                    },
                    Err(err) => {
                        return format!("signal-bot:\n                        {}\nterra-rust-bot:\n                        {:?}",signal_bot,err);
                    }
            }; 
        },           
        "\\task history" => {
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
                                 }else{
                                    display = format!("{}\n\n[{}]",display,NaiveDateTime::from_timestamp(timestamp,0).format("%d/%m/%y %H:%M:%S"));
                                 }
                                 display = format!("{}\n{}",display,&entry.key);
                            }else if timestamp == entry_timestamp {
                                 display = format!("{}\n{}",display,&entry.key);
                            } 
                        } 
                        return display;
                    },
                    Err(err) => {
                        return format!("{:?}",err);
                    }
            }; 
        },
        &_ => {
            "?"
        }
    };

    let display = match fs::read_to_string(path) {
        Ok(file) => { 
            let state: State = match serde_json::from_str(&file) {
                Ok(res) => {res},
                Err(err) => {return format!("{:?}",err);},
            };
            let mut display = format!("{}",identifier);
            let mut prev_group = "".to_string();
            let empty = "".to_string();
            for x in 0..state.len() { 
            	if let Some(entry) = &state[x] {
                    let group = &entry.group.as_ref().unwrap_or(&empty);
            		if group.contains(identifier) {
                        if prev_group != group.to_string() {
                            display = format!("{}\n{}",display,group.replace(identifier,""));   
                            prev_group = group.to_string();
                        }

	            		let prefix = entry.prefix.as_ref().unwrap_or(&empty);
	            		let suffix = entry.suffix.as_ref().unwrap_or(&empty);
	            		display = format!("{}\n{}:\n                        {} {} {}",display,entry.key,prefix,entry.value,suffix);
            		}
            	}
            }
            display
        },
        Err(err) => {
            format!("{:?}",err)
        }
    };
    return display;

} 