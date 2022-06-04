/*
 * terra-rust-api interface
 * 
 */

pub mod cosmos_rpc;
pub mod data;

use data::endpoints::{get_terra_fcd, get_terra_lcd, get_terra_chain};
use data::{GasPrices};
use serde_json::Value;
use terra_rust_api::{Terra, GasOptions};
use terra_rust_api::core_types::{Coin};
use terra_rust_api::client::tendermint_types::BlockResult;
use terra_rust_api::client::tx_types::V1TXSResult;
use terra_rust_api::{PrivateKey};
use terra_rust_api::messages::Message;
use terra_rust_api::LCDResult;
use terra_rust_api::client::tx_types::TxFeeResult;
use secp256k1::Secp256k1;
use anyhow::anyhow;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use core::str::FromStr;

pub fn get_from_account(mnemonics: &str) -> anyhow::Result<String> {
    let secp = Secp256k1::new();
    let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;
    let from_public_key = from_key.public_key(&secp);
    let from_account = from_public_key.account()?;
    Ok(from_account)
}

pub async fn execute_messages(mnemonics: &str, messages: Vec<Message>, gas_opts: GasOptions) -> anyhow::Result<String> {
    let secp = Secp256k1::new();
    let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;

    let endpoint: &str = &get_terra_lcd();
    let chain: &str = &get_terra_chain();

    let terra = Terra::lcd_client(endpoint, chain, &gas_opts, None);

    let (std_sign_msg, sigs) = terra
        .generate_transaction_to_broadcast(
            &secp,
            &from_key,
            messages,
            None,
        )
        .await?;

    // send it out
    let resp = terra.tx().broadcast_sync(&std_sign_msg, &sigs).await?;

    match resp.code {
        Some(code) => {
            let t1 = format!("{}", serde_json::to_string(&resp)?);
            let t2 = format!("Transaction returned a {} {}", code, resp.txhash);
            return Err(anyhow!("Unexpected Fee Denom: {:?}",format!("{}\n\n{}",t1,t2)));
        }
        None => {
            return Ok(format!("tx hash: {}", resp.txhash));
        }
    }
}

pub fn estimate_to_gas_opts(res: LCDResult<TxFeeResult>, only_estimate: bool, max_tx_fee: Decimal) -> anyhow::Result<GasOptions> {
    let fees: Vec<Coin> = res.result.fee.amount;

    if fees.len() != 1 {
        return Err(anyhow!("Unexpected Fee Estimate. fees.len() = {:?}",fees.len()));
    }

    let micro = Decimal::from_str("1000000").unwrap();

    let tx_fee = Decimal::from_str(fees[0].amount.to_string().as_str())?
        .checked_div(micro).unwrap()
        .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero);

    let gas_limit = res.result.fee.gas;

    if only_estimate {
        return Err(anyhow!(format!("{} UST (gas limit: {})",tx_fee,gas_limit)));
    }

    if tx_fee > max_tx_fee {
        return Err(anyhow!("Unexpected High Fee: {:?} (max_tx_fee: {})",fees,max_tx_fee));
    }
    if fees[0].denom != "uusd" {
        return Err(anyhow!("Unexpected Fee Denom: {:?}",fees));
    }

    Ok(GasOptions {
        fees: Some(Coin::create(&fees[0].denom, fees[0].amount)),
        estimate_gas: false,
        gas: Some(res.result.fee.gas),
        gas_price: None,
        gas_adjustment: None,
    })
}

pub async fn estimate_messages(wallet_acc_address: &str, messages: Vec<Message>, gas_price_uusd: Decimal, gas_adjustment: Decimal) -> anyhow::Result<LCDResult<TxFeeResult>> {
    let endpoint: &str = &get_terra_lcd();
    let chain: &str = &get_terra_chain();
    let terra = Terra::lcd_client_no_tx(endpoint, chain);
    let gas_coin = Coin::create("uusd", gas_price_uusd);
    Ok(terra.tx()
        .estimate_fee(
            wallet_acc_address,
            &messages,
            gas_adjustment.to_f64().unwrap(),
            &[&gas_coin],
        ).await?)
}

pub async fn get_fcd_or_lcd_query(contract_addr: &str, query_msg: &str) -> anyhow::Result<String> {
    /*
     * Returns the response from which ever server is faster.
     * If a request fails, the alternative server is tried once.
     */
    tokio::select! {
        v1 = get_fcd_query(contract_addr,query_msg) => {return v1},
        v1 = get_lcd_query(contract_addr,query_msg) => {return v1},
    }
    ;
    //result = get_lcd_query(query_identifier, gas_prices).await;
    //Err(anyhow!("Unexpected Error: Null"))
}

pub async fn get_fcd_else_lcd_query(contract_addr: &str, query_msg: &str) -> anyhow::Result<String> {
    let mut v1 = get_fcd_query(contract_addr, query_msg).await;
    if let Err(_) = v1 {
        v1 = get_lcd_query(contract_addr, query_msg).await;
    }
    v1
}

pub async fn get_lcd_else_fcd_query(contract_addr: &str, query_msg: &str) -> anyhow::Result<String> {
    let mut v1 = get_lcd_query(contract_addr, query_msg).await;
    if let Err(_) = v1 {
        v1 = get_fcd_query(contract_addr, query_msg).await;
    }
    v1
}

