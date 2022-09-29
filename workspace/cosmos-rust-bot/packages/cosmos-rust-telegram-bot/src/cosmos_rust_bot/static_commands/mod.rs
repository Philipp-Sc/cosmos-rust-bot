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
                    r#"🤖💬 Welcome! To get started just type /help or learn more about my development on github via /about."#
                        .to_string(),
                ],
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
                    r#"🤖💬Why did my creator make me? And who am I?

A Rust Bot for the Cosmos Ecosystem.

I created this bot to learn about smart contracts, the Terra blockchain and to get to know the Rust programming language.
What I noticed then - and is still true now for the broader Cosmos Ecosystem - is that bots play an important role in crypto, at the same time not many have access to one. Having your own bot at your command is in many ways convenient and it often gives the power back to you.

The project's purpose is to have a bot on your side:

USERS

- Save the hassle doing things manually
- Enable strategies only bots can execute
- Receive alerts and notifications
- Send commands for the bot to execute

DEVELOPERS

- Provide insights into the Cosmos Ecosystem
- Enable developers to write their own bot
- Showcase how to use cosmos-rust
- Rust

You are currently interacting with an early (beta) version of the cosmos-rust-telegram-bot.
The limitations are that this is a publicly hosted version of cosmos-rust-bot, letting the bot sign and execute transactions is not possible (a security risk). Therefore cosmos-rust-telegram-bot only automates notifications and lookups of on-chain data. If there are transactions to sign, you will need to do it manually via your wallet app.

Cosmos-rust-bot starts out with governance notifications & lookup features, that being said other interesting features such as the following are planned:
- wallet tracking
- validator profile tracking
- osmosiszone price/token/pool alerts
...
You can lookup available features via /help.
(to see the full roadmap checkout my github repository)

If you want to interact with the blockchain and execute transactions automaticly, you will have to setup your own cosmos-rust-signal-bot (for technically-skilled users).
Checkout cosmos-rust-bot on github:
https://github.com/Philipp-Sc/cosmos-rust-bot

Cosmos-rust-bot is a constant work in progress: Bug Reports and Feature Requests are welcome!
Either via github or write me directly @philipp_sc on telegram.

Thank you. ❤️

"#.to_string(),
                ],
                user_hash,
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
                    "🤖💬I am happy to help.\n\nDo you want to learn how to lookup proposals?\n/help_governance_proposals\n\nIn case you want to subscribe/unsubscribe\n/help_subscriptions\n\nYou can read about why I was created here\n/about\n".to_string(),
                ],
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
                    r#"🤖💬 Shortcuts to get you started:

/latest_proposals
-  lookup the most recent proposals (by blockchain)

/proposals_by_status
- lookup proposals by status (voting/deposit period, passed, rejected,..)

/proposal_by_id
- lookup proposal by id

/governance_proposals
- additional information"#.to_string(),
                ],
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
                    r#"🛰️ Lookup Governance Proposals 🛰️
===============================
🤖 COMMAND
/gov_prpsl_Blockchain_ProposalId_ProposalStatus_ProposalType_ProposalTime_Limit
Blockchain
['terra2', 'osmosis', 'juno', 'cosmos_hub']
ProposalId
e.g. id1,id2,..
ProposalStatus
['nil', 'passed', 'failed', 'rejected', 'deposit period', 'voting period']
ProposalType
['text', 'community pool spend', 'parameter change', 'software proposal', 'client update', 'update pool incentives', 'store code', 'unknown']
ProposalTime
['latest','submit','deposit end','voting start','voting end']
Limit
e.g. 1,2,.."#.to_string(),
                ],
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
                    "🤖💬 To manage subscriptions just append subscribe or unsubscribe to a request.\n\nList your current subscriptions:\n/subscriptions\n\nDelete all your subscriptions:\n/unsubscribe_all\n\nCommonly used subscriptions:\n/gov_prpsl_subs".to_string(),
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

/gov_prpsl_cosmos_hub_id1
- lookup proposal on Cosmos-Hub with id 1 (options are terra2, osmosis, juno or cosmos_hub)"#
                        .to_string(),
                ],
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

/gov_prpsl_latest_3
- lookup latest 3 proposals (any supported blockchain)

/gov_prpsl_terra2_latest_3
- lookup latest 3 proposals on Terra (options are terra2, osmosis, juno or cosmos_hub)

/gov_prpsl_juno_osmosis_latest_1
- lookup latest proposals on Juno or Osmosis (you can specify multiple blockchains)

"#
                        .to_string(),
                ],
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

/gov_prpsl_voting_period_latest_3
- lookup latest 3 proposals in voting period  (options are voting_period, deposit_period, rejected, passed or failed)

/gov_prpsl_terra2_deposit_period_latest_3
- lookup latest 3 proposals in deposit period on Terra

/gov_prpsl_terra2_voting_period_deposit_period_latest_3
- lookup latest 3 proposals in voting or deposit period on Terra (you can specify multiple statuses)

/gov_prpsl_terra2_passed_rejected_latest_3
- lookup latest 3 passed or rejected proposals on Terra

"#
                        .to_string(),
                ],
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
                    "🤖💬 Get notified:\n\n/gov_prpsl_latest_1_subscribe\n- a proposal was created or updated\n\n/gov_prpsl_voting_period_latest_1_subscribe\n- a proposal enters the voting period\n\n/gov_prpsl_deposit_period_latest_1_subscribe\n- a proposal enters the deposit period\n\n/gov_prpsl_voting_period_deposit_period_latest_1_subscribe\n- a proposal enters the voting or deposit period".to_string(),
                    "🤖💬 You have more control over the selection by specifying parameters, to learn more check:\n/help_governance_proposals".to_string(),
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
                user_hash,
            }),
        );
    Ok(())
}