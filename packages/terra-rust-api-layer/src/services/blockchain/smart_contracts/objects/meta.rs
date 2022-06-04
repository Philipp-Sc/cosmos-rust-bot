/*
 * terra-rust-api utilitiy functions to estimate/execute transactions.
 *
 * https://docs.rs/cw20/0.8.0/cw20/enum.Cw20ExecuteMsg.html
 * https://github.com/Anchor-Protocol
 * https://github.com/astroport-fi/astroport-core/
 * https://github.com/spectrumprotocol/contracts/
 */

pub mod api;

use api::{execute_messages, estimate_messages, estimate_to_gas_opts};


use terra_rust_api::core_types::{Coin};
use terra_rust_api::{PrivateKey};
use terra_rust_api::messages::wasm::MsgExecuteContract;
use terra_rust_api::messages::Message;

use secp256k1::Secp256k1;
use rust_decimal::Decimal;
use core::str::FromStr;
use anyhow::anyhow;


use rust_decimal::prelude::ToPrimitive;
use cosmwasm_bignumber::{Uint256};
use cosmwasm_std_deprecated::{to_binary, Uint128};

//use cosmwasm_std_deprecated::Binary;
// Binary::from_base64(&base64::encode(msg))?

use cw20::Cw20ExecuteMsg;
use moneymarket::market::ExecuteMsg;
use terraswap::asset::{Asset, AssetInfo};

use anchor_token::airdrop::ExecuteMsg as AirdropExecuteMsg;
use api::data::terra_contracts::{contracts, tokens, custom};

use api::data::terra_contracts::AssetWhitelist;
use std::sync::Arc;

//use anchor_token::airdrop::{ExecuteMsg};

/*
fn anchor_liquidation_queue_withdraw_luna_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
        let contract = get_contract("anchorprotocol","mmMarket"); 

        let execute_msg_json = r##"{"claim_liquidations": {
                                        "collateral_token": "terra1kc87mu460fwkqte29rquh4hc20m54fxwtsx7gp",
                                        "bids_idx": luna_bid_idx
                                    }"##;
        let coins: [Coin;1] = [Coin::create("uusd", coin_amount)];
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, execute_msg_json, &coins)?;
        return Ok(send);
}*/

fn anchor_claim_airdrop_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str, proof: &str, stage: u64, amount: &str) -> anyhow::Result<Message> {
    let proof_as_vector: Vec<String> = serde_json::from_str(proof)?;

    let contract = contracts(asset_whitelist, "Anchor", "Airdrop").ok_or(anyhow!("no contract_addr"))?;

    let execute_msg = AirdropExecuteMsg::Claim {
        amount: Uint128::from_str(amount)?,
        stage: stage as u8,
        proof: proof_as_vector,
    };
    let execute_msg_json = serde_json::to_string(&execute_msg)?;
    let coins: [Coin; 0] = []; // no coins needed
    let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, &execute_msg_json, &coins)?;
    return Ok(send);
}


fn anchor_repay_stable_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
    let contract = contracts(asset_whitelist, "Anchor", "Market").ok_or(anyhow!("no contract_addr"))?;

    let execute_msg = ExecuteMsg::RepayStable {};
    let execute_msg_json = serde_json::to_string(&execute_msg)?;
    let coins: [Coin; 1] = [Coin::create("uusd", coin_amount)];
    let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, &execute_msg_json, &coins)?;
    return Ok(send);
}

fn anchor_deposit_stable_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
    let contract = contracts(asset_whitelist, "Anchor", "Market").ok_or(anyhow!("no contract_addr"))?;
    let execute_msg = ExecuteMsg::DepositStable {};
    let execute_msg_json = serde_json::to_string(&execute_msg)?;
    let coins: [Coin; 1] = [Coin::create("uusd", coin_amount)];
    let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, &execute_msg_json, &coins)?;
    return Ok(send);
}

