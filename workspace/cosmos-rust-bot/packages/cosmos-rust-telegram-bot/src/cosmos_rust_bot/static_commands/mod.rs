use chrono::Utc;
use cosmos_rust_interface::utils::entry::{
    db::{notification::notify_sled_db},
    CosmosRustServerValue, Notify,
};



pub fn handle_start(user_hash: u64, msg: &str, db: &sled::Db) -> anyhow::Result<()> {
    if msg == "start" {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"🤖💬 Welcome! Save time and hassle with automatic notifications and on-chain data lookup. Ready to receive transmisions from the Cosmos? 🛰️ "#
                        .to_string(),
                ],
                buttons: vec![
                    vec![("Setup Notifications".to_string(),"/help_subscriptions".to_string())],
                    vec![("Lookup Governance Proposals".to_string(),"/help_governance_proposals".to_string())],
                    vec![("About".to_string(),"/about".to_string())]],
                user_hash,
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
                    r#"Cosmos-Rust-Bot - a Rust bot for the Cosmos Ecosystem.

This is an early beta version, so please keep in mind that certain features may not be available. Check out our GitHub repository for more information and to see our roadmap. Thank you for using cosmos-rust-telegram-bot!
"#.to_string(),
                ],
                buttons: vec![vec![("GITHUB".to_string(),"".to_string())]],
                user_hash,
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
            buttons: vec![],
            user_hash,
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
                    r#"🤖💬 Lookup Governance Proposals

Use the /gov_prpsl command to lookup governance proposals on a specified blockchain network. Refer to the man page for detailed instructions, or use one of the three shortcuts to quickly find what you're looking for.
"#.to_string(),
                ],
                buttons: vec![vec![("/latest_proposals".to_string(),"/latest_proposals".to_string())],
                              vec![("/proposals_by_status".to_string(),"/proposals_by_status".to_string())],
                              vec![("/proposal_by_id".to_string(),"/proposal_by_id".to_string())],
                              vec![("man page".to_string(),"/governance_proposals".to_string())]],
                user_hash,
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
                    r#"/gov_prpsl - look up governance proposals on a specified blockchain network

SYNOPSIS
    /gov_prpsl Blockchain ProposalId ProposalStatus ProposalType ProposalTime Limit [subscribe | unsubscribe]

DESCRIPTION
    The /gov_prpsl command allows users to look up governance proposals on one of the following blockchain networks:
        - terra2
        - osmosis
        - juno
        - cosmos_hub

    The command takes the following parameters:
        Blockchain: the blockchain network on which to look up governance proposals.
        ProposalId: the unique identifier of the proposal to look up. Valid values are:
            - id1
            - id2
        ProposalStatus: the status of the proposal to look up. Valid values are:
            - nil
            - passed
            - failed
            - rejected
            - deposit period
            - voting period
        ProposalType: the type of the proposal to look up. Valid values are:
            - text
            - community pool spend
            - parameter change
            - software proposal
            - client update
            - update pool incentives
            - store code
            - unknown
        ProposalTime: the timestamp to order the proposals by. Valid values are:
            - latest
            - submit
            - deposit end
            - voting start
            - voting end
        Limit: the maximum number of proposals to return.
        subscribe: an optional parameter that can be used to receive notifications.
        unsubscribe: an optional parameter that can be used to stop receiving notifications.

EXAMPLES
    To look up the latest proposal with ID "1" on the osmosis network:
        /gov_prpsl_osmosis_id1_latest_1

    To look up proposals of type "parameter change" that are in voting period on the cosmos_hub network:
        /gov_prpsl_cosmos_hub_voting_period_parameter_change
"#.to_string(),
                ],
                buttons: vec![],
                user_hash,
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
                    r#"🤖💬 Manage Subscriptions"#.to_string(),
                ],
                buttons: vec![vec![("Get started".to_string(),"/gov_prpsl_subs".to_string())],
                              vec![("My Subscriptions".to_string(),"/subscriptions".to_string())],
                              vec![("Unsubscribe All".to_string(),"/unsubscribe_all".to_string())]
                              ],
                user_hash,
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
                buttons: vec![],
                user_hash,
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
                    r#"🤖💬 Get proposal by id:

To lookup a proposal with ID "1" on the Cosmos-Hub blockchain, use the following command:

/gov_prpsl_cosmos_hub_id1

Note that you can also use this command to lookup proposals on the following blockchain networks:

    terra2
    osmosis
    juno

For more information, refer to the man page."#
                        .to_string(),
                ],
                buttons: vec![vec![("man page".to_string(),"/governance_proposals".to_string())]],
                user_hash,
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
                    r#"🤖💬 Get the latest proposals:
To lookup the latest 3 proposals on the Terra2 network, use the following command:

/gov_prpsl_terra2_latest_3

To lookup the latest proposals on both the Juno and Osmosis networks, use the following command:

/gov_prpsl_juno_osmosis_latest

For more information and detailed instructions, refer to the man page."#
                        .to_string(),
                ],
                buttons: vec![vec![("man page".to_string(),"/governance_proposals".to_string())]],
                user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}


pub fn handle_proposals_by_status(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "proposals by status" {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"🤖💬 Get proposals by status:

To lookup the latest 3 proposals in the "deposit period" on the Terra2 network, use the following command:

/gov_prpsl_terra2_deposit_period_latest_3

To lookup the latest 3 proposals in the "voting period" or "deposit period" on the Terra2 network, use the following command:

/gov_prpsl_terra2_voting_period_deposit_period_latest_3

To lookup the latest 3 proposals that were "passed" or "rejected" on the Terra2 network, use the following command:

/gov_prpsl_terra2_passed_rejected_latest_3

For more information and detailed instructions, refer to the man page.
"#
                        .to_string(),
                ],
                buttons: vec![vec![("man page".to_string(),"/governance_proposals".to_string())]],
                user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}

pub fn handle_common_subs(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "gov prpsl subs" {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    "🤖💬 Just want to stay updated? Then select one of the pre-defined commands, to define more restrictive subscriptions checkout the man page.".to_string(),
                ],
                buttons: vec![
                    vec![("/gov_prpsl_osmosis_subscribe".to_string(),"/gov_prpsl_osmosis_subscribe".to_string())],
                    vec![("/gov_prpsl_terra2_subscribe".to_string(),"/gov_prpsl_terra2_subscribe".to_string())],
                    vec![("/gov_prpsl_juno_subscribe".to_string(),"/gov_prpsl_juno_subscribe".to_string())],
                    vec![("/gov_prpsl_cosmos_hub_subscribe".to_string(),"/gov_prpsl_cosmos_hub_subscribe".to_string())],
                    vec![("man page".to_string(),"/governance_proposals".to_string())],
                ],
                user_hash,
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
                buttons: vec![vec![("help".to_string(),"/help".to_string())]],
                user_hash,
            }),
        );
    Ok(())
}