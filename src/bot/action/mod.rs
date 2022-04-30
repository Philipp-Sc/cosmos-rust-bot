#![allow(dead_code)]

// Action -> View
// Action to be executed based on the data seen by the view.

// Using state/control to
// register results of the action to the tasks hashmap (the model/state).

use secstr::*;

use rust_decimal::Decimal;
use core::str::FromStr; 
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::meta::{
	anchor_borrow_and_deposit_stable_tx,
	anchor_redeem_and_repay_stable_tx, 
	anchor_repay_stable_tx, 
	anchor_claim_rewards,
	anchor_governance_stake,
	anchor_governance_claim_and_stake,
	anchor_governance_claim_and_provide_to_spec_vault,
	anchor_claim_and_stake_airdrop_tx};

use crate::state::control::data_is_outdated;

use crate::state::control::model::{
	Maybe,
	register_value, 
	try_get_resolved};

use crate::view::interface::*; 
use interface_macro::maybe_struct;
use view_macro::decimal_or_return;  

use crate::view::*;

use std::collections::HashMap; 


use std::sync::Arc; 
use tokio::sync::{Mutex};

use chrono::Utc;
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::ResponseResult;


use crate::state::control::model::wallet::{decrypt_text_with_secret};



 
pub async fn anchor_claim_and_stake_airdrops(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> Maybe<String> {
		let max_tx_fee = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"max_tx_fee",false, 4).await);
		let balance = decimal_or_return!(terra_balance_to_string(maybes.clone(),"uusd",false,2).await);
	 	let min_ust_balance = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"min_ust_balance",false, 4).await);

	 	if balance < min_ust_balance || balance < max_tx_fee {
	 		return maybe_struct!((Some( "Insufficient UST balance, replenish your account!".to_string()),Some(Utc::now().timestamp())));
	 	}

	 	if data_is_outdated(maybes.clone(),&["terra_balances","anchor_airdrops"]).await {
	 		return maybe_struct!((Some( "waiting for data to refresh..".to_string()),Some(Utc::now().timestamp())));
	 	}
 
        match try_get_resolved(&maybes,"anchor_airdrops").await {
			Maybe{data: Ok(res),..} => {
                let anchor_airdrops = res.as_airdrop_response().unwrap();  
                let mut vec_proof = Vec::new();
                let mut vec_stage = Vec::new();
                let mut vec_amount = Vec::new();
                for i in 0..anchor_airdrops.len() {
                    if anchor_airdrops[i].claimable {
                    	vec_proof.push(anchor_airdrops[i].proof.to_owned());
                    	vec_stage.push(anchor_airdrops[i].stage.to_owned());
                    	vec_amount.push(anchor_airdrops[i].amount.to_owned()); 
                    }
                }
                if vec_amount.len() == 0 {
                	return maybe_struct!((Some( "waiting for airdrops..".to_string()),Some(Utc::now().timestamp())));
                }
                let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false, 10).await);
                let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

                let mnemonics = match only_estimate {
					true => {wallet_acc_address.unsecure().to_string()},
					false => {decrypt_text_with_secret(&wallet_seed_phrase)}
				};

                match anchor_claim_and_stake_airdrop_tx(&mnemonics,&vec_proof, &vec_stage, &vec_amount, gas_fees_uusd, max_tx_fee, gas_adjustment_preference,only_estimate).await {
		        	Ok(msg) => {
		        		register_value(&maybes,"anchor_auto_stake_airdrops".to_string(),msg.to_owned()).await;
		        		register_value(&maybes,"latest_transaction".to_string(),msg.to_owned()).await;
		        		return maybe_struct!((Some( msg),Some(Utc::now().timestamp())));
		        	},
		        	Err(msg) => {
		        		register_value(&maybes,"anchor_auto_stake_airdrops".to_string(),msg.to_string().to_owned()).await;
		        		register_value(&maybes,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
		        		return maybe_struct!((Some( msg.to_string()),Some(Utc::now().timestamp())));
		        	}
		        }; 
                //return serde_json::to_string_pretty(&vec_claims).unwrap_or("--".to_string());
 				
 				// checks if funds are enought to proceed
 				// checks if it is worth it to claim airdrop

 				// get an estimate
            },
			Maybe { data: Err(err), .. } => {
                return maybe_struct!((Some( format!("{:?}",err)),Some(Utc::now().timestamp())));
            }
        } 

} 



