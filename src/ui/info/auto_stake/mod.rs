use terra_rust_bot_essentials::shared::Entry;
use crate::state::control::model::{Maybe};
use crate::state::control::model::{await_function};

use crate::view::*;
use crate::view::interface::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc;
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::ResponseResult;
use tokio::sync::{Mutex};


/**
 * Anchor Auto Stake requires that the account balance has sufficient funds.
 * Info: It will not replenish the account balance. 
 * */
pub async fn lazy_anchor_account_auto_stake_rewards(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, is_test: bool) -> Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> {
    let mut view: Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> = Vec::new();

    let t1 = Entry {
        timestamp: 0i64,
        key: "ust_balance".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(1),
        group: Some("[Anchor Protocol][Auto Stake][UST]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(terra_balance_to_string(maybes.clone(), "uusd", false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_balance".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        index: Some(2),
        group: Some("[Anchor Protocol][Auto Stake][Gov]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(borrower_anc_deposited_to_string(maybes.clone(), false, 4));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_staked".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(3),
        group: Some("[Anchor Protocol][Auto Stake][Gov]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anc_staked_balance_in_ust_to_string(maybes.clone(), 4));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_staked".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        index: Some(4),
        group: Some("[Anchor Protocol][Auto Stake][Gov]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(anc_staked_balance_to_string(maybes.clone(), 4));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_rewards".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(5),
        group: Some("[Anchor Protocol][Auto Stake][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(borrower_rewards_in_ust_to_string(maybes.clone(), 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_rewards_target".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(6),
        group: Some("[Anchor Protocol][Auto Stake][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(maybes.clone(), "staking", "loan_amount", "value_next", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_rewards_target_date".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(7),
        group: Some("[Anchor Protocol][Auto Stake][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(maybes.clone(), "staking", "loan_amount", "date_next", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_rewards_annual_return".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(8),
        group: Some("[Anchor Protocol][Auto Stake][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(maybes.clone(), "staking", "loan_amount", "annual_return_auto_staking", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_rewards_auto_staking_benefit".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("%".to_string()),
        index: Some(9),
        group: Some("[Anchor Protocol][Auto Stake][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(maybes.clone(), "staking", "loan_amount", "difference", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "claim_rewards_tx_gas_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(10),
        group: Some("[Anchor Protocol][Auto Stake][Claim Tx]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_claim_rewards", "avg_gas_used".to_owned(), false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "stake_rewards_tx_gas_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(11),
        group: Some("[Anchor Protocol][Auto Stake][Stake Tx]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_staking", "avg_gas_used".to_owned(), false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "auto_stake_tx_fee_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(12),
        group: Some("[Anchor Protocol][Auto Stake][Tx]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_tx_fee_claim_and_stake(maybes.clone(), 2));
    view.push((t1, t2));

    let mut field = "auto_stake_tx_result";
    if is_test {
        field = "auto_stake_tx_estimate";
    }

    let t1 = Entry {
        timestamp: 0i64,
        key: field.to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(13),
        group: Some("[Anchor Protocol][Auto Stake][Tx]".to_string()),
    };

    // display task here
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(await_function(maybes.clone(), "anchor_auto_stake".to_owned()));
    view.push((t1, t2));

    return view;
}
 