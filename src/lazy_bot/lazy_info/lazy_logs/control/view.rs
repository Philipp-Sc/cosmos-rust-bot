#![allow(dead_code)]

pub mod interface;

use interface::model::{
    MaybeOrPromise,
    get_data_maybe_or_meta_data_maybe,
    get_data_maybe_or_await_task,
    get_meta_data_maybe_or_await_task,
    get_meta_data_maybe};  

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

macro_rules! decimal_or_return {
    ( $e:expr ) => {
        match $e {
            "--" => return String::from("--"),
            e => Decimal::from_str(e).unwrap(),
        }
    }
} 

macro_rules! percent_decimal_or_return {
    ( $e:expr ) => {
        match $e {
            "--" => return String::from("--"),
            e => {
                let mut chars = e.chars(); 
                chars.next_back(); 
                Decimal::from_str(chars.as_str()).unwrap().checked_div(Decimal::from_str("100").unwrap()).unwrap()
            },
        }
    }
} 
 
// idially only allowed to interact with the model via interface! 

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

pub async fn get_past_transaction_logs(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, field: &str) -> String {
   match get_meta_data_maybe(&tasks, field).await {
        Ok(maybe) => {
            return format!("tx: {:?}, timestamp: {}",maybe.data, maybe.timestamp);
        },
        Err(_) => {
            // no previous transaction, free to continue.
            return "--".to_string();
        }
   }
}

 
pub async fn calculate_borrow_plan(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, field: &str, digits_rounded_to: u32) -> String {

    let mut ust_amount_liquid = decimal_or_return!(terra_balance_to_string(tasks.clone(),"uusd",false,10).await.as_ref());
    let min_ust_balance = decimal_or_return!(min_ust_balance_to_string(tasks.clone(),false,10).await.as_ref());
    ust_amount_liquid = ust_amount_liquid.checked_sub(min_ust_balance).unwrap();  
 
    if field == "ust_available_to_pay_fees" {
        return ust_amount_liquid.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
    }

    let borrow_amount = decimal_or_return!(calculate_amount(tasks.clone(),"borrow",false,10).await.as_ref());

    let uusd_tax_cap = decimal_or_return!(uusd_tax_cap_to_string(tasks.clone(),false,10).await.as_ref());
    let tax_rate = decimal_or_return!(tax_rate_to_string(tasks.clone(),10).await.as_ref());
        
    let mut stability_tax = borrow_amount.checked_mul(tax_rate).unwrap();
    if stability_tax > uusd_tax_cap {
        stability_tax = uusd_tax_cap;
    }

    if field == "stability_tax_borrow" {
        return stability_tax.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
    }

    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

    let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());

    let anchor_protocol_txs_borrow_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_borrow_stable","avg_gas_used".to_owned(),false,10).await.as_ref());

    let borrow_stable_fee = anchor_protocol_txs_borrow_stable_gas_used 
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap() 
            .checked_add(stability_tax).unwrap();
   
    let able_to_borrow = ust_amount_liquid >= borrow_stable_fee;

    let anchor_protocol_txs_deposit_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_deposit_stable","avg_gas_used".to_owned(),false,10).await.as_ref());
   
    let deposit_stable_fee = anchor_protocol_txs_deposit_stable_gas_used 
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap();
          
    let mut to_deposit = borrow_amount
        .checked_sub(borrow_stable_fee).unwrap()
        .checked_sub(deposit_stable_fee).unwrap();

    let mut stability_tax = to_deposit.checked_mul(tax_rate).unwrap();
    /* stability_tax is slightly overestimated, because it is not substracted from the to_deposit value in the first place.*/

    if stability_tax > uusd_tax_cap {
        stability_tax = uusd_tax_cap;
    }
    to_deposit = to_deposit.checked_sub(stability_tax).unwrap();

    let deposit_stable_fee = deposit_stable_fee.checked_add(stability_tax).unwrap();

    let able_to_deposit = ust_amount_liquid.checked_add(borrow_amount).unwrap().checked_sub(borrow_stable_fee).unwrap() >= deposit_stable_fee;
    
    if field == "stability_tax_deposit" {
        return stability_tax.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
    }

    if field == "to_deposit" && able_to_borrow && able_to_deposit {
        return to_deposit.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
    }

    return "--".to_string();

}
 
