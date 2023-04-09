use cosmos_rust_interface::utils::entry::{db::{notification::notify_sled_db, query::socket::*}, EntriesQueryPart, SettingsPart, QueryPart, SubscriptionsQueryPart};
use regex::{Match, Regex};

use heck::{ToTitleCase, ToUpperCamelCase};
use std::collections::HashMap;
use chrono::TimeZone;

use cosmos_rust_interface::utils::entry::*;


lazy_static! {
    static ref LIST_BLOCKCHAINS: Vec<String> = {
        let data = std::fs::read_to_string("./tmp/supported_blockchains.json").expect("Unable to read file");
        let supported_blockchains: HashMap<String, serde_json::Value> = serde_json::from_str(&data).expect("Unable to parse JSON");
        supported_blockchains.into_keys().collect()
    };
}

const SUB_UNSUB: &str = "(subscribe|unsubscribe)";

const LIST_TASK_STATES: [&str;6] = ["pending","resolved","upcoming","failed","unknown","reserved"];

const LIST_PROPOSAL_STATUS: [&str;6] = ["nil","passed","failed","rejected","deposit period","voting period"];
const LIST_PROPOSAL_TYPE: [&str;8] = ["text","community pool spend","parameter change","software upgrade","client update","update pool incentives","store code","unknown"];
const LIST_PROPOSAL_TIME: [&str;5] = ["latest","submit","deposit end","voting start","voting end"];

const LIST_GOV_PRPSL_VIEWS: [&str;13] = ["status","briefing0","briefing1","briefing2","briefing3","briefing4","briefing5","briefing6","briefing7","briefing8","briefing9","briefing10","content"];

const QUERY_SOCKET: &str = "./tmp/cosmos_rust_bot_query_socket";

use lazy_static::lazy_static;

lazy_static!{
   pub static ref LOOKUP_PROPOSALS_REGEX: Regex = Regex::new(format!("gov prpsl({})?({})?(?: id([0-9]+))?({})?({})?({})?(?: ([0-9]+))?(?: {})?(?:\\s|$)",
                                                    format!("[{}]+",LIST_GOV_PRPSL_VIEWS.map(|x| " ".to_string()+x).join("|")),
                                                    format!("[{}]+",LIST_BLOCKCHAINS.iter().map(|x| " ".to_string()+x).collect::<Vec<String>>().join("|")),
                                                    format!("[{}]+",LIST_PROPOSAL_STATUS.map(|x| " ".to_string()+x).join("|")),
                                                    format!("[{}]+",LIST_PROPOSAL_TYPE.map(|x| " ".to_string()+x).join("|")),
                                                    format!("[{}]+",LIST_PROPOSAL_TIME.map(|x| " ".to_string()+x).join("|")),
                                                    SUB_UNSUB).as_str()).unwrap();
   pub static ref TASK_INFO_REGEX: Regex = Regex::new(
            format!(
                "tasks (count|list|history)({})?(?: ([0-9]+))?(?: {})?(?:\\s|$)",
                format!("[{}]+",LIST_TASK_STATES.map(|x| " ".to_string()+x).join("|")), SUB_UNSUB
            )
                .as_str(),
        )
            .unwrap();

    pub static ref LIMIT_REGEX: Regex = Regex::new(r"\s\d+").unwrap();

    pub static ref LOG_ERROR_DEBUG_REGEX: Regex = Regex::new(
            format!(
                "show (logs|errors|debug)(?: ([0-9]+))?(?: {})?(?:\\s|$)",
                SUB_UNSUB
            )
                .as_str(),
        ).unwrap();


    pub static ref VERIFY_REGEX: Regex = Regex::new(r"(verify \d+)").unwrap();

}