pub async fn query_api(api: &str) -> anyhow::Result<String> {
    // https://docs.terraswap.io/docs/howto/query/
    let response = reqwest::get(api).await?.text().await?;
    Ok(response)
}

pub async fn query_api_with_post(api: &str, body: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let res = client.post(api)
        .body(body.to_owned())
        .send()
        .await?
        .text().await?;
    Ok(res)
}

pub async fn get_fcd_query(contract_addr: &str, query_msg: &str) -> anyhow::Result<String> {
    // https://docs.terraswap.io/docs/howto/query/
    let response = reqwest::get(
        format!("{}/wasm/contracts/{}/store?query_msg={}",
                get_terra_fcd(),
                contract_addr, // contract_addr
                query_msg  // query
        ).as_str()).await;

    match response {
        Ok(e) => {
            match e.text().await {
                Ok(res) => {
                    return Ok(res);
                }
                Err(err) => {
                    return Err(anyhow!("Unexpected reqwest::Error: {:?}",err));
                }
            }
        }
        Err(e) => {
            return Err(anyhow!("Unexpected reqwest::Error: {:?}",e));
        }
    }
}

pub async fn get_lcd_query(contract_addr: &str, query_msg: &str) -> anyhow::Result<String> {
    let endpoint: &str = &get_terra_lcd();
    let chain: &str = &get_terra_chain();
    let terra = Terra::lcd_client_no_tx(endpoint, chain);


    let code_result = terra.send_cmd::<Value>(
        &format!("/wasm/contracts/{}/store?", &contract_addr),
        Some(&format!("query_msg={}", query_msg)),
    ).await;

    match code_result {
        Ok(res) => {
            return Ok(res.to_string());
        }
        Err(err) => {
            return Err(anyhow!("Unexpected Error: {:?}",err));
        }
    }
}

pub async fn query_core_block_txs(height: u64, offset: Option<u64>, limit: Option<u64>) -> anyhow::Result<V1TXSResult> {
    let endpoint: &str = &get_terra_lcd();
    let chain: &str = &get_terra_chain();
    let terra = Terra::lcd_client_no_tx(endpoint, chain);
    let sw = terra.tx().get_txs_in_block(height, offset, limit).await?;
    Ok(sw)
}

pub async fn query_core_latest_block() -> anyhow::Result<BlockResult> {
    let endpoint: &str = &get_terra_lcd();
    let chain: &str = &get_terra_chain();
    let terra = Terra::lcd_client_no_tx(endpoint, chain);
    let sw = terra.tendermint().blocks().await?;
    Ok(sw)
}

pub async fn query_core_block_at_height(height: u64) -> anyhow::Result<BlockResult> {
    let endpoint: &str = &get_terra_lcd();
    let chain: &str = &get_terra_chain();
    let terra = Terra::lcd_client_no_tx(endpoint, chain);
    let sw = terra.tendermint().blocks_at_height(height).await?;
    Ok(sw)
}

pub async fn query_core_market_swap_rate(from: &str, to: &str) -> anyhow::Result<String> {
    let endpoint: &str = &get_terra_lcd();
    let chain: &str = &get_terra_chain();
    let terra = Terra::lcd_client_no_tx(endpoint, chain);
    let coin: Coin = Coin::parse(format!("{}{}", "1000000", from).as_str())?.unwrap();
    let ask = to;

    let sw = terra.market().swap(&coin, &ask).await?; // anyhow::Result<LCDResult<Coin>>
    // {\n  \"height\": \"5470599\",\n  \"result\": {\n    \"amount\": \"1388867\",\n    \"denom\": \"uusd\"\n  }\n}
    //Ok(serde_json::from_str(&serde_json::to_string_pretty(&sw)?)?)

    Ok(serde_json::to_string_pretty(&sw)?)
}

pub async fn query_core_bank_balances(account_address: &str) -> anyhow::Result<String> {
    let endpoint: &str = &get_terra_lcd();
    let chain: &str = &get_terra_chain();
    let terra = Terra::lcd_client_no_tx(endpoint, chain);
    let sw = terra.bank().balances(&account_address).await?; // anyhow::Result<LCDResultVec<Coin>>

    Ok(serde_json::to_string_pretty(&sw)?)
}

pub async fn fetch_gas_price() -> anyhow::Result<GasPrices> {
    let gas_prices = reqwest::get(format!("{}/v1/txs/gas_prices", get_terra_fcd())).await?;
    let result = gas_prices.text().await?;
    let json = serde_json::from_str::<GasPrices>(result.as_str())?;
    return Ok(json);
}

pub fn get_gas_price() -> GasPrices {
    serde_json::from_str::<GasPrices>(r#"{"uluna":"0.01133","usdr":"0.104938","uusd":"0.15","ukrw":"169.77","umnt":"428.571","ueur":"0.125","ucny":"0.98","ujpy":"16.37","ugbp":"0.11","uinr":"10.88","ucad":"0.19","uchf":"0.14","uaud":"0.19","usgd":"0.2","uthb":"4.62","usek":"1.25","unok":"1.25","udkk":"0.9","uidr":"2180.0","uphp":"7.6","uhkd":"1.17"}"#).unwrap()
}