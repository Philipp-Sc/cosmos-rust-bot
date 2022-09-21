use chrono::Utc;
use cosmos_rust_interface::utils::entry::{
    db::{notification::notify_sled_db, query::socket::*},
    CosmosRustServerValue, Notify,
};
use regex::Regex;

use heck::ToTitleCase;
use std::collections::HashMap;

use cosmos_rust_interface::utils::entry::UserMetaData;


pub async fn handle_message(user_id: u64, message: String, db: &sled::Db) {

    // remove whitespace
    let mut msg = String::with_capacity(message.len());
    message.trim().to_lowercase().replace("/","").replace("_"," ").replace("\n","").split_whitespace().for_each(|w| {
        if !msg.is_empty() {
            msg.push(' ');
        }
        msg.push_str(w);
    });

    let msg_for_query = msg.replace(" subscribe","").replace(" unsubscribe","");
    log::info!(
            "Handle Message: Msg: {} - {:?}",
            Utc::now(),
            &msg
        );

    let blockchain_regex = "(terra2|osmosis|juno|cosmoshub)";
    let state_regex = "(pending|resolved|upcoming|failed|unknown|reserved)";
    let sub_regex = "(subscribe|unsubscribe)";
    let task_info_regex = Regex::new(
        format!(
            "tasks (count|list|history)(?: {})?(?: ([0-9]+))?(?: {})?(?:\\s|$)",
            state_regex, sub_regex
        )
        .as_str(),
    )
    .unwrap();
    let lookup_proposals_regex = Regex::new(format!("gov prpsl(?: {})?(?: #([0-9]+))?(?: (nil|passed|failed|rejected|deposit period|voting period))?(?: (text|community pool spend|parameter change|software upgrade|client update|update pool incentives|store code|unknown))?(?: (latest|submit|deposit end|voting start|voting end))?(?: ([0-9]+))?(?: {})?(?:\\s|$)", blockchain_regex, sub_regex).as_str()).unwrap();
    let log_error_debug_regex = Regex::new(
        format!(
            "tasks (logs|errors|debug)(?: ([0-9]+))?(?: {})?(?:\\s|$)",
            sub_regex
        )
        .as_str(),
    )
    .unwrap();
    let help_regex = Regex::new("help(?:\\s|$)").unwrap();
    let help_examples_regex = Regex::new("help examples(?:\\s|$)").unwrap();

    let mut request = serde_json::Value::Null;

    if msg == "start".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"🤖💬 Welcome! To get started just type /help or learn more about my development on github via /about."#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    }else if msg == "about".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"https://github.com/Philipp-Sc/cosmos-rust-bot"#.to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    }else if msg == "health".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"🤖 /tasks - Monitor Tasks
🤖 /help_tasks - Show help"#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    }else if msg == "help".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    "🤖💬I am happy to help.\nDo you want to lean how to lookup proposals?\n/help_governance_proposals\n\nIn case you want to subscribe/unsubscribe\n/help_subscriptions".to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    }else if msg == "help tasks".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"🔭️ Monitor Tasks🔭
=================
🤖 COMMAND
/tasks
<subcommand>
<state>
<limit>
SUBCOMMAND
['count', 'list', 'history']
STATE
['pending', 'resolved', 'upcoming', 'failed', 'unknown', 'reserved']
LIMIT
e.g. 1,2,..
=================
ℹ For examples check /tasks
"#.to_string(), r#"🔭️ Monitor Results🔭
==================
🤖 COMMAND
/tasks
<subcommand>
<limit>
SUBCOMMAND
['logs', 'errors', 'debug']
LIMIT
e.g. 1,2,..
=================
ℹ For examples check /tasks"#.to_string()
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    }else if msg == "help governance proposals".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"🛰️ Lookup Governance Proposals 🛰️
===============================
🤖 COMMAND
/gov_prpsl
<blockchain>
<proposal_id>
<proposal_status>
<proposal_type>
<order_by_proposal_time>
<limit>
BLOCKCHAIN
['terra2', 'osmosis', 'juno', 'cosmoshub']
PROPOSAL_ID
e.g. #1,#2,..
PROPOSAL_STATUS
['nil', 'passed', 'failed', 'rejected', 'deposit period', 'voting period']
PROPOSAL_TYPE
['text', 'community pool spend', 'parameter change', 'software proposal', 'client update', 'update pool incentives', 'store code', 'unknown']
PROPOSAL_TIME
['latest','submit','deposit end','voting start','voting end']
LIMIT
e.g. 1,2,.."#.to_string(),
                    "🤖💬 Cosmos-Rust-Bot gives you many options, but don't worry. I created common shortcuts for you.\n/governance_proposals".to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    }
    else if msg == "tasks".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Shortcuts
