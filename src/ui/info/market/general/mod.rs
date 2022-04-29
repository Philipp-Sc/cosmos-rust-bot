use terra_rust_bot_essentials::shared::Entry;
use crate::state::control::model::{Maybe};


use crate::view::interface::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc;
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::ResponseResult;
use tokio::sync::{Mutex};

pub async fn display_market_info(maybes: &HashMap<String, Arc<Mutex<Maybe<ResponseResult>>>>) -> Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> {
    let mut view: Vec<(Entry, Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>>)> = Vec::new();

    let t1 = Entry {
        timestamp: 0i64,
        key: "est_blocks_per_year".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(1),
        group: Some("[Market][Terra]".to_string()),
    };
    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(blocks_per_year_to_string(maybes.clone(), 0));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "gas_price".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(2),
        group: Some("[Market][Terra]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(gas_price_to_string(maybes.clone(), 4));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "SDT -> Luna".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(3),
        group: Some("[Market][Terra]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(core_swap_amount_to_string(maybes.clone(), "core_swap usdr uluna", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "SDT -> Luna".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(4),
        group: Some("[Market][Terra]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(core_swap_amount_to_string(maybes.clone(), "core_swap uusd usdr", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "Luna -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(5),
        group: Some("[Market][Terra]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(core_swap_amount_to_string(maybes.clone(), "core_swap uluna uusd", 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "Luna -> bLuna".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(6),
        group: Some("[Market][Anchor]".to_string()),
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
        group: Some("[Market][Anchor][Bond]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(b_luna_exchange_rate_to_string(maybes.clone(), 4));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "ANC -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(8),
        group: Some("[Market][Anchor]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(simulation_swap_return_amount_to_string(maybes.clone(), "simulation_cw20 anchorprotocol ANC terraswapAncUstPair", false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "aUST -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(9),
        group: Some("[Market][Anchor]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(a_terra_exchange_rate_to_string(maybes.clone(), 4));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "nLuna -> PSI".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        index: Some(10),
        group: Some("[Market][Nexus]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(simulation_swap_return_amount_to_string(maybes.clone(), "simulation_cw20 nexusprotocol nLunaToken Psi-nLuna_Pair", false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "PSI -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(11),
        group: Some("[Market][Nexus]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(simulation_swap_return_amount_to_string(maybes.clone(), "simulation_cw20 nexusprotocol PsiToken Psi-UST_Pair", false, 4));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "MIR -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(12),
        group: Some("[Market][Mirror]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(simulation_swap_return_amount_to_string(maybes.clone(), "simulation_cw20 uusd mir", false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "mTSLA -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(13),
        group: Some("[Market][Mirror]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(simulation_swap_return_amount_to_string(maybes.clone(), "simulation_cw20 uusd m_tsla", false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "mSPY -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(14),
        group: Some("[Market][Mirror]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(simulation_swap_return_amount_to_string(maybes.clone(), "simulation_cw20 uusd m_spy", false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "mBTC -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(15),
        group: Some("[Market][Mirror]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(simulation_swap_return_amount_to_string(maybes.clone(), "simulation_cw20 uusd m_btc", false, 2));
    view.push((t1, t2));

    let t1 = Entry {
        timestamp: 0i64,
        key: "mETH -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        index: Some(16),
        group: Some("[Market][Mirror]".to_string()),
    };

    let t2: Pin<Box<dyn Future<Output=Maybe<String>> + Send + 'static>> = Box::pin(simulation_swap_return_amount_to_string(maybes.clone(), "simulation_cw20 uusd m_eth", false, 2));
    view.push((t1, t2));

    return view;
}