fn anchor_borrow_stable_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
    let contract = contracts(asset_whitelist, "Anchor", "Market").ok_or(anyhow!("no contract_addr"))?;
    let execute_msg = ExecuteMsg::BorrowStable {
        borrow_amount: Uint256::from(coin_amount.to_u128().ok_or(anyhow!("incorrect coin_amount format"))?),
        to: None,
    };
    let execute_msg_json = serde_json::to_string(&execute_msg)?;
    let coins: [Coin; 0] = []; // no coins needed
    let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, &execute_msg_json, &coins)?;
    return Ok(send);
}

fn anchor_redeem_stable_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
    let contract_addr_a_ust = tokens(asset_whitelist, "Anchor", "aUST").ok_or(anyhow!("no contract_addr_a_ust"))?;
    let contract_addr_mm_market = contracts(asset_whitelist, "Anchor", "Market").ok_or(anyhow!("no contract_addr_mm_market"))?;

    let coins: [Coin; 0] = []; // no coins needed

    let execute_msg = Cw20ExecuteMsg::Send {
        contract: contract_addr_mm_market,
        amount: Uint128::from(coin_amount.to_u128().ok_or(anyhow!("incorrect coin_amount format"))?),
        msg: to_binary(&moneymarket::market::Cw20HookMsg::RedeemStable {}).unwrap(),
    };
    /* JSON: "{"redeem_stable":{}}"
       * Base64-encoded JSON: "eyJyZWRlZW1fc3RhYmxlIjp7fX0="
       */
    let execute_msg_json = serde_json::to_string(&execute_msg)?;
    let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_a_ust, &execute_msg_json, &coins)?;
    return Ok(send);
}

fn anchor_governance_claim_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str) -> anyhow::Result<Message> {
    let contract_addr_mm_market = contracts(asset_whitelist, "Anchor", "Market").ok_or(anyhow!("no contract_addr_mm_market"))?;

    let execute_msg = ExecuteMsg::ClaimRewards { to: None };
    let execute_msg_json = serde_json::to_string(&execute_msg)?;

    let coins: [Coin; 0] = []; // no coins needed
    let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_mm_market, &execute_msg_json, &coins)?;
    return Ok(send);
}

fn anchor_governance_stake_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
    let contract_addr_anc = tokens(asset_whitelist, "Anchor", "ANC").ok_or(anyhow!("no contract_addr_anc"))?;
    let contract_addr_gov = contracts(asset_whitelist, "Anchor", "Governance").ok_or(anyhow!("no contract_addr_gov"))?;

    let coins: [Coin; 0] = []; // no coins needed

    let execute_msg = Cw20ExecuteMsg::Send {
        contract: contract_addr_gov,
        amount: Uint128::from(coin_amount.to_u128().ok_or(anyhow!("incorrect coin_amount format"))?),
        msg: to_binary(&anchor_token::gov::Cw20HookMsg::StakeVotingTokens {}).unwrap(),
    };
    /* JSON: "{"stake_voting_tokens":{}}"
     * Base64-encoded JSON: "eyJzdGFrZV92b3RpbmdfdG9rZW5zIjp7fX0="
     */

    let execute_msg_json = serde_json::to_string(&execute_msg)?;
    let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_anc, &execute_msg_json, &coins)?;
    return Ok(send);
}

fn astroport_swap_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str, coin_amount: Decimal, max_spread: Decimal, belief_price: Decimal) -> anyhow::Result<Message> {
    let contract_addr_anc = tokens(asset_whitelist, "Anchor", "ANC").ok_or(anyhow!("no contract_addr_anc"))?;
    let contract_addr_lp = custom(asset_whitelist, "Anchor", "ANC-UST LP Minter").ok_or(anyhow!("no contract_addr_lp"))?;
    let coins: [Coin; 0] = []; // no coins needed

    let msg = astroport::pair::Cw20HookMsg::Swap {
        belief_price: Some(cosmwasm_std_deprecated::Decimal::from_str(belief_price.round_dp_with_strategy(18, rust_decimal::RoundingStrategy::ToZero).to_string().as_str())?),
        max_spread: Some(cosmwasm_std_deprecated::Decimal::from_str(max_spread.to_string().as_str())?),
        to: None,
    };

    let execute_msg = Cw20ExecuteMsg::Send {
        contract: contract_addr_lp,
        amount: Uint128::from(coin_amount.to_u128().ok_or(anyhow!("incorrect coin_amount format"))?),
        msg: to_binary(&msg).unwrap(),
    };
    let execute_msg_json = serde_json::to_string(&execute_msg)?;

    let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_anc, &execute_msg_json, &coins)?;
    return Ok(send);
}

