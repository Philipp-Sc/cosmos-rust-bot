#![allow(dead_code)]


// View -> Model
// Readonly access to everything that is within the tasks hashmap (the model/state).


pub mod interface;


use crate::state::control::model::{Maybe, try_get_resolved};

use interface::*;
use interface_macro::maybe_struct;

use std::collections::HashMap;

use rust_decimal::Decimal;
use core::str::FromStr;

//use num_format::{Locale, ToFormattedString}; 

use chrono::{Utc};


use std::sync::Arc;
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::ResponseResult;
use terra_rust_api_layer::utils::*;
use tokio::sync::Mutex;


pub mod view_macro {
    macro_rules! decimal_or_return {
        ( $e:expr ) => {
            match $e {
                Maybe{data: Ok(data),..} => {
                    match Decimal::from_str(data.as_str()) {
                        Err(err) => {
                            return Maybe::<String>{data:Err(anyhow::anyhow!(err)),timestamp:$e.timestamp}
                            },
                        Ok(d) => d,
                    }
                },
                Maybe{data: Err(err),..} => {
                   return Maybe::<String>{data:Err(anyhow::anyhow!(err)),timestamp:$e.timestamp}
                }
            }
        }
    }
    pub(crate) use decimal_or_return;
}

use view_macro::decimal_or_return;


macro_rules! percent_decimal_or_return {
        ( $e:expr ) => {
            match $e {
                Maybe{data: Err(err),..} => {
                   return Maybe::<String>{data:Err(anyhow::anyhow!(err)),timestamp:$e.timestamp}
                }
                Maybe{data: Ok(data),..} => {
                    let mut chars = data.chars();
                    chars.next_back();
                    match Decimal::from_str(chars.as_str()) {
                        Err(err) => {
                            return Maybe::<String>{data:Err(anyhow::anyhow!(err)),timestamp:$e.timestamp}
                            },
                        Ok(d) => d.checked_div(Decimal::from_str("100").unwrap()).unwrap(),
                    }
                },
            }
        }
    }


// everything returns a maybe<string>
// on ok timestamp is now.
// on error timestamp is from key.

// match all statements: (?s)(return)((\s)+?([^=_]).*?)(?=;)
// replace index: None,group: return maybe_struct!((Some($2),Some(Utc::now().timestamp())));
// maybe_struct!((Some(data),Some(Utc::now().timestamp())))

// we do this so we know for each key when it became resolved
// so we can see how old each data point is.
// if data is to old throw alerts