pub async fn calculate_repay_plan(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, field: &str, digits_rounded_to: u32) -> String {

    let min_ust_balance = decimal_or_return!(min_ust_balance_to_string(tasks.clone(),false,10).await.as_ref());
    
    let ust_amount_liquid = decimal_or_return!(terra_balance_to_string(tasks.clone(),"uusd",false,10).await.as_ref())
                            .checked_sub(min_ust_balance).unwrap();  

    let ust_balance_preference = decimal_or_return!(ust_balance_preference_to_string(tasks.clone(),false,10).await.as_ref());
    let ust_extra = ust_balance_preference.checked_sub(min_ust_balance).unwrap();

    if field == "ust_available_to_repay" {
        return ust_amount_liquid.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
    }

    let repay_amount = decimal_or_return!(calculate_amount(tasks.clone(),"repay",false,10).await.as_ref());
    
    let zero = Decimal::from_str("0").unwrap();
    let further_funds_needed = ust_amount_liquid.checked_sub(repay_amount).unwrap() < zero;

    if field == "more_funds_required" {
        return further_funds_needed.to_string();
    }
    
    let a_ust_deposit_liquid = decimal_or_return!(borrower_ust_deposited_to_string(tasks.clone(),false,10).await.as_ref());

    if field == "available_in_deposit" {
        return a_ust_deposit_liquid.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
    }

    let sufficient_funds_available = a_ust_deposit_liquid.checked_add(ust_amount_liquid).unwrap().checked_sub(repay_amount).unwrap() >= zero;

    if field == "sufficient_funds_to_repay" {
        return sufficient_funds_available.to_string();
    } 

    let mut to_withdraw_from_account = Decimal::from_str("0").unwrap();
    let mut to_withdraw_from_deposit = Decimal::from_str("0").unwrap();
   
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

    let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());

    let fee_to_redeem_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_redeem_stable","avg_gas_used".to_owned(),false,10).await.as_ref());
    
    let fee_to_redeem_stable = fee_to_redeem_stable_gas_used
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap();


        // if enough funds available the fee_to_redeem_stable will be compensated by withdrawing more.
        // else the fee_to_redeem_stable needs to be paid by the resulting amount, less UST will repay the loan.
 
    let mut fee_to_redeem_stable_pending = false;

    let mut ust_amount_leftover = Decimal::from_str("0").unwrap();
    let mut a_ust_amount_leftover = Decimal::from_str("0").unwrap();

    if ust_amount_liquid >= repay_amount || (ust_amount_liquid > zero && a_ust_deposit_liquid <= zero) { // balance only

        // case 1 only need to use UST balance.
        // case 2 only can use UST balance. 
        to_withdraw_from_account = repay_amount;  

        let leftover = ust_amount_liquid.checked_sub(repay_amount).unwrap();
        if ust_amount_leftover < leftover {
            ust_amount_leftover = leftover;
        }

    }else if a_ust_deposit_liquid > zero  {                                                             // redeem too
        // case 3 need to use aUST deposit in addition to any UST balance.
        if ust_amount_liquid > zero {
            // use all available UST
            to_withdraw_from_account = ust_amount_liquid;    // this goes to the repay amount
                                                             // at this point the fee_to_redeem_stable is not yet accounted 
        }
        // else case 4 only use aUST withdrawal

        let a_ust_demand = repay_amount
                .checked_sub(ust_amount_liquid).unwrap()     // case 3 or 4
                .checked_add(fee_to_redeem_stable).unwrap(); // include fee_to_redeem_stable
            
        fee_to_redeem_stable_pending = true;   

        if a_ust_demand <= a_ust_deposit_liquid {               // enough aUST available
            to_withdraw_from_deposit = a_ust_demand;
            a_ust_amount_leftover = a_ust_deposit_liquid.checked_sub(a_ust_demand).unwrap();
            // fee_to_redeem_stable included in to_withdraw_from_deposit
            // info: should not be included in to_repay
        }else if fee_to_redeem_stable < a_ust_deposit_liquid {  // not enough aUST available, take everything
            to_withdraw_from_deposit = a_ust_deposit_liquid;            
            // fee_to_redeem_stable included in to_withdraw_from_deposit
            // info: should not be included in to_repay
        }/*else {} // not worth since, the fee cancles the amount */
    }

    let mut to_repay = to_withdraw_from_account.checked_add(to_withdraw_from_deposit).unwrap();
    
    if fee_to_redeem_stable_pending {
        to_repay = to_repay.checked_sub(fee_to_redeem_stable).unwrap();
    } 

    let uusd_tax_cap = decimal_or_return!(uusd_tax_cap_to_string(tasks.clone(),false,10).await.as_ref());
    let tax_rate = decimal_or_return!(tax_rate_to_string(tasks.clone(),10).await.as_ref());
        
    let mut stability_tax = to_repay.checked_mul(tax_rate).unwrap();
    if stability_tax > uusd_tax_cap {
        stability_tax = uusd_tax_cap;
    }
  
    let anchor_protocol_txs_repay_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_repay_stable","avg_gas_used".to_owned(),false,10).await.as_ref());
    
    let mut fee_to_repay_stable = anchor_protocol_txs_repay_stable_gas_used 
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            .checked_add(stability_tax).unwrap();

    if ust_amount_leftover > zero {
        if fee_to_repay_stable <= ust_amount_leftover {
            to_withdraw_from_account = to_withdraw_from_account.checked_add(fee_to_repay_stable).unwrap();
            fee_to_repay_stable = Decimal::from_str("0").unwrap();
            ust_amount_leftover = ust_amount_leftover.checked_sub(fee_to_repay_stable).unwrap();
        }else {
            to_withdraw_from_account = to_withdraw_from_account.checked_add(ust_amount_leftover).unwrap();
            fee_to_repay_stable = fee_to_repay_stable.checked_sub(ust_amount_leftover).unwrap();
            ust_amount_leftover = Decimal::from_str("0").unwrap();
        }
    }
    if a_ust_amount_leftover > zero && fee_to_repay_stable > zero {
        if fee_to_repay_stable <= a_ust_amount_leftover {
            to_withdraw_from_deposit = to_withdraw_from_deposit.checked_add(fee_to_repay_stable).unwrap();
            fee_to_repay_stable = Decimal::from_str("0").unwrap();
            a_ust_amount_leftover = a_ust_amount_leftover.checked_sub(fee_to_repay_stable).unwrap();
        }else{
            to_withdraw_from_deposit = to_withdraw_from_deposit.checked_add(a_ust_amount_leftover).unwrap();
            fee_to_repay_stable = fee_to_repay_stable.checked_sub(a_ust_amount_leftover).unwrap();
            a_ust_amount_leftover = Decimal::from_str("0").unwrap();
        }
    }
    if fee_to_repay_stable > zero {  // analog to fee_to_redeem_stable_pending

        // fee_to_repay_stable could not be sourced through leftover UST or aUST. 
        // therefore repaying less
        to_repay = to_repay.checked_sub(fee_to_repay_stable).unwrap();
    }

    // looking to withdraw more UST to maintain the prefered UST balance
    if ust_extra > zero && a_ust_amount_leftover > zero && ust_amount_leftover < ust_balance_preference {
        if a_ust_amount_leftover >= ust_extra {
            to_withdraw_from_deposit = to_withdraw_from_deposit.checked_add(ust_extra).unwrap();
        }else{
            to_withdraw_from_deposit = to_withdraw_from_deposit.checked_add(a_ust_amount_leftover).unwrap();
            //ust_extra_leftover = ust_extra_leftover.checked_sub(a_ust_amount_leftover).unwrap();
        }
    } 

    let total_amount = to_withdraw_from_account.checked_add(to_withdraw_from_deposit).unwrap();
    

    if field == "total_amount" { 
        return total_amount.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string();
    }  
    if field == "to_withdraw_from_account" { 
        return to_withdraw_from_account.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string();
    }  
    if field == "to_withdraw_from_deposit" { 
        return to_withdraw_from_deposit.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string();
    } 
    if field == "to_repay" { 
        return to_repay.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string();
    }  
    if field == "max_stability_tax" {
        return uusd_tax_cap.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
    }
    if field == "stability_tax" {
        return stability_tax.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
    }

    return "--".to_string();
}


