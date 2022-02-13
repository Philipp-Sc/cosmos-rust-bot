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
use core::str::FromStr;
use anyhow::anyhow;


use rust_decimal::prelude::ToPrimitive;
use cosmwasm_bignumber::{Uint256};
use cosmwasm_std::Uint128;

use cosmwasm_std::{
    to_binary
};

use cw20_legacy::msg::ExecuteMsg as LegacyExecuteMsg;
use moneymarket::market::{ExecuteMsg,Cw20HookMsg};

//use anchor_token::airdrop::{ExecuteMsg};

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

fn anchor_claim_airdrop_msg(wallet_acc_address: &str, proof: &str, stage: u64, amount: &str) -> anyhow::Result<Message> {
        let contract = get_contract("anchorprotocol","airdrop"); 
        
        /*ExecuteMsg::Claim {
            stage: stage,
            amount: amount,
            proof: proof,
        }*/
        //serde_json::to_string();
        let execute_msg_json = format!("{}{}{}{}{}{}{}", r##"{
                                        "claim": {
                                            "proof": "##,proof.replace("\\",""),r##",
                                            "stage": "##,stage,r##",
                                            "amount": ""##,amount,r##""
                                        }
                                    }"##);
        println!("{}",execute_msg_json);
        println!("{}",proof);
        println!("{}",proof.replace("\\",""));
        let coins: [Coin;0] = []; // no coins needed
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, &execute_msg_json, &coins)?;
        return Ok(send);
}


fn anchor_repay_stable_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
		let contract = get_contract("anchorprotocol","mmMarket"); 
        let execute_msg = ExecuteMsg::RepayStable {};
        let execute_msg_json = serde_json::to_string(&execute_msg)?;
        let coins: [Coin;1] = [Coin::create("uusd", coin_amount)];
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, &execute_msg_json, &coins)?;
        return Ok(send);
}

fn anchor_deposit_stable_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
        let contract = get_contract("anchorprotocol","mmMarket"); 
        let execute_msg = ExecuteMsg::DepositStable {};
        let execute_msg_json = serde_json::to_string(&execute_msg)?;
        let coins: [Coin;1] = [Coin::create("uusd", coin_amount)];
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, &execute_msg_json, &coins)?;
        return Ok(send);
}

fn anchor_borrow_stable_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
        let contract = get_contract("anchorprotocol","mmMarket"); 
        let execute_msg = ExecuteMsg::BorrowStable {
            borrow_amount: Uint256::from(coin_amount.to_u128().ok_or(anyhow!("incorrect coin_amount format"))?),
            to: None,
        };
        let execute_msg_json = serde_json::to_string(&execute_msg)?;
        let coins: [Coin;0] = []; // no coins needed
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract, &execute_msg_json, &coins)?;
        return Ok(send);
}

fn anchor_redeem_stable_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
		let contract_addr_a_ust = get_contract("anchorprotocol","aTerra"); 
		let contract_addr_mm_market = get_contract("anchorprotocol","mmMarket"); 
        let coins: [Coin;0] = []; // no coins needed

/*
https://github.com/Anchor-Protocol/money-market-contracts/blob/fd70cc551dbe81d655cabf09808203fa88c0c38a/contracts/market/src/testing/tests.rs
        let msg = Cw20HookMsg::RedeemStable {};

        let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: "addr0000".to_string(),
            amount: Uint128::from(1000000u128),
            msg: to_binary(&Cw20HookMsg::RedeemStable {}).unwrap(),
        });
*/

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

        let execute_msg = ExecuteMsg::ClaimRewards { to: None };
        let execute_msg_json = serde_json::to_string(&execute_msg)?; 

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

