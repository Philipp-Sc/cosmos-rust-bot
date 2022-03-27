 
//use std::env; 
use chrono::{Utc};
use std::fs;
  
use slack_hook::Slack;
use slack_hook::PayloadBuilder;
 
use regex::Regex;


 #[tokio::main]
async fn main() -> anyhow::Result<()> {

        let slack = Slack::new("https://hooks.slack.com/services/abc").unwrap();
        let mut notify_out_timestamp = 0i64;
        let regex = Regex::new(r"(?mixsu)([^\s]\[[^\]].[^\]]*?[m]|\[0m)").unwrap();
                
        loop {  
            let now = Utc::now().timestamp_millis();

            if notify_out_timestamp== 0i64 || now - notify_out_timestamp > 60*1000i64 {
                
	            let display = match fs::read_to_string("./terra-rust-bot-display.txt") {
		            Ok(file) => {
		                file
		            },
		            Err(err) => {
		                format!("{:?}",err)
		            }
		        };
                let result = regex.replace_all(&display, "").to_string();
                if result.as_str().len() > 3 {
                    result = (result.as_str()[3..result.as_str().len()]).to_string();
                }
                let p = PayloadBuilder::new()
                  .text(result)
                  .channel("#terra-rust-bot") 
                  .build()
                  .unwrap();        
                slack.send(&p).ok(); 
                notify_out_timestamp = now;
            }
        } 
} 