pub fn handle_tasks_count_list_history(user_hash: u64, msg: &str, msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()>  {

        if TASK_INFO_REGEX.is_match(&msg) {
            let caps = TASK_INFO_REGEX.captures(&msg).ok_or(anyhow::anyhow!("Error: Parse Error!"))?;
            let mut filter: Vec<(String, String)> = Vec::new();
            let k = caps.get(1).map(|t| t.as_str());
            filter.push((
                "kind".to_string(),
                k.map(|t| format!("task_{}", t))
                    .unwrap_or("any".to_string()),
            ));
            let mut filter_list: Vec<Vec<(String, String)>> = Vec::new();
            filter_list.push(filter);

            filter_list = add_filter(filter_list,msg.to_string(),LIST_TASK_STATES.to_vec(),"state",("",""));


            let order_by = match k {
                Some("history") => "timestamp".to_string(),
                _ => "index".to_string(),
            };

            let limit = if LIMIT_REGEX.is_match(&msg) {LIMIT_REGEX.captures(&msg).ok_or(anyhow::anyhow!("Error: Parse Error!"))?.get(0).map(|x| {
                &x.as_str()[1..]
            }).unwrap_or("1").parse::<usize>().unwrap_or(1usize)}else{1usize};

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
                display: "default".to_string(),
                indices: vec!["task_meta_data".to_string()],
                filter: filter_list,
                order_by,
                limit,
            }), settings_part: SettingsPart {
                subscribe: Some(subscribe),
                unsubscribe: Some(unsubscribe),
                register: None,
                user_hash: Some(user_hash)
            } };

            let response = client_send_query_request(QUERY_SOCKET,request)?;
            notify_sled_db(db, response);
            return Ok(());
        }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}

pub fn handle_tasks_logs_errors_debug(user_hash: u64, msg: &str, msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()>  {
        if LOG_ERROR_DEBUG_REGEX.is_match(msg) {
            let caps = LOG_ERROR_DEBUG_REGEX.captures(msg).ok_or(anyhow::anyhow!("Error: Parse Error!"))?;
            let mut filter: Vec<(String, String)> = Vec::new();
            let k = caps.get(1).map(|t| t.as_str()).ok_or(anyhow::anyhow!("Error: Parse Error!"))?;
            filter.push((
                "kind".to_string(),
                "error".to_string(),
            ));

            let limit = if LIMIT_REGEX.is_match(&msg) {LIMIT_REGEX.captures(&msg).ok_or(anyhow::anyhow!("Error: Parse Error!"))?.get(0).map(|x| {
                &x.as_str()[1..]
            }).unwrap_or("100").parse::<usize>().unwrap_or(1usize)}else{1usize};

            let subscribe = caps
                .get(3)
                .map(|x| x.as_str() == "subscribe")
                .unwrap_or(false);
            let unsubscribe = caps
                .get(3)
                .map(|x| x.as_str() == "unsubscribe")
                .unwrap_or(false);

            let request: UserQuery = UserQuery{ query_part: QueryPart::EntriesQueryPart(EntriesQueryPart{
                message: msg_for_query.to_string(),
                display: "default".to_string(),
                indices: vec![format!("task_meta_data_{}",k)],
                filter: vec![filter],
                order_by: "timestamp".to_string(),
                limit,
            }), settings_part: SettingsPart {
                subscribe: Some(subscribe),
                unsubscribe: Some(unsubscribe),
                register: None,
                user_hash: Some(user_hash)
            } };
            println!("{:?}",request);

            let response = client_send_query_request(QUERY_SOCKET,request)?;
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
        register: None,
        user_hash: Some(user_hash)
    } };

    let response = client_send_query_request(QUERY_SOCKET,request)?;
    notify_sled_db(db, response);
    Ok(())
}