fn astroport_swap_msg(wallet_acc_address: &str, coin_amount: Decimal, max_spread: Decimal, belief_price: Decimal) -> anyhow::Result<Message> {
        let contract_addr_anc = get_contract("anchorprotocol","ANC"); 
        let contract_addr_lp = get_contract("anchorprotocol","ANC-UST LP Minter");    
        let coins: [Coin;0] = []; // no coins needed 
        let msg = format!("{}{}{}{}{}",r##"{
            "swap":
                {
                    "max_spread":""##,max_spread.to_string().as_str(),r##"",
                    "belief_price":""##,belief_price.round_dp_with_strategy(18, rust_decimal::RoundingStrategy::ToZero).to_string().as_str(),r##""
                }
            }"##);
        let execute_msg_json = format!("{}{}{}{}{}{}{}",r##"{
                                      "send": {
                                        "msg": ""##,base64::encode(msg),r##"",
                                        "amount": ""##,coin_amount.to_string().as_str(),r##"",
                                        "contract": ""##,contract_addr_lp,r##""
                                      }
                                    }"##);

        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_anc, &execute_msg_json, &coins)?;
        return Ok(send);  
} 

fn anchor_increase_allowance_msg(wallet_acc_address: &str, coin_amount: Decimal) -> anyhow::Result<Message> {
        let contract_addr_anc = get_contract("anchorprotocol","ANC"); 
        let contract_addr_lp = get_contract("anchorprotocol","SPEC ANC-UST VAULT");    
        let coins: [Coin;0] = []; // no coins needed 

        let execute_msg = LegacyExecuteMsg::IncreaseAllowance {
            spender: contract_addr_lp,
            amount: Uint128::from(coin_amount.to_u128().ok_or(anyhow!("incorrect coin_amount format"))?),
            expires: None,
        };

        let execute_msg_json = serde_json::to_string(&execute_msg)?; 
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_anc, &execute_msg_json, &coins)?;
        return Ok(send);  
} 

fn anchor_provide_to_spec_vault_msg(wallet_acc_address: &str, anc_to_keep: Decimal, ust_to_keep: Decimal) -> anyhow::Result<Message> {
        let contract_addr_anc = get_contract("anchorprotocol","ANC"); 
        let contract_addr_lp = get_contract("anchorprotocol","SPEC ANC-UST VAULT");    
        let coins: [Coin;1] = [Coin::create("uusd", ust_to_keep)];
        let execute_msg_json = format!("{}{}{}{}{}",r##"{
              "bond": {
                "assets": [
                  {
                    "amount": ""##,anc_to_keep.to_string().as_str(),r##"",
                    "info": {
                      "token": {
                        "contract_addr": "terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76"
                      }
                    }
                  },
                  {
                    "amount": ""##,ust_to_keep.to_string().as_str(),r##"",
                    "info": {
                      "native_token": {
                        "denom": "uusd"
                      }
                    }
                  }
                ],
                "compound_rate": "1",
                "contract": "terra1ukm33qyqx0qcz7rupv085rgpx0tp5wzkhmcj3f",
                "slippage_tolerance": "0.01"
              }
            }"##);
        let send = MsgExecuteContract::create_from_json(&wallet_acc_address, &contract_addr_lp, &execute_msg_json, &coins)?;
        return Ok(send);  
} 

pub async fn anchor_claim_and_stake_airdrop_tx(from_account: &str, proof: &Vec<String>, stage: &Vec<u64>, amount: &Vec<String>, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal) -> anyhow::Result<String>{
 
        let mut messages = Vec::new();
        let mut sum_anc: u64 = 0;
        for i in 0..stage.len() {
            messages.push(anchor_claim_airdrop_msg(from_account,&proof[i], stage[i], &amount[i])?); 
            sum_anc += amount[i].parse::<u64>().unwrap_or(0u64);
        }
        let send_stake = anchor_governance_stake_msg(from_account,Decimal::from_str(sum_anc.to_string().as_str())?)?;
        messages.push(send_stake);

        let res = estimate_messages(from_account,messages,gas_price_uusd,gas_adjustment).await?;

        let gas_opts = match estimate_to_gas_opts(res,true,max_tx_fee) {
            Err(err) => {
                return Err(anyhow!(format!("{:?} (gas_adjustment: {})",err,gas_adjustment)));
            },
            Ok(e) => {e}
        };
        Ok("".to_string())

}

