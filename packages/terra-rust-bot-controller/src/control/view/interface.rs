use model::{MaybeOrPromise,get_data_maybe_or_await_task,get_meta_data_maybe_or_await_task};  

use std::collections::HashMap;
use rust_decimal::Decimal;
use core::str::FromStr; 
use num_format::{Locale, ToFormattedString}; 
use std::sync::Arc; 
use tokio::sync::RwLock; 

pub mod model;

macro_rules! try_get_data_by_key {
    ( $e:expr, $d:expr ) => {
        match get_data_maybe_or_await_task($e,$d).await {
            Ok(response_result) => response_result,
            Err(_) => return "--".to_string(),
        }
    }
}
macro_rules! try_get_meta_data_by_key {
    ( $e:expr, $d:expr ) => {
        match get_meta_data_maybe_or_await_task($e,$d).await {
            Ok(response_result) => response_result,
            Err(_) => return "--".to_string(),
        }
    }
} 
macro_rules! try_convert_and_round {
    ( $number:expr, $type:expr, $as_micro:expr, $digits_rounded_to:expr ) => {
        match $number.as_str() {
            "--" => {
                $number.to_string()
            },
            number => {
                let action = match $type {
                    "is_human_readable" => {
                         match $as_micro {
                            true => "mul",
                            false => "none",
                        }
                    },
                    "is_micro" => {
                         match $as_micro {
                            false => "div",
                            true => "none", 
                        }
                    },
                    &_ => {"none"},
                };
                let value = match action {
                    "div" => {
                        Decimal::from_str(number).unwrap()
                        .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
                        .to_string()
                    },
                    "mul" => {
                        Decimal::from_str(number).unwrap()
                        .checked_mul(Decimal::from_str("1000000").unwrap()).unwrap()
                        .to_string()
                    },
                    &_ => {
                        $number.to_string()
                    },
                };
                Decimal::from_str(value.as_str()).unwrap() 
                .round_dp_with_strategy($digits_rounded_to, rust_decimal::RoundingStrategy::ToZero) // rust_decimal::RoundingStrategy::MidpointAwayFromZero
                .to_string()    
            },
        }
    }
}
macro_rules! to_percentage_and_round {
    ( $number:expr, $digits_rounded_to:expr ) => {
        match $number.as_str() {
            "--" => {
                $number.to_string()
            },
            number => {
                format!("{}%",
                     Decimal::from_str(number).unwrap() 
                    .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                    .round_dp_with_strategy($digits_rounded_to, rust_decimal::RoundingStrategy::ToZero)
                    .to_string()
                    ) 
            },
        }
    }
}

pub async fn tax_rate_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    let response_result = try_get_data_by_key!(&tasks,"tax_rate");  
    let value = &response_result.as_tax_rate().unwrap().result;
    try_convert_and_round!(&value,"is_human_readable",false, digits_rounded_to)
}

pub async fn uusd_tax_cap_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,as_micro:bool, digits_rounded_to: u32) -> String { 
    let response_result = try_get_data_by_key!(&tasks,"tax_caps");  
    let vec_tax_caps = &response_result.as_tax_caps().unwrap().result;
    for tax_cap in vec_tax_caps {
        if tax_cap.denom == "uusd".to_string() {  
            return try_convert_and_round!(&tax_cap.tax_cap,"is_micro",as_micro,digits_rounded_to);  
        }
    }                   
    return "--".to_string();
}
/**
 * min_ust_balance
 * ust_balance_preference 
 * gas_adjustment_preference
 * max_gas_adjustment
 * target_percentage
 * borrow_percentage
 * trigger_percentage
 * max_tx_fee  
 * */
pub async fn meta_data_key_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: &str, as_micro: bool, digits_rounded_to: u32) -> String { 
    let response_result = try_get_meta_data_by_key!(&tasks,key);  
    try_convert_and_round!(&response_result,"is_human_readable",as_micro,digits_rounded_to)    
}

pub async fn anc_staked_balance_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
    let response_result = try_get_data_by_key!(&tasks,"staker");  
    let value = response_result.as_staker().unwrap().result.balance.to_string();
    try_convert_and_round!(&value,"is_micro",false,digits_rounded_to)       
}

pub async fn borrower_rewards_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, as_micro: bool, digits_rounded_to: u32) -> String { 
    let response_result = try_get_data_by_key!(&tasks,"borrow_info");  
    let value = response_result.as_borrow_info().unwrap().result.pending_rewards.to_string();
    try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to)         
}

pub async fn borrower_anc_deposited_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, as_micro: bool, digits_rounded_to: u32) -> String { 
    let response_result = try_get_data_by_key!(&tasks,"anc_balance");  
    let value = response_result.as_balance().unwrap().result.balance.to_string();
    try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to) 
}