pub async fn anchor_borrow_and_deposit_stable(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> Maybe<String> {

	let max_tx_fee = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"max_tx_fee",false, 4).await);
	let balance = decimal_or_return!(terra_balance_to_string(maybes.clone(),"uusd",false,2).await);
 	let min_ust_balance = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"min_ust_balance",false, 4).await);

 	if balance < min_ust_balance || balance < max_tx_fee {
 		return maybe_struct!((Some( "Insufficient UST balance, replenish your account!".to_string()),Some(Utc::now().timestamp())));
 	}
 
    let micro = Decimal::from_str("1000000").unwrap();
    
    let to_borrow = decimal_or_return!(calculate_amount(maybes.clone(),"borrow",true,0).await);
    /* to deposit is "--" if fees can not be paid, therefore this function returns here.*/
    let to_deposit = decimal_or_return!(calculate_borrow_plan(maybes.clone(),"to_deposit",2).await)
    							.checked_mul(micro).unwrap()
								.round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

 	match check_anchor_loan_status(maybes.clone(),"borrow",2).await.data.unwrap_or("--".to_string()).as_ref() {
	    	"borrow due" => {},
	    	_ => {
	    		if !only_estimate {
	    			return maybe_struct!((Some( "waiting..".to_string()),Some(Utc::now().timestamp())));
	    		}
	    	}
	};
 
	if data_is_outdated(maybes.clone(),&["terra_balances","borrow_limit","borrow_info","balance"]).await {
	 		return maybe_struct!((Some( "waiting for data to refresh..".to_string()),Some(Utc::now().timestamp())));
	}

	let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false, 10).await);

	let mnemonics = match only_estimate {
		true => {wallet_acc_address.unsecure().to_string()},
		false => {decrypt_text_with_secret(&wallet_seed_phrase)}
	};

    match anchor_borrow_and_deposit_stable_tx(&mnemonics, to_borrow, to_deposit, gas_fees_uusd,  max_tx_fee, gas_adjustment_preference, only_estimate).await {
        	Ok(msg) => {
        		register_value(&maybes,"anchor_borrow_and_deposit_stable".to_string(),msg.to_owned()).await;
        		register_value(&maybes,"latest_transaction".to_string(),msg.to_owned()).await;
        		return maybe_struct!((Some( msg),Some(Utc::now().timestamp())));
        	},
        	Err(msg) => {
        		register_value(&maybes,"anchor_borrow_and_deposit_stable".to_string(),msg.to_string().to_owned()).await;
        		register_value(&maybes,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
        		return maybe_struct!((Some( msg.to_string()),Some(Utc::now().timestamp())));
        	}
        }  
}