pub async fn anchor_borrow_and_deposit_stable_tx(mnemonics: &str, coin_amount_borrow: Decimal,coin_amount_deposit: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{

        let from_account = match mnemonics.len() {
            44 => {
                // wallet_acc_address
                mnemonics.to_string()
            },
            _ => {
                // seed phrase
                let secp = Secp256k1::new();
                let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
                let from_public_key = from_key.public_key(&secp);
                from_public_key.account()?
            }
        };
 
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

		let from_account = match mnemonics.len() {
            44 => {
                // wallet_acc_address
                mnemonics.to_string()
            },
            _ => {
                // seed phrase
                let secp = Secp256k1::new();
                let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
                let from_public_key = from_key.public_key(&secp);
                from_public_key.account()?
            }
        };
 
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
		let from_account = match mnemonics.len() {
            44 => {
                // wallet_acc_address
                mnemonics.to_string()
            },
            _ => {
                // seed phrase
                let secp = Secp256k1::new();
                let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
                let from_public_key = from_key.public_key(&secp);
                from_public_key.account()?
            }
        };
 
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

        let from_account = match mnemonics.len() {
            44 => {
                // wallet_acc_address
                mnemonics.to_string()
            },
            _ => {
                // seed phrase
                let secp = Secp256k1::new();
                let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
                let from_public_key = from_key.public_key(&secp);
                from_public_key.account()?
            }
        };
 
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

pub async fn anchor_governance_claim_and_provide_to_spec_vault(mnemonics: &str, anc_to_keep: Decimal, ust_to_keep: Decimal, anc_to_swap: Decimal, belief_price: Decimal, max_spread: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{
         
        let from_account = match mnemonics.len() {
            44 => {
                // wallet_acc_address
                mnemonics.to_string()
            },
            _ => {
                // seed phrase
                let secp = Secp256k1::new();
                let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
                let from_public_key = from_key.public_key(&secp);
                from_public_key.account()?
            }
        };

        let send_claim = anchor_governance_claim_msg(&from_account)?;
        let send_swap = astroport_swap_msg(&from_account,anc_to_swap,max_spread,belief_price)?;
        let send_increase_allowance = anchor_increase_allowance_msg(&from_account,anc_to_keep)?;
        let send_provide = anchor_provide_to_spec_vault_msg(&from_account,anc_to_keep, ust_to_keep)?;

        
        let messages: Vec<Message> = vec![send_claim,send_swap,send_increase_allowance,send_provide];

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
        let send_swap = astroport_swap_msg(&from_account,anc_to_swap,max_spread,belief_price)?;
        let send_increase_allowance = anchor_increase_allowance_msg(&from_account,anc_to_keep)?;
        let send_provide = anchor_provide_to_spec_vault_msg(&from_account,anc_to_keep, ust_to_keep)?;

        let messages: Vec<Message> = vec![send_claim,send_swap,send_increase_allowance,send_provide];

        execute_messages(mnemonics,messages,gas_opts).await
}


pub async fn anchor_governance_claim_and_stake(mnemonics: &str, coin_amount: Decimal, gas_price_uusd: Decimal, max_tx_fee: Decimal, gas_adjustment: Decimal, only_estimate: bool) -> anyhow::Result<String>{
	 	 
	 	let from_account = match mnemonics.len() {
            44 => {
                // wallet_acc_address
                mnemonics.to_string()
            },
            _ => {
                // seed phrase
                let secp = Secp256k1::new();
                let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
                let from_public_key = from_key.public_key(&secp);
                from_public_key.account()?
            }
        };

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
		let from_account = match mnemonics.len() {
            44 => {
                // wallet_acc_address
                mnemonics.to_string()
            },
            _ => {
                // seed phrase
                let secp = Secp256k1::new();
                let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
                let from_public_key = from_key.public_key(&secp);
                from_public_key.account()?
            }
        };
 
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

        let from_account = match mnemonics.len() {
            44 => {
                // wallet_acc_address
                mnemonics.to_string()
            },
            _ => {
                // seed phrase
                let secp = Secp256k1::new();
                let from_key = PrivateKey::from_words(&secp,mnemonics,0,0)?;
                let from_public_key = from_key.public_key(&secp);
                from_public_key.account()?
            }
        };



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