pub async fn calculate_farm_plan(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, field: &str, digits_rounded_to: u32) -> String {


    let borrow_anc_rewards = decimal_or_return!(borrower_rewards_to_string(tasks.clone(),true,0).await.as_ref());
   
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

    let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());

    let tx_fee_claim_rewards_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_used".to_owned(),false,10).await.as_ref());   
     
    let fee_to_claim_anc_rewards_uusd = tx_fee_claim_rewards_gas_used 
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap();

    let exchange_rate = decimal_or_return!(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",2).await.as_ref());
    // this does already include the 0.3% terraswap tax fee.

    let borrow_anc_rewards_in_ust = borrow_anc_rewards.checked_div(exchange_rate).unwrap();

    let anc_in_ust_fee_substracted = borrow_anc_rewards_in_ust.checked_sub(fee_to_claim_anc_rewards_uusd).unwrap();
    // assertion >0
    let anc_amount = anc_in_ust_fee_substracted.checked_div(Decimal::from_str("2").unwrap()).unwrap();
    let ust_amount = borrow_anc_rewards.checked_sub(anc_amount).unwrap();
    // does not yet include the swap fee.

    match field {
        "anc_amount" => {
            return anc_amount
                .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
        },
        "ust_amount" => {
            return ust_amount
                .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();
        },
        _ => {
            return "--".to_string();
        }
    }
}



