use chrono::Utc;
use cosmos_rust_interface::utils::entry::{
    db::{notification::notify_sled_db, query::socket::*},
    CosmosRustServerValue, Notify,
};
use regex::Regex;

use heck::ToTitleCase;
use std::collections::HashMap;

use cosmos_rust_interface::utils::entry::UserMetaData;

const TASK_STATES: &str = "(pending|resolved|upcoming|failed|unknown|reserved)";
const SUB_UNSUB: &str = "(subscribe|unsubscribe)";
const BLOCKCHAINS: &str = "(terra2|osmosis|juno|cosmoshub)";


pub fn handle_tasks_count_list_history(user_hash: u64, msg: &str, msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()>  {
        let task_info_regex = Regex::new(
            format!(
                "tasks (count|list|history)(?: {})?(?: ([0-9]+))?(?: {})?(?:\\s|$)",
                TASK_STATES, SUB_UNSUB
            )
                .as_str(),
        )
            .unwrap();
        if task_info_regex.is_match(&msg) {
            let caps = task_info_regex.captures(&msg).unwrap();
            let mut filter: HashMap<String, String> = HashMap::new();
            let k = caps.get(1).map(|t| t.as_str());
            filter.insert(
                "kind".to_string(),
                k.map(|t| format!("task_{}", t))
                    .unwrap_or("any".to_string()),
            );
            filter.insert(
                "state".to_string(),
                caps.get(2)
                    .map(|t| t.as_str().to_string().to_title_case())
                    .unwrap_or("any".to_string()),
            );

            let order_by = match k {
                Some("history") => "timestamp".to_string(),
                _ => "index".to_string(),
            };

            let limit = match caps.get(3) {
                Some(t) => t.as_str().parse::<usize>().unwrap(),
                None => 1,
            };
            let subscribe = caps
                .get(4)
                .map(|x| x.as_str() == "subscribe")
                .unwrap_or(false);
            let unsubscribe = caps
                .get(4)
                .map(|x| x.as_str() == "unsubscribe")
                .unwrap_or(false);
            let request = serde_json::json!({"message": msg_for_query, "handler":"query_entries", "fields":vec!["summary"],"indices":vec!["task_meta_data"],"filter": filter, "order_by": order_by, "limit":limit, "subscribe": subscribe, "unsubscribe": unsubscribe, "user_hash": user_hash});
            let response = client_send_request(request).unwrap();
            notify_sled_db(db, response);
            return Ok(());
        }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}

pub fn handle_tasks_logs_errors_debug(user_hash: u64, msg: &str, msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()>  {
        let log_error_debug_regex = Regex::new(
            format!(
                "tasks (logs|errors|debug)(?: ([0-9]+))?(?: {})?(?:\\s|$)",
                SUB_UNSUB
            )
                .as_str(),
        ).unwrap();

        if log_error_debug_regex.is_match(msg) {
            let caps = log_error_debug_regex.captures(msg).unwrap();
            let k = caps.get(1).map(|t| t.as_str()).unwrap();
            let limit = match caps.get(2) {
                Some(t) => t.as_str().parse::<usize>().unwrap(),
                None => 1,
            };
            let subscribe = caps
                .get(3)
                .map(|x| x.as_str() == "subscribe")
                .unwrap_or(false);
            let unsubscribe = caps
                .get(3)
                .map(|x| x.as_str() == "unsubscribe")
                .unwrap_or(false);

            let filter: HashMap<String, String> = HashMap::new();
            let fields = match k {
                "logs" | "errors" => {
                    vec!["summary"]
                }
                "debug" | _ => {
                    vec!["key", "value"]
                }
            };

            let request = serde_json::json!({"message": msg_for_query, "handler":"query_entries", "fields":fields,"indices":vec![format!("task_meta_data_{}",k).as_str()],"filter": filter, "order_by": "timestamp", "limit":limit, "subscribe": subscribe, "unsubscribe": unsubscribe, "user_hash": user_hash});
            let response = client_send_request(request).unwrap();
            notify_sled_db(db, response);
            return Ok(());
        }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}

pub fn handle_subscribe_unsubscribe(user_hash: u64, msg: &str, msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()>  {
    let subscribe_option: Option<bool> = if msg == "unsubscribe all" {
            Some(false)
        } else if msg == "subscriptions" {
            Some(true)
        } else {
            None
    };
    subscribe_option.map(|x| {
        let request = serde_json::json!({"message": msg_for_query, "handler":"query_subscriptions", "unsubscribe": x, "user_hash": user_hash});
        let response = client_send_request(request).unwrap();
        notify_sled_db(db, response);
        return Ok(());
    }).unwrap_or(Err(anyhow::anyhow!("Error: Unknown Command!")))
}


pub fn handle_gov_prpsl(user_hash: u64, msg: &str, msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()> {
    let lookup_proposals_regex = Regex::new(format!("gov prpsl(?: {})?(?: #([0-9]+))?(?: (nil|passed|failed|rejected|deposit period|voting period))?(?: (text|community pool spend|parameter change|software upgrade|client update|update pool incentives|store code|unknown))?(?: (latest|submit|deposit end|voting start|voting end))?(?: ([0-9]+))?(?: {})?(?:\\s|$)", BLOCKCHAINS, SUB_UNSUB).as_str()).unwrap();

    if lookup_proposals_regex.is_match(&msg) {
        let caps = lookup_proposals_regex.captures(&msg).unwrap();
        let mut filter: HashMap<String, String> = HashMap::new();
        filter.insert(
            "proposal_blockchain".to_string(),
            caps.get(1)
                .map(|t| format!("{}", t.as_str()))
                .unwrap_or("any".to_string()),
        );
        filter.insert(
            "proposal_id".to_string(),
            caps.get(2)
                .map(|t| format!("{}", t.as_str()))
                .unwrap_or("any".to_string()),
        );
        filter.insert(
            "proposal_status".to_string(),
            caps.get(3)
                .map(|t| {
                    format!(
                        "{}",
                        format!(
                            "status{}",
                            t.as_str()
                                .chars()
                                .filter(|c| !c.is_whitespace())
                                .collect::<String>()
                        )
                    )
                })
                .unwrap_or("any".to_string()),
        );
        filter.insert(
            "proposal_type".to_string(),
            caps.get(4)
                .map(|t| {
                    format!(
                        "{}",
                        format!(
                            "{}proposal",
                            t.as_str()
                                .chars()
                                .filter(|c| !c.is_whitespace())
                                .collect::<String>()
                        )
                    )
                })
                .unwrap_or("any".to_string()),
        );

        let order_by =
            caps.get(5)
                .map(|x| format!(
                    "proposal_{}time", x
                        .as_str()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect::<String>()))
                .unwrap_or("proposal_id".to_string())
                .to_owned()
                .to_lowercase();
        let limit = caps
            .get(6)
            .map(|x| x.as_str())
            .unwrap_or("1")
            .to_owned()
            .parse::<usize>()
            .unwrap();
        let subscribe = caps
            .get(7)
            .map(|x| x.as_str() == "subscribe")
            .unwrap_or(false);
        let unsubscribe = caps
            .get(7)
            .map(|x| x.as_str() == "unsubscribe")
            .unwrap_or(false);

        let request = serde_json::json!({"message": msg_for_query, "handler":"query_entries","fields":vec!["summary"],"indices":vec!["proposal_id"],"filter": filter, "order_by": order_by, "limit":limit, "subscribe": subscribe, "unsubscribe": unsubscribe, "user_hash": user_hash});
        let response = client_send_request(request).unwrap();
        notify_sled_db(db, response);
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
