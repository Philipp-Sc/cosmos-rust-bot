#![allow(dead_code)]

pub mod view;
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

use view::interface::model::{
	MaybeOrPromise,
	register_value,
	get_meta_data_maybe,
	get_data_maybe_or_meta_data_maybe,
	get_oldest_timestamps_of_resolved_tasks};

use view::interface::*;
use view::*;

use std::collections::HashMap; 


use std::sync::Arc; 
use tokio::sync::RwLock; 


use view::interface::model::wallet::{decrypt_text_with_secret};


macro_rules! decimal_or_return {
    ( $e:expr ) => {
        match $e {
            "--" => return String::from("--"),
            e =>  {  
                Decimal::from_str(e).unwrap()
            },
        }
    }
} 

macro_rules! decimal_or_return_str {
    ( $e:expr, $s:expr ) => {
        match ($e,$s) {
            ("--",s) => return String::from(s),
            (e,_) => Decimal::from_str(e).unwrap(),
        }
    }
} 
 
pub async fn anchor_claim_and_stake_airdrops(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {
		let max_tx_fee = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"max_tx_fee",false, 4).await.as_ref());
		let balance = decimal_or_return!(terra_balance_to_string(tasks.clone(),"uusd",false,2).await.as_ref());
	 	let min_ust_balance = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"min_ust_balance",false, 4).await.as_ref());

	 	if balance < min_ust_balance || balance < max_tx_fee {
	 		return "Insufficient UST balance, replenish your account!".to_string();
	 	}

	 	// making sure the data is not outdated
	    match get_meta_data_maybe(&tasks, "latest_transaction").await {
	    	Ok(maybe) => {
	    		let req: [&str;2] = ["terra_balances","anchor_airdrops"];
	    		if get_oldest_timestamps_of_resolved_tasks(&tasks,&req).await <= maybe.timestamp + 10 {
	    			return "waiting for data to refresh..".to_string();
	    		}
	    	},
	    	Err(_) => {
	    		// no previous transaction, free to continue.
	    	}
	    }
 
        match get_data_maybe_or_meta_data_maybe(&tasks,"anchor_airdrops").await {
            Ok(res) => {
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
                	return "waiting for airdrops..".to_string();
                }
                let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"gas_adjustment_preference",false, 10).await.as_ref());
                let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

                let mnemonics = match only_estimate {
					true => {wallet_acc_address.unsecure().to_string()},
					false => {decrypt_text_with_secret(&wallet_seed_phrase)}
				};

                match anchor_claim_and_stake_airdrop_tx(&mnemonics,&vec_proof, &vec_stage, &vec_amount, gas_fees_uusd, max_tx_fee, gas_adjustment_preference,only_estimate).await {
		        	Ok(msg) => {
		        		register_value(&tasks,"anchor_auto_stake_airdrops".to_string(),msg.to_owned()).await;
		        		register_value(&tasks,"latest_transaction".to_string(),msg.to_owned()).await;
		        		return msg;
		        	},
		        	Err(msg) => {
		        		register_value(&tasks,"anchor_auto_stake_airdrops".to_string(),msg.to_string().to_owned()).await;
		        		register_value(&tasks,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
		        		return msg.to_string();
		        	}
		        }; 
                //return serde_json::to_string_pretty(&vec_claims).unwrap_or("--".to_string());
 				
 				// checks if funds are enought to proceed
 				// checks if it is worth it to claim airdrop

 				// get an estimate
            },
            Err(err) => {
                return format!("{:?}",err);
            }
        } 

} 