pub async fn calculate_borrow_plan(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, field: &str, digits_rounded_to: u32) -> Maybe<String> {
    let mut ust_amount_liquid = decimal_or_return!(terra_balance_to_string(maybes.clone(),"uusd",false,10).await);
    let min_ust_balance = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"min_ust_balance",false,10).await);
    ust_amount_liquid = ust_amount_liquid.checked_sub(min_ust_balance).unwrap();

    if field == "ust_available_to_pay_fees" {
        return maybe_struct!((Some( ust_amount_liquid.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
    }

    let borrow_amount = decimal_or_return!(calculate_amount(maybes.clone(),"borrow",false,10).await);

    let uusd_tax_cap = decimal_or_return!(uusd_tax_cap_to_string(maybes.clone(),false,10).await);
    let tax_rate = decimal_or_return!(tax_rate_to_string(maybes.clone(),10).await);

    let mut stability_tax = borrow_amount.checked_mul(tax_rate).unwrap();
    if stability_tax > uusd_tax_cap {
        stability_tax = uusd_tax_cap;
    }

    if field == "stability_tax_borrow" {
        return maybe_struct!((Some( stability_tax.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
    }

    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

    let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false,10).await);

    let anchor_protocol_txs_borrow_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(),"anchor_protocol_txs_borrow_stable","avg_gas_used".to_owned(),false,10).await);

    let borrow_stable_fee = anchor_protocol_txs_borrow_stable_gas_used
        .checked_mul(gas_fees_uusd).unwrap()
        .checked_mul(gas_adjustment_preference).unwrap()
        .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
        .checked_add(stability_tax).unwrap();

    let able_to_borrow = ust_amount_liquid >= borrow_stable_fee;

    let anchor_protocol_txs_deposit_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(),"anchor_protocol_txs_deposit_stable","avg_gas_used".to_owned(),false,10).await);

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
        return maybe_struct!((Some( stability_tax.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
    }

    if field == "to_deposit" && able_to_borrow && able_to_deposit {
        return maybe_struct!((Some( to_deposit.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
    }

    return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
}

pub async fn calculate_repay_plan(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, field: &str, digits_rounded_to: u32) -> Maybe<String> {
    let min_ust_balance = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"min_ust_balance",false,10).await);

    let ust_amount_liquid = decimal_or_return!(terra_balance_to_string(maybes.clone(),"uusd",false,10).await)
        .checked_sub(min_ust_balance).unwrap();

    let ust_balance_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"ust_balance_preference",false,10).await);
    let ust_extra = ust_balance_preference.checked_sub(min_ust_balance).unwrap();

    if field == "ust_available_to_repay" {
        return maybe_struct!((Some( ust_amount_liquid.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
    }

    let repay_amount = decimal_or_return!(calculate_amount(maybes.clone(),"repay",false,10).await);

    let zero = Decimal::from_str("0").unwrap();
    let further_funds_needed = ust_amount_liquid.checked_sub(repay_amount).unwrap() < zero;

    if field == "more_funds_required" {
        return maybe_struct!((Some( further_funds_needed.to_string()),Some(Utc::now().timestamp())));
    }

    let a_ust_deposit_liquid = decimal_or_return!(borrower_ust_deposited_to_string(maybes.clone(),false,10).await);

    if field == "available_in_deposit" {
        return maybe_struct!((Some( a_ust_deposit_liquid.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
    }

    let sufficient_funds_available = a_ust_deposit_liquid.checked_add(ust_amount_liquid).unwrap().checked_sub(repay_amount).unwrap() >= zero;

    if field == "sufficient_funds_to_repay" {
        return maybe_struct!((Some( sufficient_funds_available.to_string()),Some(Utc::now().timestamp())));
    }

    let mut to_withdraw_from_account = Decimal::from_str("0").unwrap();
    let mut to_withdraw_from_deposit = Decimal::from_str("0").unwrap();

    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

    let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false,10).await);

    let fee_to_redeem_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(),"anchor_protocol_txs_redeem_stable","avg_gas_used".to_owned(),false,10).await);

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
    } else if a_ust_deposit_liquid > zero {                                                             // redeem too
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
        } else if fee_to_redeem_stable < a_ust_deposit_liquid {  // not enough aUST available, take everything
            to_withdraw_from_deposit = a_ust_deposit_liquid;
            // fee_to_redeem_stable included in to_withdraw_from_deposit
            // info: should not be included in to_repay
        }/*else {} // not worth since, the fee cancles the amount */
    }

    let mut to_repay = to_withdraw_from_account.checked_add(to_withdraw_from_deposit).unwrap();

    if fee_to_redeem_stable_pending {
        to_repay = to_repay.checked_sub(fee_to_redeem_stable).unwrap();
    }

    let uusd_tax_cap = decimal_or_return!(uusd_tax_cap_to_string(maybes.clone(),false,10).await);
    let tax_rate = decimal_or_return!(tax_rate_to_string(maybes.clone(),10).await);

    let mut stability_tax = to_repay.checked_mul(tax_rate).unwrap();
    if stability_tax > uusd_tax_cap {
        stability_tax = uusd_tax_cap;
    }

    let anchor_protocol_txs_repay_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(),"anchor_protocol_txs_repay_stable","avg_gas_used".to_owned(),false,10).await);

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
        } else {
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
        } else {
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
        } else {
            to_withdraw_from_deposit = to_withdraw_from_deposit.checked_add(a_ust_amount_leftover).unwrap();
            //ust_extra_leftover = ust_extra_leftover.checked_sub(a_ust_amount_leftover).unwrap();
        }
    }

    let total_amount = to_withdraw_from_account.checked_add(to_withdraw_from_deposit).unwrap();


    if field == "total_amount" {
        return maybe_struct!((Some( total_amount.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string()),Some(Utc::now().timestamp())));
    }
    if field == "to_withdraw_from_account" {
        return maybe_struct!((Some( to_withdraw_from_account.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string()),Some(Utc::now().timestamp())));
    }
    if field == "to_withdraw_from_deposit" {
        return maybe_struct!((Some( to_withdraw_from_deposit.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string()),Some(Utc::now().timestamp())));
    }
    if field == "to_repay" {
        return maybe_struct!((Some( to_repay.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string()),Some(Utc::now().timestamp())));
    }
    if field == "max_stability_tax" {
        return maybe_struct!((Some( uusd_tax_cap.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
    }
    if field == "stability_tax" {
        return maybe_struct!((Some( stability_tax.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
    }

    return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
}


pub async fn calculate_farm_plan(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, field: &str, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);
    let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false,10).await);

    let exchange_rate = decimal_or_return!(simulation_swap_exchange_rate_to_string(maybes.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,10).await);

    let fee_to_claim_anc_rewards_uusd_in_anc = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_used".to_owned(),true,0).await)
        .checked_mul(gas_fees_uusd).unwrap()
        .checked_mul(gas_adjustment_preference).unwrap()
        .checked_mul(exchange_rate).unwrap();


    let fee_to_provide_in_anc = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "txs_provide_to_spec_anc_ust_vault","avg_gas_used".to_owned(),true,0).await)
        .checked_mul(gas_fees_uusd).unwrap()
        .checked_mul(gas_adjustment_preference).unwrap()
        .checked_mul(exchange_rate).unwrap();

    let fees_in_anc = fee_to_claim_anc_rewards_uusd_in_anc.checked_add(fee_to_provide_in_anc).unwrap();

    let borrow_anc_rewards = decimal_or_return!(borrower_rewards_to_string(maybes.clone(),true,0).await);


    let borrow_anc_rewards_fees_estimate_substracted = borrow_anc_rewards.checked_sub(fees_in_anc).unwrap();

    let pair_anc_excluding_exchange_fees = borrow_anc_rewards_fees_estimate_substracted.checked_div(Decimal::from_str("2").unwrap()).unwrap();

    // if pair_anc is used to swap that amount of anc
    // then the ust returned is lower than the remaining ANC value

    let exchange_return = decimal_or_return!(simulation_swap_return_amount_to_string(maybes.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,10).await);
    let return_ust_amount = pair_anc_excluding_exchange_fees.checked_div(exchange_return).unwrap();
    let return_amount_in_anc = return_ust_amount.checked_mul(exchange_rate).unwrap();

    let difference = pair_anc_excluding_exchange_fees.checked_sub(return_amount_in_anc).unwrap();

    let anc_to_swap = pair_anc_excluding_exchange_fees
        .checked_add(difference).unwrap()
        .checked_add(fees_in_anc).unwrap();
    // this way the UST side of the pair's amount is always higher
    // it includes the amount for the claim fee (est.)
    // it includes the swap transaction fee (est.)

    let anc_to_keep = borrow_anc_rewards.checked_sub(anc_to_swap).unwrap();

    // everything available for claim_and_swap_for_lp

    // check if enough UST balance available to make sure the UST part of the Pair can be fulfilled.

    let mut micro = Decimal::from_str("1").unwrap();
    if !as_micro {
        micro = Decimal::from_str("1000000").unwrap();
    }


    match field {
        "anc_to_keep" => {
            return maybe_struct!((Some( anc_to_keep
                .checked_div(micro).unwrap()
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
        },
        "anc_to_swap" => {
            return maybe_struct!((Some( anc_to_swap
                .checked_div(micro).unwrap()
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
        },
        _ => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }
}


pub async fn calculate_amount(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: &str, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    /* Calculate the repay amount required based on the desired "target_percent" value from user config.
     * target_percent is where ltv will be at once repay is complete.
     */

    let target_percent = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"target_percentage",false,10).await);
    let zero = Decimal::from_str("0").unwrap();

    //let trigger_percentage = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"trigger_percentage",false,10).await);

    let mut _borrow_limit = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "borrow_limit").await {
        Maybe { data: Ok(response_result), .. } => {
            _borrow_limit = Decimal::from_str(response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string().as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let target_loan_amount = _borrow_limit.checked_mul(target_percent).unwrap();

    let mut _loan_amount = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "borrow_info").await {
        Maybe { data: Ok(response_result), .. } => {
            _loan_amount = Decimal::from_str(response_result.as_borrow_info().unwrap().result.loan_amount.to_string().as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    if _borrow_limit <= zero || _loan_amount <= zero {
        return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
    }

    let difference_to_adjust = _loan_amount.checked_sub(target_loan_amount).unwrap();

    let mut micro = Decimal::from_str("1").unwrap();
    if !as_micro {
        micro = Decimal::from_str("1000000").unwrap();
    }

    if difference_to_adjust > zero && key == "repay" {
        return maybe_struct!((Some( difference_to_adjust
                .checked_div(micro).unwrap()
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
    } else if difference_to_adjust < zero && key == "borrow" {
        return maybe_struct!((Some( difference_to_adjust
                .checked_mul(Decimal::from_str("-1").unwrap()).unwrap()
                .checked_div(micro).unwrap()
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::AwayFromZero).to_string()),Some(Utc::now().timestamp())));
    } else {
        return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
    }
}

pub async fn check_anchor_loan_status(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: &str, digits_rounded_to: u32) -> Maybe<String> {
    let zero = Decimal::from_str("0").unwrap();

    let trigger_percentage = match key {
        "repay" => { decimal_or_return!(meta_data_key_to_string(maybes.clone(),"trigger_percentage",false,10).await) },
        "borrow" => { decimal_or_return!(meta_data_key_to_string(maybes.clone(),"borrow_percentage",false,10).await) },
        &_ => { return maybe_struct!((Some( "Invalid key".to_string()),Some(Utc::now().timestamp()))); }
    };

    let mut _borrow_limit = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "borrow_limit").await {
        Maybe { data: Ok(response_result), .. } => {
            _borrow_limit = Decimal::from_str(response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string().as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let mut _loan_amount = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "borrow_info").await {
        Maybe { data: Ok(response_result), .. } => {
            _loan_amount = Decimal::from_str(response_result.as_borrow_info().unwrap().result.loan_amount.to_string().as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    if _borrow_limit <= zero || _loan_amount <= zero {
        return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
    }

    let current_percent = _loan_amount.checked_div(_borrow_limit).unwrap();

    let left_to_trigger = match key {
        "repay" => { trigger_percentage.checked_sub(current_percent).unwrap() },
        "borrow" => { current_percent.checked_sub(trigger_percentage).unwrap() },
        &_ => { return maybe_struct!((Some( "Invalid key".to_string()),Some(Utc::now().timestamp()))); }
    };

    if left_to_trigger <= zero {
        return maybe_struct!((Some( format!("{} due",key)),Some(Utc::now().timestamp())));
    }

    return maybe_struct!((Some( format!("{}% (at {}%)",
        left_to_trigger.checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                       .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string(),
        trigger_percentage.checked_mul(Decimal::from_str("100").unwrap()).unwrap()
          .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero).to_string())),Some(Utc::now().timestamp())));
}

pub async fn estimate_anchor_protocol_next_claim_and_stake_tx(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, field_type: &str, field_amount: &str, field: &str, digits_rounded_to: u32) -> Maybe<String> {
    let mut loan_amount = Decimal::from_str("0").unwrap();

    if "loan_amount" == field_amount {
        loan_amount = decimal_or_return!(borrower_loan_amount_to_string(maybes.clone(),false, 10).await);
    } else if "target_ltv" == field_amount {
        let trigger_percentage = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"trigger_percentage",false,10).await);
        let borrow_limit = decimal_or_return!(borrow_limit_to_string(maybes.clone(), false, 10).await);
        loan_amount = borrow_limit.checked_mul(trigger_percentage).unwrap();
    }
    let pending_rewards_in_ust = decimal_or_return!(borrower_rewards_in_ust_to_string(maybes.clone(),  10).await);
    let distribution_apr = percent_decimal_or_return!(distribution_apr_to_string(maybes.clone(),  10).await);
    let pool_apy = match field_type {
        "staking" => { percent_decimal_or_return!(staking_apy_to_string(maybes.clone(),  10).await) },
        "farming" => { percent_decimal_or_return!(spec_anc_ust_lp_apy_to_string(maybes.clone(),  10).await) },
        _ => { return maybe_struct!((Some( "Error".to_string()),Some(Utc::now().timestamp()))); }
    };
    let transaction_fee = match field_type {
        "staking" => { decimal_or_return!(estimate_anchor_protocol_tx_fee_claim_and_stake(maybes.clone(),  10).await) },
        "farming" => { decimal_or_return!(estimate_anchor_protocol_tx_fee_claim_and_provide_to_spec_vault(maybes.clone(),  10).await) },
        _ => { return maybe_struct!((Some( "Error".to_string()),Some(Utc::now().timestamp()))); }
    };

    let res = estimate_optimal_next_claim_and_stake_tx(loan_amount, pending_rewards_in_ust, distribution_apr, pool_apy, transaction_fee, digits_rounded_to);

    let now = Some(Utc::now().timestamp());
    maybe_struct!((Some(res[field].as_str().unwrap_or("--").to_string()),now))
}

pub async fn estimate_anchor_protocol_tx_fee_claim_and_farm(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let tx_fee_claim_rewards = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_claim_rewards","fee_amount_at_threshold".to_owned(),false,10).await);
    let tx_fee_provide_liquidity = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_provide_liquidity","fee_amount_at_threshold".to_owned(),false,10).await);
    let tx_fee_stake_rewards = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_staking_lp","fee_amount_at_threshold".to_owned(),false,10).await);

    return maybe_struct!((Some( tx_fee_claim_rewards.checked_add(tx_fee_stake_rewards).unwrap().checked_add(tx_fee_provide_liquidity).unwrap()
                             .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                             .to_string()),Some(Utc::now().timestamp())));
}

pub async fn estimate_spec_tx_fee_provide(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

    let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false,10).await);

    let tx_fee_claim_rewards_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "txs_provide_to_spec_anc_ust_vault","avg_gas_used".to_owned(),false,10).await);

    return maybe_struct!((Some(  tx_fee_claim_rewards_gas_used
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
            .to_string()),Some(Utc::now().timestamp())));
}

pub async fn estimate_anchor_protocol_tx_fee_claim(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

    let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false,10).await);

    let tx_fee_claim_rewards_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_used".to_owned(),false,10).await);

    return maybe_struct!((Some(  tx_fee_claim_rewards_gas_used
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
            .to_string()),Some(Utc::now().timestamp())));
}


pub async fn estimate_anchor_protocol_tx_fee_claim_and_provide_to_spec_vault(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);
    let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false,10).await);

    let fee_to_claim_anc_rewards_uusd = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_used".to_owned(),false,10).await);
    let fee_to_provide = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "txs_provide_to_spec_anc_ust_vault","avg_gas_used".to_owned(),false,10).await);

    return maybe_struct!((Some( fee_to_claim_anc_rewards_uusd
            .checked_add(fee_to_provide).unwrap()
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
            .to_string()),Some(Utc::now().timestamp())));
}