fn anchor_increase_allowance_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
    let contract_addr_anc = tokens(asset_whitelist, "Anchor", "ANC").ok_or(anyhow!("no contract_addr_anc"))?;
    let contract_addr_lp_1 = custom(asset_whitelist, "Spectrum", "SPEC ANC-UST VAULT").ok_or(anyhow!("no contract_addr_lp_1"))?;
    let coins: [Coin; 0] = []; // no coins needed

    let execute_msg = Cw20ExecuteMsg::IncreaseAllowance {
        spender: contract_addr_lp_1,
        amount: Uint128::from(coin_amount.to_u128().ok_or(anyhow!("incorrect coin_amount format"))?),
        expires: None,
    };

    let execute_msg_json = serde_json::to_string(&execute_msg)?;
    let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_anc, &execute_msg_json, &coins)?;
    return Ok(send);
}

fn anchor_provide_to_spec_vault_msg(asset_whitelist: &Arc<AssetWhitelist>, wallet_acc_address: &str, anc_to_keep: Decimal, ust_to_keep: Decimal) -> anyhow::Result<Message> {
    let contract_addr_anc = tokens(asset_whitelist, "Anchor", "ANC").ok_or(anyhow!("no contract_addr_anc"))?;
    let contract_addr_lp_1 = custom(asset_whitelist, "Spectrum", "SPEC ANC-UST VAULT").ok_or(anyhow!("no contract_addr_lp_1"))?;
    let contract_addr_lp_2 = contracts(asset_whitelist, "Spectrum", "Spec Astroport ANC-UST Farm").ok_or(anyhow!("no contract_addr_lp_2"))?;

    let coins: [Coin; 1] = [Coin::create("uusd", ust_to_keep)];


    let execute_msg = ExecuteMsg::ClaimRewards { to: None };
    // TODO: latest spectrum_protocol version does not compile
    /*spectrum_protocol::staker::ExecuteMsg::bond {
        contract: contract_addr_lp_2,
        assets: [
            Asset {
                info: AssetInfo::Token {
                    contract_addr: contract_addr_anc,
                },
                amount: cosmwasm_std_deprecated::Uint128::from(anc_to_keep.to_u128().ok_or(anyhow!("incorrect ust_to_keep format"))?),
            },
            Asset {
                info: AssetInfo::NativeToken {
                    denom: "uusd".to_string(),
                },
                amount: cosmwasm_std_deprecated::Uint128::from(ust_to_keep.to_u128().ok_or(anyhow!("incorrect ust_to_keep format"))?),
            },
        ],
        slippage_tolerance: cosmwasm_std_deprecated::Decimal::from_str("0.01").unwrap(),
        compound_rate: Some(cosmwasm_std_deprecated::Decimal::percent(100u64)),
        staker_addr: None,
        asset_token: None,
    };*/

    let execute_msg_json = serde_json::to_string(&execute_msg)?;
    let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_lp_1, &execute_msg_json, &coins)?;
    return Ok(send);
}

