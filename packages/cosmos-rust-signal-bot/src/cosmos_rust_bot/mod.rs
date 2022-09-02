use bot_library::*;
// to get control over the settings
use cosmos_rust_interface::utils::entry::db::query::*;
use cosmos_rust_interface::utils::entry::db::*;
use cosmos_rust_interface::utils::entry::*;
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

    // lookup specific subset of proposals, subscribe to get notified when changes occur.
    // \lookup_proposals latest 10 \subscribe
    // \lookup_proposals terra voting latest 10 \unsubscribe
    let task_info_regex = Regex::new(format!("task (count|list)(?: {})?(?: {})?(?:\\s|$)", state_regex, sub_regex).as_str()).unwrap();
    let task_info_history_regex = Regex::new(format!("task (history)(?: {})?(?: ([0-9]+))?(?: {})?(?:\\s|$)", state_regex, sub_regex).as_str()).unwrap();

    let lookup_proposals_regex = Regex::new(format!("lookup proposals(?: {})?(?: #([0-9]+))?(?: (nil|passed|failed|rejected|deposit period|voting period))?(?: (text|community pool spend|parameter change|software upgrade|client update|update pool incentives|store code|unknown))?(?: (latest|submit|deposit end|voting start|voting end))?(?: ([0-9]+))?(?: {})?(?:\\s|$)", blockchain_regex, sub_regex).as_str()).unwrap();

    if task_info_history_regex.is_match(&msg) {
        let caps = task_info_history_regex.captures(&msg).unwrap();
        let mut filter: HashMap<String, String> = HashMap::new();
        filter.insert("kind".to_string(), format!("task_{}", caps.get(1).unwrap().as_str().to_string()));
        filter.insert("state".to_string(), caps.get(2).map(|t| t.as_str().to_string().to_title_case()).unwrap_or("any".to_string()));

        let order_by = "timestamp".to_string();

        let limit = match caps.get(3) {
            Some(t) => t.as_str().parse::<usize>().unwrap(),
            None => 1000
        };
        let response = client_send_request(serde_json::json!({"fields":vec!["summary"],"indices":vec!["task_meta_data"],"filter": filter, "order_by": order_by, "limit":limit}));

        let msg: Vec<String> = match response.unwrap().as_array() {
            Some(list) => {
                list.iter()
                    .filter(|x| x.as_object().is_some())
                    .map(|x| x.as_object().unwrap())
                    .filter(|x| x.get("summary").is_some())
                    .map(|x| x.get("summary").unwrap())
                    .filter(|x| x.as_str().is_some())
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect()
            }
            None => { vec!["".to_string()] }
        };

        return vec![msg.join("\n")];
    }
    if task_info_regex.is_match(&msg) {
        let caps = task_info_regex.captures(&msg).unwrap();
        let mut filter: HashMap<String, String> = HashMap::new();
        filter.insert("kind".to_string(), caps.get(1).map(|t| format!("task_{}", t.as_str())).unwrap_or("any".to_string()));
        filter.insert("state".to_string(), caps.get(2).map(|t| t.as_str().to_string().to_title_case()).unwrap_or("any".to_string()));

        let order_by = "index".to_string();

        let response = client_send_request(serde_json::json!({"fields":vec!["summary"],"indices":vec!["task_meta_data"],"filter": filter, "order_by": order_by, "limit":1000}));

        let msg: Vec<String> = match response.unwrap().as_array() {
            Some(list) => {
                list.iter()
                    .filter(|x| x.as_object().is_some())
                    .map(|x| x.as_object().unwrap())
                    .filter(|x| x.get("summary").is_some())
                    .map(|x| x.get("summary").unwrap())
                    .filter(|x| x.as_str().is_some())
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect()
            }
            None => { vec!["".to_string()] }
        };

        return vec![msg.join("\n")];
    } else if lookup_proposals_regex.is_match(&msg) {
        let caps = lookup_proposals_regex.captures(&msg).unwrap();
        let mut filter: HashMap<String, String> = HashMap::new();

        filter.insert("proposal_blockchain".to_string(), caps.get(1).map(|t| format!("{}", t.as_str())).unwrap_or("any".to_string()));
        filter.insert("proposal_id".to_string(), caps.get(2).map(|t| format!("{}", t.as_str())).unwrap_or("any".to_string()));
        filter.insert("proposal_status".to_string(), caps.get(3).map(|t| format!("{}", format!("status{}", t.as_str().chars().filter(|c| !c.is_whitespace()).collect::<String>()))).unwrap_or("any".to_string()));
        filter.insert("proposal_type".to_string(), caps.get(4).map(|t| format!("{}", format!("{}proposal", t.as_str().chars().filter(|c| !c.is_whitespace()).collect::<String>()))).unwrap_or("any".to_string()));

        let order_by = format!("proposal_{}time", caps.get(5).map(|x| x.as_str().chars().filter(|c| !c.is_whitespace()).collect::<String>()).unwrap_or("latest".to_string()).to_owned().to_lowercase());
        let limit = caps.get(6).map(|x| x.as_str()).unwrap_or("20").to_owned().parse::<usize>().unwrap();
        let response = client_send_request(serde_json::json!({"fields":vec!["summary"],"indices":vec!["proposal_id"],"filter": filter, "order_by": order_by, "limit":limit}));
        let msg: Vec<String> = match response.unwrap().as_array() {
            Some(list) => {
                list.iter()
                    .filter(|x| x.as_object().is_some())
                    .map(|x| x.as_object().unwrap())
                    .filter(|x| x.get("summary").is_some())
                    .map(|x| x.get("summary").unwrap())
                    .filter(|x| x.as_str().is_some())
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect()
            }
            None => { vec!["".to_string()] }
        };
        return msg;
    }
// \subscriptions list  (lists subs)
// \subscription delete 1 (deactivate/activate/delete subs)
// same as lookup_proposals but sets up notifications, which will notify when response changes, sending the difference (either new proposal (trough new id), or updated proposal (through info change))
    vec!["Hi, I read your message. ~ Your Cosmos-Rust-Bot".to_string()]
}

pub async fn handle_notifications() -> Option<String> {
    None
}