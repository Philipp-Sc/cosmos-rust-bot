/*
 * terra-rust-api utilitiy functions to estimate/execute transactions.
 *
 */

pub mod api;

use api::{execute_messages,estimate_messages,estimate_to_gas_opts};

use api::data::terra_contracts::get_contract;
 
use terra_rust_api::core_types::{Coin};  
use terra_rust_api::{PrivateKey};
use terra_rust_api::messages::wasm::MsgExecuteContract;
use terra_rust_api::messages::Message;

use secp256k1::Secp256k1;
use rust_decimal::Decimal; 
use anyhow::anyhow;


/*
fn anchor_liquidation_queue_withdraw_luna_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
        let contract = get_contract("anchorprotocol","mmMarket"); 

        let execute_msg_json = r##"{"claim_liquidations": {
                                        "collateral_token": "terra1kc87mu460fwkqte29rquh4hc20m54fxwtsx7gp",
                                        "bids_idx": luna_bid_idx
                                    }"##;
        let coins: [Coin;1] = [Coin::create("uusd", coin_amount)];
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, execute_msg_json, &coins)?;
        return Ok(send);
}*/


fn anchor_repay_stable_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
		let contract = get_contract("anchorprotocol","mmMarket"); 
        let execute_msg_json = r##"{"repay_stable":{}}"##;
        let coins: [Coin;1] = [Coin::create("uusd", coin_amount)];
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, execute_msg_json, &coins)?;
        return Ok(send);
}

fn anchor_deposit_stable_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
        let contract = get_contract("anchorprotocol","mmMarket"); 
        let execute_msg_json = r##"{"deposit_stable":{}}"##;
        let coins: [Coin;1] = [Coin::create("uusd", coin_amount)];
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, execute_msg_json, &coins)?;
        return Ok(send);
}

fn anchor_borrow_stable_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
        let contract = get_contract("anchorprotocol","mmMarket"); 
        let execute_msg_json = format!("{}{}{}", r##"{
                                        "borrow_stable": {
                                            "borrow_amount": ""##,coin_amount.to_string().as_str(),r##""
                                        }
                                    }"##);
        let coins: [Coin;0] = []; // no coins needed
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, &execute_msg_json, &coins)?;
        return Ok(send);
}

fn anchor_redeem_stable_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
		let contract_addr_a_ust = get_contract("anchorprotocol","aTerra"); 
		let contract_addr_mm_market = get_contract("anchorprotocol","mmMarket"); 
        let coins: [Coin;0] = []; // no coins needed
        /* JSON: "{"redeem_stable":{}}"
  		 * Base64-encoded JSON: "eyJyZWRlZW1fc3RhYmxlIjp7fX0="
  		 */
        let execute_msg_json = format!("{}{}{}{}{}",r##"{
									  "send": {
									    "msg": "eyJyZWRlZW1fc3RhYmxlIjp7fX0=",
									    "amount": ""##,coin_amount.to_string().as_str(),r##"",
									    "contract": ""##,contract_addr_mm_market,r##""
									  }
									}"##);
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_a_ust, &execute_msg_json, &coins)?;
        return Ok(send);
}

fn anchor_governance_claim_msg(wallet_acc_address: &str) -> anyhow::Result<Message> {
		let contract_addr_mm_market = get_contract("anchorprotocol","mmMarket"); 
        let execute_msg_json = r##"{"claim_rewards":{}}"##;
        let coins: [Coin;0] = []; // no coins needed
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_mm_market, &execute_msg_json, &coins)?;
        return Ok(send);
}

fn anchor_governance_stake_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
		let contract_addr_anc = get_contract("anchorprotocol","ANC"); 
	 	let contract_addr_gov = get_contract("anchorprotocol","gov"); 	 
        let coins: [Coin;0] = []; // no coins needed
        /* JSON: "{"stake_voting_tokens":{}}"
  		 * Base64-encoded JSON: "eyJzdGFrZV92b3RpbmdfdG9rZW5zIjp7fX0="
  		 */
        let execute_msg_json = format!("{}{}{}{}{}",r##"{
									  "send": {
									    "msg": "eyJzdGFrZV92b3RpbmdfdG9rZW5zIjp7fX0=",
									    "amount": ""##,coin_amount.to_string().as_str(),r##"",
									    "contract": ""##,contract_addr_gov,r##""
									  }
									}"##);

        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_anc, &execute_msg_json, &coins)?;
        return Ok(send);  
}