pub async fn anchor_borrow_and_deposit_stable(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {

	let max_tx_fee = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"max_tx_fee",false, 4).await.as_ref());
	let balance = decimal_or_return!(terra_balance_to_string(tasks.clone(),"uusd",false,2).await.as_ref());
 	let min_ust_balance = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"min_ust_balance",false, 4).await.as_ref());

 	if balance < min_ust_balance || balance < max_tx_fee {
 		return "Insufficient UST balance, replenish your account!".to_string();
 	}
 
    let micro = Decimal::from_str("1000000").unwrap();
    
    let to_borrow = decimal_or_return!(calculate_amount(tasks.clone(),"borrow",true,0).await.as_ref());
    /* to deposit is "--" if fees can not be paid, therefore this function returns here.*/
    let to_deposit = decimal_or_return!(calculate_borrow_plan(tasks.clone(),"to_deposit",2).await.as_ref())
    							.checked_mul(micro).unwrap()
								.round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

 	match check_anchor_loan_status(tasks.clone(),"borrow",2).await.as_ref() {
	    	"borrow due" => {},
	    	_ => {
	    		if !only_estimate {
	    			return "waiting..".to_string();
	    		}
	    	}
	};

	// making sure the data is not outdated
    match get_meta_data_maybe(&tasks, "latest_transaction").await {
    	Ok(maybe) => {
    		let req: [&str;4] = ["terra_balances","borrow_limit","borrow_info","balance"];
    		if get_oldest_timestamps_of_resolved_tasks(&tasks,&req).await <= maybe.timestamp + 10 {
    			return "waiting for data to refresh..".to_string();
    		}
    	},
    	Err(_) => {
    		// no previous transaction, free to continue.
    	}
    }

	let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"gas_adjustment_preference",false, 10).await.as_ref());

	let mnemonics = match only_estimate {
		true => {wallet_acc_address.unsecure().to_string()},
		false => {decrypt_text_with_secret(&wallet_seed_phrase)}
	};

    match anchor_borrow_and_deposit_stable_tx(&mnemonics, to_borrow, to_deposit, gas_fees_uusd,  max_tx_fee, gas_adjustment_preference, only_estimate).await {
        	Ok(msg) => {
        		register_value(&tasks,"anchor_borrow_and_deposit_stable".to_string(),msg.to_owned()).await;
        		register_value(&tasks,"latest_transaction".to_string(),msg.to_owned()).await;
        		return msg;
        	},
        	Err(msg) => {
        		register_value(&tasks,"anchor_borrow_and_deposit_stable".to_string(),msg.to_string().to_owned()).await;
        		register_value(&tasks,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
        		return msg.to_string();
        	}
        }  
}


