
use terra_rust_bot_essentials::shared::Entry;
use crate::state::control::model::{Maybe};
//use crate::state::control::model::{MaybeOrPromise,try_register_function,await_function};

use crate::state::control::model::{await_function};

use crate::view::*;
use crate::view::interface::*;

//use crate::action::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc;
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::ResponseResult;
use tokio::sync::{Mutex};

 pub async fn lazy_anchor_account_auto_borrow(maybes: &HashMap<String, Arc<Mutex<Maybe<ResponseResult>>>>, is_test: bool) -> Vec<(Entry,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)>  {

     let mut view : Vec<(Entry,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> = Vec::new();

     let t1 = Entry {
        timestamp: 0i64, 
        key: "loan_amount".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Borrow]".to_string()),
    };
 
    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(borrower_loan_amount_to_string(maybes.clone(),false,2));
    view.push((t1,t2));

    let t1 = Entry {
        timestamp: 0i64, 
        key: "borrow_limit".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Borrow]".to_string()),
    };
 
    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(borrow_limit_to_string(maybes.clone(),false,2));
    view.push((t1,t2));

    let t1 = Entry {
        timestamp: 0i64, 
        key: "loan_to_borrow_limit".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Borrow]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(borrower_ltv_to_string(maybes.clone(),2));
    view.push((t1,t2));

    let t1 = Entry {
        timestamp: 0i64, 
        key: "left_to_trigger".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Borrow]".to_string()),
    };
 
    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(check_anchor_loan_status(maybes.clone(),"borrow",2));
    view.push((t1,t2));

    let t1 = Entry {
        timestamp: 0i64, 
        key: "to_borrow".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Borrow][Borrow]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(calculate_amount(maybes.clone(),"borrow",false,2));
    view.push((t1,t2));

    let t1 = Entry {
            timestamp: 0i64, 
            key: "borrow_stable_tx_gas_estimate".to_string(),
            prefix: None,
            value: "--".to_string(),
            suffix: None,
            group: Some("[Anchor Protocol][Auto Borrow][Borrow]".to_string()),
        };

    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_tx_fee(maybes.clone(),"anchor_protocol_txs_borrow_stable","avg_gas_used".to_owned(),false,2));
    view.push((t1,t2));
    /*
     // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   est. stability fee:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(calculate_borrow_plan(maybes.clone(),"stability_tax_borrow",2));
    view.push((t1,t2));

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    */

    let t1 = Entry {
            timestamp: 0i64, 
            key: "to_deposit".to_string(),
            prefix: None,
            value: "--".to_string(),
            suffix: Some("UST".to_string()),
            group: Some("[Anchor Protocol][Auto Borrow][Deposit]".to_string()),
        };
 
    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(calculate_borrow_plan(maybes.clone(),"to_deposit",2));
    view.push((t1,t2));

    let t1 = Entry {
        timestamp: 0i64, 
        key: "deposit_stable_tx_gas_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Borrow][Deposit]".to_string()),
    };
  
    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_tx_fee(maybes.clone(),"anchor_protocol_txs_deposit_stable","avg_gas_used".to_owned(),false,2));
    view.push((t1,t2));

    /*
    // min(to_repay * tax_rate , tax_cap)
    anchor_view.push((format!("{}{}","\n".truecolor(75,219,75),"   est. stability fee:\n\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}\u{2007}".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(calculate_borrow_plan(maybes.clone(),"stability_tax_deposit",2));
    view.push((t1,t2));


    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
    */

    let t1 = Entry {
        timestamp: 0i64, 
        key: "auto_borrow_tx_fee_estimate".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Anchor Protocol][Auto Borrow][Transaction]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(estimate_anchor_protocol_auto_borrow_tx_fee(maybes.clone(),2));
    view.push((t1,t2));
 
    let mut field = "auto_borrow_tx_result";
    if is_test {
        field = "auto_borrow_tx_estimate";
    }
 
    let t1 = Entry {
        timestamp: 0i64, 
        key: field.to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Anchor Protocol][Auto Borrow][Transaction]".to_string()),
    };
         
    // display task here
    let t2: Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>> = Box::pin(await_function(maybes.clone(),"anchor_auto_borrow".to_owned()));
    view.push((t1,t2));

    return view;
 
}
 