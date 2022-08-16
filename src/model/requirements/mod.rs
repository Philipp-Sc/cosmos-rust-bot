use std::collections::HashMap;
use serde_json::json;
use bot_library::shared::UserSettings as UserSettingsImported;
use std::fs;
use serde::{Serialize, Deserialize};

pub type UserSettings = UserSettingsImported;


// around every 5s a new block is generated
const fast: i32 = 10;
// 10s
const medium: i32 = 60;
// 1m
const slow: i32 = 60 * 10;   // 10m

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskType {
    GovernanceProposals,
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
    let feature_list: Vec<Feature> = match fs::read_to_string("./cosmos-rust-bot-feature-list.json") {
        Ok(file) => {
            match serde_json::from_str(&file) {
                Ok(res) => {
                    res
                }
                Err(err) => {
                    println!("{:?}", err);
                    Default::default()
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
            Default::default()
        }
    };
    feature_list
}

pub fn feature_list_to_file() -> anyhow::Result<()> {
    let mut feature_list: Vec<Feature> = Vec::new();

    // max_age: 1 month
    feature_list.push(Feature {
        name: "governance_proposal_notifications".to_string(),
        requirements: vec![
            TaskSpec {
                kind: TaskType::GovernanceProposals,
                name: "osmosis_governance_voting_period_proposals".to_string(),
                args: json!({
                    "blockchain": "osmosis",
                    "proposal_status": "voting_period"
                }),
                refresh_rate: fast,
            }, TaskSpec {
                kind: TaskType::GovernanceProposals,
                name: "osmosis_governance_deposit_period_proposals".to_string(),
                args: json!({
                    "blockchain": "osmosis",
                    "proposal_status": "deposit_period"
                }),
                refresh_rate: fast,
            }, TaskSpec {
                kind: TaskType::GovernanceProposals,
                name: "osmosis_governance_failed_proposals".to_string(),
                args: json!({
                    "blockchain": "osmosis",
                    "proposal_status": "failed"
                }),
                refresh_rate: fast,
            }, TaskSpec {
                kind: TaskType::GovernanceProposals,
                name: "osmosis_governance_passed_proposals".to_string(),
                args: json!({
                    "blockchain": "osmosis",
                    "proposal_status": "passed"
                }),
                refresh_rate: fast,
            },
            TaskSpec {
                kind: TaskType::GovernanceProposals,
                name: "terra_governance_rejected_proposals".to_string(),
                args: json!({
                    "blockchain": "terra",
                    "proposal_status": "rejected"}),
                refresh_rate: fast,
            },
        ],
    });

    let line = format!("{}", serde_json::to_string(&feature_list).unwrap());
    fs::write("./cosmos-rust-bot-feature-list.json", &line).ok();
    Ok(())
}

fn feature_name_list(user_settings: &UserSettings) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    if user_settings.governance_proposal_notifications {
        args.push("governance_proposal_notifications".to_string());
    }
    args
}

pub fn get_feature_list(user_settings: &UserSettings) -> Vec<Feature> {
    let args = feature_name_list(user_settings);
    let mut features = feature_list();
    features = features.into_iter().filter(|x| args.contains(&x.name)).collect();
    features
}

pub fn get_requirements(user_settings: &UserSettings) -> Vec<TaskSpec> {
    let mut features: Vec<Feature> = get_feature_list(user_settings);
    let mut req: Vec<TaskSpec> = Vec::new();
    for mut f in features {
        let mut no_duplicates = f.requirements.into_iter().filter(|x| req.iter().filter(|y| y.name == x.name).count() == 0).collect();
        req.append(&mut no_duplicates);
    }
    req
}


#[cfg(test)]
mod test {

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
