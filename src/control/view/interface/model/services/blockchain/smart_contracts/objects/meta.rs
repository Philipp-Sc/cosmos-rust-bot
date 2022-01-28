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
/*
https://api.nexusprotocol.app/graphql
{
  "query": "\n  query ($address: String!) {\n    findAirdropsByAddress(address: $address) {\n      claimablePsiTokens\n      proofs\n      stage\n    }\n  }\n",
  "variables": {
    "address": "***REMOVED***"
  }
}
{"data":{"findAirdropsByAddress":[{"claimablePsiTokens":"60113231","proofs":["c03922c661a087820ddfd48e1803e2770c8742b4c1a7d517a832ce240fb91e9a","7b10227c07cfb85cb713e887a75e96be8917dd38fd4852be0018c0060621379f","bcab66a9f047fa2d64baa1a97df1e35550b89ccb0b51623730313f4fb3fe9978","c10f7f6b2eee71382078fb4b57dc7e90ee1a596526b503b10eb8c34df2485125","a8491ea876b5461c9246faf61defd7182426873c046ce0dd2989cc538d43ac69","93906a843e59846395feee60c22eaa6aeb11672ac6c7f4db218940cc7d2c64df","1a27480aedaead47314f40726e80284d5008d9ef1d2e7fbf4f36b3f34f4baec7","54683c1383e6bc308e91361538539cd3fd691bfe74db9f5c1437fa45934c6e0a","0f80e8a192c1a545fe3ef7d00019af2290c65f19e0b7ba966d4af9820d7f3c66","ed2073d52595a83fbc1c4aef8bf9688d373c425a0fbeaf35ae2f5879f99ab704","6e861c7d183b7cef8f31f87269a1ff6384334fdfb0c2b5bb301684c5c5f9dfd8","b3c31800855f64d0ddfb771e25f1d7e8035407ca38f0366f634bf03427a1cdaa","45d358cb8b3d24cc97a7d8dc8f7863748d1588a2d655d107998c696698d8de34","0266e70814d8ceebbacda9335e4025f09008080115dacce0a03192ac0c76b442"],"stage":15}]}}


https://airdrop.anchorprotocol.com/api/get?address=***REMOVED***&chainId=columbus-4
https://fcd.terra.dev/wasm/contracts/terra146ahqn6d3qgdvmj8cj96hh03dzmeedhsf0kxqm/store?query_msg={%22latest_stage%22:{}}
https://fcd.terra.dev/wasm/contracts/terra146ahqn6d3qgdvmj8cj96hh03dzmeedhsf0kxqm/store?query_msg={%22is_claimed%22:{%22stage%22:43,%22address%22:%20%22***REMOVED***%22}}


*/
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