pub async fn estimate_anchor_protocol_tx_fee_claim_and_stake(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

    let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false,10).await);

    let tx_fee_claim_rewards_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_claim_rewards","avg_gas_used".to_owned(),false,10).await);
    let tx_fee_stake_rewards_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(), "anchor_protocol_txs_staking","avg_gas_used".to_owned(),false,10).await);

    return maybe_struct!((Some(  tx_fee_claim_rewards_gas_used
            .checked_add(tx_fee_stake_rewards_gas_used).unwrap()
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
            .to_string()),Some(Utc::now().timestamp())));
}

pub async fn estimate_anchor_protocol_tx_fee(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, tx_key: &str, key: String, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let mut tax_rate = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "tax_rate").await {
        Maybe { data: Ok(response_result), .. } => {
            tax_rate = Decimal::from_str(response_result.as_tax_rate().unwrap().result.as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {}
    }

    let mut tax_cap_uusd = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "tax_caps").await {
        Maybe { data: Ok(response_result), .. } => {
            let vec_tax_caps = &response_result.as_tax_caps().unwrap().result;
            for tax_cap in vec_tax_caps {
                if tax_cap.denom == "uusd".to_string() {
                    tax_cap_uusd = Decimal::from_str(tax_cap.tax_cap.as_str()).unwrap();
                    break;
                }
            }
        },
        Maybe { data: Err(_), .. } => {}
    }

    match try_get_resolved(&maybes, tx_key).await {
        Maybe { data: Ok(response_result), .. } => {
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
            match try_get_resolved(&maybes, "gas_fees_uusd").await {
                Maybe { data: Ok(ResponseResult::Text(response_result)), .. } => {
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
                            return maybe_struct!((Some( avg_gas_wanted
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string()),Some(Utc::now().timestamp())));
                        },
                        "avg_fee_amount_without_stability_fee" => {
                            return maybe_struct!((Some( avg_fee_amount_without_stability_fee
                              .checked_div(micro).unwrap()
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string()),Some(Utc::now().timestamp())));
                        },
                        "avg_fee_amount_adjusted_without_stability_fee" => {
                            return maybe_struct!((Some( avg_fee_amount_adjusted_without_stability_fee
                              .checked_div(micro).unwrap()
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string()),Some(Utc::now().timestamp())));
                        },
                        "avg_fee_amount" => {
                            return maybe_struct!((Some( avg_fee_amount
                              .checked_div(micro).unwrap()
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string()),Some(Utc::now().timestamp())));
                        },
                        "avg_gas_adjustment" => {
                            return maybe_struct!((Some( avg_gas_adjustment
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string()),Some(Utc::now().timestamp())));
                        },
                        "avg_gas_used" => {
                            return maybe_struct!((Some( avg_gas_used
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string()),Some(Utc::now().timestamp())));
                        },
                        "fee_amount_at_threshold" => {
                            return maybe_struct!((Some( fee_amount_at_threshold
                              .checked_div(micro).unwrap()
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string()),Some(Utc::now().timestamp())));
                        },
                        "fee_amount_adjusted" => {
                            return maybe_struct!((Some( fee_amount_adjusted
                              .checked_div(micro).unwrap()
                              .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                              .to_string()),Some(Utc::now().timestamp())));
                        },
                        &_ => {
                            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
                        }
                    }
                },
                Maybe { data: Ok(_), .. } | Maybe { data: Err(_), .. } => {
                    return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
                }
            }
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }
}

