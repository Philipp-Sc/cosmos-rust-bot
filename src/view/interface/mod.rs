/*
 * Provides helper functions to access key attributes without the need to know the underlying data structure.
 * Values are returned as Strings.
 *
 */


use crate::state::control::model::{Maybe, try_get_resolved};

use std::collections::HashMap;
use rust_decimal::Decimal;
use core::str::FromStr;
use num_format::{Locale, ToFormattedString};
use std::sync::Arc;
use tokio::sync::{Mutex};

use chrono::Utc;
use terra_rust_api_layer::services::blockchain::smart_contracts::objects::{ResponseResult, SimulationResponse};

pub mod interface_macro {
    macro_rules! maybe_struct {
        ( $e:expr ) => {
            match $e {
                (Some(d),Some(t)) => {
                    Maybe::<String>{data:Ok(d),timestamp:t}
                },
                (_,_) => {
                    Maybe::<String>{data:Err(anyhow::anyhow!("--")),timestamp:Utc::now().timestamp()}
                }
            }
        }
    }

    pub(crate) use maybe_struct;
}

use interface_macro::maybe_struct;

macro_rules! try_get_data_by_key {
    ( $e:expr, $d:expr ) => {
        match try_get_resolved($e,$d).await {
            Maybe{data: Ok(response_result),timestamp: t} => {
                (response_result,t)
            },
            Maybe{data: Err(err),timestamp: t} => {
                return Maybe::<String>{data:Err(anyhow::anyhow!(err)), timestamp:t};
            }
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

pub async fn tax_rate_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"tax_rate");
    let value = &response_result.as_tax_rate().unwrap().result;
    let data = try_convert_and_round!(&value,"is_human_readable",false, digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn uusd_tax_cap_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"tax_caps");
    let vec_tax_caps = &response_result.as_tax_caps().unwrap().result;
    for tax_cap in vec_tax_caps {
        if tax_cap.denom == "uusd".to_string() {
            let data = try_convert_and_round!(&tax_cap.tax_cap,"is_micro",as_micro,digits_rounded_to);
            return maybe_struct!((Some(data),Some(timestamp)));
        }
    }
    maybe_struct!((None,None))
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
pub async fn meta_data_key_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: &str, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,key);
    let data = try_convert_and_round!(&response_result.as_text().unwrap(),"is_human_readable",as_micro,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn anc_staked_balance_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"staker");
    let value = response_result.as_staker().unwrap().result.balance.to_string();
    let data = try_convert_and_round!(&value,"is_micro",false,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn borrower_rewards_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"borrow_info");
    let value = response_result.as_borrow_info().unwrap().result.pending_rewards.to_string();
    let data = try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn borrower_anc_deposited_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"anc_balance");
    let value = response_result.as_balance().unwrap().result.balance.to_string();
    let data = try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn borrower_balance_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"balance");
    let value = response_result.as_balance().unwrap().result.balance.to_string();
    let data = try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn terra_balance_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, denom: &str, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    match try_get_resolved(&maybes, "terra_balances").await {
        Maybe { data: Ok(response_result), timestamp } => {
            let vector_balances = &response_result.as_balances().unwrap().result;
            for balance in vector_balances {
                if &balance.denom == denom {
                    let balance = Decimal::from_str(balance.amount.as_str()).unwrap();
                    if !as_micro {
                        let micro = Decimal::from_str("1000000").unwrap();
                        let data = balance.checked_div(micro).unwrap()
                            .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                            .to_string();
                        return maybe_struct!((Some(data),Some(timestamp)));
                    }
                    let data = balance.round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::ToZero)
                        .to_string();
                    return maybe_struct!((Some(data),Some(timestamp)));
                }
            }
        }
        _ => {
            return maybe_struct!((None,None));
        }
    }
    return maybe_struct!((None,None));
}