pub async fn anchor_borrow_and_deposit_stable_tx(mnemonics: &str, coin_amount_borrow: Decimal,coin_amount_deposit: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{

        let secp = Secp256k1::new();
        let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
        let from_public_key = from_key.public_key(&secp);
        let from_account = from_public_key.account()?;
 
        let mut messages = Vec::new();
        messages.push(anchor_borrow_stable_msg(&from_account,coin_amount_borrow)?);
        messages.push(anchor_deposit_stable_msg(&from_account,coin_amount_deposit)?);

        let res = estimate_messages(&from_account,messages,gas_price_uusd,gas_adjustment).await?;

        let gas_opts = match estimate_to_gas_opts(res,only_estimate,max_tx_fee) {
            Err(err) => {
                return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
            },
            Ok(e) => {e}
        };

        let mut messages = Vec::new();
        messages.push(anchor_borrow_stable_msg(&from_account,coin_amount_borrow)?);
        messages.push(anchor_deposit_stable_msg(&from_account,coin_amount_deposit)?);

        execute_messages(mnemonics,messages,gas_opts).await
}

pub async fn anchor_redeem_and_repay_stable_tx(mnemonics: &str, coin_amount_redeem: Decimal,coin_amount_repay: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{

		let secp = Secp256k1::new();
        let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
        let from_public_key = from_key.public_key(&secp);
        let from_account = from_public_key.account()?;
 
 		let mut messages = Vec::new();
 		messages.push(anchor_redeem_stable_msg(&from_account,coin_amount_redeem)?);
 		messages.push(anchor_repay_stable_msg(&from_account,coin_amount_repay)?);

        let res = estimate_messages(&from_account,messages,gas_price_uusd,gas_adjustment).await?;

    	let gas_opts = match estimate_to_gas_opts(res,only_estimate,max_tx_fee) {
            Err(err) => {
                return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
            },
            Ok(e) => {e}
        };

        let mut messages = Vec::new();
        messages.push(anchor_redeem_stable_msg(&from_account,coin_amount_redeem)?);
        messages.push(anchor_repay_stable_msg(&from_account,coin_amount_repay)?);

        execute_messages(mnemonics,messages,gas_opts).await
}

pub async fn anchor_redeem_stable_tx(mnemonics: &str, coin_amount_redeem: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{
		let secp = Secp256k1::new();
        let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
        let from_public_key = from_key.public_key(&secp);
  
        let from_account = from_public_key.account()?;
 
        let messages: Vec<Message> = vec![anchor_redeem_stable_msg(&from_account,coin_amount_redeem)?]; 

        let res = estimate_messages(&from_account,messages,gas_price_uusd,gas_adjustment).await?;

        let gas_opts = match estimate_to_gas_opts(res,only_estimate,max_tx_fee) {
            Err(err) => {
                return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
            },
            Ok(e) => {e}
        };

        let messages: Vec<Message> = vec![anchor_redeem_stable_msg(&from_account,coin_amount_redeem)?]; 

        execute_messages(mnemonics,messages,gas_opts).await
}

pub async fn anchor_repay_stable_tx(mnemonics: &str, coin_amount_repay: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{

        let secp = Secp256k1::new();
        let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
        let from_public_key = from_key.public_key(&secp);
        let from_account = from_public_key.account()?;
 
        let mut messages = Vec::new();
        messages.push(anchor_repay_stable_msg(&from_account,coin_amount_repay)?);

        let res = estimate_messages(&from_account,messages,gas_price_uusd,gas_adjustment).await?;

        let gas_opts = match estimate_to_gas_opts(res,only_estimate,max_tx_fee) {
            Err(err) => {
                return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
            },
            Ok(e) => {e}
        };

        let mut messages = Vec::new();
        messages.push(anchor_repay_stable_msg(&from_account,coin_amount_repay)?);

        execute_messages(mnemonics,messages,gas_opts).await
}

pub async fn anchor_governance_claim_and_stake(mnemonics: &str, coin_amount: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{
	 	 
	 	let secp = Secp256k1::new();
        let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
        let from_public_key = from_key.public_key(&secp);
        let from_account = from_public_key.account()?;

        let send_claim = anchor_governance_claim_msg(&from_account)?;
        let send_stake = anchor_governance_stake_msg(&from_account,coin_amount)?;
 		
        let messages: Vec<Message> = vec![send_claim,send_stake];

		let res = estimate_messages(&from_account,messages,gas_price_uusd,gas_adjustment).await?;

        //let estimate_json = serde_json::to_string(&res.result); 
		//{"fee":{"amount":[{"amount":"90462","denom":"uusd"}],"gas":"603080"}}
		let gas_opts = match estimate_to_gas_opts(res,only_estimate,max_tx_fee) {
            Err(err) => {
                return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
            },
            Ok(e) => {e}
        };

	 	let send_claim = anchor_governance_claim_msg(&from_account)?;
        let send_stake = anchor_governance_stake_msg(&from_account,coin_amount)?;

        let messages: Vec<Message> = vec![send_claim,send_stake];

        execute_messages(mnemonics,messages,gas_opts).await
}



pub async fn anchor_governance_stake(mnemonics: &str, coin_amount: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{
		let secp = Secp256k1::new();
        let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
        let from_public_key = from_key.public_key(&secp);
        let from_account = from_public_key.account()?;
 
        let send_stake = anchor_governance_stake_msg(&from_account,coin_amount)?;
 		
        let messages: Vec<Message> = vec![send_stake];

		let res = estimate_messages(&from_account,messages,gas_price_uusd,gas_adjustment).await?;

        //let estimate_json = serde_json::to_string(&res.result); 
		//{"fee":{"amount":[{"amount":"90462","denom":"uusd"}],"gas":"603080"}}

		let gas_opts = match estimate_to_gas_opts(res,only_estimate,max_tx_fee) {
            Err(err) => {
                return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
            },
            Ok(e) => {e}
        };

        let send_stake = anchor_governance_stake_msg(&from_account,coin_amount)?;

        let messages: Vec<Message> = vec![send_stake];

        execute_messages(mnemonics,messages,gas_opts).await
}

pub async fn anchor_claim_rewards(mnemonics: &str, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{

	 	let secp = Secp256k1::new();
        let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
        let from_public_key = from_key.public_key(&secp);
        let from_account = from_public_key.account()?;

        let send_claim = anchor_governance_claim_msg(&from_account)?; 
 		
        let messages: Vec<Message> = vec![send_claim];

		let res = estimate_messages(&from_account,messages,gas_price_uusd,gas_adjustment).await?;

        //let estimate_json = serde_json::to_string(&res.result); 
		//{"fee":{"amount":[{"amount":"90462","denom":"uusd"}],"gas":"603080"}}

		let gas_opts = match estimate_to_gas_opts(res,only_estimate,max_tx_fee) {
            Err(err) => {
                return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
            },
            Ok(e) => {e}
        };

	 	let send_claim = anchor_governance_claim_msg(&from_account)?; 

        let messages: Vec<Message> = vec![send_claim];

        execute_messages(mnemonics,messages,gas_opts).await
}