pub async fn apy_on_collateral_by(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, amount_field: &str, apr_field: &str, digits_rounded_to: u32) -> Maybe<String> {
    let borrow_limit = decimal_or_return!(borrow_limit_to_string(maybes.clone(), false, 10).await);

    let mut loan_amount = Decimal::from_str("0").unwrap();

    if amount_field == "loan_amount" {
        loan_amount = decimal_or_return!(borrower_loan_amount_to_string(maybes.clone(),false, 10).await);
    } else if amount_field == "deposit_amount" {
        loan_amount = decimal_or_return!(borrower_ust_deposited_to_string(maybes.clone(),false, 10).await);
    } else if amount_field == "target_ltv" {
        loan_amount = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"trigger_percentage",false,10).await)
            .checked_mul(borrow_limit).unwrap();
    }

    let mut apr = Decimal::from_str("0").unwrap();

    if "net_apr" == apr_field {
        let net_apr = percent_decimal_or_return!(net_apr_to_string(maybes.clone(),  10).await);
        let earn_apr = percent_decimal_or_return!(earn_apr_to_string(maybes.clone(),  10).await);
        apr = net_apr.checked_add(earn_apr).unwrap();
    } else if "earn_apr" == apr_field {
        apr = percent_decimal_or_return!(earn_apr_to_string(maybes.clone(),  10).await);
    } else if "borrow_apr" == apr_field {
        apr = percent_decimal_or_return!(borrow_apr_to_string(maybes.clone(),  10).await);
    } else if "distribution_apr" == apr_field {
        apr = percent_decimal_or_return!(distribution_apr_to_string(maybes.clone(),  10).await);
    }


    let max_ltv = decimal_or_return!(max_ltv_to_string(maybes.clone(), "BLUNA", 2).await);
    let collateral_value = borrow_limit.checked_div(max_ltv).unwrap();

    match apr
        .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
        .checked_mul(loan_amount).unwrap().checked_div(collateral_value) {
        None => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        },
        Some(e) => {
            return maybe_struct!((Some( format!("{}%",e
                  .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                  .to_string())),Some(Utc::now().timestamp())));
        }
    }
}

