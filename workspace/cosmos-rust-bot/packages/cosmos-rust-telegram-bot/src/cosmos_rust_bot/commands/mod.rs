use cosmos_rust_interface::utils::entry::{db::{notification::notify_sled_db, query::socket::*}, EntriesQueryPart, SettingsPart, QueryPart, SubscriptionsQueryPart};
use regex::{Match, Regex};

use heck::{ToTitleCase, ToUpperCamelCase};
use std::collections::HashMap;
use chrono::TimeZone;

use cosmos_rust_interface::utils::entry::*;

const SUB_UNSUB: &str = "(subscribe|unsubscribe)";

const LIST_TASK_STATES: [&str;6] = ["pending","resolved","upcoming","failed","unknown","reserved"];

const LIST_BLOCKCHAINS: [&str;5] = ["terra","terra2","osmosis","juno","cosmos hub"];
const LIST_PROPOSAL_STATUS: [&str;6] = ["nil","passed","failed","rejected","deposit period","voting period"];
const LIST_PROPOSAL_TYPE: [&str;8] = ["text","community pool spend","parameter change","software upgrade","client update","update pool incentives","store code","unknown"];
const LIST_PROPOSAL_TIME: [&str;5] = ["latest","submit","deposit end","voting start","voting end"];

const QUERY_SOCKET: &str = "./tmp/cosmos_rust_bot_query_socket";

