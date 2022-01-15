#![allow(dead_code)]

pub mod interface;

use interface::model::{MaybeOrPromise,get_data_maybe_or_meta_data_maybe,get_data_maybe_or_await_task,get_meta_data_maybe_or_await_task};  

use interface::*;

use std::collections::HashMap;

use rust_decimal::Decimal;
use core::str::FromStr;
//use std::convert::TryFrom;
use rust_decimal::prelude::ToPrimitive;
   
use num_format::{Locale, ToFormattedString}; 
 
use chrono::{Utc};


use std::sync::Arc; 
use tokio::sync::RwLock; 

// only allowed to interact with the model via interface!
// this keeps the code readable.


fn duration_to_string(duration: chrono::Duration) -> String {

        let days = ((duration.num_seconds() / 60) / 60) / 24; 
        let hours = ((duration.num_seconds() / 60) / 60) % 24;
        let minutes = (duration.num_seconds() / 60) % 60;

        format!("{}d, {}h, {}m",days, hours, minutes)

}

pub fn timestamp_now_to_string() -> String {
    let dt = Utc::now();//.timestamp()
    let now = dt.format("%d/%m/%y %H:%M:%S");
    return now.to_string();              
}
 

pub async fn calculate_repay_plan(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, field: &str, digits_rounded_to: u32) -> String {

    let mut ust_amount_liquid = match terra_balance_to_string(tasks.clone(),"uusd",false,digits_rounded_to).await.as_ref() {
        "--" => {
            Decimal::from_str("0").unwrap()
        },
        e => {
            Decimal::from_str(e).unwrap()
        }
    };

    match min_ust_balance_to_string(tasks.clone(),false,digits_rounded_to).await.as_ref() {
        "--" => { 
            return "--".to_string();
        },
        e => { 
            ust_amount_liquid = ust_amount_liquid.checked_sub(Decimal::from_str(e).unwrap()).unwrap();             
        }
    }

    if field == "ust_available_to_repay" {
        return ust_amount_liquid.to_string();
    }

    let repay_amount = match calculate_repay_amount(tasks.clone(),false,digits_rounded_to).await.as_ref() {
        "--" => {
            return "--".to_string();
        },
        e => {
            Decimal::from_str(e).unwrap()
        }
    };
    
    let zero = Decimal::from_str("0").unwrap();
    let further_funds_needed = ust_amount_liquid.checked_sub(repay_amount).unwrap() < zero;

    if field == "more_funds_required" {
        return further_funds_needed.to_string();
    }
    
    let a_ust_deposit_liquid = match borrower_ust_deposited_to_string(tasks.clone(),false,digits_rounded_to).await.as_ref() {
        "--" => {
            Decimal::from_str("0").unwrap()
        },
        e => {
            Decimal::from_str(e).unwrap()
        }
    };

    if field == "available_in_deposit" {
        return a_ust_deposit_liquid.to_string();
    }

    let sufficient_funds_available = a_ust_deposit_liquid.checked_add(ust_amount_liquid).unwrap().checked_sub(repay_amount).unwrap() >= zero;

    if field == "sufficient_funds_to_repay" {
        return sufficient_funds_available.to_string();
    } 

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

    if field == "to_withdraw_from_account" { 
        return to_withdraw_from_account.to_string();
    }  
    if field == "to_withdraw_from_deposit" { 
        return to_withdraw_from_deposit.to_string();
    } 
    let to_repay = to_withdraw_from_account.checked_add(to_withdraw_from_deposit).unwrap();

    if field == "to_repay" { 
        return to_repay.to_string();
    }  
    let uusd_tax_cap = match uusd_tax_cap_to_string(tasks.clone(),10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => {
                Decimal::from_str(e).unwrap().checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            }
        };
    if field == "max_stability_tax" {
        return uusd_tax_cap.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
    }
    if field == "stability_tax" {
        let tax_rate = match tax_rate_to_string(tasks.clone(),10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => {
                Decimal::from_str(e).unwrap()
            }
        };
        
        let tx = to_repay.checked_mul(tax_rate).unwrap();
        if tx > uusd_tax_cap {
            return uusd_tax_cap.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
        }
        return tx.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
    }
    return "--".to_string();
}

