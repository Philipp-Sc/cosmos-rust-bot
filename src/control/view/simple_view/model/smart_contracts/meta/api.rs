pub mod data;

  
use data::endpoints::{get_terra_fcd,get_terra_lcd,get_terra_chain};
use data::{GasPrices};
 
use serde::Deserialize;
use serde::Serialize;

 
//use serde_json::json;
use serde_json::Value;


use terra_rust_api::{Terra, GasOptions};
use terra_rust_api::core_types::{Coin};  

use terra_rust_api::client::tendermint_types::BlockResult;
use terra_rust_api::client::tx_types::V1TXSResult;

use terra_rust_api::{PrivateKey};
use terra_rust_api::messages::wasm::MsgExecuteContract;
//use terra_rust_api::core_types::{StdSignMsg, StdSignature}; 
//use terra_rust_api::messages::bank::MsgSend;

use terra_rust_api::messages::Message;

use secp256k1::Secp256k1;


use anyhow::anyhow;


use std::time::SystemTime;


use rust_decimal::Decimal;
//use rust_decimal_macros::dec;
//use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use core::str::FromStr;

use data::terra_contracts::get_contract;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStatus {
    pub is_ok: bool,
    pub message: String,
    pub timestamp: u128
}
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResponse<T> {
    pub response_status: Option<ResponseStatus>,
    pub response: Option<T> 
}  
 
pub async fn anchor_governance_claim_and_stake(mnemonics: &str, coin_amount: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, max_tx_fee_setting: Decimal, only_estimate: bool) -> anyhow::Result<String>{
	 	let contract_addr_anc = get_contract("anchorprotocol","ANC"); 
	 	let contract_addr_gov = get_contract("anchorprotocol","gov"); 
	 	let contract_addr_mm_market = get_contract("anchorprotocol","mmMarket"); 

	 	let secp = Secp256k1::new();
        let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
        let from_public_key = from_key.public_key(&secp);
  
        let from_account = from_public_key.account()?;
 
        let execute_msg_json_claim = r##"{"claim_rewards":{}}"##;

        let coins: [Coin;0] = []; // no coins needed

        let send_claim = MsgExecuteContract::create_from_json(&from_account, &contract_addr_mm_market, execute_msg_json_claim, &coins)?;

        /* JSON: "{"stake_voting_tokens":{}}"
  		 * Base64-encoded JSON: "eyJzdGFrZV92b3RpbmdfdG9rZW5zIjp7fX0="
  		 */
        let execute_msg_json_stake = format!("{}{}{}{}{}",r##"{
									  "send": {
									    "msg": "eyJzdGFrZV92b3RpbmdfdG9rZW5zIjp7fX0=",
									    "amount": ""##,coin_amount.to_string().as_str(),r##"",
									    "contract": ""##,contract_addr_gov,r##""
									  }
									}"##);

        let send_stake = MsgExecuteContract::create_from_json(&from_account, &contract_addr_anc, &execute_msg_json_stake, &coins)?;
        
        let messages: [Message;2] = [send_claim,send_stake];

        // get fee estimate

		let endpoint: &str = &get_terra_lcd();
		let chain: &str = &get_terra_chain();
 		let terra = Terra::lcd_client_no_tx(endpoint,chain); 

 		let gas_coin = Coin::create("uusd", gas_price_uusd);

		let res = terra.tx()
        .estimate_fee(
            &from_account,
            &messages,
            gas_adjustment.to_f64().unwrap(),
            &[&gas_coin],
        ).await?;

        //let estimate_json = serde_json::to_string(&res.result); 
		//{"fee":{"amount":[{"amount":"90462","denom":"uusd"}],"gas":"603080"}}

        let fees: Vec<Coin> = res.result.fee.amount; 
 
        if fees.len() != 1 {
        	return Err(anyhow!("Unexpected Fee Estimate. fees.len() = {:?}",fees.len()));
        }

		let tx_fee = Decimal::from_str(fees[0].amount.to_string().as_str())?;
		let micro = Decimal::from_str("1000000").unwrap();
		let tx_fee = tx_fee.checked_div(micro).unwrap();
		let gas_limit = res.result.fee.gas;

        if only_estimate {
			return Ok(format!("tx fee: {} UST (gas limit: {})",tx_fee,gas_limit));
        }
 
        if fees[0].amount > max_tx_fee || fees[0].amount > max_tx_fee_setting {
			return Err(anyhow!("Unexpected High Fee: {:?}",fees));
        }
        if fees[0].denom != "uusd" {
			return Err(anyhow!("Unexpected Fee Denom: {:?}",fees));        	
        } 
 
        let gas_opts = GasOptions {
            fees: Some(Coin::create(&fees[0].denom,fees[0].amount)),
            estimate_gas: false,
            gas: Some(res.result.fee.gas),
            gas_price: None,
            gas_adjustment: None,
        };

	 	let terra = Terra::lcd_client(endpoint,chain, &gas_opts, None);
        

	 	let send_claim = MsgExecuteContract::create_from_json(&from_account, &contract_addr_mm_market, &execute_msg_json_claim, &coins)?;

        let send_stake = MsgExecuteContract::create_from_json(&from_account, &contract_addr_anc, &execute_msg_json_stake, &coins)?;
       
        let messages: Vec<Message> = vec![send_claim,send_stake];

        let (std_sign_msg, sigs) = terra
               .generate_transaction_to_broadcast(
                   &secp,
                   &from_key,
                   messages,
                   None
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
				 return Ok(format!("tx hash: {}, tx fee: {} UST (gas limit: {})",resp.txhash,tx_fee,gas_limit)); 
		     }
		 }

}



