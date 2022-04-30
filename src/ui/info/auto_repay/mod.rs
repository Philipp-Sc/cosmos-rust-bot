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
use tokio::sync::Mutex;


/**
 * Anchor Auto Repay requires that the account balance has sufficient funds.
 * Anchor Auto Repay tries to be net neutral regarding the UST account balance.
 * Info: In some edge cases the account balance still can fall bellow the limit.
 *
 * All the requirements specified by the tag "anchor_auto_repay" are mostly loaded concurrently.
 * Each requirement has it's own refresh time (fast=10s) 
 * There are no guarantees that they resolve in time (in 10s).
 * Anchor Auto Repay non blocking waits for unresolved requirements to be fulfilled.
 *
 * */

pub async fn lazy_anchor_account_auto_repay(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, is_test: bool) -> Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> {
    let mut view: Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> = Vec::new();

    let t1 = Entry {
        timestamp: 0i64,
        key: "loan_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(1),
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(borrower_loan_amount_to_string(maybes.clone(), false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "borrow_limit".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(2),
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(borrow_limit_to_string(maybes.clone(), false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "loan_to_borrow_limit".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(3),
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(borrower_ltv_to_string(maybes.clone(), 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "left_to_trigger".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(4),
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(check_anchor_loan_status(maybes.clone(), "repay", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "ideal_repay_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(5),
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(calculate_amount(maybes.clone(), "repay", false, 2));
    view.push((t1, t2));

    /*
        let available_to_repay = min_ust_balance_to_string(maybes.clone(),false,2).await;
        add_string_to_display(new_display,*offset,format!("{}{}","\n\n\n   [Auto Repay UST]                account limit:                   ".truecolor(75,219,75), format!("{} UST",available_to_repay).yellow())).await.ok();
        *offset += 1;

        let available_to_repay = calculate_repay_plan(maybes.clone(),"ust_available_to_repay",2).await;
        add_string_to_display(new_display,*offset,format!("{}{}","\n   [Auto Repay UST]                available UST:                   ".truecolor(75,219,75), format!("{} UST",available_to_repay).yellow())).await.ok();
        *offset += 1;
    */
    let t1 = Entry {
        timestamp: 0i64,
        key: "to_withdraw_from_account".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(6),
        group: Some("[Anchor Protocol][Auto Repay]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(calculate_repay_plan(maybes.clone(), "to_withdraw_from_account", 2));
    view.push((t1, t2));

    /*
        let available_in_deposit = calculate_repay_plan(maybes.clone(),"available_in_deposit",2).await;
        add_string_to_display(new_display,*offset,format!("{}{}","\n\n   [Auto Repay Redeem]         max amount:                      ".truecolor(75,219,75), format!("{} UST",available_in_deposit).yellow())).await.ok();
        *offset += 1;
    */

    let t1 = Entry {
        timestamp: 0i64,
        key: "redeem_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(7),
        group: Some("[Anchor Protocol][Auto Repay][Redeem]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(calculate_repay_plan(maybes.clone(), "to_withdraw_from_deposit", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "redeem_stable_tx_gas_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(8),
        group: Some("[Anchor Protocol][Auto Repay][Redeem]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_redeem_stable", "avg_gas_used".to_owned(), false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "to_repay".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(9),
        group: Some("[Anchor Protocol][Auto Repay][Repay]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(calculate_repay_plan(maybes.clone(), "to_repay", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "repay_stable_tx_gas_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(10),
        group: Some("[Anchor Protocol][Auto Repay][Repay]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_repay_stable", "avg_gas_used".to_owned(), false, 2));
    view.push((t1, t2));

    /*
    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   est. stability fee:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(calculate_repay_plan(maybes.clone(),"stability_tax",2));
    view.push((t1,t2));

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;*/

    let t1 = Entry {
        timestamp: 0i64,
        key: "total_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(11),
        group: Some("[Anchor Protocol][Auto Repay][Transaction]".to_string()),
    };


    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(calculate_repay_plan(maybes.clone(), "total_amount", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "auto_repay_tx_fee_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(12),
        group: Some("[Anchor Protocol][Auto Repay][Transaction]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_auto_repay_tx_fee(maybes.clone(), 2));
    view.push((t1, t2));

    let mut field = "auto_repay_tx_result";
    if is_test {
        field = "auto_repay_tx_estimate";
    }

    let t1 = Entry {
        timestamp: 0i64,
        key: field.to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(13),
        group: Some("[Anchor Protocol][Auto Repay][Transaction]".to_string()),
    };

    // display task here
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(await_function(maybes.clone(), "anchor_auto_repay".to_owned()));
    view.push((t1, t2));

    return view;
}