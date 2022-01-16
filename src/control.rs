#![allow(dead_code)]

pub mod view;
use secstr::*;

use rust_decimal::Decimal;
use core::str::FromStr;

use view::interface::model::smart_contracts::meta::{anchor_redeem_and_repay_stable_tx, anchor_claim_rewards,anchor_governance_stake,anchor_governance_claim_and_stake};
use view::interface::model::{MaybeOrPromise,get_meta_data_maybe_or_await_task};

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


pub async fn anchor_reedem_stable(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_seed_phrase: &SecUtf8, only_estimate: bool) -> String {
    // ust balance is needed, no way to do anything otherwise.
    let mut ust_amount_liquid = decimal_or_return!(terra_balance_to_string(tasks.clone(),"uusd",true,0).await.as_ref());

    let min_ust_balance = decimal_or_return!(min_ust_balance_to_string(tasks.clone(),true,0).await.as_ref());
    ust_amount_liquid = ust_amount_liquid.checked_sub(min_ust_balance).unwrap();      

    let repay_amount = decimal_or_return!(calculate_repay_amount(tasks.clone(),true,0).await.as_ref());
    
    let zero = Decimal::from_str("0").unwrap();
    let further_funds_needed = ust_amount_liquid.checked_sub(repay_amount).unwrap() < zero;

    let a_ust_deposit_liquid = match borrower_ust_deposited_to_string(tasks.clone(),true,0).await.as_ref() {
        "--" => {
            Decimal::from_str("0").unwrap()
        },
        e => {
            Decimal::from_str(e).unwrap()
        }
    };

    let sufficient_funds_available = a_ust_deposit_liquid.checked_add(ust_amount_liquid).unwrap().checked_sub(repay_amount).unwrap() >= zero;

    let mut to_withdraw_from_account = Decimal::from_str("0").unwrap();
    let mut to_withdraw_from_deposit = Decimal::from_str("0").unwrap();

    if ust_amount_liquid >= repay_amount || (ust_amount_liquid > zero && a_ust_deposit_liquid <= zero) { 

        // case only use UST balance
        // either because UST balance is sufficient or because there are still UST available but no aUST to withdraw.
        to_withdraw_from_account = repay_amount;  

    }else if a_ust_deposit_liquid > zero  { 
        // case use both UST balance and aUST withdrawal
        // there are still UST available and aUST to withdraw.

        // also matches case only use aUST withdrawal 
        if ust_amount_liquid > zero {
            to_withdraw_from_account = ust_amount_liquid;  
        }

        let a_ust_liquidity_needed = repay_amount.checked_sub(ust_amount_liquid).unwrap();
        if a_ust_liquidity_needed <= a_ust_deposit_liquid {
            to_withdraw_from_deposit = a_ust_liquidity_needed;
        }else{ 
            to_withdraw_from_deposit = a_ust_deposit_liquid; 
        }
    }
  
    let to_repay = to_withdraw_from_account.checked_add(to_withdraw_from_deposit).unwrap();
 
    let tax_rate = decimal_or_return!(tax_rate_to_string(tasks.clone(),10).await.as_ref());

    let uusd_tax_cap = decimal_or_return!(uusd_tax_cap_to_string(tasks.clone(),true,0).await.as_ref());

    let mut tx = to_repay.checked_mul(tax_rate).unwrap();
    if tx > uusd_tax_cap {
        tx = uusd_tax_cap;
    }

	let mut max_gas_adjustment = decimal_or_return!(max_gas_adjustment_to_string(tasks.clone(),10).await.as_ref());

	let mut avg_gas_adjustment = Decimal::from_str("0").unwrap();

	match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_redeem_stable","avg_gas_adjustment".to_owned(),false,4).await.as_ref() {
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

	match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_repay_stable","avg_gas_adjustment_for_stability_fee_case".to_owned(),false,4).await.as_ref() {
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

	let exchange_rate = decimal_or_return!(a_terra_exchange_rate_to_string(tasks.clone(),10).await.as_ref());

    to_withdraw_from_deposit = to_withdraw_from_deposit.checked_div(exchange_rate).unwrap().round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

    let micro = Decimal::from_str("1000000").unwrap();
	let est_auto_repay_fees = match estimate_anchor_protocol_auto_repay_tx_fee(tasks.clone(),2).await.as_ref() {
        "--" => {
            return "--".to_string();
        },
        e => {
            Decimal::from_str(e).unwrap().checked_mul(micro).unwrap()
        }
    };

    let to_repay = to_repay.checked_sub(est_auto_repay_fees).unwrap().round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

    if to_withdraw_from_deposit > zero {
		/*
        match anchor_reedem_stable_tx(wallet_seed_phrase.unsecure(),to_withdraw_from_deposit,gas_fees_uusd, max_gas_adjustment, true).await {
        	Ok(msg) => {
        		return msg;
        	},
        	Err(msg) => {
        		return msg.to_string();
        	}
        }*/

        match anchor_redeem_and_repay_stable_tx(wallet_seed_phrase.unsecure(),to_withdraw_from_deposit,to_repay,gas_fees_uusd,  Decimal::from_str("5").unwrap(), max_gas_adjustment,  Decimal::from_str("5").unwrap(), true).await {
        	Ok(msg) => {
        		return msg;
        	},
        	Err(msg) => {
        		return msg.to_string();
        	}
        }        
	}
    "nothing".to_string()
}