pub async fn anchor_governance_stake(mnemonics: &str, coin_amount: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{
		let contract = get_contract("anchorprotocol","ANC"); 
	 	let contract_addr_gov = get_contract("anchorprotocol","gov");
 		// transaction message

		let secp = Secp256k1::new();
        let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
        let from_public_key = from_key.public_key(&secp);
  
        let from_account = from_public_key.account()?;
  
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


        let coins: [Coin;0] = [];  

        let send = MsgExecuteContract::create_from_json(&from_account, &contract, &execute_msg_json, &coins)?;
        let messages: [Message;1] = [send];

        // get fee estimate

		let endpoint: &str = &get_terra_lcd();
		let chain: &str = &get_terra_chain();
 		let terra = Terra::lcd_client_no_tx(endpoint,chain); 

 		let gas_coin = Coin::create("uusd", gas_price_uusd);

		let res = terra.tx()
        .estimate_fee(
            &from_account,
            &messages,
            gas_adjustment.to_f64().unwrap(),
            &[&gas_coin],
        ).await?;

        let estimate_json = serde_json::to_string(&res.result);
        let fees: Vec<Coin> = res.result.fee.amount; 

        if fees.len() != 1 {
        	return Err(anyhow!("Unexpected Fee Estimate. fees.len() = {:?}",fees.len()));
        }

        
        if only_estimate {
        	return Ok(format!("{}",estimate_json?));
        }
 
        if fees[0].amount > max_tx_fee {
			return Err(anyhow!("Unexpected High Fee: {:?}",fees));
        }
        if fees[0].denom != "uusd" {
			return Err(anyhow!("Unexpected Fee Denom: {:?}",fees));        	
        } 
 
        let gas_opts = GasOptions {
            fees: Some(Coin::create(&fees[0].denom,fees[0].amount)),
            estimate_gas: false,
            gas: Some(res.result.fee.gas),
            gas_price: None,
            gas_adjustment: None,
        };
 

	 	let terra = Terra::lcd_client(endpoint,chain, &gas_opts, None);
       

        let send = MsgExecuteContract::create_from_json(&from_account, &contract, &execute_msg_json, &coins)?;
        let messages: Vec<Message> = vec![send];

        let (std_sign_msg, sigs) = terra
               .generate_transaction_to_broadcast(
                   &secp,
                   &from_key,
                   messages,
                   None
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
		         return Ok(format!("{}", resp.txhash));
		     }
		 }
}