pub async fn anc_staked_balance_in_ust_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let mut _exchange_rate = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "simulation_cw20 anchorprotocol ANC terraswapAncUstPair").await {
        Maybe { data: Ok(response_result), .. } => {
            let amount: cosmwasm_std::Decimal = cosmwasm_std::Decimal::from_str(response_result.as_simulation().unwrap().result.return_amount.to_string().as_str()).unwrap();
            let micro: cosmwasm_std::Uint128 = cosmwasm_std::Uint128::from_str("1000000").unwrap();
            _exchange_rate = Decimal::from_str((amount / micro).to_string().as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    match try_get_resolved(&maybes, "staker").await {
        Maybe { data: Ok(response_result), .. } => {
            let balance = response_result.as_staker().unwrap().result.balance;
            let balance = Decimal::from_str(balance.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            return maybe_struct!((Some( balance.checked_div(micro).unwrap().checked_mul(_exchange_rate).unwrap()
                   .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                   .to_string()),Some(Utc::now().timestamp())));
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }
}


pub async fn anchor_claim_and_stake_transaction_gas_fees_ratio_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let mut _pending_rewards = Decimal::from_str("0").unwrap();
    match try_get_resolved(&maybes, "borrow_info").await {
        Maybe { data: Ok(response_result), .. } => {
            _pending_rewards = Decimal::from_str(response_result.as_borrow_info().unwrap().result.pending_rewards.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _pending_rewards = _pending_rewards.checked_div(micro).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let mut _exchange_rate = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "simulation_cw20 anchorprotocol ANC terraswapAncUstPair").await {
        Maybe { data: Ok(response_result), .. } => {
            let amount: cosmwasm_std::Decimal = cosmwasm_std::Decimal::from_str(response_result.as_simulation().unwrap().result.return_amount.to_string().as_str()).unwrap();
            let micro: cosmwasm_std::Uint128 = cosmwasm_std::Uint128::from_str("1000000").unwrap();
            _exchange_rate = Decimal::from_str((amount / micro).to_string().as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    _pending_rewards = _pending_rewards.checked_mul(_exchange_rate).unwrap();


    let anchor_protocol_tx_fee = decimal_or_return!(estimate_anchor_protocol_tx_fee_claim_and_stake(maybes.clone(),  10).await);

    match anchor_protocol_tx_fee.checked_div(_pending_rewards) {
        None => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        },
        Some(e) => {
            return maybe_struct!((Some( format!("{}%",e.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string())),Some(Utc::now().timestamp())));
        }
    }
}

pub async fn borrower_rewards_in_ust_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let mut _pending_rewards = Decimal::from_str("0").unwrap();
    match try_get_resolved(&maybes, "borrow_info").await {
        Maybe { data: Ok(response_result), .. } => {
            _pending_rewards = Decimal::from_str(response_result.as_borrow_info().unwrap().result.pending_rewards.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _pending_rewards = _pending_rewards.checked_div(micro).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let mut _exchange_rate = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "simulation_cw20 anchorprotocol ANC terraswapAncUstPair").await {
        Maybe { data: Ok(response_result), .. } => {
            let amount: cosmwasm_std::Decimal = cosmwasm_std::Decimal::from_str(response_result.as_simulation().unwrap().result.return_amount.to_string().as_str()).unwrap();
            let micro: cosmwasm_std::Uint128 = cosmwasm_std::Uint128::from_str("1000000").unwrap();
            _exchange_rate = Decimal::from_str((amount / micro).to_string().as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    return maybe_struct!((Some( _pending_rewards.checked_mul(_exchange_rate).unwrap()
                   .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                   .to_string()),Some(Utc::now().timestamp())));
}

pub async fn borrower_deposit_liquidity_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let mut _balance = Decimal::from_str("0").unwrap();
    match try_get_resolved(&maybes, "balance").await {
        Maybe { data: Ok(response_result), .. } => {
            _balance = Decimal::from_str(response_result.as_balance().unwrap().result.balance.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _balance = _balance.checked_div(micro).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let mut _exchange_rate = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "epoch_state anchorprotocol mmMarket").await {
        Maybe { data: Ok(response_result), .. } => {
            let result = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate;
            _exchange_rate = Decimal::from_str(result.to_string().as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let ust_deposited = _balance.checked_mul(_exchange_rate).unwrap();

    let mut _borrow_limit = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "borrow_limit").await {
        Maybe { data: Ok(response_result), .. } => {
            _borrow_limit = Decimal::from_str(response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _borrow_limit = _borrow_limit.checked_div(micro).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    return maybe_struct!((Some( format!("{}%",ust_deposited.checked_div(_borrow_limit).unwrap()
           .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
           .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
           .to_string())),Some(Utc::now().timestamp())));
}

pub async fn borrower_ltv_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let mut _borrow_limit = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "borrow_limit").await {
        Maybe { data: Ok(response_result), .. } => {
            _borrow_limit = Decimal::from_str(response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _borrow_limit = _borrow_limit.checked_div(micro).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }


    let mut _loan_amount = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "borrow_info").await {
        Maybe { data: Ok(response_result), .. } => {
            _loan_amount = Decimal::from_str(response_result.as_borrow_info().unwrap().result.loan_amount.to_string().as_str()).unwrap();
            let micro = Decimal::from_str("1000000").unwrap();
            _loan_amount = _loan_amount.checked_div(micro).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }
    /*
        let max_ltv = decimal_or_return!(max_ltv_to_string(maybes.clone(), "BLUNA", 2).await);
        let collateral_value = _borrow_limit.checked_div(max_ltv).unwrap();
    */
    match _loan_amount.checked_div(_borrow_limit) {
        None => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        },
        Some(e) => {
            return maybe_struct!((Some( format!("{}%",e
                   .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                   .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                   .to_string())),Some(Utc::now().timestamp())));
        }
    }
}

pub async fn borrower_ust_deposited_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let mut _balance = Decimal::from_str("0").unwrap();
    match try_get_resolved(&maybes, "balance").await {
        Maybe { data: Ok(response_result), .. } => {
            _balance = Decimal::from_str(response_result.as_balance().unwrap().result.balance.to_string().as_str()).unwrap();
            if !as_micro {
                let micro = Decimal::from_str("1000000").unwrap();
                _balance = _balance.checked_div(micro).unwrap();
            }
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let mut _exchange_rate = Decimal::from_str("0").unwrap();

    match try_get_resolved(&maybes, "epoch_state anchorprotocol mmMarket").await {
        Maybe { data: Ok(response_result), .. } => {
            let result = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate;
            _exchange_rate = Decimal::from_str(result.to_string().as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }
    return maybe_struct!((Some( _balance.checked_mul(_exchange_rate).unwrap()
           .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
           .to_string()),Some(Utc::now().timestamp())));
}


pub async fn borrow_apr_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    // utilisationRatio = stablecoinsLent / stablecoinsDeposited
    // borrowRate = utilisationRatio * interestMultiplier + baseRate
    // borrow_apr = blocksPerYear * borrowRate

    let mut _total_liabilities = cosmwasm_bignumber::Decimal256::zero();

    let mut _a_terra_exchange_rate = cosmwasm_bignumber::Decimal256::zero();
    let mut _a_terra_supply = cosmwasm_bignumber::Uint256::zero();

    match try_get_resolved(&maybes, "state anchorprotocol mmMarket").await {
        Maybe { data: Ok(response_result), .. } => {
            _total_liabilities = response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    match try_get_resolved(&maybes, "epoch_state anchorprotocol mmMarket").await {
        Maybe { data: Ok(response_result), .. } => {
            _a_terra_exchange_rate = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate;
            _a_terra_supply = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.aterra_supply;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let stablecoins_lent: Decimal = Decimal::from_str(_total_liabilities.to_string().as_str()).unwrap();

    let stablecoins_deposited: Decimal = Decimal::from_str(_a_terra_supply.to_string().as_str()).unwrap()
        .checked_mul(Decimal::from_str(_a_terra_exchange_rate.to_string().as_str()).unwrap())
        .unwrap();

    let utilization_ratio: Decimal = stablecoins_lent
        .checked_div(stablecoins_deposited)
        .unwrap();

    let mut _interest_multiplier = cosmwasm_bignumber::Decimal256::zero();
    let mut _base_rate = cosmwasm_bignumber::Decimal256::zero();

    match try_get_resolved(&maybes, "config anchorprotocol mmInterestModel").await {
        Maybe { data: Ok(response_result), .. } => {
            _base_rate = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.base_rate;
            _interest_multiplier = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.interest_multiplier;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let borrow_rate_without_base_rate = Decimal::from_str(_interest_multiplier.to_string().as_str()).unwrap()
        .checked_mul(utilization_ratio).unwrap();

    let borrow_rate = borrow_rate_without_base_rate
        .checked_add(Decimal::from_str(_base_rate.to_string().as_str()).unwrap())
        .unwrap();

    match try_get_resolved(&maybes, "blocks_per_year").await {
        Maybe { data: Ok(response_result), .. } => {
            let blocks_per_year = Decimal::from_str(response_result.as_blocks().unwrap().result.blocks_per_year.to_string().as_str()).unwrap();

            let borrow_apr = blocks_per_year
                .checked_mul(borrow_rate).unwrap()
                .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                .to_string();
            return maybe_struct!((Some( format!("{}%",borrow_apr)),Some(Utc::now().timestamp())));
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }
}

pub async fn anchor_airdrops_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>) -> Maybe<String> {
    match try_get_resolved(&maybes, "anchor_airdrops").await {
        Maybe { data: Ok(res), .. } => {
            let anchor_airdrops = res.as_airdrop_response().unwrap();
            let mut amount_unclaimed: u64 = 0;
            let mut amount_claimed: u64 = 0;
            for i in 0..anchor_airdrops.len() {
                if anchor_airdrops[i].claimable {
                    amount_unclaimed += anchor_airdrops[i].amount.parse::<u64>().unwrap_or(0u64);
                } else {
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
            return maybe_struct!((Some( format!("available to claim: {}, amount already claimed: {}",amount_unclaimed,amount_claimed)),Some(Utc::now().timestamp())));
        },
        Maybe { data: Err(err), .. } => {
            return maybe_struct!((Some( format!("{:?}",err)),Some(Utc::now().timestamp())));
        }
    }
}

pub async fn anything_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: &str) -> Maybe<String> {
    match try_get_resolved(&maybes, key).await {
        Maybe { data: Ok(res), .. } => {
            return maybe_struct!((Some( serde_json::to_string_pretty(&res).unwrap_or("--".to_string())),Some(Utc::now().timestamp())));
        },
        Maybe { data: Err(err), .. } => {
            return maybe_struct!((Some( format!("{:?}",err)),Some(Utc::now().timestamp())));
        }
    }
}

pub async fn net_apr_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    // utilisationRatio = stablecoinsLent / stablecoinsDeposited
    // borrowRate = utilisationRatio * interestMultiplier + baseRate
    // borrow_apr = blocksPerYear * borrowRate

    let mut _total_liabilities = cosmwasm_bignumber::Decimal256::zero();

    let mut _a_terra_exchange_rate = cosmwasm_bignumber::Decimal256::zero();
    let mut _a_terra_supply = cosmwasm_bignumber::Uint256::zero();

    match try_get_resolved(&maybes, "state anchorprotocol mmMarket").await {
        Maybe { data: Ok(response_result), .. } => {
            _total_liabilities = response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    match try_get_resolved(&maybes, "epoch_state anchorprotocol mmMarket").await {
        Maybe { data: Ok(response_result), .. } => {
            _a_terra_exchange_rate = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate;
            _a_terra_supply = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.aterra_supply;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let stablecoins_lent: Decimal = Decimal::from_str(_total_liabilities.to_string().as_str()).unwrap();

    let stablecoins_deposited: Decimal = Decimal::from_str(_a_terra_supply.to_string().as_str()).unwrap()
        .checked_mul(Decimal::from_str(_a_terra_exchange_rate.to_string().as_str()).unwrap())
        .unwrap();

    let utilization_ratio: Decimal = stablecoins_lent
        .checked_div(stablecoins_deposited)
        .unwrap();

    let mut _interest_multiplier = cosmwasm_bignumber::Decimal256::zero();
    let mut _base_rate = cosmwasm_bignumber::Decimal256::zero();

    match try_get_resolved(&maybes, "config anchorprotocol mmInterestModel").await {
        Maybe { data: Ok(response_result), .. } => {
            _base_rate = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.base_rate;
            _interest_multiplier = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.interest_multiplier;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let borrow_rate_without_base_rate = Decimal::from_str(_interest_multiplier.to_string().as_str()).unwrap()
        .checked_mul(utilization_ratio).unwrap();

    let borrow_rate = borrow_rate_without_base_rate
        .checked_add(Decimal::from_str(_base_rate.to_string().as_str()).unwrap())
        .unwrap();

    let mut _blocks_per_year = Decimal::from_str("0").unwrap(); // 4656810
    match try_get_resolved(&maybes, "blocks_per_year").await {
        Maybe { data: Ok(response_result), .. } => {
            _blocks_per_year = Decimal::from_str(response_result.as_blocks().unwrap().result.blocks_per_year.to_string().as_str()).unwrap();
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }
    let borrow_apr = _blocks_per_year
        .checked_mul(borrow_rate).unwrap();

    match try_get_resolved(&maybes, "api/v2/distribution-apy").await {
        Maybe { data: Ok(response_result), .. } => {
            let distribution_apr: cosmwasm_std::Decimal = response_result.as_distribution_apy().unwrap().distribution_apy;
            return maybe_struct!((Some( format!("{}%",
                    Decimal::from_str(distribution_apr.to_string().as_str()).unwrap()
                    .checked_add(borrow_apr.checked_mul(Decimal::from_str("-1").unwrap()).unwrap()).unwrap()
                    .checked_mul(Decimal::from_str("100").unwrap()).unwrap()
                    .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string()
                    )),Some(Utc::now().timestamp())));
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }
}


pub async fn borrow_rate_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: &str, key_1: &str, key_2: &str, digits_rounded_to: u32) -> Maybe<String> {
    let mut _interest_multiplier = cosmwasm_bignumber::Decimal256::zero();
    let mut _base_rate = cosmwasm_bignumber::Decimal256::zero();

    match try_get_resolved(&maybes, key).await {
        Maybe { data: Ok(response_result), .. } => {
            _base_rate = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.base_rate;
            _interest_multiplier = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.interest_multiplier;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let mut _total_liabilities = cosmwasm_bignumber::Decimal256::zero();

    let mut _a_terra_exchange_rate = cosmwasm_bignumber::Decimal256::zero();
    let mut _a_terra_supply = cosmwasm_bignumber::Uint256::zero();

    match try_get_resolved(&maybes, key_1).await {
        Maybe { data: Ok(response_result), .. } => {
            _total_liabilities = response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    match try_get_resolved(&maybes, key_2).await {
        Maybe { data: Ok(response_result), .. } => {
            _a_terra_exchange_rate = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate;
            _a_terra_supply = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.aterra_supply;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let stablecoins_lent: Decimal = Decimal::from_str(_total_liabilities.to_string().as_str()).unwrap().checked_mul(Decimal::from_str(_a_terra_exchange_rate.to_string().as_str()).unwrap()).unwrap();
    let utilization_ratio: Decimal = stablecoins_lent.checked_div(Decimal::from_str(_a_terra_supply.to_string().as_str()).unwrap()).unwrap();
    return maybe_struct!((Some( Decimal::from_str(_interest_multiplier.to_string().as_str()).unwrap().checked_mul(utilization_ratio).unwrap().checked_add(Decimal::from_str(_base_rate.to_string().as_str()).unwrap()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string()),Some(Utc::now().timestamp())));
}

pub async fn utilization_ratio_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let mut _total_liabilities = cosmwasm_bignumber::Decimal256::zero();

    let mut _a_terra_exchange_rate = cosmwasm_bignumber::Decimal256::zero();
    let mut _a_terra_supply = cosmwasm_bignumber::Uint256::zero();

    match try_get_resolved(&maybes, "state anchorprotocol mmMarket").await {
        Maybe { data: Ok(response_result), .. } => {
            _total_liabilities = response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    match try_get_resolved(&maybes, "epoch_state anchorprotocol mmMarket").await {
        Maybe { data: Ok(response_result), .. } => {
            _a_terra_exchange_rate = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate;
            _a_terra_supply = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.aterra_supply;
        },
        Maybe { data: Err(_), .. } => {
            return maybe_struct!((Some( "--".to_string()),Some(Utc::now().timestamp())));
        }
    }

    let stablecoins_lent: Decimal = Decimal::from_str(_total_liabilities.to_string().as_str()).unwrap().checked_mul(Decimal::from_str(_a_terra_exchange_rate.to_string().as_str()).unwrap()).unwrap();
    let utilization_ratio = stablecoins_lent.checked_div(Decimal::from_str(_a_terra_supply.to_string().as_str()).unwrap()).unwrap();
    return maybe_struct!((Some( format!("{}%",utilization_ratio.checked_mul(Decimal::from_str("100").unwrap()).unwrap().round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string())),Some(Utc::now().timestamp())));
}

pub async fn estimate_anchor_protocol_auto_repay_tx_fee(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

    let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false,10).await);

    let fee_to_redeem_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(),"anchor_protocol_txs_redeem_stable","avg_gas_used".to_owned(),false,10).await);

    let anchor_protocol_txs_repay_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(),"anchor_protocol_txs_repay_stable","avg_gas_used".to_owned(),false,10).await);
    // min(to_repay * tax_rate , tax_cap)
    let stability_tax = decimal_or_return!(calculate_repay_plan(maybes.clone(),"stability_tax",10).await);

    return maybe_struct!((Some( fee_to_redeem_stable_gas_used
            .checked_add(anchor_protocol_txs_repay_stable_gas_used).unwrap()
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            .checked_add(stability_tax).unwrap()
            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string()),Some(Utc::now().timestamp())));
}

pub async fn estimate_anchor_protocol_auto_borrow_tx_fee(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let gas_fees_uusd = decimal_or_return!(gas_price_to_string(maybes.clone(),10).await);

    let gas_adjustment_preference = decimal_or_return!(meta_data_key_to_string(maybes.clone(),"gas_adjustment_preference",false,10).await);

    let anchor_protocol_txs_borrow_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(),"anchor_protocol_txs_borrow_stable","avg_gas_used".to_owned(),false,10).await);
    // min(to_repay * tax_rate , tax_cap)
    let stability_tax_borrow = decimal_or_return!(calculate_borrow_plan(maybes.clone(),"stability_tax_borrow",10).await);

    let anchor_protocol_txs_deposit_stable_gas_used = decimal_or_return!(estimate_anchor_protocol_tx_fee(maybes.clone(),"anchor_protocol_txs_deposit_stable","avg_gas_used".to_owned(),false,10).await);
    // min(to_repay * tax_rate , tax_cap) 
    let stability_tax_deposit = decimal_or_return!(calculate_borrow_plan(maybes.clone(),"stability_tax_deposit",10).await);

    return maybe_struct!((Some(  anchor_protocol_txs_borrow_stable_gas_used
            .checked_add(anchor_protocol_txs_deposit_stable_gas_used).unwrap()
            .checked_mul(gas_fees_uusd).unwrap()
            .checked_mul(gas_adjustment_preference).unwrap()
            .checked_div(Decimal::from_str("1000000").unwrap()).unwrap()
            .checked_add(stability_tax_borrow).unwrap() 
            .checked_add(stability_tax_deposit).unwrap() 
            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero).to_string()),Some(Utc::now().timestamp())));
}