pub async fn calculate_amount(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: &str, as_micro: bool, digits_rounded_to: u32) -> String {
    /* Calculate the repay amount required based on the desired "target_percent" value from user config.
     * target_percent is where ltv will be at once repay is complete.
     */

    let target_percent = decimal_or_return!(target_percentage_to_string(tasks.clone(),10).await.as_ref()); 
    let zero =  Decimal::from_str("0").unwrap(); 
 
    //let trigger_percentage = decimal_or_return!(trigger_percentage_to_string(tasks.clone(),10).await.as_ref());

    let mut _borrow_limit =  Decimal::from_str("0").unwrap(); 

    match get_data_maybe_or_await_task(&tasks,"borrow_limit").await {
        Ok(response_result) => { 
            _borrow_limit = Decimal::from_str(response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string().as_str()).unwrap();
        },
        Err(_) => {
            return "--".to_string();
        }
    }

    let target_loan_amount = _borrow_limit.checked_mul(target_percent).unwrap();

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

    if difference_to_adjust > zero && key == "repay" {
        return difference_to_adjust
                .checked_div(micro).unwrap()
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();

    }else if difference_to_adjust < zero && key == "borrow" {
        return difference_to_adjust
                .checked_mul(Decimal::from_str("-1").unwrap()).unwrap()
                .checked_div(micro).unwrap()
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string();

    }else {
        return "--".to_string();
    }
}

pub async fn check_anchor_loan_status(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str, digits_rounded_to: u32) -> String {
 
    let zero =  Decimal::from_str("0").unwrap(); 

    let trigger_percentage = match key {
        "repay" => {decimal_or_return!(trigger_percentage_to_string(tasks.clone(),10).await.as_ref())},
        "borrow" => {decimal_or_return!(borrow_percentage_to_string(tasks.clone(),10).await.as_ref())},
        &_ => {return "Invalid key".to_string();}
    };

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

    let left_to_trigger = match key {
        "repay" => {trigger_percentage.checked_sub(current_percent).unwrap()},
        "borrow" => {current_percent.checked_sub(trigger_percentage).unwrap()},
        &_ => {return "Invalid key".to_string();}
    };

    if left_to_trigger <= zero {
        return format!("{} due",key);
    }

    return format!("{}% (at {}%)",
        left_to_trigger.checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                       .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string(),
        trigger_percentage.checked_mul(Decimal::from_str("100").unwrap()).unwrap()
          .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string());

}