pub async fn anchor_claim_rewards(mnemonics: &str, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{

	 	let contract = get_contract("anchorprotocol","mmMarket");
 		// transaction message

		let secp = Secp256k1::new();
        let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
        let from_public_key = from_key.public_key(&secp);
  
        let from_account = from_public_key.account()?;
 
        let execute_msg_json = r##"{"claim_rewards":{}}"##;

        let coins: [Coin;0] = []; // no coins needed

        let send = MsgExecuteContract::create_from_json(&from_account, &contract, execute_msg_json, &coins)?;

        let messages: [Message;1] = [send];

        // get fee estimate

		let endpoint: &str = &get_terra_lcd();
		let chain: &str = &get_terra_chain();
 		let terra = Terra::lcd_client_no_tx(endpoint,chain); 

 		let gas_coin = Coin::create("uusd", gas_price_uusd);

		let res = terra.tx()
        .estimate_fee(
            &from_account,
            &messages,
            gas_adjustment.to_f64().unwrap(),
            &[&gas_coin],
        ).await?;


        let estimate_json = serde_json::to_string(&res.result);
        let fees: Vec<Coin> = res.result.fee.amount; 

        if fees.len() != 1 {
        	return Err(anyhow!("Unexpected Fee Estimate. fees.len() = {:?}",fees.len()));
        }
        
        if only_estimate {
        	return Ok(format!("{}",estimate_json?));
        }
 
        if fees[0].amount > max_tx_fee {
			return Err(anyhow!("Unexpected High Fee: {:?}",fees));
        }
        if fees[0].denom != "uusd" {
			return Err(anyhow!("Unexpected Fee Denom: {:?}",fees));        	
        } 
 
        let gas_opts = GasOptions {
            fees: Some(Coin::create(&fees[0].denom,fees[0].amount)),
            estimate_gas: false,
            gas: Some(res.result.fee.gas),
            gas_price: None,
            gas_adjustment: None,
        };
 

	 	let terra = Terra::lcd_client(endpoint,chain, &gas_opts, None);
       
        let send = MsgExecuteContract::create_from_json(&from_account, &contract, execute_msg_json, &coins)?;
        let messages: Vec<Message> = vec![send];

        let (std_sign_msg, sigs) = terra
               .generate_transaction_to_broadcast(
                   &secp,
                   &from_key,
                   messages,
                   None
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
		         return Ok(format!("{}", resp.txhash));
		     }
		 }
}


pub async fn get_fcd_or_lcd_query(contract_addr: &str, query_msg: &str, gas_prices: &GasPrices) -> anyhow::Result<String>{

	/*
     * Returns the response from which ever server is faster.
     * If a request fails, the alternative server is tried once.
	 */ 
	 tokio::select! {
        v1 = get_fcd_query(contract_addr,query_msg) => {return v1},
        v1 = get_lcd_query(contract_addr,query_msg, gas_prices) => {return v1},
    };
    //result = get_lcd_query(query_identifier, gas_prices).await;
    //Err(anyhow!("Unexpected Error: Null"))

}

pub async fn get_fcd_else_lcd_query(contract_addr: &str, query_msg: &str, gas_prices: &GasPrices) -> anyhow::Result<String>{
	let mut v1 = get_fcd_query(contract_addr,query_msg).await;
	if let Err(_) = v1 { 
	    	v1 = get_lcd_query(contract_addr,query_msg, gas_prices).await; 
	} 
	v1
}