pub async fn calculate_repay_amount(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,as_micro: bool, digits_rounded_to: u32) -> String {
    /* Calculate the repay amount required based on the desired "target_percent" value from user config.
     * target_percent is where ltv will be at once repay is complete.
     */

    let mut _target_percent = Decimal::from_str("0").unwrap(); 
    match get_meta_data_maybe_or_await_task(&tasks,"target_percentage").await {
                    Ok(response_result) => { 
                        _target_percent = Decimal::from_str(response_result.as_str()).unwrap();           
                    },
                    Err(_) => {
                        return "--".to_string();
                    }
                }

    let zero =  Decimal::from_str("0").unwrap(); 

    let mut _trigger_percentage =  Decimal::from_str("0.85").unwrap(); 

    match get_meta_data_maybe_or_await_task(&tasks,"trigger_percentage").await {
                    Ok(response_result) => { 
                        _trigger_percentage = Decimal::from_str(response_result.as_str()).unwrap();           
                    },
                    Err(_) => {
                        return "--".to_string();
                    }
                }


    let mut _borrow_limit =  Decimal::from_str("0").unwrap(); 

    match get_data_maybe_or_await_task(&tasks,"borrow_limit").await {
        Ok(response_result) => { 
            _borrow_limit = Decimal::from_str(response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string().as_str()).unwrap();
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    let target_loan_amount = _borrow_limit.checked_mul(_target_percent).unwrap();

    let mut _loan_amount = Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"borrow_info").await {
        Ok(response_result) => { 
            _loan_amount = Decimal::from_str(response_result.as_borrow_info().unwrap().result.loan_amount.to_string().as_str()).unwrap();   
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    if _borrow_limit <= zero || _loan_amount <= zero {
        return "--".to_string();
    }

    let difference_to_adjust = _loan_amount.checked_sub(target_loan_amount).unwrap();

    let mut micro = Decimal::from_str("1").unwrap();
    if !as_micro {
        micro = Decimal::from_str("1000000").unwrap();                
    }

    if difference_to_adjust > zero {
        return difference_to_adjust
                .checked_div(micro).unwrap()
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();

    }else {
        return "--".to_string();
    }

    
   
}

pub async fn check_anchor_loan_status(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String {
 
    let zero =  Decimal::from_str("0").unwrap(); 

    let mut _trigger_percentage =  Decimal::from_str("0.85").unwrap(); 

    match get_meta_data_maybe_or_await_task(&tasks,"trigger_percentage").await {
                    Ok(response_result) => { 
                        _trigger_percentage = Decimal::from_str(response_result.as_str()).unwrap();           
                    },
                    Err(_) => {
                        return "--".to_string();
                    }
                }


    let mut _borrow_limit =  Decimal::from_str("0").unwrap(); 

    match get_data_maybe_or_await_task(&tasks,"borrow_limit").await {
        Ok(response_result) => { 
            _borrow_limit = Decimal::from_str(response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string().as_str()).unwrap();
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    let mut _loan_amount = Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"borrow_info").await {
        Ok(response_result) => { 
            _loan_amount = Decimal::from_str(response_result.as_borrow_info().unwrap().result.loan_amount.to_string().as_str()).unwrap();  
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    if _borrow_limit <= zero || _loan_amount <= zero {
        return "--".to_string();
    }

    let current_percent = _loan_amount.checked_div(_borrow_limit).unwrap();

    let left_to_trigger = _trigger_percentage.checked_sub(current_percent).unwrap();

    if left_to_trigger <= zero {
        return "repay due".to_string();
    }

    return format!("{}%",left_to_trigger.checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                          .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string());

}


pub async fn estimate_anchor_protocol_next_claim_and_stake_tx(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, field_amount: &str, field: &str, digits_rounded_to: u32) -> String {
  
            let mut _collateral_value = Decimal::from_str("0").unwrap();  
            let mut _borrower_rewards_in_ust = Decimal::from_str("0").unwrap();  
 
            match borrower_rewards_in_ust_to_string(tasks.clone(),  10).await.as_ref() {
                "--" => {
                    return "--".to_string();
                },
                e => {  
                    _borrower_rewards_in_ust = Decimal::from_str(e).unwrap();
                }
            } 

            let mut loan_amount = Decimal::from_str("0").unwrap();  

            let mut _borrow_limit = Decimal::from_str("0").unwrap(); 

            match borrow_limit_to_string(tasks.clone(), 10).await.as_ref() {
                "--" => {
                    return "--".to_string();
                },
                e => { 
                    _borrow_limit = Decimal::from_str(e).unwrap();
                    let max_ltv = Decimal::from_str("0.6").unwrap(); 
                    _collateral_value = _borrow_limit.checked_div(max_ltv).unwrap(); 
                }
            }

            if "loan_amount"==field_amount {
 
                match  borrower_loan_amount_to_string(tasks.clone(),  10).await.as_ref() {
                    "--" => {
                        return "--".to_string();
                    },
                    e => {  
                        loan_amount = Decimal::from_str(e).unwrap();
                    }
                } 
            }else if "target_ltv"==field_amount {

                match get_meta_data_maybe_or_await_task(&tasks,"trigger_percentage").await {
                    Ok(response_result) => { 
                        loan_amount = _borrow_limit.checked_mul(Decimal::from_str(response_result.as_str()).unwrap()).unwrap();             
                    },
                    Err(_) => {
                        return "--".to_string();
                    }
                }
            }
            let mut _distribution_apr = Decimal::from_str("0").unwrap(); 
        
            match distribution_apr_to_string(tasks.clone(),  10).await.as_ref() {
                "--" => {
                    return "--".to_string();
                },
                e => { 
                    // removing % symbol
                    let mut chars = e.chars(); 
                    chars.next_back(); 
                    _distribution_apr = Decimal::from_str(chars.as_str()).unwrap().checked_div(Decimal::from_str("100").unwrap()).unwrap(); 
                }
            }

            let mut _staking_apy = Decimal::from_str("0").unwrap(); 
        
            match staking_apy_to_string(tasks.clone(),  10).await.as_ref() {
                "--" => {
                    return "--".to_string();
                },
                e => { 
                    // removing % symbol
                    let mut chars = e.chars(); 
                    chars.next_back(); 
                    _staking_apy = Decimal::from_str(chars.as_str()).unwrap().checked_div(Decimal::from_str("100").unwrap()).unwrap(); 
                }
            }

            let mut _transaction_fee = Decimal::from_str("0").unwrap(); 
        
            match estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),  10).await.as_ref() {
                "--" => {
                    return "--".to_string();
                },
                e => {  
                    _transaction_fee = Decimal::from_str(e).unwrap();
                }
            }
            
            let mut _optimal_time_to_wait: Option<Decimal> = None; 
            let mut _optimal_anc_ust_value: Option<Decimal> = None;
            let mut _total_returns_in_ust: Option<Decimal> = None;
  
            let one_year_equals_this_many_time_frames = Decimal::new(365*24,0);
           
            let anc_dist_returns_per_timeframe = _distribution_apr.checked_div(one_year_equals_this_many_time_frames).unwrap();
            let anc_dist_returns_per_time_frame_in_ust = loan_amount.checked_mul(anc_dist_returns_per_timeframe).unwrap(); 
            
            let anc_staking_returns_per_timeframe = _staking_apy.checked_div(one_year_equals_this_many_time_frames).unwrap();

            let claim_and_stake_gas_fee = Decimal::from_str("-1").unwrap().checked_mul(_transaction_fee).unwrap();
            
            let mut max_value: Option<Decimal> = None;
            let mut max_index: Option<Decimal> = None;

            let timeframes = one_year_equals_this_many_time_frames.checked_add(Decimal::new(1,0)).unwrap().to_i64().unwrap();
            
            for n in 1..timeframes {
                // amount ANC rewards available after n timeframes
                let total_anc_dist_returns_n_timeframes_ust = anc_dist_returns_per_time_frame_in_ust.checked_mul(Decimal::new(n,0)).unwrap();

                // amount ANC staked, by claiming and staking the outstanding amount after n timeframes
                let total_anc_staked_n_timeframes_in_ust_after_tx = total_anc_dist_returns_n_timeframes_ust.checked_add(claim_and_stake_gas_fee).unwrap();

                let total_anc_staking_rewards_one_year_in_ust = total_anc_staked_n_timeframes_in_ust_after_tx
                .checked_mul(anc_staking_returns_per_timeframe).unwrap()
                .checked_mul(one_year_equals_this_many_time_frames.checked_sub(Decimal::new(n,0)).unwrap()).unwrap() // remove the timeframes that already passed in the reference year
                .checked_div(Decimal::new(n,0)).unwrap() // now normalize the result, to represent the ANC staking rewards in the reference year
                .checked_mul(one_year_equals_this_many_time_frames).unwrap();
                
                if let Some(max) = max_value {
                    if max < total_anc_staking_rewards_one_year_in_ust {
                        max_value = Some(total_anc_staking_rewards_one_year_in_ust);
                        max_index = Some(Decimal::new(n,0));
                    }
                }else{
                    max_value = Some(total_anc_staking_rewards_one_year_in_ust);
                    max_index = Some(Decimal::new(n,0));
                }
            }  

            _optimal_time_to_wait = max_index;
            _optimal_anc_ust_value = anc_dist_returns_per_time_frame_in_ust.checked_mul(max_index.unwrap());
            let mut n = 1;
            let mut value: Option<Decimal> = Some(Decimal::new(0,0));
            while n < timeframes {
                let staked_n_timeframes_anc_value = anc_staking_returns_per_timeframe.checked_mul(one_year_equals_this_many_time_frames.checked_sub(Decimal::new(n-1,0)).unwrap()).unwrap().checked_mul(_optimal_anc_ust_value.unwrap());
                value = value.unwrap().checked_add(staked_n_timeframes_anc_value.unwrap());
                n = n + _optimal_time_to_wait.unwrap().to_i64().unwrap(); 
            }
            _total_returns_in_ust = value;

            let _optimal_time_to_wait = _optimal_time_to_wait.unwrap().checked_mul(Decimal::new(60*60,0));
            let time_to_wait_already_passed = _borrower_rewards_in_ust
                                                .checked_mul(Decimal::new(60*60,0)).unwrap()
                                                .checked_div(anc_dist_returns_per_time_frame_in_ust);


            let wait_loan_taken = chrono::Duration::seconds(_optimal_time_to_wait.unwrap().to_i64().unwrap());

            let mut time = _optimal_time_to_wait.unwrap().to_i64().unwrap();
            if let Some(ttwap) = time_to_wait_already_passed {
                time = time-(ttwap.to_i64().unwrap());
            }

            let minus_already_wait_loan_taken = chrono::Duration::seconds(time);
            
            let duration = duration_to_string(wait_loan_taken);
            let dt = Utc::now();
            let trigger_date = dt.checked_add_signed(minus_already_wait_loan_taken).unwrap().format("%d/%m/%y %H:%M");
              

            if "date_next"==field {
                if time <= 0 {
                    return "now".to_string();
                }
                return trigger_date.to_string();
            }else if "value_next"==field {
                return _optimal_anc_ust_value.unwrap() 
                         .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                         .to_string();
            }else if "duration_next"==field {
                return duration;
            }else if "total_returns"==field && _total_returns_in_ust!=None {
                return _total_returns_in_ust.unwrap() 
                         .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                         .to_string();
            }else if "apr"==field && _total_returns_in_ust!=None  {
                match _total_returns_in_ust.unwrap().checked_div(_collateral_value) {
                    None => {
                        return "--".to_string();
                    },
                    Some(e) => {
                        return  format!("{}%",
                            e
                            .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                            .to_string()
                            );
                    }
                }
                
            }
            return "--".to_string();

}
 


pub async fn estimate_anchor_protocol_tx_fee_claim_and_stake(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
  let tx_fee_claim_rewards = estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","fee_amount_adjusted".to_owned(),false,10).await;   
  let tx_fee_stake_rewards = estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking","fee_amount_adjusted".to_owned(),false,10).await;   

  if tx_fee_claim_rewards.as_str() == "--" || tx_fee_stake_rewards.as_str() == "--" {
    return "--".to_string();
  }

  let tx_fee_claim_rewards = Decimal::from_str(tx_fee_claim_rewards.as_str()).unwrap();
  let tx_fee_stake_rewards = Decimal::from_str(tx_fee_stake_rewards.as_str()).unwrap();

  return tx_fee_claim_rewards.checked_add(tx_fee_stake_rewards).unwrap()
                             .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                             .to_string();
}

pub async fn estimate_anchor_protocol_tx_fee(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, tx_key: &str, key: String, as_micro: bool, digits_rounded_to: u32) -> String { 
 
    let mut tax_rate = Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"tax_rate").await {
        Ok(response_result) => { 
            tax_rate = Decimal::from_str(response_result.as_tax_rate().unwrap().result.as_str()).unwrap();
        },
        Err(_) => { 
        }
    }

    let mut tax_cap_uusd = Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"tax_caps").await {
        Ok(response_result) => { 
            let vec_tax_caps = &response_result.as_tax_caps().unwrap().result;
            for tax_cap in vec_tax_caps {
                if tax_cap.denom == "uusd".to_string() {
                    tax_cap_uusd = Decimal::from_str(tax_cap.tax_cap.as_str()).unwrap();
                    break;
                }
            }                   
        },
        Err(_) => { 
        }
    }

    match get_data_maybe_or_await_task(&tasks,tx_key).await {
        Ok(response_result) => { 
            let result = &response_result.as_transactions().unwrap().result;
            let mut avg_fee_amount = Decimal::from_str("0").unwrap();
            let mut avg_gas_adjustment = Decimal::from_str("0").unwrap(); // gas_wanted * gas_adjustment = fee_amount
            let mut avg_gas_adjustment_for_stability_fee_case = Decimal::from_str("0").unwrap(); 
            let mut avg_gas_used = Decimal::from_str("0").unwrap();
            let mut avg_gas_wanted = Decimal::from_str("0").unwrap();
            let mut avg_fee_amount_without_stability_fee = Decimal::from_str("0").unwrap();
            let mut avg_fee_amount_adjusted_without_stability_fee = Decimal::from_str("0").unwrap();
            // estimate_fee_amount = avg_gas_adjustment * avg_gas_used;
            for entry in result {
 

                let stability_tax = entry.amount.checked_mul(tax_rate).unwrap();
                let mut fee_amount_without_stability_fee = Decimal::from_str("0").unwrap();
                if stability_tax < tax_cap_uusd {
                    fee_amount_without_stability_fee = entry.fee_amount.checked_sub(stability_tax).unwrap();
                } else {                    
                    fee_amount_without_stability_fee = entry.fee_amount.checked_sub(tax_cap_uusd).unwrap();
                }
                avg_fee_amount_adjusted_without_stability_fee = avg_fee_amount_adjusted_without_stability_fee.checked_add(fee_amount_without_stability_fee).unwrap();

                let gas_adjustment = entry.gas_wanted.checked_div(entry.gas_used).unwrap();
                fee_amount_without_stability_fee = fee_amount_without_stability_fee.checked_div(gas_adjustment).unwrap();
                avg_gas_adjustment_for_stability_fee_case = avg_gas_adjustment_for_stability_fee_case.checked_add(gas_adjustment).unwrap();

                avg_fee_amount_without_stability_fee = avg_fee_amount_without_stability_fee.checked_add(fee_amount_without_stability_fee).unwrap();
                let gas_adjustment = entry.fee_amount.checked_div(entry.gas_wanted).unwrap(); 

                avg_fee_amount = avg_fee_amount.checked_add(entry.fee_amount).unwrap();
                avg_gas_adjustment = avg_gas_adjustment.checked_add(gas_adjustment).unwrap();
                avg_gas_used = avg_gas_used.checked_add(entry.gas_used).unwrap(); 
                avg_gas_wanted = avg_gas_wanted.checked_add(entry.gas_wanted).unwrap(); 
                //println!("gas_wanted: {}, gas_used: {}, fee_denom: {}, fee_amount: {}, claim_amount: {}",entry.gas_wanted, entry.gas_used, entry.fee_denom, entry.fee_amount, entry.claim_amount);
            }
             match get_meta_data_maybe_or_await_task(&tasks,"gas_fees_uusd").await {
                Ok(response_result) => { 
                    let count_entries = Decimal::from_str(result.len().to_string().as_str()).unwrap();
                    let gas_fees_uusd = Decimal::from_str(response_result.as_str()).unwrap();  
                    avg_fee_amount_adjusted_without_stability_fee = avg_fee_amount_adjusted_without_stability_fee.checked_div(count_entries).unwrap();
                    avg_fee_amount_without_stability_fee = avg_fee_amount_without_stability_fee.checked_div(count_entries).unwrap();
                    avg_fee_amount = avg_fee_amount.checked_div(count_entries).unwrap();
                    avg_gas_adjustment = avg_gas_adjustment.checked_div(gas_fees_uusd).unwrap().checked_div(count_entries).unwrap();
                    avg_gas_used = avg_gas_used.checked_div(count_entries).unwrap();
                    avg_gas_wanted = avg_gas_wanted.checked_div(count_entries).unwrap();
                    avg_gas_adjustment_for_stability_fee_case = avg_gas_adjustment_for_stability_fee_case.checked_div(count_entries).unwrap();
                    let fee_amount_at_threshold = avg_gas_used.checked_mul(gas_fees_uusd).unwrap();
                    let estimated_fee_amount = avg_gas_used.checked_mul(gas_fees_uusd).unwrap().checked_mul(avg_gas_adjustment).unwrap();
                    
                    let mut micro = Decimal::from_str("1").unwrap();
                    if !as_micro {
                        micro = Decimal::from_str("1000000").unwrap();                
                    }

                     match key.as_ref() {
                        "avg_gas_wanted" => {
                            return avg_gas_wanted 
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string();
                        },
                        "avg_fee_amount_without_stability_fee" => {
                            return avg_fee_amount_without_stability_fee
                              .checked_div(micro).unwrap()
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string();
                        },
                        "avg_fee_amount_adjusted_without_stability_fee" => {
                            return avg_fee_amount_adjusted_without_stability_fee
                              .checked_div(micro).unwrap()
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string();
                        },
                        "avg_fee_amount" => {
                            return avg_fee_amount
                              .checked_div(micro).unwrap()
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string();
                        },
                        "avg_gas_adjustment" => {
                            return avg_gas_adjustment
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string();
                        },
                        "avg_gas_adjustment_for_stability_fee_case" => {
                            return avg_gas_adjustment_for_stability_fee_case
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string();
                        },
                        "avg_gas_used" => {
                            return avg_gas_used
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string();
                        },
                        "fee_amount_at_threshold" => {
                            return fee_amount_at_threshold
                              .checked_div(micro).unwrap()
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string();
                        },
                        "fee_amount_adjusted" => {
                            return estimated_fee_amount
                              .checked_div(micro).unwrap()
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string();
                        },
                        &_ => { 
                            return "--".to_string();
                        }  
                    }
                },
                Err(_) => {
                            return "--".to_string();
                }
            }
           },
        Err(_) => {
            return "--".to_string();
        }
    }
}


pub async fn apy_on_collateral_by(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, amount_field: &str, apr_field: &str, digits_rounded_to: u32) -> String { 

    let mut _collateral_value = Decimal::from_str("0").unwrap();  
    let mut _borrow_limit = Decimal::from_str("0").unwrap(); 

    match borrow_limit_to_string(tasks.clone(), 10).await.as_ref() {
        "--" => {
            return "--".to_string();
        },
        e => { 
            _borrow_limit = Decimal::from_str(e).unwrap();
            let max_ltv = Decimal::from_str("0.6").unwrap(); 
            _collateral_value = _borrow_limit.checked_div(max_ltv).unwrap(); 
        }
    }

    let mut _loan_amount = Decimal::from_str("0").unwrap();  

    if amount_field == "loan_amount" {
        match  borrower_loan_amount_to_string(tasks.clone(),  10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => {  
                _loan_amount = Decimal::from_str(e).unwrap();
            }
        }
    }else if amount_field == "deposit_amount" {
        match borrower_ust_deposited_to_string(tasks.clone(),false, 10).await.as_ref() {
            "--" => {
                return "--".to_string();
            }, 
            e => { 
                _loan_amount = Decimal::from_str(e).unwrap();             
            }
        }
    }else if amount_field == "target_ltv" { 
        match get_meta_data_maybe_or_await_task(&tasks,"trigger_percentage").await {
            Ok(response_result) => { 
                _loan_amount = _borrow_limit.checked_mul(Decimal::from_str(response_result.as_str()).unwrap()).unwrap();             
            },
            Err(_) => {
                return "--".to_string();
            }
        }
    }

    let mut apr = Decimal::from_str("0").unwrap();

    if "net_apr" == apr_field { 

        let mut _net_apr = Decimal::from_str("0").unwrap(); 
        
        match net_apr_to_string(tasks.clone(),  10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => { 
                // removing % symbol
                let mut chars = e.chars(); 
                chars.next_back(); 
                _net_apr = Decimal::from_str(chars.as_str()).unwrap(); 
            }
        }

        let mut _earn_apr = Decimal::from_str("0").unwrap(); 
        
        match earn_apr_to_string(tasks.clone(),  10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => { 
                // removing % symbol
                let mut chars = e.chars(); 
                chars.next_back(); 
                _earn_apr = Decimal::from_str(chars.as_str()).unwrap(); 
            }
        }
        apr = _net_apr.checked_add(_earn_apr).unwrap();
    }else if "earn_apr" == apr_field {
        let mut _earn_apr = Decimal::from_str("0").unwrap(); 
        
        match earn_apr_to_string(tasks.clone(),  10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => { 
                // removing % symbol
                let mut chars = e.chars(); 
                chars.next_back(); 
                _earn_apr = Decimal::from_str(chars.as_str()).unwrap(); 
            }
        }
        apr = _earn_apr;
    }else if "borrow_apr"== apr_field {
        let mut _borrow_apr = Decimal::from_str("0").unwrap(); 
        
        match borrow_apr_to_string(tasks.clone(),  10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => { 
                // removing % symbol
                let mut chars = e.chars(); 
                chars.next_back(); 
                _borrow_apr = Decimal::from_str(chars.as_str()).unwrap(); 
            }
        }
        apr = _borrow_apr;
    }else if "distribution_apr" == apr_field {
        let mut _distribution_apr = Decimal::from_str("0").unwrap(); 
        
        match distribution_apr_to_string(tasks.clone(),  10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => { 
                // removing % symbol
                let mut chars = e.chars(); 
                chars.next_back(); 
                _distribution_apr = Decimal::from_str(chars.as_str()).unwrap(); 
            }
        }
        apr = _distribution_apr;
    }


    match apr.checked_mul(_loan_amount).unwrap().checked_div(_collateral_value) {
        None => { 
            return "--".to_string();
        },
        Some(e) => {
            return format!("{}%",e
                  .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                  .to_string()); 
        }
    }
}

pub async fn anc_staked_balance_in_ust_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    let mut _exchange_rate = Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"simulation_cw20 anchorprotocol ANC terraswapAncUstPair").await {
            Ok(response_result) => {
                let amount: cosmwasm_std::Decimal = cosmwasm_std::Decimal::from_str(response_result.as_simulation().unwrap().result.return_amount.to_string().as_str()).unwrap(); 
                let micro: cosmwasm_std::Uint128 = cosmwasm_std::Uint128::from_str("1000000").unwrap();
                _exchange_rate = Decimal::from_str((amount / micro).to_string().as_str()).unwrap();
            },
            Err(_) => {
                return "--".to_string();
            }
        }

    match get_data_maybe_or_await_task(&tasks,"staker").await {
        Ok(response_result) => {
            let balance = response_result.as_staker().unwrap().result.balance; 
            let balance = Decimal::from_str(balance.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            return balance.checked_div(micro).unwrap().checked_mul(_exchange_rate).unwrap()
                   .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                   .to_string();
            
        },
        Err(_) => {
            return "--".to_string();
        }
    }
}


pub async fn anchor_claim_and_stake_transaction_gas_fees_ratio_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    let mut _pending_rewards = Decimal::from_str("0").unwrap();
    match get_data_maybe_or_await_task(&tasks,"borrow_info").await {
        Ok(response_result) => { 
            _pending_rewards = Decimal::from_str(response_result.as_borrow_info().unwrap().result.pending_rewards.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _pending_rewards = _pending_rewards.checked_div(micro).unwrap();
            
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    let mut _exchange_rate = Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"simulation_cw20 anchorprotocol ANC terraswapAncUstPair").await {
            Ok(response_result) => {
                let amount: cosmwasm_std::Decimal = cosmwasm_std::Decimal::from_str(response_result.as_simulation().unwrap().result.return_amount.to_string().as_str()).unwrap(); 
                let micro: cosmwasm_std::Uint128 = cosmwasm_std::Uint128::from_str("1000000").unwrap();
                _exchange_rate = Decimal::from_str((amount / micro).to_string().as_str()).unwrap();
            },
            Err(_) => {
                return "--".to_string();
            }
        }

    _pending_rewards = _pending_rewards.checked_mul(_exchange_rate).unwrap();

    let anchor_protocol_tx_fee = estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),  10).await;

    if anchor_protocol_tx_fee.as_str() == "--" {
        return "--".to_string();
    }

    let anchor_protocol_tx_fee = Decimal::from_str(anchor_protocol_tx_fee.as_str()).unwrap();             
      
    match anchor_protocol_tx_fee.checked_div(_pending_rewards){
        None => {
            return "--".to_string();
        },
        Some(e) => {
            return format!("{}%",e.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string());
        }
    }

}

pub async fn borrower_rewards_in_ust_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    let mut _pending_rewards = Decimal::from_str("0").unwrap();
    match get_data_maybe_or_await_task(&tasks,"borrow_info").await {
        Ok(response_result) => { 
            _pending_rewards = Decimal::from_str(response_result.as_borrow_info().unwrap().result.pending_rewards.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _pending_rewards = _pending_rewards.checked_div(micro).unwrap();
            
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    let mut _exchange_rate = Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"simulation_cw20 anchorprotocol ANC terraswapAncUstPair").await {
            Ok(response_result) => {
                let amount: cosmwasm_std::Decimal = cosmwasm_std::Decimal::from_str(response_result.as_simulation().unwrap().result.return_amount.to_string().as_str()).unwrap(); 
                let micro: cosmwasm_std::Uint128 = cosmwasm_std::Uint128::from_str("1000000").unwrap();
                _exchange_rate = Decimal::from_str((amount / micro).to_string().as_str()).unwrap();
            },
            Err(_) => {
                return "--".to_string();
            }
        }

    return _pending_rewards.checked_mul(_exchange_rate).unwrap()
                   .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                   .to_string();
}

pub async fn borrower_deposit_liquidity_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    
    let mut _balance = Decimal::from_str("0").unwrap();
    match get_data_maybe_or_await_task(&tasks,"balance").await {
        Ok(response_result) => { 
            _balance = Decimal::from_str(response_result.as_balance().unwrap().result.balance.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _balance = _balance.checked_div(micro).unwrap();
            
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    let mut _exchange_rate = Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"epoch_state anchorprotocol mmMarket").await {
            Ok(response_result) => {
                let result: cosmwasm_std::Decimal256 = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate; 
                _exchange_rate = Decimal::from_str(result.to_string().as_str()).unwrap();
            },
            Err(_) => {
                return "--".to_string();
            }
        }

    let ust_deposited = _balance.checked_mul(_exchange_rate).unwrap();

    let mut _borrow_limit =  Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"borrow_limit").await {
        Ok(response_result) => { 
            _borrow_limit = Decimal::from_str(response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _borrow_limit = _borrow_limit.checked_div(micro).unwrap();
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    return format!("{}%",ust_deposited.checked_div(_borrow_limit).unwrap()
           .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
           .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
           .to_string());
}

pub async fn borrower_ltv_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
     
    let mut _borrow_limit =  Decimal::from_str("0").unwrap();
    let ltv_max =  Decimal::from_str("0.6").unwrap();

    match get_data_maybe_or_await_task(&tasks,"borrow_limit").await {
        Ok(response_result) => { 
            _borrow_limit = Decimal::from_str(response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _borrow_limit = _borrow_limit.checked_div(micro).unwrap();
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    let collateral_value = _borrow_limit.checked_div(ltv_max).unwrap();

    let mut _loan_amount = Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"borrow_info").await {
        Ok(response_result) => { 
            _loan_amount = Decimal::from_str(response_result.as_borrow_info().unwrap().result.loan_amount.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _loan_amount = _loan_amount.checked_div(micro).unwrap();
            
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    match _loan_amount.checked_div(collateral_value) {
        None => { 
            return "--".to_string();
        },
        Some(e) => {
            return format!("{}%",e
                   .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                   .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                   .to_string());
        }
    }

}



pub async fn borrower_ust_deposited_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, as_micro: bool, digits_rounded_to: u32) -> String { 
    let mut _balance = Decimal::from_str("0").unwrap();
    match get_data_maybe_or_await_task(&tasks,"balance").await {
        Ok(response_result) => { 
            _balance = Decimal::from_str(response_result.as_balance().unwrap().result.balance.to_string().as_str()).unwrap();
            if !as_micro { 
            let micro = Decimal::from_str("1000000").unwrap();
                _balance = _balance.checked_div(micro).unwrap();
            }
            
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    let mut _exchange_rate = Decimal::from_str("0").unwrap();

    match get_data_maybe_or_await_task(&tasks,"epoch_state anchorprotocol mmMarket").await {
            Ok(response_result) => {
                let result: cosmwasm_std::Decimal256 = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate; 
                _exchange_rate = Decimal::from_str(result.to_string().as_str()).unwrap();
            },
            Err(_) => {
                return "--".to_string();
            }
        }
    return _balance.checked_mul(_exchange_rate).unwrap()
           .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
           .to_string();
}



pub async fn borrow_apr_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        // utilisationRatio = stablecoinsLent / stablecoinsDeposited
        // borrowRate = utilisationRatio * interestMultiplier + baseRate
        // borrow_apr = blocksPerYear * borrowRate

        let mut _total_liabilities: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero(); 

        let mut _a_terra_exchange_rate: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero();
        let mut _a_terra_supply: cosmwasm_std::Uint256 = cosmwasm_std::Uint256::zero();
                
        match get_data_maybe_or_await_task(&tasks,"state anchorprotocol mmMarket").await {
            Ok(response_result) => {
                _total_liabilities = response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        match get_data_maybe_or_await_task(&tasks,"epoch_state anchorprotocol mmMarket").await {
            Ok(response_result) => {
                _a_terra_exchange_rate = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate; 
                _a_terra_supply = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.aterra_supply; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        let stablecoins_lent: Decimal = Decimal::from_str(_total_liabilities.to_string().as_str()).unwrap();

        let stablecoins_deposited: Decimal = Decimal::from_str(_a_terra_supply.to_string().as_str()).unwrap()
                                             .checked_mul(Decimal::from_str(_a_terra_exchange_rate.to_string().as_str()).unwrap())
                                             .unwrap();
        
        let utilization_ratio: Decimal = stablecoins_lent
                                         .checked_div(stablecoins_deposited)
                                         .unwrap();

        let mut _interest_multiplier: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero();
        let mut _base_rate: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero();

        match get_data_maybe_or_await_task(&tasks,"config anchorprotocol mmInterestModel").await {
            Ok(response_result) => {
                _base_rate  = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.base_rate; 
                _interest_multiplier = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.interest_multiplier; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        let borrow_rate_without_base_rate = Decimal::from_str(_interest_multiplier.to_string().as_str()).unwrap()
                                            .checked_mul(utilization_ratio).unwrap();

        let borrow_rate = borrow_rate_without_base_rate
                          .checked_add(Decimal::from_str(_base_rate.to_string().as_str()).unwrap())
                          .unwrap();
       
        match get_data_maybe_or_await_task(&tasks,"blocks_per_year").await {
            Ok(response_result) => {
                let blocks_per_year = Decimal::from_str(response_result.as_blocks().unwrap().result.blocks_per_year.to_string().as_str()).unwrap();
               
                let borrow_apr = blocks_per_year
                                 .checked_mul(borrow_rate).unwrap()
                                 .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                                 .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                                 .to_string();  
                format!("{}%",borrow_apr)
              },
            Err(_) => {
                return "--".to_string();
            }
        }      
}


pub async fn anything_to_err(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: &str) -> String { 
     
        match get_data_maybe_or_meta_data_maybe(&tasks,key).await {
            Ok(_) => {
               return "--".to_string();
            },
            Err(err) => {
                return format!("{:?}",err);
            }
        } 
}


pub async fn net_apr_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        // utilisationRatio = stablecoinsLent / stablecoinsDeposited
        // borrowRate = utilisationRatio * interestMultiplier + baseRate
        // borrow_apr = blocksPerYear * borrowRate

        let mut _total_liabilities: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero(); 

        let mut _a_terra_exchange_rate: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero();
        let mut _a_terra_supply: cosmwasm_std::Uint256 = cosmwasm_std::Uint256::zero();
                
        match get_data_maybe_or_await_task(&tasks,"state anchorprotocol mmMarket").await {
            Ok(response_result) => {
                _total_liabilities = response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        match get_data_maybe_or_await_task(&tasks,"epoch_state anchorprotocol mmMarket").await {
            Ok(response_result) => {
                _a_terra_exchange_rate = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate; 
                _a_terra_supply = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.aterra_supply; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        let stablecoins_lent: Decimal = Decimal::from_str(_total_liabilities.to_string().as_str()).unwrap();

        let stablecoins_deposited: Decimal = Decimal::from_str(_a_terra_supply.to_string().as_str()).unwrap()
                                             .checked_mul(Decimal::from_str(_a_terra_exchange_rate.to_string().as_str()).unwrap())
                                             .unwrap();
        
        let utilization_ratio: Decimal = stablecoins_lent
                                         .checked_div(stablecoins_deposited)
                                         .unwrap();

        let mut _interest_multiplier: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero();
        let mut _base_rate: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero();

        match get_data_maybe_or_await_task(&tasks,"config anchorprotocol mmInterestModel").await {
            Ok(response_result) => {
                _base_rate  = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.base_rate; 
                _interest_multiplier = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.interest_multiplier; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        let borrow_rate_without_base_rate = Decimal::from_str(_interest_multiplier.to_string().as_str()).unwrap()
                                            .checked_mul(utilization_ratio).unwrap();

        let borrow_rate = borrow_rate_without_base_rate
                          .checked_add(Decimal::from_str(_base_rate.to_string().as_str()).unwrap())
                          .unwrap();
       
       let mut _blocks_per_year = Decimal::from_str("0").unwrap(); // 4656810
       match get_data_maybe_or_await_task(&tasks,"blocks_per_year").await {
            Ok(response_result) => {
                _blocks_per_year = Decimal::from_str(response_result.as_blocks().unwrap().result.blocks_per_year.to_string().as_str()).unwrap();
              },
            Err(_) => {
                return "--".to_string();
            }
        }       
        let borrow_apr = _blocks_per_year
                         .checked_mul(borrow_rate).unwrap();

        match get_data_maybe_or_await_task(&tasks,"api/v2/distribution-apy").await {
            Ok(response_result) => {
                let distribution_apr: cosmwasm_std::Decimal = response_result.as_distribution_apy().unwrap().distribution_apy; 
                return format!("{}%",
                    Decimal::from_str(distribution_apr.to_string().as_str()).unwrap()
                    .checked_add(borrow_apr.checked_mul(Decimal::from_str("-1").unwrap()).unwrap()).unwrap()
                    .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string()
                    );
            },
            Err(_) => {
                return "--".to_string();
            }
        } 
}


pub async fn borrow_rate_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: &str, key_1: &str, key_2: &str, digits_rounded_to: u32) -> String { 
        
        let mut _interest_multiplier: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero();
        let mut _base_rate: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero();

        match get_data_maybe_or_await_task(&tasks,key).await {
            Ok(response_result) => {
                _base_rate  = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.base_rate; 
                _interest_multiplier = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.interest_multiplier; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        let mut _total_liabilities: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero(); 

        let mut _a_terra_exchange_rate: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero();
        let mut _a_terra_supply: cosmwasm_std::Uint256 = cosmwasm_std::Uint256::zero();
                
        match get_data_maybe_or_await_task(&tasks,key_1).await {
            Ok(response_result) => {
                _total_liabilities = response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        match get_data_maybe_or_await_task(&tasks,key_2).await {
            Ok(response_result) => {
                _a_terra_exchange_rate = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate; 
                _a_terra_supply = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.aterra_supply; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        let stablecoins_lent: Decimal = Decimal::from_str(_total_liabilities.to_string().as_str()).unwrap().checked_mul(Decimal::from_str(_a_terra_exchange_rate.to_string().as_str()).unwrap()).unwrap();
        let utilization_ratio: Decimal = stablecoins_lent.checked_div(Decimal::from_str(_a_terra_supply.to_string().as_str()).unwrap()).unwrap();
        return Decimal::from_str(_interest_multiplier.to_string().as_str()).unwrap().checked_mul(utilization_ratio).unwrap().checked_add(Decimal::from_str(_base_rate.to_string().as_str()).unwrap()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();  
}

pub async fn utilization_ratio_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key_1: &str,key_2: &str, digits_rounded_to: u32) -> String { 
        
        let mut _total_liabilities: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero(); 

        let mut _a_terra_exchange_rate: cosmwasm_std::Decimal256 = cosmwasm_std::Decimal256::zero();
        let mut _a_terra_supply: cosmwasm_std::Uint256 = cosmwasm_std::Uint256::zero();
                
        match get_data_maybe_or_await_task(&tasks,key_1).await {
            Ok(response_result) => {
                _total_liabilities = response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        match get_data_maybe_or_await_task(&tasks,key_2).await {
            Ok(response_result) => {
                _a_terra_exchange_rate = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate; 
                _a_terra_supply = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.aterra_supply; 
            },
            Err(_) => {
                return "--".to_string();
            }
        }

        let stablecoins_lent: Decimal = Decimal::from_str(_total_liabilities.to_string().as_str()).unwrap().checked_mul(Decimal::from_str(_a_terra_exchange_rate.to_string().as_str()).unwrap()).unwrap();
        let utilization_ratio = stablecoins_lent.checked_div(Decimal::from_str(_a_terra_supply.to_string().as_str()).unwrap()).unwrap();
        return format!("{}%",utilization_ratio.checked_mul(Decimal::from_str("100").unwrap()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string());


}


pub async fn estimate_anchor_protocol_auto_repay_tx_fee(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
     

    // does include gas_adjustment
    let mut _fee_to_redeem_stable = Decimal::from_str("0").unwrap();
    match estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_redeem_stable","fee_amount_adjusted".to_owned(),false,10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => { 
                 _fee_to_redeem_stable = Decimal::from_str(e).unwrap(); 
            }
    }
    // does include gas_adjustment
    let mut _anchor_protocol_txs_repay_stable = Decimal::from_str("0").unwrap();
    match estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_repay_stable","avg_fee_amount_adjusted_without_stability_fee".to_owned(),false,10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => { 
                 _anchor_protocol_txs_repay_stable = Decimal::from_str(e).unwrap(); 
            }
    }
    // min(to_repay * tax_rate , tax_cap)
    let mut _stability_tax = Decimal::from_str("0").unwrap();
    match calculate_repay_plan(tasks.clone(),"stability_tax",10).await.as_ref() {
            "--" => {
                return "--".to_string();
            },
            e => { 
                 _stability_tax = Decimal::from_str(e).unwrap(); 
            }
    }

    return _fee_to_redeem_stable.checked_add(_anchor_protocol_txs_repay_stable).unwrap()
                                .checked_add(_stability_tax).unwrap()
                                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();

}