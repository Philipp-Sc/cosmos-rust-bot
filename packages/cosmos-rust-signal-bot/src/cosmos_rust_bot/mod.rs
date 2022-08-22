use bot_library::*;
// to get control over the settings
use cosmos_rust_interface::utils::entry::db::query::query_entries;
use cosmos_rust_interface::utils::entry::db::*;
use cosmos_rust_interface::utils::entry::{EntryValue, Entry};
use regex::Regex;

use std::collections::HashMap;

// add a state_storage to remember the user and provide it to both functions mutable.

// todo: task list, errors, logs commands. !!!!

pub async fn handle_message(msg: String) -> String {
    let entries: Vec<Entry> = load_entries("../../cosmos-rust-bot-db.json").await.unwrap_or(Vec::new());
    // lookup specific subset of proposals, subscribe to get notified when changes occur.
    // \lookup_proposals latest 10 \subscribe
    // \lookup_proposals terra voting latest 10 \unsubscribe
    let task_info_regex = Regex::new("\\\\task_info(?: (count|list))(?: (pending|resolved|upcoming|failed|unknown|reserved|all))?(?: \\\\(subscribe|unsubscribe))?(?:\\s|$)").unwrap();
    let task_info_history_regex = Regex::new("\\\\task_info(?: (history))(?: ([0-9]+))?(?: \\\\(subscribe|unsubscribe))?(?:\\s|$)").unwrap();

    // todo: load blockchain_names and join them, from list/file.
    // todo: or just match any blockchain name. user is responsible.(adjust regex)
    let lookup_proposals_regex = Regex::new("\\\\lookup_proposals(?: (Terra|Osmosis|Juno))?(?: (StatusNil|StatusPassed|StatusFailed|StatusRejected|StatusDepositPeriod|StatusVotingPeriod))?(?: (TextProposal|CommunityPoolSpendProposal|ParameterChangeProposal|SoftwareUpgradeProposal|ClientUpdateProposal|UpdatePoolIncentivesProposal|StoreCodeProposal|UnknownProposalType))?(?: (LatestTime))?(?: ([0-9]+))?(?: \\\\(subscribe|unsubscribe))?(?:\\s|$)").unwrap();

    // get proposal by id, subscribe to get notified when the proposal gets updated.
    // \get_proposal terra 1 \subscribe
    let get_proposal_regex = Regex::new("\\\\get_proposal(?: (Terra|Osmosis|Juno))(?: #([0-9]+))(?: \\\\(subscribe|unsubscribe))?(?:\\s|$)").unwrap();

    if task_info_regex.is_match(&msg) {
        let caps = task_info_regex.captures(&msg).unwrap();
        let mut filter: HashMap<String, String> = HashMap::new();
        match caps.get(1) {
            Some(t) => {
                filter.insert("group".to_string(), format!("task_{}", t.as_str()));
            }
            _ => {
                filter.insert("group".to_string(), "any".to_string());
            }
        };
        match caps.get(2) {
            Some(t) => {
                filter.insert("key".to_string(), t.as_str().to_string());
            }
            _ => {
                filter.insert("key".to_string(), "any".to_string());
            }
        };

        let order_by = "index".to_string();
        let subset = query_entries(&entries, filter, order_by, 20);
        let mut return_msg = "".to_string();
        for i in 0..subset.len() {
            if let EntryValue::Value(ref val) = subset[i].value {
                return_msg = format!("{}\n\n{}", return_msg, val["info"].as_str().unwrap().to_string());
            }
        }
        return return_msg;
    } else if lookup_proposals_regex.is_match(&msg) {
        let default_lookup = vec!["any", "any", "any", "LatestTime", "1"];
        let caps = lookup_proposals_regex.captures(&msg).unwrap();
        let mut lookup_proposals: Vec<String> = Vec::new();
        for i in 1..caps.len() - 1 {
            match caps.get(i) {
                Some(t) => {
                    lookup_proposals.push(format!("{}", t.as_str()));
                }
                None => {
                    lookup_proposals.push(default_lookup.get(i - 1).unwrap().to_string());
                }
            }
        }
        let mut filter: HashMap<String, String> = HashMap::new();
        filter.insert("blockchain".to_string(), lookup_proposals[0].to_owned());
        filter.insert("status".to_string(), lookup_proposals[1].to_owned());
        filter.insert("type".to_string(), lookup_proposals[2].to_owned());
        let order_by = lookup_proposals[3].to_owned();
        let limit = lookup_proposals[4].parse::<usize>().unwrap();
        let subset = query_entries(&entries, filter, order_by, limit);
        let mut return_msg = "".to_string();
        for i in 0..subset.len() {
            if let EntryValue::Value(ref val) = subset[i].value {
                return_msg = format!("{}\n\n{}", return_msg, val["info"].as_str().unwrap().to_string());
            }
        }
        return return_msg;
    } else if get_proposal_regex.is_match(&msg) {}
// \subscriptions list  (lists subs)
// \subscription delete 1 (deactivate/activate/delete subs)
// same as lookup_proposals but sets up notifications, which will notify when response changes, sending the difference (either new proposal (trough new id), or updated proposal (through info change))
    "Hi, I read your message. ~ Your Cosmos-Rust-Bot".to_string()
}

pub async fn handle_notifications() -> Option<String> {
    None
}