ℹ️ /tasks_count_6
ℹ️️ /tasks_count_pending
ℹ️ /tasks_count_failed
ℹ️ /tasks_count_resolved
ℹ️ /tasks_list_6
ℹ️ /tasks_list_pending
ℹ️ /tasks_list_failed
ℹ️ /tasks_list_resolved
ℹ️ /tasks_history_100
ℹ️ /tasks_history_pending_1
ℹ️ /tasks_history_failed_1
ℹ️ /tasks_history_resolved_1
ℹ️ /tasks_errors_1"#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    }else if msg == "governance proposals".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"🤖💬 Shortcuts
🔭 /latest_proposals
🔭 /proposals_voting_period
🔭 /proposals_deposit_period
🔭 /proposals_rejected
🔭 /proposals_passed
🔭 /proposals_failed
🔭 /proposal_by_id
"#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    } else if msg == "proposal by id".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Get proposal by id
🔭 /gov_prpsl_terra2 #<id>
🔭 /gov_prpsl_osmosis #<id>
🔭 /gov_prpsl_juno #<id>
🔭 /gov_prpsl_cosmoshub #<id>
"#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    } else if msg == "latest proposals".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Get the latest proposal
🔭 /gov_prpsl_latest_1
🔭 /gov_prpsl_terra2_1
🔭 /gov_prpsl_osmosis_1
🔭 /gov_prpsl_juno_1
🔭 /gov_prpsl_cosmoshub_1
"#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    } else if msg == "proposals voting period".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Get the latest proposal in voting period
🔭 /gov_prpsl_voting_period_latest_1
🔭 /gov_prpsl_terra2_voting_period_1
🔭 /gov_prpsl_osmosis_voting_period_1
🔭 /gov_prpsl_juno_voting_period_1
🔭 /gov_prpsl_cosmoshub_voting_period_1
"#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    } else if msg == "proposals deposit period".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Get the latest proposal in deposit period
🔭 /gov_prpsl_deposit_period_latest_1
🔭 /gov_prpsl_terra2_deposit_period_1
🔭 /gov_prpsl_osmosis_deposit_period_1
🔭 /gov_prpsl_juno_deposit_period_1
🔭 /gov_prpsl_cosmoshub_deposit_period_1
"#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    } else if msg == "proposals rejected".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Get the latest rejected proposal
🔭 /gov_prpsl_rejected_latest_1
🔭 /gov_prpsl_terra2_rejected_1
🔭 /gov_prpsl_osmosis_rejected_1
🔭 /gov_prpsl_juno_rejected_1
🔭 /gov_prpsl_cosmoshub_rejected_1
"#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    } else if msg == "proposals passed".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Get the latest passed proposal
🔭 /gov_prpsl_passed_latest_1
🔭 /gov_prpsl_terra2_passed_1
🔭 /gov_prpsl_osmosis_passed_1
🔭 /gov_prpsl_juno_passed_1
🔭 /gov_prpsl_cosmoshub_passed_1
"#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    } else if msg == "proposals failed".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Get the latest failed proposal
🔭 /gov_prpsl_failed_latest_1
🔭 /gov_prpsl_terra2_failed_1
🔭 /gov_prpsl_osmosis_failed_1
🔭 /gov_prpsl_juno_failed_1
🔭 /gov_prpsl_cosmoshub_failed_1
"#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    }   else if log_error_debug_regex.is_match(&msg) {
        let caps = log_error_debug_regex.captures(&msg).unwrap();
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

        request = serde_json::json!({"message": msg_for_query, "handler":"query_entries", "fields":fields,"indices":vec![format!("task_meta_data_{}",k).as_str()],"filter": filter, "order_by": "timestamp", "limit":limit, "subscribe": subscribe, "unsubscribe": unsubscribe, "user_id": user_id});
    } else if task_info_regex.is_match(&msg) {
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
        request = serde_json::json!({"message": msg_for_query, "handler":"query_entries", "fields":vec!["summary"],"indices":vec!["task_meta_data"],"filter": filter, "order_by": order_by, "limit":limit, "subscribe": subscribe, "unsubscribe": unsubscribe, "user_id": user_id});
    }  else if msg == "help subscriptions".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    "🤖💬 To manage notifications just append subscribe or unsubscribe like in the following example:\n/gov_prpsl_voting_period_latest_1_subscribe\n/gov_prpsl_voting_period_latest_1_unsubscribe\n\nHere you can find commonly used notifications:\n/common_subs\n\nTo see your current subscriptions\n/subscriptions\n\nIn case you want to delete all your subscriptions\n/unsubscribe_all".to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    }  else if msg == "common subs".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    "🤖💬 Get notified when:\n\n - the latest proposal changes (any blockchain)\n/gov_prpsl_latest_1_subscribe\n\n - there is a new proposal in voting period\n/gov_prpsl_voting_period_latest_1_subscribe\n\n - or only follow your favourite cosmos-chains:\n/gov_prpsl_terra2_voting_period_1_subscribe\n/gov_prpsl_osmosis_voting_period_1_subscribe\n/gov_prpsl_comoshub_voting_period_1_subscribe\n/gov_prpsl_juno_voting_period_1_subscribe".to_string(),
                    "🤖💬 To learn more about the different parameters:\n/help_governance_proposals".to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    } else if msg=="unsubscribe all" {
        request = serde_json::json!({"message": msg_for_query, "handler":"query_subscriptions", "unsubscribe": true, "user_id": user_id});
    } else if msg=="subscriptions" {
        request = serde_json::json!({"message": msg_for_query, "handler":"query_subscriptions", "unsubscribe": false, "user_id": user_id});
    }
    else if lookup_proposals_regex.is_match(&msg) {
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
                "proposal_{}time",x
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

        request = serde_json::json!({"message": msg_for_query, "handler":"query_entries","fields":vec!["summary"],"indices":vec!["proposal_id"],"filter": filter, "order_by": order_by, "limit":limit, "subscribe": subscribe, "unsubscribe": unsubscribe, "user_id": user_id});
    }else {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Unknown command.
Type /help to see all the commands."#
                        .to_string(),
                ],
                user_hash: UserMetaData::user_hash(user_id),
            }),
        );
    }
    if !request.is_null() {
        log::info!(
            "Handle Message: Query: {} - {:?}",
            Utc::now(),
            &request
        );
        let response = client_send_request(request).unwrap();
        notify_sled_db(db, response);
    }
}
