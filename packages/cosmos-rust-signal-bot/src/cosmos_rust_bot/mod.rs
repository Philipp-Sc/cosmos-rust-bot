use bot_library::*;
// to get control over the settings
use cosmos_rust_interface::utils::entry::db::query::query_entries;
use cosmos_rust_interface::utils::entry::db::*;
use cosmos_rust_interface::utils::entry::{EntryValue, Entry};
use regex::Regex;

use heck::ToTitleCase;
use heck::ToUpperCamelCase;
use std::collections::HashMap;

// add a state_storage to remember the user and provide it to both functions mutable.

// todo: task list, errors, logs commands. !!!!

pub async fn handle_message(msg: String) -> Vec<String> {
    // todo: load blockchain_names and join them, from list/file.
    // todo: or just match any blockchain name. user is responsible.(adjust regex)
    let blockchain_regex = "(terra|osmosis|juno)";
    let state_regex = "(pending|resolved|upcoming|failed|unknown|reserved)";
    let sub_regex = "(subscribe|unsubscribe)";

    let msg = msg.to_lowercase();
    let entries: Vec<Entry> = load_entries("../../cosmos-rust-bot-db.json").await.unwrap_or(Vec::new());
    // lookup specific subset of proposals, subscribe to get notified when changes occur.
    // \lookup_proposals latest 10 \subscribe
    // \lookup_proposals terra voting latest 10 \unsubscribe
    let task_info_regex = Regex::new(format!("task (count|list)(?: {})?(?: {})?(?:\\s|$)", state_regex, sub_regex).as_str()).unwrap();
    let task_info_history_regex = Regex::new(format!("task (history)(?: {})?(?: ([0-9]+))?(?: {})?(?:\\s|$)", state_regex, sub_regex).as_str()).unwrap();

    let lookup_proposals_regex = Regex::new(format!("lookup proposals(?: {})?(?: (nil|passed|failed|rejected|deposit period|voting period))?(?: (text|community pool spend|parameter change|software upgrade|client update|update pool incentives|store code|unknown))?(?: (latest))?(?: ([0-9]+))?(?: {})?(?:\\s|$)", blockchain_regex, sub_regex).as_str()).unwrap();
    let lookup_proposal_regex = Regex::new(format!("lookup proposal(?: {})(?: #([0-9]+))(?: {})?(?:\\s|$)", blockchain_regex, sub_regex).as_str()).unwrap();

    if task_info_history_regex.is_match(&msg) {
        let caps = task_info_history_regex.captures(&msg).unwrap();
        let mut filter: HashMap<String, String> = HashMap::new();
        filter.insert("group".to_string(), format!("task_{}", caps.get(1).unwrap().as_str().to_string()));
        filter.insert("state".to_string(), caps.get(2).map(|t| t.as_str().to_string().to_title_case()).unwrap_or("any".to_string()));

        let order_by = "timestamp".to_string();

        let limit = match caps.get(3) {
            Some(t) => t.as_str().parse::<usize>().unwrap(),
            None => 1000
        };
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({"filter": filter, "order_by": order_by, "limit":limit})).unwrap());
        let subset = query_entries(&entries, filter, order_by, limit);
        let mut return_msg = "".to_string();
        for i in 0..subset.len() {
            if let EntryValue::Value(ref val) = subset[i].value {
                return_msg = format!("{}\n\n{}", return_msg, val["info"].as_str().unwrap().to_string());
            }
        }
        return vec![return_msg];
    }
    if task_info_regex.is_match(&msg) {
        let caps = task_info_regex.captures(&msg).unwrap();
        let mut filter: HashMap<String, String> = HashMap::new();
        filter.insert("group".to_string(), caps.get(1).map(|t| format!("task_{}", t.as_str())).unwrap_or("any".to_string()));
        filter.insert("state".to_string(), caps.get(2).map(|t| t.as_str().to_string().to_title_case()).unwrap_or("any".to_string()));

        let order_by = "index".to_string();

        println!("{}", serde_json::to_string_pretty(&serde_json::json!({"filter": filter, "order_by": order_by, "limit":1000})).unwrap());
        let subset = query_entries(&entries, filter, order_by, 1000);
        let mut return_msg = "".to_string();
        for i in 0..subset.len() {
            if let EntryValue::Value(ref val) = subset[i].value {
                return_msg = format!("{}\n\n{}", return_msg, val["info"].as_str().unwrap().to_string());
            }
        }
        return vec![return_msg];
    } else if lookup_proposals_regex.is_match(&msg) {
        let caps = lookup_proposals_regex.captures(&msg).unwrap();
        let mut filter: HashMap<String, String> = HashMap::new();
        filter.insert("blockchain".to_string(), caps.get(1).map(|t| format!("{}", t.as_str()).to_title_case()).unwrap_or("any".to_string()));
        filter.insert("status".to_string(), caps.get(2).map(|t| format!("{}", format!("status {}", t.as_str()).to_upper_camel_case())).unwrap_or("any".to_string()));
        filter.insert("type".to_string(), caps.get(3).map(|t| format!("{}", format!("{} proposal", t.as_str()).to_upper_camel_case())).unwrap_or("any".to_string()));

        let order_by = format!("{}Time", caps.get(4).map(|x| x.as_str()).unwrap_or("latest").to_owned().to_title_case());
        let limit = caps.get(5).map(|x| x.as_str()).unwrap_or("1").to_owned().parse::<usize>().unwrap();
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({"filter": filter, "order_by": order_by, "limit":limit})).unwrap());
        let subset = query_entries(&entries, filter, order_by, limit);
        let mut msg_list = Vec::new();
        for i in 0..subset.len() {
            if let EntryValue::Value(ref val) = subset[i].value {
                msg_list.push(val["info"].as_str().unwrap().to_string());
            }
        }
        return msg_list;
    } else if lookup_proposal_regex.is_match(&msg) {
        let caps = lookup_proposal_regex.captures(&msg).unwrap();
        let mut filter: HashMap<String, String> = HashMap::new();

        filter.insert("blockchain".to_string(), caps.get(1).unwrap().as_str().to_string());
        filter.insert("id".to_string(), caps.get(2).unwrap().as_str().to_string());

        let order_by = "id".to_string();
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({"filter": filter, "order_by": order_by, "limit":20})).unwrap());
        let subset = query_entries(&entries, filter, order_by, 20);
        let mut return_msg = "".to_string();
        for i in 0..subset.len() {
            if let EntryValue::Value(ref val) = subset[i].value {
                return_msg = format!("{}\n\n{}", return_msg, val["info"].as_str().unwrap().to_string());
            }
        }
        return vec![return_msg];
    }
// \subscriptions list  (lists subs)
// \subscription delete 1 (deactivate/activate/delete subs)
// same as lookup_proposals but sets up notifications, which will notify when response changes, sending the difference (either new proposal (trough new id), or updated proposal (through info change))
    vec!["Hi, I read your message. ~ Your Cosmos-Rust-Bot".to_string()]
}

pub async fn handle_notifications() -> Option<String> {
    None
}