pub async fn borrower_balance_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, as_micro: bool, digits_rounded_to: u32) -> String { 
    let response_result = try_get_data_by_key!(&tasks,"balance");  
    let value = response_result.as_balance().unwrap().result.balance.to_string();
    try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to) 
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

pub async fn borrower_loan_amount_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, as_micro: bool, digits_rounded_to: u32) -> String { 
    let response_result = try_get_data_by_key!(&tasks,"borrow_info");  
    let value = response_result.as_borrow_info().unwrap().result.loan_amount.to_string();
    try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to) 
}

pub async fn borrow_limit_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, as_micro: bool, digits_rounded_to: u32) -> String { 
    let response_result = try_get_data_by_key!(&tasks,"borrow_limit");  
    let value = response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string();
    try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to) 
}

pub async fn earn_apr_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"earn_apy");  
        let value = response_result.as_earn_apy().unwrap().result.apy.to_string();
        to_percentage_and_round!(&value,digits_rounded_to)
}

pub async fn distribution_apr_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String {  
        let response_result = try_get_data_by_key!(&tasks,"api/v2/distribution-apy");  
        let value = response_result.as_distribution_apy().unwrap().distribution_apy.to_string();
        to_percentage_and_round!(&value,digits_rounded_to)
}

pub async fn gas_price_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_meta_data_by_key!(&tasks,"gas_fees_uusd");  
        let value = response_result;
        try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to) 
}

pub async fn spec_anc_ust_lp_apy_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"api/data?type=lpVault");  
        let dpr = response_result.as_spec_astro_vault().unwrap().dpr.to_string();
        let value = Decimal::from_str(dpr.as_str()).unwrap()
                    .checked_mul(Decimal::from_str("365").unwrap()).unwrap()
                    .to_string();
        to_percentage_and_round!(&value,digits_rounded_to)
}

pub async fn anc_ust_lp_apy_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"api/v2/ust-lp-reward");  
        let value = response_result.as_lp_reward().unwrap().apy.to_string();
        to_percentage_and_round!(&value,digits_rounded_to)
}

pub async fn staking_apy_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"api/v2/gov-reward");  
        let value = response_result.as_gov_reward().unwrap().current_apy.to_string();
        to_percentage_and_round!(&value,digits_rounded_to)
}

pub async fn max_ltv_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>,key: &str, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"anchor_protocol_whitelist"); 
        let anchor_asstes = &response_result.as_anchor_whitelist_response().unwrap().result.elems;
        for i in 0..anchor_asstes.len() {
            if &anchor_asstes[i].symbol == key {
                let value = &anchor_asstes[i].max_ltv.to_string();
                return try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to);
            }
        } 
        "--".to_string()
}

pub async fn interest_multiplier_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"config anchorprotocol mmInterestModel");  
        let value = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.interest_multiplier.to_string();
        try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to) 
}

pub async fn blocks_per_year_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"blocks_per_year");  
        let value = response_result.as_blocks().unwrap().result.blocks_per_year.to_string();
        try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to) 
}    

pub async fn base_rate_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"config anchorprotocol mmInterestModel");  
        let value = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.base_rate.to_string();
        try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to) 
}

pub async fn a_terra_supply_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"epoch_state anchorprotocol mmMarket");  
        let res = &response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result;  
        let value = (res.aterra_supply / res.exchange_rate).to_string();  
        try_convert_and_round!(&value,"is_micro",false,digits_rounded_to).parse::<u128>().unwrap().to_formatted_string(&Locale::en)
}

pub async fn a_terra_exchange_rate_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"epoch_state anchorprotocol mmMarket");  
        let value = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate.to_string();
        try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to) 
}

pub async fn b_luna_exchange_rate_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"state anchorprotocol bLunaHub");  
        let value = response_result.as_state().unwrap().as_b_luna_hub().unwrap().result.bluna_exchange_rate.to_string();
        try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to) 
}

pub async fn total_liabilities_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,"state anchorprotocol mmMarket");  
        let value = response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities.to_string();
        try_convert_and_round!(&value,"is_micro",false,digits_rounded_to).parse::<u128>().unwrap().to_formatted_string(&Locale::en)
}

pub async fn simulation_swap_exchange_rate_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: &str, as_micro: bool, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,key);  
        let res = &response_result.as_simulation().unwrap().result; 
        let value = (res.return_amount+res.commission_amount).to_string();
        try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to) 
}

pub async fn simulation_swap_return_amount_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: &str, as_micro: bool, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,key);  
        let value = response_result.as_simulation().unwrap().result.return_amount.to_string();
        try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to) 
}

pub async fn core_swap_amount_to_string(tasks: Arc<RwLock<HashMap<String, MaybeOrPromise>>>, key: &str, digits_rounded_to: u32) -> String { 
        let response_result = try_get_data_by_key!(&tasks,key);  
        let value = response_result.as_core_swap().unwrap().result.amount.to_string();
        try_convert_and_round!(&value,"is_micro",false,digits_rounded_to) 
} 