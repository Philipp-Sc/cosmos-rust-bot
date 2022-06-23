use terra_rust_bot_essentials::shared::Entry;
use crate::state::control::model::{Maybe};


use crate::view::*;
use crate::view::interface::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc;
use cosmos_rust_interface::services::blockchain::smart_contracts::objects::ResponseResult;
use tokio::sync::{Mutex};

pub async fn anchor_info(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>) -> Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> {
    let mut view: Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> = Vec::new();

    let t1 = Entry {
        timestamp: 0i64,
        key: "stablecoins_lent".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(1),
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(total_liabilities_to_string(maybes.clone(), 0));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "stablecoins_deposited".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(2),
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(a_terra_supply_to_string(maybes.clone(), 0));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "utilization_ratio".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(3),
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    };
    //\n  *The utilization ratio quantifies a stablecoin's borrow demand relative to the amount of deposited stablecoins.\n
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(utilization_ratio_to_string(maybes.clone(), 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "base_rate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(4),
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    };
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(base_rate_to_string(maybes.clone(), 10));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "interest_multiplier".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(5),
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    };
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(interest_multiplier_to_string(maybes.clone(), 10));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "borrow_rate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(6),
        group: Some("[Anchor Protocol Info][Expert]".to_string()),
    };
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(borrow_rate_to_string(maybes.clone(), "config,Anchor,Interest Model", "state,Anchor,Market", "epoch_state,Anchor,Market", 10));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "net_apr".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(7),
        group: Some("[Anchor Protocol Info][Borrow]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(net_apr_to_string(maybes.clone(), 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "borrow_apr".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(8),
        group: Some("[Anchor Protocol Info][Borrow]".to_string()),
    };
    // The borrow rate equation incentivizes markets to have sufficient liquidity at their equilibrium. An increase in borrow demand is met with higher borrow rates, incentivizing repayments, and restoring market liquidity.
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(borrow_apr_to_string(maybes.clone(), 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "distribution_apr".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(9),
        group: Some("[Anchor Protocol Info][Borrow]".to_string()),
    };
    // Borrower incentives: 400M (40%) tokens are linearly released to be used as borrower incentives over a period of 4 years. 
    // TODO: figure out the distribution apy calculation from the smart contracts.
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(distribution_apr_to_string(maybes.clone(), 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "fee_to_claim".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(10),
        group: Some("[Anchor Protocol Info][Borrow]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_claim_rewards", "fee_amount_adjusted".to_owned(), false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "anc_staking_apy".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(11),
        group: Some("[Anchor Protocol Info][Gov]".to_string()),
    };
    // Anchor periodically distributes portion of ANC tokens purchased from protocol fees are distributed to ANC stakers to incentivize governance participation and decrease circulating ANC supply
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(staking_apy_to_string(maybes.clone(), 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "fee_to_stake".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(12),
        group: Some("[Anchor Protocol Info][Gov]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_staking", "fee_amount_adjusted".to_owned(), false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "deposit_apy".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(13),
        group: Some("[Anchor Protocol Info][Earn]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(earn_apr_to_string(maybes.clone(), 2));
    view.push((t1, t2));

    return view;
}
 
