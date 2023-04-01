use bot_library::shared::UserSettings as UserSettingsImported;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;

const TASKS_PATH: &str = "./tmp/cosmos-rust-bot-feature-list.json";

pub type UserSettings = UserSettingsImported;

// around every 5s a new block is generated

const MINUTES_1: i32 = 60 * 1;
const MINUTES_5: i32 = 60 * 5;
const MINUTES_10: i32 = 60 * 10;


lazy_static! {
    static ref LIST_BLOCKCHAINS: Vec<String> = {
        let data = std::fs::read_to_string("./tmp/supported_blockchains.json").expect("Unable to read file");
        let supported_blockchains: HashMap<String, serde_json::Value> = serde_json::from_str(&data).expect("Unable to parse JSON");
        supported_blockchains.into_keys().collect()
    };
}

#[derive(Debug, Serialize, Deserialize,PartialEq)]
pub enum TaskType {
    ChainRegistry,
    FraudDetection,
    GPT3,
    LinkToText,
    GovernanceProposals,
    TallyResults,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    name: String,
    requirements: Vec<TaskSpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSpec {
    pub name: String,
    pub kind: TaskType,
    pub refresh_rate: i32,
    pub args: serde_json::Value,
}

pub fn feature_list() -> Vec<Feature> {
    let feature_list: Vec<Feature> = match fs::read_to_string(TASKS_PATH)
    {
        Ok(file) => match serde_json::from_str(&file) {
            Ok(res) => res,
            Err(err) => {
                println!("{:?}", err);
                Default::default()
            }
        },
        Err(err) => {
            println!("{:?}", err);
            Default::default()
        }
    };
    feature_list
}

pub fn feature_list_to_file() -> anyhow::Result<()> {
    let mut feature_list: Vec<Feature> = Vec::new();

    let mut governance_proposals: Vec<TaskSpec> = Vec::new();
    let mut tally_results: Vec<TaskSpec> = Vec::new();

    let proposal_status_list = vec![
        "voting_period",
        "deposit_period",
        "failed",
        "passed",
        "rejected",
        /*"nil",*/ // TODO query only if no other states of that proposal exist.
    ];

    for blockchain in LIST_BLOCKCHAINS.iter() {
        let task = TaskSpec {
            kind: TaskType::TallyResults,
            name: format!("{}_tally_results_{}_proposals", blockchain, "voting_period"),
            args: json!({
                    "blockchain": blockchain,
                    "proposal_status": "voting_period"
                }),
            refresh_rate: MINUTES_10,
        };
        tally_results.push(task);
        for proposal_status in &proposal_status_list {
            let task = TaskSpec {
                kind: TaskType::GovernanceProposals,
                name: format!("{}_governance_{}_proposals", blockchain, proposal_status),
                args: json!({
                    "blockchain": blockchain,
                    "proposal_status": proposal_status
                }),
                refresh_rate: MINUTES_5,
            };
            governance_proposals.push(task);
        }
    }

    feature_list.push(Feature {
        name: "governance_proposal_notifications".to_string(),
        requirements: governance_proposals,
    });
    feature_list.push(Feature {
        name: "governance_proposal_tally_results".to_string(),
        requirements: tally_results,
    });

    let mut chain_registry: Vec<TaskSpec> = Vec::new();
    let task = TaskSpec {
        kind: TaskType::ChainRegistry,
        name: format!("chain_registry"),
        args: json!({
                    "path": "./chain-registry",
                }),
        refresh_rate: MINUTES_10,
    };
    chain_registry.push(task);

    feature_list.push(Feature {
        name: "chain_registry".to_string(),
        requirements: chain_registry,
    });

    let mut fraud_detection: Vec<TaskSpec> = Vec::new();
    let task = TaskSpec {
        kind: TaskType::FraudDetection,
        name: format!("fraud_detection"),
        args: json!({
            // here should be the socket path
                }),
        refresh_rate: 10,
    };
    fraud_detection.push(task);

    feature_list.push(Feature {
        name: "fraud_detection".to_string(),
        requirements: fraud_detection,
    });

    let mut gpt3: Vec<TaskSpec> = Vec::new();
    let task = TaskSpec {
        kind: TaskType::GPT3,
        name: format!("gpt3"),
        args: json!({
            // here should be the socket path
                }),
        refresh_rate: 10,
    };
    gpt3.push(task);

    let mut link_to_text: Vec<TaskSpec> = Vec::new();
    let task = TaskSpec {
        kind: TaskType::LinkToText,
        name: format!("link_to_text"),
        args: json!({
            // here should be the socket path
                }),
        refresh_rate: 10,
    };
    gpt3.push(task);

    feature_list.push(Feature {
        name: "gpt3".to_string(),
        requirements: gpt3,
    });

    // TODO: create new feature task that gets all wallet info.

    let line = format!("{}", serde_json::to_string(&feature_list).unwrap());
    fs::write(TASKS_PATH, &line).unwrap();
    Ok(())
}

fn feature_name_list(user_settings: &UserSettings) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    if user_settings.governance_proposal_notifications {
        args.push("governance_proposal_notifications".to_string());
        args.push("governance_proposal_tally_results".to_string());
    }
    args.push("chain_registry".to_string());
    args.push("fraud_detection".to_string());
    args.push("gpt3".to_string());
    args
}

pub fn get_feature_list(user_settings: &UserSettings) -> Vec<Feature> {
    let args = feature_name_list(user_settings);
    let mut features = feature_list();
    features = features
        .into_iter()
        .filter(|x| args.contains(&x.name))
        .collect();
    features
}

pub fn get_requirements(user_settings: &UserSettings) -> Vec<TaskSpec> {
    let mut features: Vec<Feature> = get_feature_list(user_settings);
    let mut req: Vec<TaskSpec> = Vec::new();
    for mut f in features {
        let mut no_duplicates = f
            .requirements
            .into_iter()
            .filter(|x| req.iter().filter(|y| y.name == x.name).count() == 0)
            .collect();
        req.append(&mut no_duplicates);
    }
    req
}

#[cfg(test)]
mod test {

    use cosmos_rust_interface::cosmos_rust_package::tokio as tokio;
    // cargo test -- --nocapture

    #[tokio::test]
    pub async fn feature_list_to_file() -> anyhow::Result<()> {
        super::feature_list_to_file()
    }

    #[tokio::test]
    pub async fn feature_list() -> anyhow::Result<()> {
        println!("{:?}", super::feature_list());
        Ok(())
    }
}