pub async fn anchor_redeem_and_repay_stable(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {
    let max_tx_fee = decimal_or_return_str!(meta_data_key_to_string(tasks.clone(),"max_tx_fee",false, 4).await.as_ref(),"(waiting for max_tx_fee)");
	let balance = decimal_or_return_str!(terra_balance_to_string(tasks.clone(),"uusd",false,2).await.as_ref(),"(waiting for ust_balance)");
 	let min_ust_balance = decimal_or_return_str!(meta_data_key_to_string(tasks.clone(),"min_ust_balance",false, 4).await.as_ref(),"waiting for min_ust_balance)");

 	if balance < min_ust_balance || balance < max_tx_fee {
 		return "Insufficient UST balance, replenish your account!".to_string();
 	}

    let zero = Decimal::from_str("0").unwrap(); 
    let micro = Decimal::from_str("1000000").unwrap();
    
    let exchange_rate = decimal_or_return_str!(a_terra_exchange_rate_to_string(tasks.clone(),10).await.as_ref(),"(waiting for a_ust_exchange_rate)");
   
	let to_withdraw_from_deposit = decimal_or_return_str!(calculate_repay_plan(tasks.clone(),"to_withdraw_from_deposit",2).await.as_ref(),"(waiting for redeem_amount)")
								   .checked_div(exchange_rate).unwrap()
								   .checked_mul(micro).unwrap()
								   .round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

    let to_repay = decimal_or_return_str!(calculate_repay_plan(tasks.clone(),"to_repay",2).await.as_ref(),"(waiting for to_repay)")
    			   .checked_mul(micro).unwrap()
				   .round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

    let gas_fees_uusd = decimal_or_return_str!(gas_price_to_string(tasks.clone(),10).await.as_ref(),"(waiting for gas_fee)");

    match check_anchor_loan_status(tasks.clone(),"repay",2).await.as_ref() {
	    	"repay due" => {},
	    	_ => {
	    		if !only_estimate {
	    			return "waiting..".to_string();
	    		}
	    	}
	};

	// making sure the data is not outdated
    match get_meta_data_maybe(&tasks, "latest_transaction").await {
    	Ok(maybe) => {
    		let req: [&str;4] = ["terra_balances","borrow_limit","borrow_info","balance"];
    		if get_oldest_timestamps_of_resolved_tasks(&tasks,&req).await <= maybe.timestamp + 10{ 
    			return "waiting for data to refresh..".to_string();
    		}
    	},
    	Err(_) => {
    		// no previous transaction, free to continue.
    	}
    }

    if to_withdraw_from_deposit > zero && to_repay > zero {
  
 
		let gas_adjustment_preference = decimal_or_return_str!(meta_data_key_to_string(tasks.clone(),"gas_adjustment_preference",false, 10).await.as_ref(),"(waiting for gas_adjustment_preference)");
		 
		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

        match anchor_redeem_and_repay_stable_tx(&mnemonics,to_withdraw_from_deposit,to_repay,gas_fees_uusd,  max_tx_fee, gas_adjustment_preference, only_estimate).await {
        	Ok(msg) => {
        		register_value(&tasks,"anchor_redeem_and_repay_stable".to_string(),msg.to_owned()).await;
        		register_value(&tasks,"latest_transaction".to_string(),msg.to_owned()).await;
        		return msg;
        	},
        	Err(msg) => {
        		register_value(&tasks,"anchor_redeem_and_repay_stable".to_string(),msg.to_string().to_owned()).await;
        		register_value(&tasks,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
        		return msg.to_string();
        	}
        }  

	}else if to_repay > zero {
		// no redeem, just repay. 

		let gas_adjustment_preference = decimal_or_return_str!(meta_data_key_to_string(tasks.clone(),"gas_adjustment_preference",false, 10).await.as_ref(),"(waiting for gas_adjustment_preference)");
		  
		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

        match anchor_repay_stable_tx(&mnemonics, to_repay, gas_fees_uusd, max_tx_fee, gas_adjustment_preference, only_estimate).await {
        	Ok(msg) => {
        		register_value(&tasks,"anchor_redeem_and_repay_stable".to_string(),msg.to_owned()).await;
        		register_value(&tasks,"latest_transaction".to_string(),msg.to_owned()).await;
        		return msg;
        	},
        	Err(msg) => {
        		register_value(&tasks,"anchor_redeem_and_repay_stable".to_string(),msg.to_string().to_owned()).await;
        		register_value(&tasks,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
        		return msg.to_string();
        	}
        }

	}else {
		return "nothing to repay".to_string();
	}
}


pub async fn anchor_borrow_claim_and_farm_rewards(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {
		let max_tx_fee = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"max_tx_fee",false, 4).await.as_ref());
		let balance = decimal_or_return!(terra_balance_to_string(tasks.clone(),"uusd",false,2).await.as_ref());
	 	let min_ust_balance = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"min_ust_balance",false, 4).await.as_ref());

	 	// making sure the data is not outdated
	    match get_meta_data_maybe(&tasks, "latest_transaction").await {
	    	Ok(maybe) => {
	    		let req: [&str;3] = ["terra_balances","borrow_limit","borrow_info"];
	    		if get_oldest_timestamps_of_resolved_tasks(&tasks,&req).await <= maybe.timestamp + 10{ 
	    			return "waiting for data to refresh..".to_string();
	    		}
	    	},
	    	Err(_) => {
	    		// no previous transaction, free to continue.
	    	}
	    }

	 	if balance < min_ust_balance || balance < max_tx_fee {
	 		return "Insufficient UST balance, replenish your account!".to_string();
	 	}
	 	match estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"farming","loan_amount","date_next",2).await.as_ref(){
	 		"now" => {},
	 		 _ => {return "waiting..".to_string()}
	 	}; 
	
		let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"gas_adjustment_preference",false, 10).await.as_ref());

	    let anc_to_swap = decimal_or_return!(calculate_farm_plan(tasks.clone(),"anc_to_swap",true,0).await.as_ref());

	    let anc_to_keep = decimal_or_return!(calculate_farm_plan(tasks.clone(),"anc_to_keep",true,0).await.as_ref());
	
	    let exchange_rate = decimal_or_return!(simulation_swap_exchange_rate_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,10).await.as_ref());
   
	    let ust_to_keep = anc_to_keep.checked_mul(exchange_rate).unwrap().round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

	    let zero = Decimal::from_str("1").unwrap();

	    if anc_to_keep <= zero {
	    	return "waiting..".to_string();
	    }

	    let belief_price = decimal_or_return!(simulation_swap_exchange_rate_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,10).await.as_ref());

	    let max_spread = Decimal::from_str("0.001").unwrap();

        let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

	 	// prepare input params, then meta execute msg.
	    match anchor_governance_claim_and_provide_to_spec_vault(&mnemonics, anc_to_keep, ust_to_keep, anc_to_swap, belief_price, max_spread, gas_fees_uusd, max_tx_fee, gas_adjustment_preference, only_estimate).await {
        	Ok(msg) => {
        		register_value(&tasks,"anchor_governance_claim_and_farm".to_string(),msg.to_owned()).await;
        		register_value(&tasks,"latest_transaction".to_string(),msg.to_owned()).await;
        		return msg;
        	},
        	Err(msg) => {
        		register_value(&tasks,"anchor_governance_claim_and_farm".to_string(),msg.to_string().to_owned()).await;
        		register_value(&tasks,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
        		return msg.to_string();
        	}
        }
}