pub fn handle_register(user_hash: u64, msg: &str, _msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()>  {
    let register: bool = if msg == "sign up" {
        true
    } else if msg == "get token" {
        false
    } else {
        return Err(anyhow::anyhow!("Error: Unknown Command!"));
    };
    let request: UserQuery = UserQuery{ query_part: QueryPart::RegisterQueryPart(RegisterQueryPart{}), settings_part: SettingsPart {
        subscribe: None,
        unsubscribe: None,
        register: Some(register),
        user_hash: Some(user_hash)
    } };

    let response = client_send_query_request(QUERY_SOCKET,request)?;
    notify_sled_db(db, response);
    Ok(())
}

pub fn handle_verify(user_hash: u64, msg: &str, _msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()>  {
    if VERIFY_REGEX.is_match(&msg){
        let caps = VERIFY_REGEX.captures(&msg).ok_or(anyhow::anyhow!("Error: Parse Error!"))?;
        let token = caps.get(2).map(|t| format!("{}", t.as_str())).ok_or(anyhow::anyhow!("Error: Parse Error!"))?.parse::<u64>()?;

        let request: UserQuery = UserQuery{ query_part: QueryPart::AuthQueryPart(AuthQueryPart{ token, user_hash }), settings_part: SettingsPart {
            subscribe: None,
            unsubscribe: None,
            register: None,
            user_hash: Some(user_hash)
        } };

        let response = client_send_query_request(QUERY_SOCKET,request)?;
        notify_sled_db(db, response);
        return Ok(());
    } else {
        return Err(anyhow::anyhow!("Error: Unknown Command!"));
    };

}

pub fn handle_gov_prpsl(user_hash: u64, msg: &str, msg_for_query: &str, db: &sled::Db) -> anyhow::Result<()> {
    if LOOKUP_PROPOSALS_REGEX.is_match(&msg) {
        let caps = LOOKUP_PROPOSALS_REGEX.captures(&msg).ok_or(anyhow::anyhow!("Error: Parse Error!"))?;
        let mut filter: Vec<(String, String)> = Vec::new();
        filter.push((
            "proposal_id".to_string(),
            caps.get(3)
                .map(|t| format!("{}", t.as_str()))
                .unwrap_or("any".to_string()),
        ));/*
        filter.push((
            "proposal_VotingEndTime".to_string(),
            format!("lt {}",chrono::Utc::now().timestamp()+60*60*48),
        ));*/

        let mut filter_list: Vec<Vec<(String, String)>> = Vec::new();
        filter_list.push(filter);

        filter_list= add_filter(filter_list,msg.to_string(),LIST_BLOCKCHAINS.iter().map(|s| s.as_str()).collect(),"proposal_blockchain",("",""));
        // TODO: add check if terra and terra2 included make sure to remove terra if not intended.
        filter_list= add_filter(filter_list,msg.to_string(),LIST_PROPOSAL_STATUS.to_vec(),"proposal_status",("Status",""));
        filter_list= add_filter(filter_list,msg.to_string(),LIST_PROPOSAL_TYPE.to_vec(),"proposal_type",("","Proposal"));


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


        let limit = if LIMIT_REGEX.is_match(&msg) {LIMIT_REGEX.captures(&msg).ok_or(anyhow::anyhow!("Error: Parse Error!"))?.get(0).map(|x| {
            &x.as_str()[1..]
        }).unwrap_or("1").parse::<usize>().unwrap_or(1usize)}else{1usize};

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
            display: "default".to_string(),
            indices: vec!["proposal_id".to_string()],
            filter: filter_list,
            order_by,
            limit,
        }), settings_part: SettingsPart {
            subscribe: Some(subscribe),
            unsubscribe: Some(unsubscribe),
            register: None,
            user_hash: Some(user_hash)
        } };
        println!("{:?}",&request);

        let response = client_send_query_request(QUERY_SOCKET,request)?;
        notify_sled_db(db, response);
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}


// This function takes in a list of filters, a string of text, a list of strings, a name, and a format string as parameters.
fn add_filter(filter_list: Vec<Vec<(String, String)>>, text: String, list: Vec<&str>, name: &str, format_str: (&str,&str)) -> Vec<Vec<(String, String)>> {
    
    let add_filter_item = |filter_list_copy: &mut Vec<Vec<(String, String)>>, name: &str, item: &str, format_str: (&str, &str)| {
        // Iterate over each filter in `filter_list_copy`
        filter_list_copy.iter_mut().for_each(|filter| {
            // Push a new filter item to the current filter
            filter.push((
                name.to_string(),
                format!("{}{}{}",
                        format_str.0,
                        item.to_upper_camel_case(),
                        format_str.1,
                ),
            ));
        });
    };

    // Create a new empty list to store the updated filters.
    let mut new_filter_list: Vec<Vec<(String, String)>> = Vec::new();

    // Iterate through the given list of strings.

    for word in text.split_whitespace() {
        if list.contains(&word) {
            // Add the word to the filter list

            // Make a copy of the filter list.
            let mut filter_list_copy = filter_list.clone();

            // If the filter list is empty,
            if filter_list_copy.is_empty() {
                // Create a new filter and add it to the new filter list.
                let mut filter: Vec<(String, String)> = Vec::new();
                new_filter_list.push(filter);
                add_filter_item(&mut filter_list_copy, name, word, format_str);
            } else {
                // Iterate through the copied filter list and add the current string to each filter.
                add_filter_item(&mut filter_list_copy, name, word, format_str);
            }

            // Append the copied and updated filter list to the new filter list.
            new_filter_list.append(&mut filter_list_copy);
        }
    }

    // If the new filter list is not empty, return it.
    if !new_filter_list.is_empty() {
        return new_filter_list;
    }

    // If the new filter list is empty, make a copy of the filter list and add a default filter to each filter in the copied list.
    let mut filter_list_copy = filter_list.clone();
    for i in 0..filter_list_copy.len() {
        filter_list_copy[i].push((
            name.to_string(),
            "any".to_string()
        ));
    }

    // Append the copied and updated filter list to the new filter list and return it.
    new_filter_list.append(&mut filter_list_copy);
    new_filter_list
}