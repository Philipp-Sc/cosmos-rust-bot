
use terra_rust_bot_output::output::*;


use terra_rust_bot_output::output::pretty::Entry; 
use crate::state::control::model::{Maybe};

use crate::state::control::model::{MaybeOrPromise};
  
use crate::view::interface::*; 

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;    
use chrono::Utc;

pub async fn display_market_info(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, state: &Arc<RwLock<Vec<Option<Entry>>>> ,offset: &mut usize, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> {

    let mut market_view: Vec<(Entry,usize)> = Vec::new();

    let mut market_tasks: Vec<(usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>)> = vec![];

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "est_blocks_per_year".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Market][Terra]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(blocks_per_year_to_string(tasks.clone(),0)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "gas_price".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Market][Terra]".to_string()),
    },*offset)); 

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(gas_price_to_string(tasks.clone(),4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "SDT -> Luna".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Market][Terra]".to_string()),
    },*offset));
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(core_swap_amount_to_string(tasks.clone(),"core_swap usdr uluna",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "SDT -> Luna".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Market][Terra]".to_string()),
    },*offset));

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(core_swap_amount_to_string(tasks.clone(), "core_swap uusd usdr",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "Luna -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Market][Terra]".to_string()),
    },*offset));

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(core_swap_amount_to_string(tasks.clone(), "core_swap uluna uusd",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "Luna -> bLuna".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Market][Anchor]".to_string()),
    },*offset));
  
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation anchorprotocol uluna terraswapblunaLunaPair",false,4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "Luna -> bLuna".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Market][Anchor][Bond]".to_string()),
    },*offset));

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(b_luna_exchange_rate_to_string(tasks.clone(),4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "ANC -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Market][Anchor]".to_string()),
    },*offset));
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "aUST -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Market][Anchor]".to_string()),
    },*offset));
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(a_terra_exchange_rate_to_string(tasks.clone(),4)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "nLuna -> PSI".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: None,
        group: Some("[Market][Nexus]".to_string()),
    },*offset));

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(), "simulation_cw20 nexusprotocol nLunaToken Psi-nLuna_Pair",false,2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "PSI -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Market][Nexus]".to_string()),
    },*offset));
      
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(), "simulation_cw20 nexusprotocol PsiToken Psi-UST_Pair",false,4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "MIR -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Market][Mirror]".to_string()),
    },*offset));
 
    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd mir",false,2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "mTSLA -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Market][Mirror]".to_string()),
    },*offset));

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_tsla",false,2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "mSPY -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Market][Mirror]".to_string()),
    },*offset));

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_spy",false,2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((Entry {
        timestamp: 0i64, 
        key: "mBTC -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Market][Mirror]".to_string()),
    },*offset));

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_btc",false,2)));
    market_tasks.push(t);
    *offset += 1;
     
    market_view.push((Entry {
        timestamp: 0i64, 
        key: "mETH -> UST".to_string(),
        prefix: None,
        value: "--".to_string(),
        suffix: Some("UST".to_string()),
        group: Some("[Market][Mirror]".to_string()),
    },*offset));

    let t: (usize,Pin<Box<dyn Future<Output = Maybe<String>> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_eth",false,2)));
    market_tasks.push(t);
    *offset += 1; 
 

    if is_first_run {
        add_view_to_state(&state, market_view).await;
    }

    return market_tasks;

}
