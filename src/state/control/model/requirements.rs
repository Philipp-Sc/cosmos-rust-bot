use std::collections::HashMap;
use serde_json::json;
use terra_rust_bot_essentials::shared::UserSettings as UserSettingsImported;
use crate::state::control::model::requirements::RequirementType::GovernanceProposals;

pub type UserSettings = UserSettingsImported;

pub enum RequirementType {
    GovernanceProposals,
    None,
}

pub struct Feature {
    name: String,
    dependencies: Vec<Requirement>,
}

pub struct Requirement {
    pub kind: RequirementType,
    pub name: String,
    // UNIQUE
    pub args: serde_json::Value,
    pub refresh_rate_in_seconds: i32,
}

pub fn feature_list() -> Vec<Feature> {

    // note: around every 6s a new block is generated.
    let fast: i32 = 10;      // 10s for short lived information
    let medium: i32 = 60;    // 1m  for short lived information
    let slow: i32 = 60 * 10;   // 10m for relative constant information.

    let mut feature_list: Vec<Feature> = Vec::new();

    feature_list.push(Feature {
        name: "osmosis_governance_failed_proposals".to_string(),
        dependencies: vec![
            Requirement {
                kind: GovernanceProposals,
                name: "osmosis_governance_failed_proposals".to_string(),
                args: json!({
                    "blockchain": "osmosis",
                    "proposal_status": "failed"
                }), // add governance_proposals_notifications (state of the proposals) here as optional args
                refresh_rate_in_seconds: fast,
            }],
    });
    feature_list.push(Feature {
        name: "osmosis_governance_passed_proposals".to_string(),
        dependencies: vec![
            Requirement {
                kind: GovernanceProposals,
                name: "osmosis_governance_passed_proposals".to_string(),
                args: json!({
                    "blockchain": "osmosis",
                    "proposal_status": "passed"
                }), // add governance_proposals_notifications (state of the proposals) here as optional args
                refresh_rate_in_seconds: fast,
            }],
    });
    feature_list.push(Feature {
        name: "terra_governance_rejected_proposals".to_string(),
        dependencies: vec![
            Requirement {
                kind: GovernanceProposals,
                name: "terra_governance_rejected_proposals".to_string(),
                args: json!({
                    "blockchain": "terra",
                    "proposal_status": "rejected"}),
                refresh_rate_in_seconds: fast,
            }
            /*
            Requirement {
                name: "gas_fees".to_string(),
                args: Default::default(),
                refresh_rate_in_seconds: medium,
            },
            Requirement {
                name: "core_swap".to_string(),
                args: json!(r#"{from: "uusd", to: "usdr"}"#),
                refresh_rate_in_seconds: fast,
            },
            Requirement {
                name: "core_swap".to_string(),
                args: json!(r#"{from: "usdr", to: "uluna"}"#),
                refresh_rate_in_seconds: fast,
            },
            Requirement {
                name: "core_swap".to_string(),
                args: json!(r#"{from: "uluna", to: "uusd"}"#),
                refresh_rate_in_seconds: fast,
            },
            Requirement {
                name: "simulate_swap".to_string(),
                args: json!(r#"{dex: "terraswap", protocol_from: "none", from: "uluna", protocol_to: "Anchor", to: "bLuna"}"#),
                refresh_rate_in_seconds: fast,
            },
            */
        ],
    });

    feature_list

    /*
    // (key, target_refresh_time, dependency_tag)
    vec![
        (vec!["governance_proposals_terra"], fast, vec!["governance_proposals_terra"]),
        (vec!["governance_proposals_osmosis"], fast, vec!["governance_proposals_osmosis"]),
        /* <from settings> */
        (vec!["trigger_percentage"], fast, vec!["anchor_account", "anchor_auto_repay"]),
        (vec!["target_percentage"], fast, vec!["anchor_auto_repay", "anchor_auto_borrow"]),
        (vec!["borrow_percentage"], fast, vec!["anchor_auto_borrow"]),
        (vec!["gas_adjustment_preference"], fast, vec!["anchor_account", "anchor_auto_farm", "anchor_auto_stake", "anchor_auto_stake_airdrops", "anchor_auto_repay", "anchor_auto_borrow"]),
        (vec!["min_ust_balance"], fast, vec!["anchor_account", "anchor_auto_farm", "anchor_auto_stake", "anchor_auto_stake_airdrops", "anchor_auto_repay", "anchor_auto_borrow"]),
        (vec!["ust_balance_preference"], fast, vec!["anchor_auto_repay"]),
        (vec!["max_tx_fee"], fast, vec!["anchor_auto_farm", "anchor_auto_stake", "anchor_auto_stake_airdrops", "anchor_auto_repay", "anchor_auto_borrow"]),
        /* <for gas fees>*/
        (vec!["gas_fees_uusd"], medium, vec!["market", "anchor", "anchor_account", "anchor_auto_farm", "anchor_auto_stake", "anchor_auto_stake_airdrops", "anchor_auto_repay", "anchor_auto_borrow"]),
        (vec!["tax_rate"], medium, vec!["anchor_auto_repay", "anchor_auto_borrow", "anchor_auto_farm", "anchor_auto_stake"]),
        (vec!["tax_caps"], medium, vec!["anchor_auto_repay", "anchor_auto_borrow", "anchor_auto_farm", "anchor_auto_stake"]),
        /**/
        (vec!["terra_balances"], fast, vec!["anchor_auto_farm", "anchor_auto_stake", "anchor_auto_stake_airdrops", "anchor_auto_repay", "anchor_auto_borrow"]),
        /* <market_info> */
        /* core_tokens */
        (vec!["core_swap uusd usdr"], fast, vec!["market"]),
        (vec!["core_swap usdr uluna"], fast, vec!["market"]),
        (vec!["core_swap uluna uusd"], fast, vec!["market"]),
        // "simulation terraswap uluna uusd_uluna_pair_contract",
        /* anchor_tokens */
        (vec!["simulate_swap", "terraswap", "none", "uluna", "Anchor", "bLuna"], fast, vec!["market", "anchor_account", "arbitrage"]),
        (vec!["state", "Anchor", "bLuna Hub"], fast, vec!["market", "anchor_account"]),
        (vec!["simulate_swap", "terraswap", "Anchor", "ANC", "none", "uusd"], fast, vec!["market", "anchor_account", "anchor_auto_farm", "anchor_auto_stake"]),
        (vec!["epoch_state", "Anchor", "Market"], fast, vec!["anchor", "market", "anchor_account", "anchor_auto_repay"]),
        /* nexus_tokens */
        (vec!["simulate_swap", "terraswap", "Nexus", "nLuna", "Nexus", "Psi"], fast, vec!["market"]),
        (vec!["simulate_swap", "terraswap", "Nexus", "Psi", "none", "uusd"], fast, vec!["market"]),
        /* mirror_tokens */
        (vec!["simulate_swap", "terraswap", "Mirror", "MIR", "none", "uusd"], fast, vec!["market"]),
        (vec!["simulate_swap", "terraswap", "Mirror", "mTSLA", "none", "uusd"], fast, vec!["market"]),
        (vec!["simulate_swap", "terraswap", "Mirror", "mBTC", "none", "uusd"], fast, vec!["market"]),
        (vec!["simulate_swap", "terraswap", "Mirror", "mETH", "none", "uusd"], fast, vec!["market"]),
        (vec!["simulate_swap", "terraswap", "Mirror", "mSPY", "none", "uusd"], fast, vec!["market"]),
        //("terra_money_assets_cw20_tokens"], slow, vec!["market"]),
        //("terra_money_assets_cw20_pairs"], slow, vec!["market"]),
        //("terra_money_assets_cw20_contracts"], slow, vec!["market"]),
        /* <other> */
        /* <anchor_protocol> */
        (vec!["state", "Anchor", "Market"], fast, vec!["anchor", "anchor_account"]),
        (vec!["api/v2/distribution-apy"], fast, vec!["anchor", "anchor_account", "anchor_auto_farm", "anchor_auto_stake"]),
        (vec!["api/v2/gov-reward"], fast, vec!["anchor", "anchor_account", "anchor_auto_stake"]),
        (vec!["config", "Anchor", "Interest Model"], fast, vec!["anchor", "anchor_account"]),
        //("config Anchor Fee Collector",every_minute),
        /* <anchor_protocol account> */
        (vec!["anchor_airdrops"], fast, vec!["anchor_auto_stake_airdrops"]),
        (vec!["borrow_limit"], fast, vec!["anchor_account", "anchor_auto_repay", "anchor_auto_borrow"]),
        (vec!["borrow_info"], fast, vec!["anchor_account", "anchor_auto_farm", "anchor_auto_stake", "anchor_auto_repay", "anchor_auto_borrow"]),
        (vec!["balance"], fast, vec!["anchor_account", "anchor_auto_repay", "anchor_auto_borrow"]),
        (vec!["anc_balance"], fast, vec!["anchor_account", "anchor_auto_stake"]),
        (vec!["staker"], fast, vec!["anchor_account", "anchor_auto_stake"]),
        (vec!["blocks_per_year"], slow, vec!["market", "anchor", "anchor_account"]),
        (vec!["earn_apy"], slow, vec!["anchor", "anchor_account"]),
        (vec!["anchor_protocol_whitelist"], slow, vec!["anchor_account"]),
        /* <meta data> */
        (vec!["anchor_protocol_txs_claim_rewards"], slow, vec!["anchor", "anchor_account", "anchor_auto_farm", "anchor_auto_stake"]),
        (vec!["anchor_protocol_txs_staking"], slow, vec!["anchor", "anchor_account", "anchor_auto_stake"]),
        (vec!["anchor_protocol_txs_redeem_stable"], slow, vec!["anchor_auto_repay"]),
        (vec!["anchor_protocol_txs_deposit_stable"], slow, vec!["anchor_auto_borrow"]),
        (vec!["anchor_protocol_txs_borrow_stable"], slow, vec!["anchor_auto_borrow"]),
        (vec!["anchor_protocol_txs_repay_stable"], slow, vec!["anchor_auto_repay"]),
//        ("anchor_protocol_txs_provide_liquidity"], slow, vec!["anchor_auto_farm"]), 
//        ("anchor_protocol_txs_staking_lp"], slow, vec!["anchor_auto_farm"]), 
        (vec!["txs_provide_to_spec_anc_ust_vault"], slow, vec!["anchor_auto_farm"]),
//        ("api/v2/ust-lp-reward"], slow, vec!["anchor_auto_farm"]), 
        (vec!["api/data?type=lpVault"], slow, vec!["anchor_auto_farm"]),
    ]*/
}