pub async fn anchor_borrow_claim_and_stake_rewards(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {
		let max_tx_fee = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"max_tx_fee",false, 4).await.as_ref());
		let balance = decimal_or_return!(terra_balance_to_string(tasks.clone(),"uusd",false,2).await.as_ref());
	 	let min_ust_balance = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"min_ust_balance",false, 4).await.as_ref());

	 	// making sure the data is not outdated
	    match get_meta_data_maybe(&tasks, "latest_transaction").await {
	    	Ok(maybe) => {
	    		let req: [&str;3] = ["terra_balances","borrow_limit","borrow_info"];
	    		if get_oldest_timestamps_of_resolved_tasks(&tasks,&req).await <= maybe.timestamp + 10{ 
	    			return "waiting for data to refresh..".to_string();
	    		}
	    	},
	    	Err(_) => {
	    		// no previous transaction, free to continue.
	    	}
	    }

	 	if balance < min_ust_balance || balance < max_tx_fee {
	 		return "Insufficient UST balance, replenish your account!".to_string();
	 	}
	 	match estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"staking","loan_amount","date_next",2).await.as_ref(){
	 		"now" => {},
	 		 _ => {return "waiting..".to_string()}
	 	};

	
		let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"gas_adjustment_preference",false, 10).await.as_ref());

	    let anc_to_claim = decimal_or_return!(borrower_rewards_to_string(tasks.clone(), true,0).await.as_ref());
        let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

	    match anchor_governance_claim_and_stake(&mnemonics, anc_to_claim, gas_fees_uusd, max_tx_fee, gas_adjustment_preference, only_estimate).await {
        	Ok(msg) => {
        		register_value(&tasks,"anchor_governance_claim_and_stake".to_string(),msg.to_owned()).await;
        		register_value(&tasks,"latest_transaction".to_string(),msg.to_owned()).await;
        		return msg;
        	},
        	Err(msg) => {
        		register_value(&tasks,"anchor_governance_claim_and_stake".to_string(),msg.to_string().to_owned()).await;
        		register_value(&tasks,"latest_transaction".to_string(),msg.to_string().to_owned()).await;
        		return msg.to_string();
        	}
        }
}
 
pub async fn anchor_borrow_claim_rewards(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {
         
        let max_tx_fee = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"max_tx_fee",false, 4).await.as_ref());
		 
		let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"gas_adjustment_preference",false, 10).await.as_ref());
  
        let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());
		
		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

        match anchor_claim_rewards(&mnemonics, gas_fees_uusd, max_tx_fee, gas_adjustment_preference,only_estimate).await {
        	Ok(msg) => {
        		return msg;
        	},
        	Err(msg) => {
        		return msg.to_string();
        	}
        }
}
pub async fn anchor_governance_stake_balance(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  wallet_acc_address: Arc<SecUtf8>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {
        
        let max_tx_fee = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"max_tx_fee",false, 4).await.as_ref());

		let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(tasks.clone(),"gas_adjustment_preference",false, 10).await.as_ref());
	 
		let anc_balance = decimal_or_return!(borrower_anc_deposited_to_string(tasks.clone(), true,0).await.as_ref());
  
		let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

		let mnemonics = match only_estimate {
			true => {wallet_acc_address.unsecure().to_string()},
			false => {decrypt_text_with_secret(&wallet_seed_phrase)}
		};

	    match anchor_governance_stake(&mnemonics, anc_balance, gas_fees_uusd, max_tx_fee, gas_adjustment_preference,only_estimate).await {
	    	Ok(msg) => {
	    		return msg;
	    	},
	    	Err(msg) => {
	    		return msg.to_string();
	    	}
	    }  
} 