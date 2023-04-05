use chrono::Utc;
use lazy_static::lazy_static;
use cosmos_rust_interface::utils::entry::{
    db::{notification::notify_sled_db},
    CosmosRustServerValue, Notify,
};
use std::collections::HashMap;

lazy_static! {
    static ref SUPPORTED_BLOCKCHAINS: Vec<(String, serde_json::Value)> = {
        let data = std::fs::read_to_string("./tmp/supported_blockchains.json").expect("Unable to read file");
        let mut blockchains: HashMap<String, serde_json::Value> = serde_json::from_str(&data).expect("Unable to parse JSON");
        let mut blockchains: Vec<(String, serde_json::Value)> = blockchains.into_iter().collect();
        blockchains.sort_by_key(|(_,b)| b.get("rank").unwrap().as_u64().unwrap());
        blockchains
    };
}


pub fn handle_start(user_hash: u64, msg: &str, db: &sled::Db) -> anyhow::Result<()> {
    if msg == "start" {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![ // 🤖💬 Welcome! Save time and hassle with automatic notifications and on-chain data lookup.
                    r#"Ready to receive transmisions from the Cosmos? 🛰️"#
                        .to_string(),
                ],
                buttons: vec![vec![
                    // (Daily) Cosmos Governance Briefing (CGB)
                    vec![("🔔 Get Updates".to_string(),"/get_updates".to_string())],
                  /*  vec![("🔔 List Subscriptions".to_string(),"/subscriptions".to_string()),("❌ Unsubscribe All".to_string(),"/unsubscribe_all".to_string())], */
                  /*  vec![("🔍 Search".to_string(),"/search".to_string())], */
                    vec![("❔ Info".to_string(),"/about".to_string())]]],
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
r#"
Welcome to @cosmos_governance_briefings_bot! I am an independent developer with expertise in blockchain technology and governance, and I am dedicated to improving participation in the Cosmos ecosystem. My bot provides alerts and analysis on governance proposals, making it easier for users to navigate the process and stay informed about the proposals that are being considered.

One of the unique features of my bot is the integrated fraud detection feature, which helps users identify potential scams or malicious proposals. This is particularly important in the cryptocurrency space, where scams and fraud can be a significant issue. I also use open source technology and AI to generate brief explanations of key aspects of governance proposals, helping users better understand complex technical language and make informed voting decisions. This is particularly useful for users who may be intimidated by the technical language or unfamiliar with the governance process, as it provides a more accessible and user-friendly way to participate.

In addition to providing notifications and analysis, the bot also allows users to easily access and review proposals, staying up-to-date on the latest governance developments in the Cosmos ecosystem. This can be particularly helpful for users who may not have the time or resources to keep track of all the proposals on their own.

I believe that my governance notification bot can be a valuable tool for improving governance participation in the Cosmos ecosystem. By providing clear and concise information about proposals, I can help users make more informed decisions about how to contribute to the governance process. I am committed to maintaining the bot and ensuring its ongoing reliability and effectiveness.

Thank you for choosing @cosmos_governance_briefings_bot as your go-to resource for governance notifications and analysis in the Cosmos ecosystem! I welcome any feedback or suggestions for improvement.
"#.to_string(),
                ],
                buttons: vec![vec![vec![("GitHub".to_string(),"https://github.com/Philipp-Sc/cosmos-rust-bot".to_string())]]],
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
    if msg == "search".to_string() {
        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: vec![
                    r#"Lookup Governance Proposals

Use the /gov_prpsl command to lookup governance proposals on a specified blockchain network. Refer to the man page for detailed instructions, or use one of the three shortcuts to quickly find what you're looking for.
"#.to_string(),
                ],
                buttons: vec![vec![vec![("/latest_proposals".to_string(),"/latest_proposals".to_string())],
                              vec![("/proposals_by_status".to_string(),"/proposals_by_status".to_string())],
                              vec![("/proposal_by_id".to_string(),"/proposal_by_id".to_string())],
                              vec![("man page".to_string(),"/governance_proposals".to_string())]]],
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
        - kujira

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
                    r#"Get proposal by id:

To lookup a proposal with ID "1" on the Cosmos-Hub blockchain, use the following command:

/gov_prpsl_cosmos_hub_id1

Note that you can also use this command to lookup proposals on the following blockchain networks:

    terra2
    osmosis
    juno

For more information, refer to the man page."#
                        .to_string(),
                ],
                buttons: vec![vec![vec![("man page".to_string(),"/governance_proposals".to_string())]]],
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
                    r#"Get the latest proposals:
To lookup the latest 3 proposals on the Terra2 network, use the following command:

/gov_prpsl_terra2_latest_3

To lookup the latest proposals on both the Juno and Osmosis networks, use the following command:

/gov_prpsl_juno_osmosis_latest

For more information and detailed instructions, refer to the man page."#
                        .to_string(),
                ],
                buttons: vec![vec![vec![("man page".to_string(),"/governance_proposals".to_string())]]],
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
                    r#"Get proposals by status:

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
                buttons: vec![vec![vec![("man page".to_string(),"/governance_proposals".to_string())]]],
                user_hash,
            }),
        );
        return Ok(());
    }
    Err(anyhow::anyhow!("Error: Unknown Command!"))
}

pub fn handle_common_subs(user_hash: u64, msg: &str, db: &sled::Db)  -> anyhow::Result<()> {
    if msg == "get updates" {
        let mut msg_vec = vec![
            "🔔 You can receive updates for the following:\n\n- Proposal enters deposit period (💰)\n\n- Proposal enters voting period (🗳)\n\n- Proposal outcome (🟢 passed, 🔴 rejected, ❌ failed)".to_string()
        ];

        let mut buttons_vec = vec![vec![]];


        for (k, v) in SUPPORTED_BLOCKCHAINS.iter() {
            let display_name = v.get("display").unwrap().as_str().unwrap();
            msg_vec.push(display_name.to_string());
            let mut button_vec =
                vec![
                    vec![
                        ("💰".to_string(),format!("/gov_prpsl_{}_deposit_period_subscribe",k)),
                        ("🗳".to_string(),format!("/gov_prpsl_{}_voting_period_subscribe",k)),
                        ("🟢 ❌ 🔴".to_string(),format!("/gov_prpsl_{}_passed_rejected_failed_subscribe",k))
                    ]];
            buttons_vec.push(button_vec);
        }

        notify_sled_db(
            db,
            CosmosRustServerValue::Notify(Notify {
                timestamp: Utc::now().timestamp(),
                msg: msg_vec,

                buttons: buttons_vec,
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
                    r#"Unknown command."#
                        .to_string(),
                ],
                buttons: vec![vec![vec![("Help".to_string(),"/start".to_string())]]],
                user_hash,
            }),
        );
    Ok(())
}