pub async fn estimate_anchor_protocol_next_claim_and_stake_tx(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, field_amount: &str, field: &str, digits_rounded_to: u32) -> String {
  

            let borrower_rewards_in_ust = decimal_or_return!(borrower_rewards_in_ust_to_string(tasks.clone(),  10).await.as_ref());

            let mut loan_amount = Decimal::from_str("0").unwrap();  

            let borrow_limit = decimal_or_return!(borrow_limit_to_string(tasks.clone(), 10).await.as_ref()); 

            if "loan_amount"==field_amount {
                loan_amount = decimal_or_return!(borrower_loan_amount_to_string(tasks.clone(), 10).await.as_ref()); 
            }else if "target_ltv"==field_amount {
                let trigger_percentage = decimal_or_return!(trigger_percentage_to_string(tasks.clone(),10).await.as_ref());
                loan_amount = borrow_limit.checked_mul(trigger_percentage).unwrap();
            }
            let distribution_apr = percent_decimal_or_return!(distribution_apr_to_string(tasks.clone(),  10).await.as_ref()); 
            let staking_apy = percent_decimal_or_return!(staking_apy_to_string(tasks.clone(),  10).await.as_ref()); 
            let transaction_fee = decimal_or_return!(estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),  10).await.as_ref());
            
            
            let mut _optimal_time_to_wait: Option<Decimal> = None; 
            let mut _optimal_anc_ust_value: Option<Decimal> = None;
            let mut _total_returns_in_ust: Option<Decimal> = None;
  
            let one_year_equals_this_many_time_frames = Decimal::new(365*24,0);
           
            let anc_dist_returns_per_timeframe = distribution_apr.checked_div(one_year_equals_this_many_time_frames).unwrap();
            let anc_dist_returns_per_time_frame_in_ust = loan_amount.checked_mul(anc_dist_returns_per_timeframe).unwrap(); 
            
            let anc_staking_returns_per_timeframe = staking_apy.checked_div(one_year_equals_this_many_time_frames).unwrap();

            let claim_and_stake_gas_fee = Decimal::from_str("-1").unwrap().checked_mul(transaction_fee).unwrap();
            
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
            let time_to_wait_already_passed = borrower_rewards_in_ust
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
                
                let max_ltv = decimal_or_return!(max_ltv_to_string(tasks.clone(), "BLUNA", 2).await.as_ref());
                let collateral_value = borrow_limit.checked_div(max_ltv).unwrap(); 
            
                match _total_returns_in_ust.unwrap().checked_div(collateral_value) {
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

pub async fn estimate_anchor_protocol_tx_fee_claim_and_farm(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
  
  let tx_fee_claim_rewards = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","fee_amount_at_threshold".to_owned(),false,10).await.as_ref());   
  let tx_fee_provide_liquidity = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_provide_liquidity","fee_amount_at_threshold".to_owned(),false,10).await.as_ref());   
  let tx_fee_stake_rewards = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking_lp","fee_amount_at_threshold".to_owned(),false,10).await.as_ref());   
  
  return tx_fee_claim_rewards.checked_add(tx_fee_stake_rewards).unwrap().checked_add(tx_fee_provide_liquidity).unwrap()
                             .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                             .to_string();
}
 

pub async fn estimate_anchor_protocol_tx_fee_claim(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

    let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());

    let tx_fee_claim_rewards_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_used".to_owned(),false,10).await.as_ref());   
    
    return  tx_fee_claim_rewards_gas_used 
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
            .to_string();
}

