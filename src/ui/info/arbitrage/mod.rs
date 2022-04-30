use terra_rust_bot_essentials::shared::Entry;
use crate::state::control::model::{Maybe};


use crate::view::interface::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc;
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::ResponseResult;
use tokio::sync::{Mutex};

pub async fn display_arbitrage_info(maybes: &HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>) -> Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> {
    let mut view: Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> = Vec::new();


    let t1 = Entry {
        timestamp: 0i64,
        key: "Luna -> bLuna".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(6),
        group: Some("[Arbitrage][Terraswap]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(simulation_swap_return_amount_to_string(maybes.clone(), "simulation anchorprotocol uluna terraswapblunaLunaPair", false, 4));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "Luna -> bLuna".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(7),
        group: Some("[Arbitrage][Anchor][Bond]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(b_luna_exchange_rate_to_string(maybes.clone(), 4));
    view.push((t1, t2));

    return view;
}