pub async fn anchor_redeem_and_repay_stable(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> Maybe<String> {
    let max_tx_fee = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"max_tx_fee",false, 4).await);
	let balance = decimal_or_return!(terra_balance_to_string(maybes.clone(),"uusd",false,2).await);
 	let min_ust_balance = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"min_ust_balance",false, 4).await);

 	if balance < min_ust_balance || balance < max_tx_fee {
 		return maybe_struct!((Some( "Insufficient UST balance, replenish your account!".to_string()),Some(Utc::now().timestamp())));
 	}

    let zero = Decimal::from_str("0").unwrap(); 
    let micro = Decimal::from_str("1000000").unwrap();
    
    let exchange_rate = decimal_or_return!(a_terra_exchange_rate_to_string(maybes.clone(),10).await);
   
	let to_withdraw_from_deposit = decimal_or_return!(calculate_repay_plan(maybes.clone(),"to_withdraw_from_deposit",2).await)
								   .checked_div(exchange_rate).unwrap()
								   .checked_mul(micro).unwrap()
								   .round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

    let to_repay = decimal_or_return!(calculate_repay_plan(maybes.clone(),"to_repay",2).await)
    			   .checked_mul(micro).unwrap()
				   .round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

    match check_anchor_loan_status(maybes.clone(),"repay",2).await.data.unwrap_or("--".to_string()).as_ref() {
	    	"repay due" => {},
	    	_ => {
	    		if !only_estimate {
	    			return maybe_struct!((Some( "waiting..".to_string()),Some(Utc::now().timestamp())));
	    		}
	    	}
	};

	if data_is_outdated(maybes.clone(),&["terra_balances","borrow_limit","borrow_info","balance"]).await {
	 		return maybe_struct!((Some( "waiting for data to refresh..".to_string()),Some(Utc::now().timestamp())));
	}
	
    if to_withdraw_from_deposit > zero && to_repay > zero {
  
 
		let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false, 10).await);
		 
		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

        match anchor_redeem_and_repay_stable_tx(&mnemonics,to_withdraw_from_deposit,to_repay,gas_fees_uusd,  max_tx_fee, gas_adjustment_preference, only_estimate).await {
        	Ok(msg) => {
        		register_value(&maybes,"anchor_redeem_and_repay_stable".to_string(),msg.to_owned()).await;
        		register_value(&maybes,"latest_transaction".to_string(),msg.to_owned()).await;
        		return maybe_struct!((Some( msg),Some(Utc::now().timestamp())));
        	},
        	Err(msg) => {
        		register_value(&maybes,"anchor_redeem_and_repay_stable".to_string(),msg.to_string().to_owned()).await;
        		register_value(&maybes,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
        		return maybe_struct!((Some( msg.to_string()),Some(Utc::now().timestamp())));
        	}
        }  

	}else if to_repay > zero {
		// no redeem, just repay. 

		let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false, 10).await);
		  
		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

        match anchor_repay_stable_tx(&mnemonics, to_repay, gas_fees_uusd, max_tx_fee, gas_adjustment_preference, only_estimate).await {
        	Ok(msg) => {
        		register_value(&maybes,"anchor_redeem_and_repay_stable".to_string(),msg.to_owned()).await;
        		register_value(&maybes,"latest_transaction".to_string(),msg.to_owned()).await;
        		return maybe_struct!((Some( msg),Some(Utc::now().timestamp())));
        	},
        	Err(msg) => {
        		register_value(&maybes,"anchor_redeem_and_repay_stable".to_string(),msg.to_string().to_owned()).await;
        		register_value(&maybes,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
        		return maybe_struct!((Some( msg.to_string()),Some(Utc::now().timestamp())));
        	}
        }

	}else {
		return maybe_struct!((Some( "nothing to repay".to_string()),Some(Utc::now().timestamp())));
	}
}


pub async fn anchor_borrow_claim_and_farm_rewards(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> Maybe<String> {
		let max_tx_fee = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"max_tx_fee",false, 4).await);
		let balance = decimal_or_return!(terra_balance_to_string(maybes.clone(),"uusd",false,2).await);
	 	let min_ust_balance = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"min_ust_balance",false, 4).await);

	 	if data_is_outdated(maybes.clone(),&["terra_balances","borrow_limit","borrow_info"]).await {
	 		return maybe_struct!((Some( "waiting for data to refresh..".to_string()),Some(Utc::now().timestamp())));
		}

	 	if balance < min_ust_balance || balance < max_tx_fee {
	 		return maybe_struct!((Some( "Insufficient UST balance, replenish your account!".to_string()),Some(Utc::now().timestamp())));
	 	}
	 	match estimate_anchor_protocol_next_claim_and_stake_tx(maybes.clone(),"farming","loan_amount","date_next",2).await.data.unwrap_or("--".to_string()).as_ref() {
	 		"now" => {},
	 		 _ => {
	 		 	return maybe_struct!((Some( "waiting..".to_string()),Some(Utc::now().timestamp())));
	 			}
	 	} 
	
		let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false, 10).await);

	    let anc_to_swap = decimal_or_return!(calculate_farm_plan(maybes.clone(),"anc_to_swap",true,0).await);

	    let anc_to_keep = decimal_or_return!(calculate_farm_plan(maybes.clone(),"anc_to_keep",true,0).await);
	
	    let exchange_rate = decimal_or_return!(simulation_swap_exchange_rate_to_string(maybes.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,10).await);
   
	    let ust_to_keep = anc_to_keep.checked_mul(exchange_rate).unwrap().round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

	    let zero = Decimal::from_str("1").unwrap();

	    if anc_to_keep <= zero {
	    	return maybe_struct!((Some( "waiting..".to_string()),Some(Utc::now().timestamp())));
	    }

	    let belief_price = decimal_or_return!(simulation_swap_exchange_rate_to_string(maybes.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,10).await);

	    let max_spread = Decimal::from_str("0.001").unwrap();

        let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

	 	// prepare input params, then meta execute msg.
	    match anchor_governance_claim_and_provide_to_spec_vault(&mnemonics, anc_to_keep, ust_to_keep, anc_to_swap, belief_price, max_spread, gas_fees_uusd, max_tx_fee, gas_adjustment_preference, only_estimate).await {
        	Ok(msg) => {
        		register_value(&maybes,"anchor_governance_claim_and_farm".to_string(),msg.to_owned()).await;
        		register_value(&maybes,"latest_transaction".to_string(),msg.to_owned()).await;
        		return maybe_struct!((Some( msg),Some(Utc::now().timestamp())));
        	},
        	Err(msg) => {
        		register_value(&maybes,"anchor_governance_claim_and_farm".to_string(),msg.to_string().to_owned()).await;
        		register_value(&maybes,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
        		return maybe_struct!((Some( msg.to_string()),Some(Utc::now().timestamp())));
        	}
        }
}

