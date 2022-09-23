use chrono::Utc;
use cosmos_rust_interface::utils::entry::{
    db::{notification::notify_sled_db, query::socket::*},
    CosmosRustServerValue, Notify,
};
use regex::Regex;

use heck::ToTitleCase;
use std::collections::HashMap;

use cosmos_rust_interface::utils::entry::UserMetaData;

pub fn handle_start(user_hash: u64, msg: &str, db: &sled::Db) -> anyhow::Result<()> {
    if msg == "start" {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"🤖💬 Welcome! To get started just type /help or learn more about my development on github via /about."#
                        .to_string(),
                ],
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}


pub fn handle_about(user_hash: u64, msg: &str, db: &sled::Db) -> anyhow::Result<()> {
    if msg == "about" {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"https://github.com/Philipp-Sc/cosmos-rust-bot"#.to_string(),
                ],
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}

pub fn handle_help(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "help" {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    "🤖💬I am happy to help.\nDo you want to learn how to lookup proposals?\n/help_governance_proposals\n\nIn case you want to subscribe/unsubscribe\n/help_subscriptions".to_string(),
                ],
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}


pub fn handle_help_tasks(user_hash: u64, msg: &str, db: &sled::Db) -> anyhow::Result<()>  {
    if msg == "help tasks" {
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
            user_hash: user_hash,
        }),
    );
        return Ok(());
}
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
pub fn handle_help_governance_proposals(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "help governance proposals".to_string() {
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
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
pub fn handle_help_subscriptions(user_hash: u64, msg: &str, db: &sled::Db) -> anyhow::Result<()>  {
    if msg == "help subscriptions" {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    "🤖💬 To manage notifications just append subscribe or unsubscribe like in the following example:\n/gov_prpsl_voting_period_latest_1_subscribe\n/gov_prpsl_voting_period_latest_1_unsubscribe\n\nHere you can find commonly used notifications:\n/common_subs\n\nTo see your current subscriptions\n/subscriptions\n\nIn case you want to delete all your subscriptions\n/unsubscribe_all".to_string(),
                ],
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
pub fn handle_tasks(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "tasks" {
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
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
pub fn handle_governance_proposals(user_hash: u64, msg: &str, db: &sled::Db) -> anyhow::Result<()>  {
    if msg == "governance proposals" {
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
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
pub fn handle_proposal_by_id(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "proposal by id" {
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
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
pub fn handle_latest_proposals(user_hash: u64, msg: &str, db: &sled::Db) -> anyhow::Result<()>  {
    if msg == "latest proposals" {
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
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}


pub fn handle_proposals_voting_period(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "proposals voting period" {
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
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
pub fn handle_proposals_deposit_period(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "proposals deposit period" {
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
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
pub fn handle_proposals_rejected(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "proposals rejected" {
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
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
pub fn handle_proposals_passed(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "proposals passed" {
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
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}
pub fn handle_proposals_failed(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "proposals failed" {
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
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}

pub fn handle_common_subs(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "common subs" {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    "🤖💬 Get notified when:\n\n - the latest proposal changes (any blockchain)\n/gov_prpsl_latest_1_subscribe\n\n - there is a new proposal in voting period\n/gov_prpsl_voting_period_latest_1_subscribe\n\n - or only follow your favourite cosmos-chains:\n/gov_prpsl_terra2_voting_period_1_subscribe\n/gov_prpsl_osmosis_voting_period_1_subscribe\n/gov_prpsl_comoshub_voting_period_1_subscribe\n/gov_prpsl_juno_voting_period_1_subscribe".to_string(),
                    "🤖💬 To learn more about the different parameters:\n/help_governance_proposals".to_string(),
                ],
                user_hash: user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}



pub fn handle_unknown_command(user_hash: u64, db: &sled::Db) -> anyhow::Result<()> {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Unknown command.
Type /help to see all the commands."#
                        .to_string(),
                ],
                user_hash: user_hash,
            }),
        );
    Ok(())
}