pub async fn estimate_anchor_protocol_tx_fee_claim_and_stake(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

    let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());

    let tx_fee_claim_rewards_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_used".to_owned(),false,10).await.as_ref());   
    let tx_fee_stake_rewards_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(), "anchor_protocol_txs_staking","avg_gas_used".to_owned(),false,10).await.as_ref());   
  
    return  tx_fee_claim_rewards_gas_used
            .checked_add(tx_fee_stake_rewards_gas_used).unwrap()
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
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
            let mut avg_gas_used = Decimal::from_str("0").unwrap();
            let mut avg_gas_wanted = Decimal::from_str("0").unwrap();
            let mut avg_fee_amount_without_stability_fee = Decimal::from_str("0").unwrap();
            let mut avg_fee_amount_adjusted_without_stability_fee = Decimal::from_str("0").unwrap();
            // estimate_fee_amount = avg_gas_adjustment * avg_gas_used;
            for entry in result {
                let stability_tax = entry.amount.checked_mul(tax_rate).unwrap();
                let mut _fee_amount_without_stability_fee = Decimal::from_str("0").unwrap();
                if stability_tax < tax_cap_uusd {
                    _fee_amount_without_stability_fee = entry.fee_amount.checked_sub(stability_tax).unwrap();
                } else {                    
                    _fee_amount_without_stability_fee = entry.fee_amount.checked_sub(tax_cap_uusd).unwrap();
                }

                // adjusted means the gas_adjustment is part of the fee_amount
                avg_fee_amount_adjusted_without_stability_fee = avg_fee_amount_adjusted_without_stability_fee.checked_add(_fee_amount_without_stability_fee).unwrap();

                // we can not know the real gas adjustment, but this is a good guess
                let gas_adjustment = entry.gas_wanted.checked_div(entry.gas_used).unwrap(); 
                
                // removing the gas_adjustment that was applied
                _fee_amount_without_stability_fee = _fee_amount_without_stability_fee.checked_div(gas_adjustment).unwrap();
                avg_fee_amount_without_stability_fee = avg_fee_amount_without_stability_fee.checked_add(_fee_amount_without_stability_fee).unwrap();

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
                    avg_gas_adjustment = avg_gas_adjustment/*.checked_div(gas_fees_uusd).unwrap()*/.checked_div(count_entries).unwrap();
                    avg_gas_used = avg_gas_used.checked_div(count_entries).unwrap();
                    avg_gas_wanted = avg_gas_wanted.checked_div(count_entries).unwrap();
                    let fee_amount_at_threshold = avg_gas_used.checked_mul(gas_fees_uusd).unwrap();
                    let fee_amount_adjusted = avg_gas_used.checked_mul(gas_fees_uusd).unwrap().checked_mul(avg_gas_adjustment).unwrap();
                    
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
                            return fee_amount_adjusted
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
 
    let borrow_limit = decimal_or_return!(borrow_limit_to_string(tasks.clone(), 10).await.as_ref());
          
    let mut loan_amount = Decimal::from_str("0").unwrap();  

    if amount_field == "loan_amount" {
        loan_amount = decimal_or_return!(borrower_loan_amount_to_string(tasks.clone(), 10).await.as_ref()); 

    }else if amount_field == "deposit_amount" {
        loan_amount = decimal_or_return!(borrower_ust_deposited_to_string(tasks.clone(),false, 10).await.as_ref()); 

    }else if amount_field == "target_ltv" { 
        loan_amount = decimal_or_return!(trigger_percentage_to_string(tasks.clone(),10).await.as_ref());
    }

    let mut apr = Decimal::from_str("0").unwrap();

    if "net_apr" == apr_field { 
        let net_apr = percent_decimal_or_return!(net_apr_to_string(tasks.clone(),  10).await.as_ref()); 
        let earn_apr = percent_decimal_or_return!(earn_apr_to_string(tasks.clone(),  10).await.as_ref()); 
        apr = net_apr.checked_add(earn_apr).unwrap();
    }else if "earn_apr" == apr_field { 
        apr = percent_decimal_or_return!(earn_apr_to_string(tasks.clone(),  10).await.as_ref());
    }else if "borrow_apr"== apr_field {
        apr = percent_decimal_or_return!(borrow_apr_to_string(tasks.clone(),  10).await.as_ref());
    }else if "distribution_apr" == apr_field {
        apr = percent_decimal_or_return!(distribution_apr_to_string(tasks.clone(),  10).await.as_ref());
    }


    let max_ltv = decimal_or_return!(max_ltv_to_string(tasks.clone(), "BLUNA", 2).await.as_ref());
    let collateral_value = borrow_limit.checked_div(max_ltv).unwrap(); 

    match apr
            .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
            .checked_mul(loan_amount).unwrap().checked_div(collateral_value) {
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


    let anchor_protocol_tx_fee = decimal_or_return!(estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),  10).await.as_ref());    
      
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
/*
    let max_ltv = decimal_or_return!(max_ltv_to_string(tasks.clone(), "BLUNA", 2).await.as_ref());
    let collateral_value = _borrow_limit.checked_div(max_ltv).unwrap();
*/
    match _loan_amount.checked_div(_borrow_limit) {
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

pub async fn anchor_airdrops_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>) -> String {  
        match get_data_maybe_or_meta_data_maybe(&tasks,"anchor_airdrops").await {
            Ok(res) => {
                let anchor_airdrops = res.as_airdrop_response().unwrap();
                let mut amount_unclaimed: u64 = 0;
                let mut amount_claimed: u64 = 0;
                for i in 0..anchor_airdrops.len() {
                    if anchor_airdrops[i].claimable {
                        amount_unclaimed += anchor_airdrops[i].amount.parse::<u64>().unwrap_or(0u64);
                    }else{
                        amount_claimed += anchor_airdrops[i].amount.parse::<u64>().unwrap_or(0u64);
                    }
                }

                let micro = Decimal::from_str("1000000").unwrap();  
                let amount_unclaimed = Decimal::from_str(amount_unclaimed.to_string().as_str()).unwrap()
                                        .checked_div(micro).unwrap() 
                                        .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
                   
                let amount_claimed = Decimal::from_str(amount_claimed.to_string().as_str()).unwrap()
                                        .checked_div(micro).unwrap() 
                                        .round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
                return format!("available to claim: {}, amount already claimed: {}",amount_unclaimed,amount_claimed);
            },
            Err(err) => {
                return format!("{:?}",err);
            }
        } 

}