pub async fn borrower_loan_amount_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"borrow_info");
    let value = response_result.as_borrow_info().unwrap().result.loan_amount.to_string();
    let data = try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn borrow_limit_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"borrow_limit");
    let value = response_result.as_borrow_limit().unwrap().result.borrow_limit.to_string();
    let data = try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn earn_apr_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"earn_apy");
    let value = response_result.as_earn_apy().unwrap().result.apy.to_string();
    let data = to_percentage_and_round!(&value,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn distribution_apr_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"api/v2/distribution-apy");
    let value = response_result.as_distribution_apy().unwrap().distribution_apy.to_string();
    let data = to_percentage_and_round!(&value,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn gas_price_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"gas_fees_uusd");
    let value = response_result.as_text().unwrap();
    let data = try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn spec_anc_ust_lp_apy_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"api/data?type=lpVault");
    let dpr = response_result.as_spec_astro_vault().unwrap().dpr.to_string();
    let value = Decimal::from_str(dpr.as_str()).unwrap()
        .checked_mul(Decimal::from_str("365").unwrap()).unwrap()
        .to_string();
    let data = to_percentage_and_round!(&value,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn anc_ust_lp_apy_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"api/v2/ust-lp-reward");
    let value = response_result.as_lp_reward().unwrap().apy.to_string();
    let data = to_percentage_and_round!(&value,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn staking_apy_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"api/v2/gov-reward");
    let value = response_result.as_gov_reward().unwrap().current_apy.to_string();
    let data = to_percentage_and_round!(&value,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn max_ltv_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: &str, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"anchor_protocol_whitelist");
    let anchor_asstes = &response_result.as_anchor_whitelist_response().unwrap().result.elems;
    for i in 0..anchor_asstes.len() {
        if &anchor_asstes[i].symbol == key {
            let value = &anchor_asstes[i].max_ltv.to_string();
            let data = try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to);
            return maybe_struct!((Some(data),Some(timestamp)));
        }
    }
    maybe_struct!((None,None))
}

pub async fn interest_multiplier_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"config anchorprotocol mmInterestModel");
    let value = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.interest_multiplier.to_string();
    let data = try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn blocks_per_year_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"blocks_per_year");
    let value = response_result.as_blocks().unwrap().result.blocks_per_year.to_string();
    let data = try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn base_rate_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"config anchorprotocol mmInterestModel");
    let value = response_result.as_config().unwrap().as_mm_interest_model().unwrap().result.base_rate.to_string();
    let data = try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn a_terra_supply_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"epoch_state anchorprotocol mmMarket");
    let res = &response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result;
    let value = (res.aterra_supply / res.exchange_rate).to_string();
    let data = try_convert_and_round!(&value,"is_micro",false,digits_rounded_to).parse::<u128>().unwrap().to_formatted_string(&Locale::en);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn a_terra_exchange_rate_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"epoch_state anchorprotocol mmMarket");
    let value = response_result.as_epoch_state().unwrap().as_mm_market().unwrap().result.exchange_rate.to_string();
    let data = try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn b_luna_exchange_rate_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"state anchorprotocol bLunaHub");
    let value = response_result.as_state().unwrap().as_b_luna_hub().unwrap().result.bluna_exchange_rate.to_string();
    let data = try_convert_and_round!(&value,"is_human_readable",false,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn total_liabilities_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,"state anchorprotocol mmMarket");
    let value = response_result.as_state().unwrap().as_mm_market().unwrap().result.total_liabilities.to_string();
    let data = try_convert_and_round!(&value,"is_micro",false,digits_rounded_to).parse::<u128>().unwrap().to_formatted_string(&Locale::en);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn simulation_swap_exchange_rate_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: &str, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,key);

    let value = match &response_result.as_simulation().unwrap().result {
        SimulationResponse::terraswap(res) => {
            Decimal::from_str(&res.return_amount.to_string().as_str()).unwrap().checked_add(Decimal::from_str(&res.commission_amount.to_string().as_str()).unwrap()).unwrap().to_string()
        }
        SimulationResponse::astroport(res) => {
            Decimal::from_str(&res.return_amount.to_string().as_str()).unwrap().checked_add(Decimal::from_str(&res.commission_amount.to_string().as_str()).unwrap()).unwrap().to_string()
        }
    };
    let data = try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn simulation_swap_return_amount_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: &str, as_micro: bool, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,key);
    let value = match &response_result.as_simulation().unwrap().result {
        SimulationResponse::terraswap(res) => {
            res.return_amount.to_string()
        }
        SimulationResponse::astroport(res) => {
            res.return_amount.to_string()
        }
    };
    let data = try_convert_and_round!(&value,"is_micro",as_micro,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}

pub async fn core_swap_amount_to_string(maybes: HashMap<String, Arc<Mutex<Vec<Maybe<ResponseResult>>>>>, key: &str, digits_rounded_to: u32) -> Maybe<String> {
    let (response_result, timestamp) = try_get_data_by_key!(&maybes,key);
    let value = response_result.as_core_swap().unwrap().result.amount.to_string();
    let data = try_convert_and_round!(&value,"is_micro",false,digits_rounded_to);
    maybe_struct!((Some(data),Some(timestamp)))
}