pub fn my_feature_list(user_settings: &UserSettings) -> Vec<Feature> {
    let args = settings_to_key_list(user_settings);
    let mut features = feature_list();
    features = features.into_iter().filter(|x| args.contains(&x.name)).collect();
    features
}

pub fn my_requirement_list(user_settings: &UserSettings) -> Vec<Requirement> {
    let mut features = my_feature_list(user_settings);
    let mut req: Vec<Requirement> = Vec::new();
    for mut f in features {
        req.append(&mut f.dependencies);
    }
    req
}

fn settings_to_key_list(user_settings: &UserSettings) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    if user_settings.anchor_protocol_auto_stake {
        args.push("anchor_auto_stake".to_string());
    }
    if user_settings.anchor_protocol_auto_farm {
        args.push("anchor_auto_farm".to_string());
    }
    if user_settings.anchor_protocol_auto_repay {
        args.push("anchor_auto_repay".to_string());
    }
    if user_settings.anchor_protocol_auto_borrow {
        args.push("anchor_auto_borrow".to_string());
    }
    if user_settings.terra_market_info {
        args.push("market".to_string());
    }
    if user_settings.anchor_general_info {
        args.push("anchor".to_string());
    }
    if user_settings.anchor_account_info {
        args.push("anchor_account".to_string());
    }
    match user_settings.governance_blockchains.as_ref() {
        None => {}
        Some(blockchains) => {
            for b in blockchains {
                for c in user_settings.governance_proposals_notifications.as_ref().unwrap() {
                    args.push(format!("{}_governance_{}_proposals", b, c));
                }
            }
        }
    }
    args
}