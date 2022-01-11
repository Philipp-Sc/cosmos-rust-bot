#![allow(dead_code)]

pub mod view;
use secstr::*;


use rust_decimal::Decimal;
use core::str::FromStr;

use view::simple_view::model::smart_contracts::meta::api::{anchor_claim_rewards,anchor_governance_stake,anchor_governance_claim_and_stake};
use view::simple_view::model::{MaybeOrPromise,get_meta_data_maybe_or_await_task};

use view::simple_view::*;
use view::*;

use std::collections::HashMap; 


use std::sync::Arc; 
use tokio::sync::RwLock; 



pub async fn anchor_borrow_claim_and_stake_rewards(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, wallet_seed_phrase: &SecUtf8, only_estimate: bool) -> String {
 

		let mut max_gas_adjustment = Decimal::from_str("1.67").unwrap();

		match get_meta_data_maybe_or_await_task(&tasks,"max_gas_adjustment").await {
	        Ok(response_result) => { 
	            max_gas_adjustment = Decimal::from_str(response_result.as_str()).unwrap();             
	        },
	        Err(_) => {
        	}
		}


		let mut max_tx_fee = Decimal::from_str("5").unwrap();

		match max_tx_fee_to_string(tasks.clone(), 4).await.as_ref() {
			"--" => {
			},
			e => {
				max_tx_fee = Decimal::from_str(e).unwrap();
			}
		};

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

		match get_meta_data_maybe_or_await_task(&tasks,"gas_adjustment_preference").await {
	        Ok(response_result) => { 
	            let gas_adjustment_preference = Decimal::from_str(response_result.as_str()).unwrap();       
	            max_gas_adjustment = max_gas_adjustment
	            					 .checked_add(gas_adjustment_preference).unwrap()
	            					 .checked_div(Decimal::from_str("2").unwrap()).unwrap();    
	        },
	        Err(_) => {
        	}
		}
 
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

		let mut _anc_to_claim = Decimal::from_str("0").unwrap();

		match borrower_rewards_to_string(tasks.clone(), true,0).await.as_ref() {
			"--" => {
				return "Could not get pending ANC rewards: Likely no ANC available.".to_string();
			},
			e => {
				_anc_to_claim = Decimal::from_str(e).unwrap() 
			}
		};

	   //println!("{:?}",(anc_to_claim, avg_tx_fee, max_gas_adjustment) ); 
	   match get_meta_data_maybe_or_await_task(&tasks,"gas_fees_uusd").await {
            Ok(response_result) => { 
                let gas_fees_uusd = Decimal::from_str(response_result.as_str()).unwrap();   

                match anchor_governance_claim_and_stake(wallet_seed_phrase.unsecure(),_anc_to_claim,gas_fees_uusd, avg_tx_fee, max_gas_adjustment, max_tx_fee, only_estimate).await {
		        	Ok(msg) => {
		        		return msg;
		        	},
		        	Err(msg) => {
		        		return msg.to_string();
		        	}
		        }
            },
            Err(err) => {
            	return format!("Unexpected Error: {:?}", err);
            }
        } 
}
 
pub async fn anchor_borrow_claim_rewards(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  wallet_seed_phrase: &SecUtf8, only_estimate: bool) -> String {

		let mut max_gas_adjustment = Decimal::from_str("1.67").unwrap();
		
		match get_meta_data_maybe_or_await_task(&tasks,"max_gas_adjustment").await {
	        Ok(response_result) => { 
	            max_gas_adjustment = Decimal::from_str(response_result.as_str()).unwrap();             
	        },
	        Err(_) => {
        	}
		}

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

		match get_meta_data_maybe_or_await_task(&tasks,"gas_adjustment_preference").await {
	        Ok(response_result) => { 
	            let gas_adjustment_preference = Decimal::from_str(response_result.as_str()).unwrap();       
	            max_gas_adjustment = max_gas_adjustment
	            					 .checked_add(gas_adjustment_preference).unwrap()
	            					 .checked_div(Decimal::from_str("2").unwrap()).unwrap();    
	        },
	        Err(_) => {
        	}
		}
		  
		let avg_tx_fee = match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_fee_amount".to_owned(),true,0).await.as_ref() {
			"--" => {
				Decimal::from_str("250657").unwrap() // 0.25 UST 
			},
			e => {
				Decimal::from_str(e).unwrap()
			}
		};
  
	   match get_meta_data_maybe_or_await_task(&tasks,"gas_fees_uusd").await {
            Ok(response_result) => { 
                let gas_fees_uusd = Decimal::from_str(response_result.as_str()).unwrap();   

                match anchor_claim_rewards(wallet_seed_phrase.unsecure(),gas_fees_uusd, avg_tx_fee, max_gas_adjustment,only_estimate).await {
		        	Ok(msg) => {
		        		return msg;
		        	},
		        	Err(msg) => {
		        		return msg.to_string();
		        	}
		        }
            },
            Err(err) => {
            	return format!("Unexpected Error: {:?}", err);
            }
        }
    
}
pub async fn anchor_governance_stake_balance(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  wallet_seed_phrase: &SecUtf8, only_estimate: bool) -> String {

		let mut max_gas_adjustment = Decimal::from_str("1.67").unwrap();
		
		match get_meta_data_maybe_or_await_task(&tasks,"max_gas_adjustment").await {
	        Ok(response_result) => { 
	            max_gas_adjustment = Decimal::from_str(response_result.as_str()).unwrap();             
	        },
	        Err(_) => {
        	}
		}

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

		match get_meta_data_maybe_or_await_task(&tasks,"gas_adjustment_preference").await {
	        Ok(response_result) => { 
	            let gas_adjustment_preference = Decimal::from_str(response_result.as_str()).unwrap();       
	            max_gas_adjustment = max_gas_adjustment
	            					 .checked_add(gas_adjustment_preference).unwrap()
	            					 .checked_div(Decimal::from_str("2").unwrap()).unwrap();    
	        },
	        Err(_) => {
        	}
		}
 
		let avg_tx_fee = match estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking","avg_fee_amount".to_owned(),true,0).await.as_ref() {
			"--" => {
				Decimal::from_str("250657").unwrap() // 0.25 UST 
			},
			e => {
				Decimal::from_str(e).unwrap()  
			}
		};


		let mut _anc_balance = Decimal::from_str("0").unwrap();

		match borrower_anc_deposited_to_string(tasks.clone(), true,0).await.as_ref() {
			"--" => {
				return "Could not get ANC balance: Likely no ANC available.".to_string();
			},
			e => {
				_anc_balance = Decimal::from_str(e).unwrap()  
			}
		};
  
	    match get_meta_data_maybe_or_await_task(&tasks,"gas_fees_uusd").await {
            Ok(response_result) => { 
                let gas_fees_uusd = Decimal::from_str(response_result.as_str()).unwrap();   
 

                match anchor_governance_stake(wallet_seed_phrase.unsecure(),_anc_balance, gas_fees_uusd, avg_tx_fee, max_gas_adjustment,only_estimate).await {
		        	Ok(msg) => {
		        		return msg;
		        	},
		        	Err(msg) => {
		        		return msg.to_string();
		        	}
		        }
            },
            Err(err) => {
            	return format!("Unexpected Error: {:?}", err);
            }
        }
    
} 