pub async fn get_lcd_else_fcd_query(contract_addr: &str, query_msg: &str, gas_prices: &GasPrices) -> anyhow::Result<String>{
    let mut v1 = get_lcd_query(contract_addr,query_msg, gas_prices).await;
	if let Err(_) = v1 { 
	    	v1 = get_fcd_query(contract_addr,query_msg).await; 
	} 
	v1
}



pub async fn query_api(api: &str) -> anyhow::Result<String> {
	// https://docs.terraswap.io/docs/howto/query/  
	let response = reqwest::get(api).await?.text().await?; 
    Ok(response)
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
				},
				Err(err) => {
					return Err(anyhow!("Unexpected reqwest::Error: {:?}",err))
				}
			}
		},
		Err(e) => {
			return Err(anyhow!("Unexpected reqwest::Error: {:?}",e));
		}
	} 
}

pub async fn get_lcd_query(contract_addr: &str, query_msg: &str, gas_prices: &GasPrices) -> anyhow::Result<String>{
  
	let gas_opts = GasOptions::create_with_gas_estimate(format!("{}{}", gas_prices.uusd, "uusd").as_str(),1.4)?; 
	let endpoint: &str = &get_terra_lcd();
	let chain: &str = &get_terra_chain();
 	let terra = Terra::lcd_client(endpoint,chain, &gas_opts, None);
  	

	let code_result = terra.send_cmd::<Value>(
                &format!("/wasm/contracts/{}/store?", &contract_addr),
                Some(&format!("query_msg={}", query_msg)),
            ).await;
 
    match code_result {
    	Ok(res) => {
    		return Ok(res.to_string());
    	},
    	Err(err) => {
    		return Err(anyhow!("Unexpected Error: {:?}",err));
    	}
    }

}
  
pub async fn query_core_block_txs(height: u64, offset: Option<u64>, limit: Option<u64>) -> anyhow::Result<V1TXSResult> {  
	let endpoint: &str = &get_terra_lcd();
	let chain: &str = &get_terra_chain();
 	let terra = Terra::lcd_client_no_tx(endpoint,chain); 
	let sw = terra.tx().get_txs_in_block(height,offset,limit).await?;
    Ok(sw)
} 

pub async fn query_core_latest_block() -> anyhow::Result<BlockResult> {  
	let endpoint: &str = &get_terra_lcd();
	let chain: &str = &get_terra_chain();
 	let terra = Terra::lcd_client_no_tx(endpoint,chain); 
	let sw = terra.tendermint().blocks().await?;
    Ok(sw)
}

pub async fn query_core_block_at_height(height: u64) -> anyhow::Result<BlockResult> {  
	let endpoint: &str = &get_terra_lcd();
	let chain: &str = &get_terra_chain();
 	let terra = Terra::lcd_client_no_tx(endpoint,chain); 
	let sw = terra.tendermint().blocks_at_height(height).await?; 
	Ok(sw)
}


pub async fn query_core_market_swap_rate(from: &str, to: &str,gas_prices: &GasPrices) -> anyhow::Result<String> { 
 
	let gas_opts = GasOptions::create_with_gas_estimate(format!("{}{}", gas_prices.uusd, "uusd").as_str(),1.4)?; 
	let endpoint: &str = &get_terra_lcd();
	let chain: &str = &get_terra_chain();
 	let terra = Terra::lcd_client(endpoint,chain, &gas_opts,None); 
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
 	let terra = Terra::lcd_client_no_tx(endpoint,chain); 	 
	let sw = terra.bank().balances(&account_address).await?; // anyhow::Result<LCDResultVec<Coin>>

    Ok(serde_json::to_string_pretty(&sw)?)
}