pub async fn anything_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: &str) -> String { 
     
        match get_data_maybe_or_meta_data_maybe(&tasks,key).await {
            Ok(res) => {
               return serde_json::to_string_pretty(&res).unwrap_or("--".to_string());
            },
            Err(err) => {
                return format!("{:?}",err);
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
     
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

    let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());

    let fee_to_redeem_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_redeem_stable","avg_gas_used".to_owned(),false,10).await.as_ref());
    
    let anchor_protocol_txs_repay_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_repay_stable","avg_gas_used".to_owned(),false,10).await.as_ref());
    // min(to_repay * tax_rate , tax_cap)
    let stability_tax = decimal_or_return!(calculate_repay_plan(tasks.clone(),"stability_tax",10).await.as_ref());

    return fee_to_redeem_stable_gas_used
            .checked_add(anchor_protocol_txs_repay_stable_gas_used).unwrap()
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            .checked_add(stability_tax).unwrap()
            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
}

pub async fn estimate_anchor_protocol_auto_borrow_tx_fee(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
   
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(tasks.clone(),10).await.as_ref());

    let gas_adjustment_preference = decimal_or_return!(gas_adjustment_preference_to_string(tasks.clone(),10).await.as_ref());

    let anchor_protocol_txs_borrow_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_borrow_stable","avg_gas_used".to_owned(),false,10).await.as_ref());
    // min(to_repay * tax_rate , tax_cap)
    let stability_tax_borrow = decimal_or_return!(calculate_borrow_plan(tasks.clone(),"stability_tax_borrow",10).await.as_ref());

    let anchor_protocol_txs_deposit_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_deposit_stable","avg_gas_used".to_owned(),false,10).await.as_ref());
    // min(to_repay * tax_rate , tax_cap) 
    let stability_tax_deposit = decimal_or_return!(calculate_borrow_plan(tasks.clone(),"stability_tax_deposit",10).await.as_ref());

    return  anchor_protocol_txs_borrow_stable_gas_used
            .checked_add(anchor_protocol_txs_deposit_stable_gas_used).unwrap()
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            .checked_add(stability_tax_borrow).unwrap() 
            .checked_add(stability_tax_deposit).unwrap() 
            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string();
}