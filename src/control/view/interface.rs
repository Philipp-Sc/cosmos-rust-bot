use model::{MaybeOrPromise,get_data_maybe_or_await_task,get_meta_data_maybe_or_await_task};  

use std::collections::HashMap;

use rust_decimal::Decimal;
use core::str::FromStr; 
   
use num_format::{Locale, ToFormattedString}; 
  

use std::sync::Arc; 
use tokio::sync::RwLock; 

pub mod model;
 

pub async fn tax_rate_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_data_maybe_or_await_task(&tasks,"tax_rate").await {
        Ok(response_result) => { 
            return Decimal::from_str(response_result.as_tax_rate().unwrap().result.as_str()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .to_string();             
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn uusd_tax_cap_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,as_micro:bool, digits_rounded_to: u32) -> String { 
    match get_data_maybe_or_await_task(&tasks,"tax_caps").await {
        Ok(response_result) => { 
            let vec_tax_caps = &response_result.as_tax_caps().unwrap().result;
            for tax_cap in vec_tax_caps {
                if tax_cap.denom == "uusd".to_string() {
                    if as_micro {
                        return Decimal::from_str(tax_cap.tax_cap.as_str()).unwrap()
                        .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                        .to_string();     
                    }else {
                        return Decimal::from_str(tax_cap.tax_cap.as_str()).unwrap()
                        .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()  
                        .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                        .to_string();    
                    }
                }
            }                   
        },
        Err(_) => {
            return "--".to_string();
        }
    }
    return "--".to_string();
}


pub async fn min_ust_balance_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, as_micro: bool, digits_rounded_to: u32) -> String { 
    match get_meta_data_maybe_or_await_task(&tasks,"min_ust_balance").await {
        Ok(response_result) => { 
             if as_micro {
                let micro = Decimal::from_str("1000000").unwrap();
                return Decimal::from_str(response_result.as_str()).unwrap().checked_mul(micro).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .to_string();  
            }
            return Decimal::from_str(response_result.as_str()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .to_string();             
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}


pub async fn gas_adjustment_preference_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_meta_data_maybe_or_await_task(&tasks,"gas_adjustment_preference").await {
        Ok(response_result) => { 
            return Decimal::from_str(response_result.as_str()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .to_string();             
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn max_gas_adjustment_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_meta_data_maybe_or_await_task(&tasks,"max_gas_adjustment").await {
        Ok(response_result) => { 
            return Decimal::from_str(response_result.as_str()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .to_string();             
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn target_percentage_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_meta_data_maybe_or_await_task(&tasks,"target_percentage").await {
        Ok(response_result) => { 
            return Decimal::from_str(response_result.as_str()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .to_string();             
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn borrow_percentage_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_meta_data_maybe_or_await_task(&tasks,"borrow_percentage").await {
        Ok(response_result) => { 
            return Decimal::from_str(response_result.as_str()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .to_string();             
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn trigger_percentage_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_meta_data_maybe_or_await_task(&tasks,"trigger_percentage").await {
        Ok(response_result) => { 
            return Decimal::from_str(response_result.as_str()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .to_string();             
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn max_tx_fee_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_meta_data_maybe_or_await_task(&tasks,"max_tx_fee").await {
        Ok(response_result) => { 
            return Decimal::from_str(response_result.as_str()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .to_string();             
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn anc_staked_balance_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_data_maybe_or_await_task(&tasks,"staker").await {
        Ok(response_result) => {
            let balance = response_result.as_staker().unwrap().result.balance; 
            let balance = Decimal::from_str(balance.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            return balance.checked_div(micro).unwrap()
                   .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                   .to_string();
            
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn borrower_rewards_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, as_micro: bool, digits_rounded_to: u32) -> String { 
    match get_data_maybe_or_await_task(&tasks,"borrow_info").await {
        Ok(response_result) => {
            let pending_rewards = response_result.as_borrow_info().unwrap().result.pending_rewards; 
            let pending_rewards = Decimal::from_str(pending_rewards.to_string().as_str()).unwrap();
            let mut _micro = Decimal::from_str("1").unwrap();
            if !as_micro {
                _micro = Decimal::from_str("1000000").unwrap();
                return pending_rewards.checked_div(_micro).unwrap()
                       .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                       .to_string();                
            }else{
                return pending_rewards.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero)
                       .to_string();
            }
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}


pub async fn borrower_anc_deposited_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, as_micro: bool, digits_rounded_to: u32) -> String { 
    match get_data_maybe_or_await_task(&tasks,"anc_balance").await {
        Ok(response_result) => { 
            let mut balance = Decimal::from_str(response_result.as_balance().unwrap().result.balance.to_string().as_str()).unwrap();
            let mut micro = Decimal::from_str("1").unwrap();
            if !as_micro {
                micro = Decimal::from_str("1000000").unwrap();                
            }
            balance = balance.checked_div(micro).unwrap();
            return balance
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                .to_string();
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn borrower_balance_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_data_maybe_or_await_task(&tasks,"balance").await {
        Ok(response_result) => {
            let balance = response_result.as_balance().unwrap().result.balance; 
            let balance = Decimal::from_str(balance.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            return balance.checked_div(micro).unwrap()
                   .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                   .to_string();
            
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn terra_balance_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, denom: &str, as_micro: bool, digits_rounded_to: u32) -> String { 
    match get_data_maybe_or_await_task(&tasks,"terra_balances").await {
        Ok(response_result) => {
            let vector_balances = &response_result.as_balances().unwrap().result; 
            for balance in vector_balances {
                if &balance.denom == denom {  
                    let balance = Decimal::from_str(balance.amount.as_str()).unwrap(); 
                    if !as_micro {
                        let micro = Decimal::from_str("1000000").unwrap();    
                        return balance.checked_div(micro).unwrap()
                           .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                           .to_string();            
                    }                   
                    return balance.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero)
                            .to_string();
                } 
            }            
        },
        Err(_) => {
            return "--".to_string();
        }
    }
    return "0".to_string();
}

pub async fn borrower_loan_amount_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_data_maybe_or_await_task(&tasks,"borrow_info").await {
        Ok(response_result) => {
            let loan_amount = response_result.as_borrow_info().unwrap().result.loan_amount; 
            let loan_amount = Decimal::from_str(loan_amount.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            return loan_amount.checked_div(micro).unwrap()
                   .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                   .to_string();
            
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn borrow_limit_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    match get_data_maybe_or_await_task(&tasks,"borrow_limit").await {
        Ok(response_result) => {
            let borrow_limit = response_result.as_borrow_limit().unwrap().result.borrow_limit; 
            let borrow_limit = Decimal::from_str(borrow_limit.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            return borrow_limit.checked_div(micro).unwrap()
                   .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                   .to_string();
            
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}

pub async fn earn_apr_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
     
        match get_data_maybe_or_await_task(&tasks,"earn_apy").await {
            Ok(response_result) => {
                let apy: rust_decimal::Decimal = response_result.as_earn_apy().unwrap().result.apy; 
                return format!("{}%",
                    apy
                    .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string()
                    );
            },
            Err(_) => {
                return "--".to_string();
            }
        } 
}

pub async fn distribution_apr_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,"api/v2/distribution-apy").await {
            Ok(response_result) => {
                let distribution_apr: cosmwasm_std::Decimal = response_result.as_distribution_apy().unwrap().distribution_apy; 
                return format!("{}%",Decimal::from_str(distribution_apr.to_string().as_str()).unwrap().checked_mul(Decimal::from_str("100").unwrap()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string());
            },
            Err(_) => {
                return "--".to_string();
            }
        }
}

pub async fn gas_price_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
          match get_meta_data_maybe_or_await_task(&tasks,"gas_fees_uusd").await {
                    Ok(response_result) => { 
                        return Decimal::from_str(response_result.as_str()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();             
                    },
                    Err(_) => {
                        return "--".to_string();
                    }
                }
}

pub async fn staking_apy_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,"api/v2/gov-reward").await {
            Ok(response_result) => {
                let current_apy: cosmwasm_std::Decimal = response_result.as_gov_reward().unwrap().current_apy; 
                return format!("{}%",Decimal::from_str(current_apy.to_string().as_str()).unwrap().checked_mul(Decimal::from_str("100").unwrap()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string());
            },
            Err(_) => {
                return "--".to_string();
            }
        }
}

pub async fn interest_multiplier_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,"config anchorprotocol mmInterestModel").await {
            Ok(response_result) => {
                let interest_multiplier: cosmwasm_std::Decimal256 = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.interest_multiplier; 
                return Decimal::from_str(interest_multiplier.to_string().as_str()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
            },
            Err(_) => {
                return "--".to_string();
            }
        }
}

pub async fn blocks_per_year_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,"blocks_per_year").await {
            Ok(response_result) => {
                let blocks_per_year = Decimal::from_str(response_result.as_blocks().unwrap().result.blocks_per_year.to_string().as_str()).unwrap();
                return blocks_per_year.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
            },
            Err(_) => {
                return "--".to_string();
            }
        }
}    

pub async fn base_rate_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,"config anchorprotocol mmInterestModel").await {
            Ok(response_result) => {
                let base_rate: cosmwasm_std::Decimal256 = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.base_rate; 
                return Decimal::from_str(base_rate.to_string().as_str()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
            },
            Err(_) => {
                return "--".to_string();
            }
        }
}


pub async fn a_terra_supply_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,"epoch_state anchorprotocol mmMarket").await {
            Ok(response_result) => {
                let exchange_rate: cosmwasm_std::Decimal256 = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate; 
                let supply: cosmwasm_std::Uint256 = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.aterra_supply; 
                let micro: cosmwasm_std::Uint256 = cosmwasm_std::Uint256::from_str("1000000").unwrap();
                let supply = Decimal::from_str((supply / (micro * exchange_rate)).to_string().as_str()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
                return supply.parse::<u128>().unwrap().to_formatted_string(&Locale::en);
            },
            Err(_) => {
                return "--".to_string();
            }
        }
}

pub async fn a_terra_exchange_rate_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,"epoch_state anchorprotocol mmMarket").await {
            Ok(response_result) => {
                let exchange_rate: cosmwasm_std::Decimal256 = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate; 
                return Decimal::from_str(exchange_rate.to_string().as_str()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
            },
            Err(_) => {
                return "--".to_string();
            }
        }
}

pub async fn b_luna_exchange_rate_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,"state anchorprotocol bLunaHub").await {
            Ok(response_result) => {
                let exchange_rate: cosmwasm_std::Decimal = response_result.as_state().unwrap().as_b_luna_hub().unwrap().result.bluna_exchange_rate; 
                return Decimal::from_str(exchange_rate.to_string().as_str()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
            },
            Err(_) => {
                return "--".to_string();
            }
        }
}

pub async fn total_liabilities_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,"state anchorprotocol mmMarket").await {
            Ok(response_result) => {
                let _total_liabilities: Decimal = Decimal::from_str(response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities.to_string().as_str()).unwrap(); 
                let micro: Decimal = Decimal::from_str("1000000").unwrap();
                let _total_liabilities = _total_liabilities.checked_div(micro).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
                return _total_liabilities.parse::<u128>().unwrap().to_formatted_string(&Locale::en);
            },
            Err(_) => {
                return "--".to_string();
            }
        }
}


pub async fn simulation_swap_return_amount_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: &str, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,key).await {
            Ok(response_result) => {
                let amount: cosmwasm_std::Decimal = cosmwasm_std::Decimal::from_str(response_result.as_simulation().unwrap().result.return_amount.to_string().as_str()).unwrap(); 
                let micro: cosmwasm_std::Uint128 = cosmwasm_std::Uint128::from_str("1000000").unwrap();
                return Decimal::from_str((amount / micro).to_string().as_str()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
            },
            Err(_) => {
                return "--".to_string();
            }
        }
}

pub async fn core_swap_amount_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: &str, digits_rounded_to: u32) -> String { 
        match get_data_maybe_or_await_task(&tasks,key).await {
            Ok(response_result) => {
                let amount: cosmwasm_std::Decimal = cosmwasm_std::Decimal::from_str(response_result.as_core_swap().unwrap().result.amount.to_string().as_str()).unwrap(); 
                let micro: cosmwasm_std::Uint128 = cosmwasm_std::Uint128::from_str("1000000").unwrap();
                return Decimal::from_str((amount / micro).to_string().as_str()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
            },
            Err(_) => {
                return "--".to_string();
            }
        }
} 