pub async fn fetch_gas_price() -> QueryResponse<GasPrices> {

    let gas_prices_response = reqwest::get(format!("{}/v1/txs/gas_prices",get_terra_fcd())).await;
    if let Ok(e) = gas_prices_response { 
        let result = e.text().await;
        match result {
            Ok(result) => {
                let json = serde_json::from_str::<GasPrices>(result.as_str());
                match json {
                    Ok(j) => {
                        let query_response: QueryResponse<GasPrices> = QueryResponse {
                                response_status: Some(ResponseStatus{ is_ok: true, message: format!("").to_string(), timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()}),               
                                response: Some(j)
                        };    
                        return query_response;
                    },
                    Err(ref e) => {
                        let query_response: QueryResponse<GasPrices> = QueryResponse {
                                response_status: Some(ResponseStatus{ is_ok: false, message: format!("WARNING: Parsing failed for: \n{:?}\nDEBUG: {:?}",result,e).to_string(), timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()}),               
                                response: Some(serde_json::from_str::<GasPrices>(r#"{"uluna":"0.01133","usdr":"0.104938","uusd":"0.15","ukrw":"169.77","umnt":"428.571","ueur":"0.125","ucny":"0.98","ujpy":"16.37","ugbp":"0.11","uinr":"10.88","ucad":"0.19","uchf":"0.14","uaud":"0.19","usgd":"0.2","uthb":"4.62","usek":"1.25","unok":"1.25","udkk":"0.9","uidr":"2180.0","uphp":"7.6","uhkd":"1.17"}"#).unwrap())
                        }; 
                        return query_response;
                        
                    }
                }
            },
            Err(ref e) => {
                let query_response: QueryResponse<GasPrices> = QueryResponse {
                        response_status: Some(ResponseStatus{ is_ok: false, message: format!("WARNING: Could not get response: \n{:?}\nDEBUG: {:?}",result,e).to_string(), timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()}),               
                        response: Some(serde_json::from_str::<GasPrices>(r#"{"uluna":"0.01133","usdr":"0.104938","uusd":"0.15","ukrw":"169.77","umnt":"428.571","ueur":"0.125","ucny":"0.98","ujpy":"16.37","ugbp":"0.11","uinr":"10.88","ucad":"0.19","uchf":"0.14","uaud":"0.19","usgd":"0.2","uthb":"4.62","usek":"1.25","unok":"1.25","udkk":"0.9","uidr":"2180.0","uphp":"7.6","uhkd":"1.17"}"#).unwrap())
                }; 
                return query_response;
            }
        } 
    }
    let query_response: QueryResponse<GasPrices> = QueryResponse {
                        response_status: Some(ResponseStatus{ is_ok: false, message: ("WARNING: Unable to get current gas_prices.\nWARNING: Using static values for the gas_prices.").to_string(), timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()}),               
                        response: Some(serde_json::from_str::<GasPrices>(r#"{"uluna":"0.01133","usdr":"0.104938","uusd":"0.15","ukrw":"169.77","umnt":"428.571","ueur":"0.125","ucny":"0.98","ujpy":"16.37","ugbp":"0.11","uinr":"10.88","ucad":"0.19","uchf":"0.14","uaud":"0.19","usgd":"0.2","uthb":"4.62","usek":"1.25","unok":"1.25","udkk":"0.9","uidr":"2180.0","uphp":"7.6","uhkd":"1.17"}"#).unwrap())
    }; 
    query_response
}
 
/*
pub async fn get_gas_fees_fcd -> Result<GasOptions> {
	let client = reqwest::Client::new();
	let gas_opts = GasOptions::create_with_fcd(&client,get_terra_fcd(),"uusd",1.2);
	gas_opts
}

pub async fn get_gas_fees_fcd -> Result<GasOptions> {
	let client = reqwest::Client::new();
	let gas_opts = GasOptions::create_with_fcd(&client,get_terra_fcd(),"uusd",1.2);
	gas_opts
} */
                //let anchor = &query_core_bank_balances("terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s",&gas_prices).await?;
                //let total_deposits = Decimal::from_str(anchor["result"][0]["amount"].to_string().replace(r#"""#,"").as_str())?;
 
                //println!("{:?}",anchor);
                //println!("\n[Anchor] total deposits:      {}",total_deposits); 
                // https://api.anchorprotocol.com/api/v1/deposit



                /*pub mod queries;

use queries::params;
use queries::smart_contracts::meta::endpoints::{get_terra_fcd,get_terra_lcd,get_terra_chain};
use queries::smart_contracts::meta::{GasPrices, Params};

use queries::smart_contracts::{bluna_hub_state};

use serde::Deserialize;
use serde::Serialize;


use serde_json::Value;
use serde_json::json;


use terra_rust_api::{Terra, GasOptions};
use terra_rust_api::core_types::{Coin};  


#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SwapResponse {
	pub amount: String,
	pub denom: String,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct TerraSwapResponse {
	pub return_amount: String,
	pub spread_amount: String,
	pub commission_amount: String,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct MarketResponse<T> {
	pub height: String,
	pub result: T,
} 

// improvement -> pass function into params -> returns MarketResponse<T>
// get_fastest_query -> returns enum
// point where get_fastest_query was called matches/unwraps the right struct.

// improvement, get_fastest_query can only return objects I know.
// most importantly I see all the Response types in code, I do not need to guess them.
// params gets improved

pub async fn get_fastest_query(query_identifier: &str, gas_prices: &GasPrices) -> anyhow::Result<Value>{

	let x = get_fcd_query_1;
	let y = bluna_hub_state(&x,gas_prices).await?;
	loop {
		println!("{:?}",y);
	}
	/*
     * Returns the response from which ever server is faster.
     * If a request fails, the alternative server is tried once.
	 */
	let mut result: anyhow::Result<Value> = Ok(json!(null));
	 tokio::select! {
        v1 = get_fcd_query(query_identifier) => {
        	if let Err(_) = result { 
			    	result = get_lcd_query(query_identifier, gas_prices).await; 
			}else{
				result = v1;
			}
		},
        v1 = get_lcd_query(query_identifier, gas_prices) => {
        	if let Err(_) = result { 
			    	result = get_fcd_query(query_identifier).await; 
			}else{
				result = v1;
			}
		},
    };


    //result = get_lcd_query(query_identifier, gas_prices).await;
    result

}

pub async fn query_api(api: &str) -> anyhow::Result<Value> {
	// https://docs.terraswap.io/docs/howto/query/  
	let response = reqwest::get(api).await?.text().await?;
    let json: Value = serde_json::from_str(&response)?;

    Ok(json)
}

pub async fn get_fcd_query_1(contract_addr: &str, query: &str) -> anyhow::Result<String> {
	// https://docs.terraswap.io/docs/howto/query/  
	let response = reqwest::get(
		format!("{}/wasm/contracts/{}/store?query_msg={}", 
		get_terra_fcd(),
		contract_addr,
		query
		).as_str()).await?.text().await?;
 
    Ok(response)
}

pub async fn get_fcd_query(query_identifier: &str) -> anyhow::Result<Value> {
	// https://docs.terraswap.io/docs/howto/query/ 
	let param = params(query_identifier);
	let response = reqwest::get(
		format!("{}/wasm/contracts/{}/store?query_msg={}", 
		get_terra_fcd(),
		param.1, // contract_addr
		param.0  // query
		).as_str()).await?.text().await?;

    let json: Value = serde_json::from_str(&response)?;

    Ok(json)
}

pub async fn get_lcd_query(query_identifier: &str, gas_prices: &GasPrices) -> anyhow::Result<Value>{

	// This structure is used to determine what your preferences are 
	// by default Higher fees may be given preference by the validator to include the transaction in their block

	// When Submitting transactions you need to either submit gas or a fee to the validator 

	let gas_opts = GasOptions::create_with_gas_estimate(format!("{}{}", gas_prices.uusd, "uusd").as_str(),1.4)?;
	// set up the LCD client
	let endpoint: &str = &get_terra_lcd();
	let chain: &str = &get_terra_chain();
 	let terra = Terra::lcd_client(endpoint,chain, &gas_opts,None);

 	let param = params(query_identifier);
	
 	let query = param.0;

	let code_result = terra
                .wasm()
                .query::<serde_json::Value>(&param.1, &query)
                .await?; 

    Ok(code_result)
}

pub async fn query_core_market_swap_rate(from: &str, to: &str,gas_prices: &GasPrices) -> anyhow::Result<MarketResponse<SwapResponse>> { 

	// This structure is used to determine what your preferences are 
	// by default Higher fees may be given preference by the validator to include the transaction in their block

	// When Submitting transactions you need to either submit gas or a fee to the validator 

	let gas_opts = GasOptions::create_with_gas_estimate(format!("{}{}", gas_prices.uusd, "uusd").as_str(),1.4)?;
	// set up the LCD client
	let endpoint: &str = &get_terra_lcd();
	let chain: &str = &get_terra_chain();
 	let terra = Terra::lcd_client(endpoint,chain, &gas_opts,None);
	//let terra = Terra::lcd_client("https://bombay-lcd.terra.dev/", "bombay-12", &gas_opts,None).await?;
	let coin: Coin = Coin::parse(format!("{}{}", "1000000", from).as_str())?.unwrap();
	let ask = to; 

	let sw = terra.market().swap(&coin, &ask).await?; // anyhow::Result<LCDResult<Coin>>
 
	// {\n  \"height\": \"5470599\",\n  \"result\": {\n    \"amount\": \"1388867\",\n    \"denom\": \"uusd\"\n  }\n}

    //Ok(serde_json::from_str(&serde_json::to_string_pretty(&sw)?)?)

    Ok(serde_json::from_str::<MarketResponse<SwapResponse>>(&serde_json::to_string_pretty(&sw)?)?)
}

pub async fn query_core_bank_balances(account_address: &str, gas_prices: &GasPrices) -> anyhow::Result<Value> { 

	// This structure is used to determine what your preferences are 
	// by default Higher fees may be given preference by the validator to include the transaction in their block

	// When Submitting transactions you need to either submit gas or a fee to the validator 

	let gas_opts = GasOptions::create_with_gas_estimate(format!("{}{}", gas_prices.uusd, "uusd").as_str(),1.4)?;
	// set up the LCD client
	let endpoint: &str = &get_terra_lcd();
	let chain: &str = &get_terra_chain();
 	let terra = Terra::lcd_client(endpoint,chain, &gas_opts,None);
	//let terra = Terra::lcd_client("https://bombay-lcd.terra.dev/", "bombay-12", &gas_opts,None).await?;
	 
	let sw = terra.bank().balances(&account_address).await?; // anyhow::Result<LCDResultVec<Coin>>

    Ok(serde_json::from_str(&serde_json::to_string_pretty(&sw)?)?)

}
 
/*
pub async fn get_gas_fees_fcd -> Result<GasOptions> {
	let client = reqwest::Client::new();
	let gas_opts = GasOptions::create_with_fcd(&client,get_terra_fcd(),"uusd",1.2);
	gas_opts
}

pub async fn get_gas_fees_fcd -> Result<GasOptions> {
	let client = reqwest::Client::new();
	let gas_opts = GasOptions::create_with_fcd(&client,get_terra_fcd(),"uusd",1.2);
	gas_opts
} */
                //let anchor = &query_core_bank_balances("terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s",&gas_prices).await?;
                //let total_deposits = Decimal::from_str(anchor["result"][0]["amount"].to_string().replace(r#"""#,"").as_str())?;
 
                //println!("{:?}",anchor);
                //println!("\n[Anchor] total deposits:      {}",total_deposits); 
                // https://api.anchorprotocol.com/api/v1/deposit*/