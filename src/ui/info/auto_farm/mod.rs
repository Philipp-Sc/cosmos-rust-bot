use terra_rust_bot_essentials::shared::Entry;
use crate::state::control::model::{Maybe};
use crate::state::control::model::{await_function};

use crate::view::*;
use crate::view::interface::*;

//use crate::action::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;
use core::future::*;

use std::sync::Arc;
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::ResponseResult;
use tokio::sync::{Mutex};

/**
 * Anchor Auto Farm requires that the account balance has sufficient funds.
 * Info: It will not replenish the account balance. 
 * */
pub async fn lazy_anchor_account_auto_farm_rewards(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, is_test: bool) -> Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> {
    let mut view: Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> = Vec::new();

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_rewards".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(1),
        group: Some("[Anchor Protocol][Auto Farm][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(borrower_rewards_in_ust_to_string(maybes.clone(), 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_rewards".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        index: Some(2),
        group: Some("[Anchor Protocol][Auto Farm][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(borrower_rewards_to_string(maybes.clone(), false, 2));
    view.push((t1, t2));


    let t1 = Entry {
        timestamp: 0i64,
        key: "claim_rewards_tx_fee_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(3),
        group: Some("[Anchor Protocol][Auto Farm][Claim Tx]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_tx_fee_claim(maybes.clone(), 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "spec_provide_rewards_tx_fee_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(4),
        group: Some("[Anchor Protocol][Auto Farm][SPEC Provide Tx]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_spec_tx_fee_provide(maybes.clone(), 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_to_keep".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        index: Some(5),
        group: Some("[Anchor Protocol][Auto Farm]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(calculate_farm_plan(maybes.clone(), "anc_to_keep", false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_to_swap".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("ANC".to_string()),
        index: Some(6),
        group: Some("[Anchor Protocol][Auto Farm]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(calculate_farm_plan(maybes.clone(), "anc_to_swap", false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "belief_price".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(8),
        group: Some("[Anchor Protocol][Auto Farm]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(simulation_swap_exchange_rate_to_string(maybes.clone(), "simulate_swap,terraswap,Anchor,ANC,none,uusd", false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "max_spread".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(9),
        group: Some("[Anchor Protocol][Auto Farm]".to_string()),
    };
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(ready(Maybe { data: Ok("0.001".to_string()), timestamp: 0i64 }));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_rewards_target".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(10),
        group: Some("[Anchor Protocol][Auto Farm][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(maybes.clone(), "farming", "loan_amount", "value_next", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_rewards_target_date".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(11),
        group: Some("[Anchor Protocol][Auto Farm][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(maybes.clone(), "farming", "loan_amount", "date_next", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "annual_return".to_string(),
        prefix: Some(">".to_string()),
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(12),
        group: Some("[Anchor Protocol][Auto Farm][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(maybes.clone(), "farming", "loan_amount", "annual_return_auto_staking", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_rewards_auto_staking_benefit".to_string(),
        prefix: Some(">".to_string()),
        value: "--".to_string(),
        suffix: Some("%".to_string()),
        index: Some(13),
        group: Some("[Anchor Protocol][Auto Farm][Rewards]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(maybes.clone(), "farming", "loan_amount", "difference", 2));
    view.push((t1, t2));

    let mut field = "auto_farm_tx_result";
    if is_test {
        field = "auto_farm_tx_estimate";
    }

    let t1 = Entry {
        timestamp: 0i64,
        key: field.to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(14),
        group: Some("[Anchor Protocol][Auto Farm][Tx]".to_string()),
    };

    // display task here
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(await_function(maybes.clone(), "anchor_auto_farm".to_owned()));
    view.push((t1, t2));

    return view;
}