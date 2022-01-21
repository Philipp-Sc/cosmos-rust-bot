#![allow(dead_code)]

pub mod view;
use secstr::*;

use rust_decimal::Decimal;
use core::str::FromStr;

use view::interface::model::services::blockchain::smart_contracts::objects::meta::{
	anchor_redeem_and_repay_stable_tx, 
	anchor_repay_stable_tx, 
	anchor_claim_rewards,
	anchor_governance_stake,
	anchor_governance_claim_and_stake};

use view::interface::model::{
	MaybeOrPromise,
	register_value,
	get_meta_data_maybe,
	get_oldest_timestamps_of_resolved_tasks};

use view::interface::*;
use view::*;

use std::collections::HashMap; 


use std::sync::Arc; 
use tokio::sync::RwLock; 

macro_rules! decimal_or_return {
    ( $e:expr ) => {
        match $e {
            "--" => return String::from("--"),
            e => Decimal::from_str(e).unwrap(),
        }
    }
} 

pub async fn anchor_redeem_and_repay_stable(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {
    let max_tx_fee = decimal_or_return!(max_tx_fee_to_string(tasks.clone(), 4).await.as_ref());
	let likely_tx_fee = decimal_or_return!(estimate_anchor_protocol_auto_repay_tx_fee(tasks.clone(),2).await.as_ref());
 	let balance = decimal_or_return!(terra_balance_to_string(tasks.clone(),"uusd",false,2).await.as_ref());
 	let min_ust_balance = decimal_or_return!(min_ust_balance_to_string(tasks.clone(),false,2).await.as_ref());

 	if balance < min_ust_balance || balance < max_tx_fee || balance < likely_tx_fee {
 		return "Insufficient UST balance, replenish your account!".to_string();
 	}

    let zero = Decimal::from_str("0").unwrap(); 
    let micro = Decimal::from_str("1000000").unwrap();
    
    let exchange_rate = decimal_or_return!(a_terra_exchange_rate_to_string(tasks.clone(),10).await.as_ref());
   
	let to_withdraw_from_deposit = decimal_or_return!(calculate_repay_plan(tasks.clone(),"to_withdraw_from_deposit",2).await.as_ref())
								   .checked_div(exchange_rate).unwrap()
								   .checked_mul(micro).unwrap()
								   .round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

    let to_repay = decimal_or_return!(calculate_repay_plan(tasks.clone(),"to_repay",2).await.as_ref())
    			   .checked_mul(micro).unwrap()
				   .round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

    match check_anchor_loan_status(tasks.clone(),2).await.as_ref() {
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
    		if get_oldest_timestamps_of_resolved_tasks(&tasks,&req).await <= maybe.timestamp { 
    			return "waiting for data to refresh..".to_string();
    		}
    	},
    	Err(err) => {
    		// no previous transaction, free to continue.
    	}
    }

    if to_withdraw_from_deposit > zero && to_repay > zero {
  

		let mut max_gas_adjustment = decimal_or_return!(max_gas_adjustment_to_string(tasks.clone(),10).await.as_ref());
		let mut avg_gas_adjustment = Decimal::from_str("0").unwrap();

		match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_redeem_stable","avg_gas_adjustment".to_owned(),false,4).await.as_ref() {
			"--" => {
			},
			e => {
				avg_gas_adjustment = Decimal::from_str(e).unwrap();
			}
		};
 
		match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_repay_stable","avg_gas_adjustment".to_owned(),false,4).await.as_ref() {
			"--" => {
				avg_gas_adjustment = max_gas_adjustment
									 .checked_add(avg_gas_adjustment).unwrap();
			},
			e => {
				let gas_adjustment = Decimal::from_str(e).unwrap(); 
				avg_gas_adjustment = gas_adjustment
									 .checked_add(avg_gas_adjustment).unwrap() 
			}
		};
		avg_gas_adjustment = avg_gas_adjustment.checked_div(Decimal::from_str("2").unwrap()).unwrap();

		if avg_gas_adjustment < max_gas_adjustment {
			max_gas_adjustment = avg_gas_adjustment;
		}

		let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());
		max_gas_adjustment = max_gas_adjustment
	            					 .checked_add(gas_adjustment_preference).unwrap()
	            					 .checked_div(Decimal::from_str("2").unwrap()).unwrap();   

        match anchor_redeem_and_repay_stable_tx(wallet_seed_phrase.unsecure(),to_withdraw_from_deposit,to_repay,gas_fees_uusd,  max_tx_fee, max_gas_adjustment, only_estimate).await {
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

		let mut max_gas_adjustment = decimal_or_return!(max_gas_adjustment_to_string(tasks.clone(),10).await.as_ref());
		let mut avg_gas_adjustment = Decimal::from_str("0").unwrap();
 
		match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_repay_stable","avg_gas_adjustment".to_owned(),false,4).await.as_ref() {
			"--" => {
				avg_gas_adjustment = max_gas_adjustment
									 .checked_add(avg_gas_adjustment).unwrap();
			},
			e => {
				let gas_adjustment = Decimal::from_str(e).unwrap(); 
				avg_gas_adjustment = gas_adjustment
									 .checked_add(avg_gas_adjustment).unwrap() 
			}
		}; 

		if avg_gas_adjustment < max_gas_adjustment {
			max_gas_adjustment = avg_gas_adjustment;
		}

		let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());
		max_gas_adjustment = max_gas_adjustment
	            					 .checked_add(gas_adjustment_preference).unwrap()
	            					 .checked_div(Decimal::from_str("2").unwrap()).unwrap();   

        match anchor_repay_stable_tx(wallet_seed_phrase.unsecure(), to_repay, gas_fees_uusd, max_tx_fee, max_gas_adjustment, only_estimate).await {
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


pub async fn anchor_borrow_claim_and_stake_rewards(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {
		let max_tx_fee = decimal_or_return!(max_tx_fee_to_string(tasks.clone(), 4).await.as_ref());
		let likely_tx_fee = decimal_or_return!(estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),2).await.as_ref());
	 	let balance = decimal_or_return!(terra_balance_to_string(tasks.clone(),"uusd",false,2).await.as_ref());
	 	let min_ust_balance = decimal_or_return!(min_ust_balance_to_string(tasks.clone(),false,2).await.as_ref());

	 	// making sure the data is not outdated
	    match get_meta_data_maybe(&tasks, "latest_transaction").await {
	    	Ok(maybe) => {
	    		let req: [&str;3] = ["terra_balances","borrow_limit","borrow_info"];
	    		if get_oldest_timestamps_of_resolved_tasks(&tasks,&req).await <= maybe.timestamp { 
	    			return "waiting for data to refresh..".to_string();
	    		}
	    	},
	    	Err(err) => {
	    		// no previous transaction, free to continue.
	    	}
	    }

	 	if balance < min_ust_balance || balance < max_tx_fee || balance < likely_tx_fee {
	 		return "Insufficient UST balance, replenish your account!".to_string();
	 	}
	 	match estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","date_next",2).await.as_ref(){
	 		"now" => {},
	 		 _ => {return "waiting..".to_string()}
	 	};

		let mut max_gas_adjustment = decimal_or_return!(max_gas_adjustment_to_string(tasks.clone(),10).await.as_ref());
		let mut avg_gas_adjustment = Decimal::from_str("0").unwrap();



		match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_adjustment".to_owned(),false,4).await.as_ref() {
			"--" => {
			},
			e => {
				avg_gas_adjustment = Decimal::from_str(e).unwrap();
			}
		};
 
		match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking","avg_gas_adjustment".to_owned(),false,4).await.as_ref() {
			"--" => {
				avg_gas_adjustment = max_gas_adjustment
									 .checked_add(avg_gas_adjustment).unwrap();
			},
			e => {
				let gas_adjustment = Decimal::from_str(e).unwrap(); 
				avg_gas_adjustment = gas_adjustment
									 .checked_add(avg_gas_adjustment).unwrap() 
			}
		};
		avg_gas_adjustment = avg_gas_adjustment.checked_div(Decimal::from_str("2").unwrap()).unwrap();

		if avg_gas_adjustment < max_gas_adjustment {
			max_gas_adjustment = avg_gas_adjustment;
		}

		let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());
		max_gas_adjustment = max_gas_adjustment
	            					 .checked_add(gas_adjustment_preference).unwrap()
	            					 .checked_div(Decimal::from_str("2").unwrap()).unwrap();   

	    let anc_to_claim = decimal_or_return!(borrower_rewards_to_string(tasks.clone(), true,0).await.as_ref());
        let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());


	    match anchor_governance_claim_and_stake(wallet_seed_phrase.unsecure(),anc_to_claim,gas_fees_uusd, max_tx_fee, max_gas_adjustment, only_estimate).await {
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
 
pub async fn anchor_borrow_claim_rewards(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {
         
        let max_tx_fee = decimal_or_return!(max_tx_fee_to_string(tasks.clone(), 4).await.as_ref());
		let mut max_gas_adjustment = decimal_or_return!(max_gas_adjustment_to_string(tasks.clone(),10).await.as_ref());
		
		match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_adjustment".to_owned(),false,4).await.as_ref() {
			"--" => {
			},
			e => {
				let gas_adjustment = Decimal::from_str(e).unwrap();
				if gas_adjustment < max_gas_adjustment {
					max_gas_adjustment = gas_adjustment;
				}
			}
		};

		let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());
		max_gas_adjustment = max_gas_adjustment
	            					 .checked_add(gas_adjustment_preference).unwrap()
	            					 .checked_div(Decimal::from_str("2").unwrap()).unwrap();   
  
        let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

        match anchor_claim_rewards(wallet_seed_phrase.unsecure(),gas_fees_uusd, max_tx_fee, max_gas_adjustment,only_estimate).await {
        	Ok(msg) => {
        		return msg;
        	},
        	Err(msg) => {
        		return msg.to_string();
        	}
        }
}
pub async fn anchor_governance_stake_balance(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  wallet_seed_phrase: Arc<SecUtf8>, only_estimate: bool) -> String {
        
        let max_tx_fee = decimal_or_return!(max_tx_fee_to_string(tasks.clone(), 4).await.as_ref());
		let mut max_gas_adjustment = decimal_or_return!(max_gas_adjustment_to_string(tasks.clone(),10).await.as_ref());
		
		match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking","avg_gas_adjustment".to_owned(),false,4).await.as_ref() {
			"--" => {
			},
			e => {
				let gas_adjustment = Decimal::from_str(e).unwrap();
				if gas_adjustment < max_gas_adjustment {
					max_gas_adjustment = gas_adjustment;
				}
			}
		};

		let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());
		max_gas_adjustment = max_gas_adjustment
	            					 .checked_add(gas_adjustment_preference).unwrap()
	            					 .checked_div(Decimal::from_str("2").unwrap()).unwrap();   
 

		let anc_balance = decimal_or_return!(borrower_anc_deposited_to_string(tasks.clone(), true,0).await.as_ref());
  
		let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

	    match anchor_governance_stake(wallet_seed_phrase.unsecure(),anc_balance, gas_fees_uusd, max_tx_fee, max_gas_adjustment,only_estimate).await {
	    	Ok(msg) => {
	    		return msg;
	    	},
	    	Err(msg) => {
	    		return msg.to_string();
	    	}
	    }  
} 