pub async fn anchor_claim_and_stake_airdrop_tx(asset_whitelist: &Arc<AssetWhitelist>, mnemonics: &str, proof: &Vec<String>, stage: &Vec<u64>, amount: &Vec<String>, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String> {
    let from_account = match mnemonics.len() {
        44 => {
            // wallet_acc_address
            mnemonics.to_string()
        }
        _ => {
            // seed phrase
            let secp = Secp256k1::new();
            let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;
            let from_public_key = from_key.public_key(&secp);
            from_public_key.account()?
        }
    };

    let mut messages = Vec::new();
    let mut sum_anc: u64 = 0;
    for i in 0..stage.len() {
        messages.push(anchor_claim_airdrop_msg(asset_whitelist, &from_account, &proof[i], stage[i], &amount[i])?);
        sum_anc += amount[i].parse::<u64>().unwrap_or(0u64);
    }
    let send_stake = anchor_governance_stake_msg(asset_whitelist, &from_account, Decimal::from_str(sum_anc.to_string().as_str())?)?;
    messages.push(send_stake);

    //println!("{}",serde_json::to_string(&messages)?);

    let res = estimate_messages(&from_account, messages, gas_price_uusd, gas_adjustment).await?;

    let gas_opts = match estimate_to_gas_opts(res, only_estimate, max_tx_fee) {
        Err(err) => {
            return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
        }
        Ok(e) => { e }
    };

    let mut messages = Vec::new();
    let mut sum_anc: u64 = 0;
    for i in 0..stage.len() {
        messages.push(anchor_claim_airdrop_msg(asset_whitelist, &from_account, &proof[i], stage[i], &amount[i])?);
        sum_anc += amount[i].parse::<u64>().unwrap_or(0u64);
    }
    let send_stake = anchor_governance_stake_msg(asset_whitelist, &from_account, Decimal::from_str(sum_anc.to_string().as_str())?)?;
    messages.push(send_stake);

    execute_messages(mnemonics, messages, gas_opts).await
}

pub async fn anchor_borrow_and_deposit_stable_tx(asset_whitelist: &Arc<AssetWhitelist>, mnemonics: &str, coin_amount_borrow: Decimal, coin_amount_deposit: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String> {
    let from_account = match mnemonics.len() {
        44 => {
            // wallet_acc_address
            mnemonics.to_string()
        }
        _ => {
            // seed phrase
            let secp = Secp256k1::new();
            let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;
            let from_public_key = from_key.public_key(&secp);
            from_public_key.account()?
        }
    };

    let mut messages = Vec::new();
    messages.push(anchor_borrow_stable_msg(asset_whitelist, &from_account, coin_amount_borrow)?);
    messages.push(anchor_deposit_stable_msg(asset_whitelist, &from_account, coin_amount_deposit)?);

    let res = estimate_messages(&from_account, messages, gas_price_uusd, gas_adjustment).await?;

    let gas_opts = match estimate_to_gas_opts(res, only_estimate, max_tx_fee) {
        Err(err) => {
            return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
        }
        Ok(e) => { e }
    };

    let mut messages = Vec::new();
    messages.push(anchor_borrow_stable_msg(asset_whitelist, &from_account, coin_amount_borrow)?);
    messages.push(anchor_deposit_stable_msg(asset_whitelist, &from_account, coin_amount_deposit)?);

    execute_messages(mnemonics, messages, gas_opts).await
}

pub async fn anchor_redeem_and_repay_stable_tx(asset_whitelist: &Arc<AssetWhitelist>, mnemonics: &str, coin_amount_redeem: Decimal, coin_amount_repay: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String> {
    let from_account = match mnemonics.len() {
        44 => {
            // wallet_acc_address
            mnemonics.to_string()
        }
        _ => {
            // seed phrase
            let secp = Secp256k1::new();
            let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;
            let from_public_key = from_key.public_key(&secp);
            from_public_key.account()?
        }
    };

    let mut messages = Vec::new();
    messages.push(anchor_redeem_stable_msg(asset_whitelist, &from_account, coin_amount_redeem)?);
    messages.push(anchor_repay_stable_msg(asset_whitelist, &from_account, coin_amount_repay)?);

    let res = estimate_messages(&from_account, messages, gas_price_uusd, gas_adjustment).await?;

    let gas_opts = match estimate_to_gas_opts(res, only_estimate, max_tx_fee) {
        Err(err) => {
            return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
        }
        Ok(e) => { e }
    };

    let mut messages = Vec::new();
    messages.push(anchor_redeem_stable_msg(asset_whitelist, &from_account, coin_amount_redeem)?);
    messages.push(anchor_repay_stable_msg(asset_whitelist, &from_account, coin_amount_repay)?);

    execute_messages(mnemonics, messages, gas_opts).await
}

pub async fn anchor_redeem_stable_tx(asset_whitelist: &Arc<AssetWhitelist>, mnemonics: &str, coin_amount_redeem: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String> {
    let from_account = match mnemonics.len() {
        44 => {
            // wallet_acc_address
            mnemonics.to_string()
        }
        _ => {
            // seed phrase
            let secp = Secp256k1::new();
            let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;
            let from_public_key = from_key.public_key(&secp);
            from_public_key.account()?
        }
    };

    let messages: Vec<Message> = vec![anchor_redeem_stable_msg(asset_whitelist, &from_account, coin_amount_redeem)?];

    let res = estimate_messages(&from_account, messages, gas_price_uusd, gas_adjustment).await?;

    let gas_opts = match estimate_to_gas_opts(res, only_estimate, max_tx_fee) {
        Err(err) => {
            return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
        }
        Ok(e) => { e }
    };

    let messages: Vec<Message> = vec![anchor_redeem_stable_msg(asset_whitelist, &from_account, coin_amount_redeem)?];

    execute_messages(mnemonics, messages, gas_opts).await
}

pub async fn anchor_repay_stable_tx(asset_whitelist: &Arc<AssetWhitelist>, mnemonics: &str, coin_amount_repay: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String> {
    let from_account = match mnemonics.len() {
        44 => {
            // wallet_acc_address
            mnemonics.to_string()
        }
        _ => {
            // seed phrase
            let secp = Secp256k1::new();
            let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;
            let from_public_key = from_key.public_key(&secp);
            from_public_key.account()?
        }
    };

    let mut messages = Vec::new();
    messages.push(anchor_repay_stable_msg(asset_whitelist, &from_account, coin_amount_repay)?);

    let res = estimate_messages(&from_account, messages, gas_price_uusd, gas_adjustment).await?;

    let gas_opts = match estimate_to_gas_opts(res, only_estimate, max_tx_fee) {
        Err(err) => {
            return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
        }
        Ok(e) => { e }
    };

    let mut messages = Vec::new();
    messages.push(anchor_repay_stable_msg(asset_whitelist, &from_account, coin_amount_repay)?);

    execute_messages(mnemonics, messages, gas_opts).await
}

pub async fn anchor_governance_claim_and_provide_to_spec_vault(asset_whitelist: &Arc<AssetWhitelist>, mnemonics: &str, anc_to_keep: Decimal, ust_to_keep: Decimal, anc_to_swap: Decimal, belief_price: Decimal, max_spread: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String> {
    let from_account = match mnemonics.len() {
        44 => {
            // wallet_acc_address
            mnemonics.to_string()
        }
        _ => {
            // seed phrase
            let secp = Secp256k1::new();
            let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;
            let from_public_key = from_key.public_key(&secp);
            from_public_key.account()?
        }
    };

    let send_claim = anchor_governance_claim_msg(asset_whitelist, &from_account)?;
    let send_swap = astroport_swap_msg(asset_whitelist, &from_account, anc_to_swap, max_spread, belief_price)?;
    let send_increase_allowance = anchor_increase_allowance_msg(asset_whitelist, &from_account, anc_to_keep)?;
    let send_provide = anchor_provide_to_spec_vault_msg(asset_whitelist, &from_account, anc_to_keep, ust_to_keep)?;


    let messages: Vec<Message> = vec![send_claim, send_swap, send_increase_allowance, send_provide];

    let res = estimate_messages(&from_account, messages, gas_price_uusd, gas_adjustment).await?;

    //let estimate_json = serde_json::to_string(&res.result);
    //{"fee":{"amount":[{"amount":"90462","denom":"uusd"}],"gas":"603080"}}
    let gas_opts = match estimate_to_gas_opts(res, only_estimate, max_tx_fee) {
        Err(err) => {
            return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
        }
        Ok(e) => { e }
    };

    let send_claim = anchor_governance_claim_msg(asset_whitelist, &from_account)?;
    let send_swap = astroport_swap_msg(asset_whitelist, &from_account, anc_to_swap, max_spread, belief_price)?;
    let send_increase_allowance = anchor_increase_allowance_msg(asset_whitelist, &from_account, anc_to_keep)?;
    let send_provide = anchor_provide_to_spec_vault_msg(asset_whitelist, &from_account, anc_to_keep, ust_to_keep)?;

    let messages: Vec<Message> = vec![send_claim, send_swap, send_increase_allowance, send_provide];

    execute_messages(mnemonics, messages, gas_opts).await
}


pub async fn anchor_governance_claim_and_stake(asset_whitelist: &Arc<AssetWhitelist>, mnemonics: &str, coin_amount: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String> {
    let from_account = match mnemonics.len() {
        44 => {
            // wallet_acc_address
            mnemonics.to_string()
        }
        _ => {
            // seed phrase
            let secp = Secp256k1::new();
            let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;
            let from_public_key = from_key.public_key(&secp);
            from_public_key.account()?
        }
    };

    let send_claim = anchor_governance_claim_msg(asset_whitelist, &from_account)?;
    let send_stake = anchor_governance_stake_msg(asset_whitelist, &from_account, coin_amount)?;

    let messages: Vec<Message> = vec![send_claim, send_stake];

    let res = estimate_messages(&from_account, messages, gas_price_uusd, gas_adjustment).await?;

    //let estimate_json = serde_json::to_string(&res.result);
    //{"fee":{"amount":[{"amount":"90462","denom":"uusd"}],"gas":"603080"}}
    let gas_opts = match estimate_to_gas_opts(res, only_estimate, max_tx_fee) {
        Err(err) => {
            return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
        }
        Ok(e) => { e }
    };

    let send_claim = anchor_governance_claim_msg(asset_whitelist, &from_account)?;
    let send_stake = anchor_governance_stake_msg(asset_whitelist, &from_account, coin_amount)?;

    let messages: Vec<Message> = vec![send_claim, send_stake];

    execute_messages(mnemonics, messages, gas_opts).await
}


pub async fn anchor_governance_stake(asset_whitelist: &Arc<AssetWhitelist>, mnemonics: &str, coin_amount: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String> {
    let from_account = match mnemonics.len() {
        44 => {
            // wallet_acc_address
            mnemonics.to_string()
        }
        _ => {
            // seed phrase
            let secp = Secp256k1::new();
            let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;
            let from_public_key = from_key.public_key(&secp);
            from_public_key.account()?
        }
    };

    let send_stake = anchor_governance_stake_msg(asset_whitelist, &from_account, coin_amount)?;

    let messages: Vec<Message> = vec![send_stake];

    let res = estimate_messages(&from_account, messages, gas_price_uusd, gas_adjustment).await?;

    //let estimate_json = serde_json::to_string(&res.result);
    //{"fee":{"amount":[{"amount":"90462","denom":"uusd"}],"gas":"603080"}}

    let gas_opts = match estimate_to_gas_opts(res, only_estimate, max_tx_fee) {
        Err(err) => {
            return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
        }
        Ok(e) => { e }
    };

    let send_stake = anchor_governance_stake_msg(asset_whitelist, &from_account, coin_amount)?;

    let messages: Vec<Message> = vec![send_stake];

    execute_messages(mnemonics, messages, gas_opts).await
}

pub async fn anchor_claim_rewards(asset_whitelist: &Arc<AssetWhitelist>, mnemonics: &str, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String> {
    let from_account = match mnemonics.len() {
        44 => {
            // wallet_acc_address
            mnemonics.to_string()
        }
        _ => {
            // seed phrase
            let secp = Secp256k1::new();
            let from_key = PrivateKey::from_words(&secp, mnemonics, 0, 0)?;
            let from_public_key = from_key.public_key(&secp);
            from_public_key.account()?
        }
    };


    let send_claim = anchor_governance_claim_msg(asset_whitelist, &from_account)?;

    let messages: Vec<Message> = vec![send_claim];

    let res = estimate_messages(&from_account, messages, gas_price_uusd, gas_adjustment).await?;

    //let estimate_json = serde_json::to_string(&res.result);
    //{"fee":{"amount":[{"amount":"90462","denom":"uusd"}],"gas":"603080"}}

    let gas_opts = match estimate_to_gas_opts(res, only_estimate, max_tx_fee) {
        Err(err) => {
            return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
        }
        Ok(e) => { e }
    };

    let send_claim = anchor_governance_claim_msg(asset_whitelist, &from_account)?;

    let messages: Vec<Message> = vec![send_claim];

    execute_messages(mnemonics, messages, gas_opts).await
}