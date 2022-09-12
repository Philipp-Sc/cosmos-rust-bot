//use bot_library::*;
// to get control over the settings
use cosmos_rust_interface::utils::entry::db::{notification::notify_sled_db, query::socket::*};
use regex::Regex;

use heck::ToTitleCase;
//use heck::ToUpperCamelCase;
use std::collections::HashMap;

pub async fn handle_message(msg: String, db: &sled::Db) {
    let blockchain_regex = "(terra|osmosis|juno)";
    let state_regex = "(pending|resolved|upcoming|failed|unknown|reserved)";
    let sub_regex = "(subscribe|unsubscribe)";

    let msg = msg.to_lowercase();

    let task_info_regex = Regex::new(
        format!(
            "task (count|list|history)(?: {})?(?: ([0-9]+))?(?: {})?(?:\\s|$)",
            state_regex, sub_regex
        )
        .as_str(),
    )
    .unwrap();

    let lookup_proposals_regex = Regex::new(format!("lookup proposals(?: {})?(?: #([0-9]+))?(?: (nil|passed|failed|rejected|deposit period|voting period))?(?: (text|community pool spend|parameter change|software upgrade|client update|update pool incentives|store code|unknown))?(?: (latest|submit|deposit end|voting start|voting end))?(?: ([0-9]+))?(?: {})?(?:\\s|$)", blockchain_regex, sub_regex).as_str()).unwrap();

    // TODO: add help_regex, returns an explaination for each command (basically parses the regex and returns it in a human readable format)
    let log_error_debug_regex = Regex::new(
        format!(
            "task (logs|errors|debug)(?: ([0-9]+))?(?: {})?(?:\\s|$)",
            sub_regex
        )
        .as_str(),
    )
    .unwrap();
    if log_error_debug_regex.is_match(&msg) {
        let caps = log_error_debug_regex.captures(&msg).unwrap();
        let k = caps.get(1).map(|t| t.as_str()).unwrap();
        let limit = match caps.get(2) {
            Some(t) => t.as_str().parse::<usize>().unwrap(),
            None => 1000,
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

        let request = serde_json::json!({"fields":fields,"indices":vec![format!("task_meta_data_{}",k).as_str()],"filter": filter, "order_by": "timestamp", "limit":limit, "subscribe": subscribe, "unsubscribe": unsubscribe});
        //println!("{:?}", &request);
        let response = client_send_request(request).unwrap();
        notify_sled_db(db, response);
    }
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
            None => 1000,
        };
        let subscribe = caps
            .get(4)
            .map(|x| x.as_str() == "subscribe")
            .unwrap_or(false);
        let unsubscribe = caps
            .get(4)
            .map(|x| x.as_str() == "unsubscribe")
            .unwrap_or(false);
        let response = client_send_request(
            serde_json::json!({"fields":vec!["summary"],"indices":vec!["task_meta_data"],"filter": filter, "order_by": order_by, "limit":limit, "subscribe": subscribe, "unsubscribe": unsubscribe}),
        ).unwrap();
        notify_sled_db(db, response);
    } else if lookup_proposals_regex.is_match(&msg) {
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

        let order_by = format!(
            "proposal_{}time",
            caps.get(5)
                .map(|x| x
                    .as_str()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>())
                .unwrap_or("latest".to_string())
                .to_owned()
                .to_lowercase()
        );
        let limit = caps
            .get(6)
            .map(|x| x.as_str())
            .unwrap_or("20")
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

        let response = client_send_request(
            serde_json::json!({"fields":vec!["summary"],"indices":vec!["proposal_id"],"filter": filter, "order_by": order_by, "limit":limit, "subscribe": subscribe, "unsubscribe": unsubscribe}),
        ).unwrap();
        notify_sled_db(db, response);
    }
    // TODO \subscriptions list  (lists subs)
    // TODO \subscription delete 1 (deactivate/activate/delete subs)
    // same as lookup_proposals but sets up notifications, which will notify when response changes, sending the difference (either new proposal (trough new id), or updated proposal (through info change))
}