pub fn handle_tasks_count_list_history(user_hash: u64, msg: &str, msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()>  {
        let task_info_regex = Regex::new(
            format!(
                "tasks (count|list|history)({})?(?: ([0-9]+))?(?: {})?(?:\\s|$)",
                format!("[{}]+",LIST_TASK_STATES.map(|x| " ".to_string()+x).join("|")), SUB_UNSUB
            )
                .as_str(),
        )
            .unwrap();
        if task_info_regex.is_match(&msg) {
            let caps = task_info_regex.captures(&msg).unwrap();
            let mut filter: Vec<(String, String)> = Vec::new();
            let k = caps.get(1).map(|t| t.as_str());
            filter.push((
                "kind".to_string(),
                k.map(|t| format!("task_{}", t))
                    .unwrap_or("any".to_string()),
            ));
            let mut filter_list: Vec<Vec<(String, String)>> = Vec::new();
            filter_list.push(filter);

            filter_list = add_filter(filter_list,caps.get(2),LIST_TASK_STATES.to_vec(),"state",("",""));


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

            let request: UserQuery = UserQuery{ query_part: QueryPart::EntriesQueryPart(EntriesQueryPart{
                message: msg_for_query.to_string(),
                fields: vec!["summary".to_string()],
                indices: vec!["task_meta_data".to_string()],
                filter: filter_list,
                order_by,
                limit,
            }), settings_part: SettingsPart {
                subscribe: Some(subscribe),
                unsubscribe: Some(unsubscribe),
                user_hash: Some(user_hash)
            } };

            let response = client_send_query_request(QUERY_SOCKET,request).unwrap();
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

            let mut filter: Vec<(String, String)> = Vec::new();
            let fields = match k {
                "logs" | "errors" => {
                    vec!["summary".to_string()]
                }
                "debug" | _ => {
                    vec!["key".to_string(), "value".to_string()]
                }
            };
            let request: UserQuery = UserQuery{ query_part: QueryPart::EntriesQueryPart(EntriesQueryPart{
                message: msg_for_query.to_string(),
                fields,
                indices: vec![format!("task_meta_data_{}",k)],
                filter: vec![filter],
                order_by: "timestamp".to_string(),
                limit,
            }), settings_part: SettingsPart {
                subscribe: Some(subscribe),
                unsubscribe: Some(unsubscribe),
                user_hash: Some(user_hash)
            } };

            let response = client_send_query_request(QUERY_SOCKET,request).unwrap();
            notify_sled_db(db, response);
            return Ok(());
        }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}

pub fn handle_subscribe_unsubscribe(user_hash: u64, msg: &str, msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()>  {
    let unsubscribe: bool = if msg == "unsubscribe all" {
            true
        } else if msg == "subscriptions" {
            false
        } else {
            return Err(anyhow::anyhow!("Error: Unknown Command!"));
    };
    let request: UserQuery = UserQuery{ query_part: QueryPart::SubscriptionsQueryPart(SubscriptionsQueryPart{
        message: msg_for_query.to_string(),
    }), settings_part: SettingsPart {
        subscribe: None,
        unsubscribe: Some(unsubscribe),
        user_hash: Some(user_hash)
    } };

    let response = client_send_query_request(QUERY_SOCKET,request).unwrap();
    notify_sled_db(db, response);
    Ok(())
}


pub fn handle_gov_prpsl(user_hash: u64, msg: &str, msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()> {
    let lookup_proposals_regex = Regex::new(format!("gov prpsl({})?(?: id([0-9]+))?({})?({})?({})?(?: ([0-9]+))?(?: {})?(?:\\s|$)",
                                                    format!("[{}]+",LIST_BLOCKCHAINS.map(|x| " ".to_string()+x).join("|")),
                                                    format!("[{}]+",LIST_PROPOSAL_STATUS.map(|x| " ".to_string()+x).join("|")),
                                                    format!("[{}]+",LIST_PROPOSAL_TYPE.map(|x| " ".to_string()+x).join("|")),
                                                    format!("[{}]+",LIST_PROPOSAL_TIME.map(|x| " ".to_string()+x).join("|")),
                                                    SUB_UNSUB).as_str()).unwrap();

    if lookup_proposals_regex.is_match(&msg) {
        let caps = lookup_proposals_regex.captures(&msg).unwrap();
        let mut filter: Vec<(String, String)> = Vec::new();
        filter.push((
            "proposal_id".to_string(),
            caps.get(2)
                .map(|t| format!("{}", t.as_str()))
                .unwrap_or("any".to_string()),
        ));/*
        filter.push((
            "proposal_VotingEndTime".to_string(),
            format!("lt {}",chrono::Utc::now().timestamp()+60*60*48),
        ));*/

        let mut filter_list: Vec<Vec<(String, String)>> = Vec::new();
        filter_list.push(filter);

        filter_list= add_filter(filter_list,caps.get(1),LIST_BLOCKCHAINS.to_vec(),"proposal_blockchain",("",""));

        filter_list= add_filter(filter_list,caps.get(0),LIST_PROPOSAL_STATUS.to_vec(),"proposal_status",("Status",""));

        filter_list= add_filter(filter_list,caps.get(0),LIST_PROPOSAL_TYPE.to_vec(),"proposal_type",("","Proposal"));


        let order_by =
            caps.get(0)
                .map(|x| {
                    let text = x.as_str();
                    for time in LIST_PROPOSAL_TIME {
                        if text.contains(time) {
                            return format!("proposal_{}Time", time.to_upper_camel_case());
                        }
                    }
                    "proposal_id".to_string()
                })
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
            .get(0)
            .map(|x| x.as_str().contains("subscribe") && !x.as_str().contains("unsubscribe") )
            .unwrap_or(false);
        let unsubscribe = caps
            .get(0)
            .map(|x| x.as_str().contains("unsubscribe"))
            .unwrap_or(false);

        let request: UserQuery = UserQuery{ query_part: QueryPart::EntriesQueryPart(EntriesQueryPart{
            message: msg_for_query.to_string(),
            fields: vec!["summary".to_string()],
            indices: vec!["proposal_id".to_string()],
            filter: filter_list,
            order_by,
            limit,
        }), settings_part: SettingsPart {
            subscribe: Some(subscribe),
            unsubscribe: Some(unsubscribe),
            user_hash: Some(user_hash)
        } };

        let response = client_send_query_request(QUERY_SOCKET,request).unwrap();
        notify_sled_db(db, response);
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}

fn add_filter(filter_list: Vec<Vec<(String, String)>>, regex_match: Option<Match>, list: Vec<&str>, name: &str, format_str: (&str,&str)) -> Vec<Vec<(String, String)>> {
    let mut new_filter_list: Vec<Vec<(String, String)>> = Vec::new();
    match regex_match {
        Some(t) => {
            let text = t.as_str().to_string();
            for item in list {
                if text.contains(item) {
                    let mut filter_list_copy = filter_list.clone();
                    if filter_list_copy.is_empty() {
                        let mut filter: Vec<(String, String)> = Vec::new();
                        filter.push((
                            name.to_string(),
                            format!("{}{}{}",
                                    format_str.0,
                                    item.to_upper_camel_case(),
                                    format_str.1,
                            )
                        ));
                        new_filter_list.push(filter);
                    }else {
                        for i in 0..filter_list_copy.len() {
                            filter_list_copy[i].push((
                                name.to_string(),
                                format!("{}{}{}",
                                        format_str.0,
                                        item.to_upper_camel_case(),
                                        format_str.1,
                                )
                            ));
                        }
                    }
                    new_filter_list.append(&mut filter_list_copy);
                }
            }
            if !new_filter_list.is_empty() {
                return new_filter_list;
            }
        },
        None => {
        }
    };

    let mut filter_list_copy = filter_list.clone();
    for i in 0..filter_list_copy.len() {
        filter_list_copy[i].push((
            name.to_string(),
            "any".to_string()
        ));
    }
    new_filter_list.append(&mut filter_list_copy);
    new_filter_list
}