pub async fn anchor_borrow_claim_and_stake_rewards(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> Maybe<String> {
		let max_tx_fee = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"max_tx_fee",false, 4).await);
		let balance = decimal_or_return!(terra_balance_to_string(maybes.clone(),"uusd",false,2).await);
	 	let min_ust_balance = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"min_ust_balance",false, 4).await);

	 	if data_is_outdated(maybes.clone(),&["terra_balances","borrow_limit","borrow_info"]).await {
	 		return maybe_struct!((Some( "waiting for data to refresh..".to_string()),Some(Utc::now().timestamp())));
		}

	 	if balance < min_ust_balance || balance < max_tx_fee {
	 		return maybe_struct!((Some( "Insufficient UST balance, replenish your account!".to_string()),Some(Utc::now().timestamp())));
	 	}
	 	match estimate_anchor_protocol_next_claim_and_stake_tx(maybes.clone(),"staking","loan_amount","date_next",2).await.data.unwrap_or("--".to_string()).as_ref() {
	 		"now" => {},
	 		 _ => {
	 		 	return maybe_struct!((Some( "waiting..".to_string()),Some(Utc::now().timestamp())));
	 		}
	 	};

	
		let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false, 10).await);

	    let anc_to_claim = decimal_or_return!(borrower_rewards_to_string(maybes.clone(), true,0).await);
        let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

	    match anchor_governance_claim_and_stake(&mnemonics, anc_to_claim, gas_fees_uusd, max_tx_fee, gas_adjustment_preference, only_estimate).await {
        	Ok(msg) => {
        		register_value(&maybes,"anchor_governance_claim_and_stake".to_string(),msg.to_owned()).await;
        		register_value(&maybes,"latest_transaction".to_string(),msg.to_owned()).await;
        		return maybe_struct!((Some( msg),Some(Utc::now().timestamp())));
        	},
        	Err(msg) => {
        		register_value(&maybes,"anchor_governance_claim_and_stake".to_string(),msg.to_string().to_owned()).await;
        		register_value(&maybes,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
        		return maybe_struct!((Some( msg.to_string()),Some(Utc::now().timestamp())));
        	}
        }
}
 
pub async fn anchor_borrow_claim_rewards(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,  wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> Maybe<String> {
         
        let max_tx_fee = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"max_tx_fee",false, 4).await);
		 
		let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false, 10).await);
  
        let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);
		
		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

        match anchor_claim_rewards(&mnemonics, gas_fees_uusd, max_tx_fee, gas_adjustment_preference,only_estimate).await {
        	Ok(msg) => {
        		return maybe_struct!((Some( msg),Some(Utc::now().timestamp())));
        	},
        	Err(msg) => {
        		return maybe_struct!((Some( msg.to_string()),Some(Utc::now().timestamp())));
        	}
        }
}
pub async fn anchor_governance_stake_balance(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>,  wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> Maybe<String> {
        
        let max_tx_fee = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"max_tx_fee",false, 4).await);

		let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false, 10).await);
	 
		let anc_balance = decimal_or_return!(borrower_anc_deposited_to_string(maybes.clone(), true,0).await);
  
		let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

	    match anchor_governance_stake(&mnemonics, anc_balance, gas_fees_uusd, max_tx_fee, gas_adjustment_preference,only_estimate).await {
	    	Ok(msg) => {
	    		return maybe_struct!((Some( msg),Some(Utc::now().timestamp())));
	    	},
	    	Err(msg) => {
	    		return maybe_struct!((Some( msg.to_string()),Some(Utc::now().timestamp())));
	    	}
	    }  
} 