pub async fn anchor_borrow_claim_and_stake_rewards(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_seed_phrase: &SecUtf8, only_estimate: bool) -> String {
		let max_tx_fee = decimal_or_return!(max_tx_fee_to_string(tasks.clone(), 4).await.as_ref());
		let mut max_gas_adjustment = decimal_or_return!(max_gas_adjustment_to_string(tasks.clone(),10).await.as_ref());
		
		let mut avg_gas_adjustment = Decimal::from_str("0").unwrap();

		match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_adjustment".to_owned(),false,4).await.as_ref() {
			"--" => {
			},
			e => {
				avg_gas_adjustment = Decimal::from_str(e).unwrap();
			}
		};

		let mut avg_tx_fee = Decimal::from_str("250657").unwrap();

		match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_fee_amount".to_owned(),true,0).await.as_ref() {
			"--" => { 
			},
			e => {
				avg_tx_fee = Decimal::from_str(e).unwrap();
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
 
		  
 
		match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking","avg_fee_amount".to_owned(),true,0).await.as_ref() {
			"--" => {
				avg_tx_fee = avg_tx_fee
							.checked_add(Decimal::from_str("250657").unwrap()).unwrap();
			},
			e => {
				avg_tx_fee = avg_tx_fee
							.checked_add(Decimal::from_str(e).unwrap()).unwrap();
			}
		};

		let anc_to_claim = decimal_or_return!(borrower_rewards_to_string(tasks.clone(), true,0).await.as_ref());
		
	    //println!("{:?}",(anc_to_claim, avg_tx_fee, max_gas_adjustment) ); 

        let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

	    match anchor_governance_claim_and_stake(wallet_seed_phrase.unsecure(),anc_to_claim,gas_fees_uusd, avg_tx_fee, max_gas_adjustment, max_tx_fee, only_estimate).await {
        	Ok(msg) => {
        		return msg;
        	},
        	Err(msg) => {
        		return msg.to_string();
        	}
        }
}
 
pub async fn anchor_borrow_claim_rewards(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  wallet_seed_phrase: &SecUtf8, only_estimate: bool) -> String {
         
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
 
		let avg_tx_fee = match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_fee_amount".to_owned(),true,0).await.as_ref() {
			"--" => {
				Decimal::from_str("250657").unwrap() // 0.25 UST 
			},
			e => {
				Decimal::from_str(e).unwrap()
			}
		};
  
        let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

        match anchor_claim_rewards(wallet_seed_phrase.unsecure(),gas_fees_uusd, avg_tx_fee, max_gas_adjustment,max_tx_fee,only_estimate).await {
        	Ok(msg) => {
        		return msg;
        	},
        	Err(msg) => {
        		return msg.to_string();
        	}
        }
}
pub async fn anchor_governance_stake_balance(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  wallet_seed_phrase: &SecUtf8, only_estimate: bool) -> String {
        
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
 
		let avg_tx_fee = match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking","avg_fee_amount".to_owned(),true,0).await.as_ref() {
			"--" => {
				Decimal::from_str("250657").unwrap() // 0.25 UST 
			},
			e => {
				Decimal::from_str(e).unwrap()  
			}
		};

		let anc_balance = decimal_or_return!(borrower_anc_deposited_to_string(tasks.clone(), true,0).await.as_ref());
  
		let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

	    match anchor_governance_stake(wallet_seed_phrase.unsecure(),anc_balance, gas_fees_uusd, avg_tx_fee, max_gas_adjustment,max_tx_fee,only_estimate).await {
	    	Ok(msg) => {
	    		return msg;
	    	},
	    	Err(msg) => {
	    		return msg.to_string();
	    	}
	    }  
} 