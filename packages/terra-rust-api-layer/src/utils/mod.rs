use serde_json::Value;
use serde_json::json;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use chrono::Utc;


pub fn duration_to_string(duration: chrono::Duration) -> String {
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

pub fn estimate_optimal_next_claim_and_stake_tx(loan_amount: Decimal, pending_rewards_in_ust: Decimal, distribution_apr: Decimal, pool_apy: Decimal, transaction_fee: Decimal, digits_rounded_to: u32) -> Value {

    let mut _optimal_time_to_wait: Option<Decimal> = None;
    let mut _optimal_token_ust_value: Option<Decimal> = None;
    let _total_returns_in_ust: Decimal;

    let one_year_equals_this_many_time_frames = Decimal::new(365*24,0);

    let token_dist_returns_per_timeframe = distribution_apr.checked_div(one_year_equals_this_many_time_frames).unwrap();
    let token_dist_returns_per_timeframe_in_ust = loan_amount.checked_mul(token_dist_returns_per_timeframe).unwrap();

    let pool_returns_per_timeframe = pool_apy.checked_div(one_year_equals_this_many_time_frames).unwrap();

    let mut max_value: Option<Decimal> = None;
    let mut max_index: Option<Decimal> = None;

    let timeframes = one_year_equals_this_many_time_frames.checked_add(Decimal::new(1,0)).unwrap().to_i64().unwrap();

    let worst_token_ust_value = token_dist_returns_per_timeframe_in_ust
        .checked_mul(Decimal::new(timeframes,0)).unwrap()
        .checked_sub(transaction_fee).unwrap();

    for n in 1..timeframes {
        // amount ANC rewards available after n timeframes
        let total_token_dist_returns_n_timeframes_ust = token_dist_returns_per_timeframe_in_ust.checked_mul(Decimal::new(n,0)).unwrap();

        // amount ANC staked, by claiming and staking the outstanding amount after n timeframes
        let total_token_staked_n_timeframes_in_ust_after_tx = total_token_dist_returns_n_timeframes_ust.checked_sub(transaction_fee).unwrap();

        let total_token_staking_rewards_one_year_in_ust = total_token_staked_n_timeframes_in_ust_after_tx
            .checked_mul(pool_returns_per_timeframe).unwrap()
            .checked_mul(one_year_equals_this_many_time_frames.checked_sub(Decimal::new(n,0)).unwrap()).unwrap() // remove the timeframes that already passed in the reference year
            .checked_div(Decimal::new(n,0)).unwrap() // now normalize the result, to represent the ANC staking rewards in the reference year
            .checked_mul(one_year_equals_this_many_time_frames).unwrap();

        if let Some(max) = max_value {
            if max < total_token_staking_rewards_one_year_in_ust {
                max_value = Some(total_token_staking_rewards_one_year_in_ust);
                max_index = Some(Decimal::new(n,0));
            }
        }else{
            max_value = Some(total_token_staking_rewards_one_year_in_ust);
            max_index = Some(Decimal::new(n,0));
        }
    }

    _optimal_time_to_wait = max_index;
    _optimal_token_ust_value = token_dist_returns_per_timeframe_in_ust.checked_mul(max_index.unwrap());

    //_total_returns_in_ust = _optimal_token_ust_value.unwrap().checked_mul(Decimal::new(timeframes,0).checked_div(_optimal_time_to_wait.unwrap()).unwrap()).unwrap();
    // now need to add staking returns for each

    // a1 * b1 + a1 * b2 + ..
    // a1 * (b1 + b2 + b3 ...)
    // _optimal_token_ust_value * ( pool_apy + (pool_apy/timestamps) * (timestamps - (_optimal_time_to_wait*n)) + ...

    let mut sum_pool_apy = Decimal::new(0,0);
    let amount_time_to_wait_in_timeframes = Decimal::new(timeframes,0).checked_div(_optimal_time_to_wait.unwrap()).unwrap().to_i64().unwrap();
    let mut n = 0i64;
    let pool_apy_per_timestamp =pool_apy.checked_div(Decimal::new(timeframes,0)).unwrap();

    while n<=amount_time_to_wait_in_timeframes {
        let time_to_wait_from_start = _optimal_time_to_wait.unwrap().checked_mul(Decimal::new(n,0)).unwrap();
        let partial_sum = pool_apy_per_timestamp.checked_mul(Decimal::new(timeframes,0).checked_sub(time_to_wait_from_start).unwrap()).unwrap();
        sum_pool_apy = sum_pool_apy.checked_add(partial_sum.checked_add(Decimal::new(1,0)).unwrap()).unwrap();
        n = n+1;
    }
    _total_returns_in_ust = _optimal_token_ust_value.unwrap().checked_mul(sum_pool_apy).unwrap();

    let _optimal_time_to_wait = _optimal_time_to_wait.unwrap().checked_mul(Decimal::new(60*60,0));
    let time_to_wait_already_passed = pending_rewards_in_ust
        .checked_mul(Decimal::new(60*60,0)).unwrap()
        .checked_div(token_dist_returns_per_timeframe_in_ust);


    let wait_loan_taken = chrono::Duration::seconds(_optimal_time_to_wait.unwrap().to_i64().unwrap());

    let mut time = _optimal_time_to_wait.unwrap().to_i64().unwrap();
    if let Some(ttwap) = time_to_wait_already_passed {
        time = time-(ttwap.to_i64().unwrap());
    }

    let minus_already_wait_loan_taken = chrono::Duration::seconds(time);

    let duration = duration_to_string(wait_loan_taken);
    let dt = Utc::now();
    let trigger_date = dt.checked_add_signed(minus_already_wait_loan_taken).unwrap().format("%d/%m/%y %H:%M");

    let date_next = match time {
        t if t <=0 => {
            "now".to_string()
        },
        _ => {
            trigger_date.to_string()
        },
    };
    let value_next = _optimal_token_ust_value
        .unwrap_or(Decimal::new(0,0))
        .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
        .to_string();
    let total_returns = _total_returns_in_ust
        .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
        .to_string();
    let total_amount_without = worst_token_ust_value.to_string();
    let difference = _total_returns_in_ust
        .checked_div(worst_token_ust_value).unwrap()
        .checked_sub(Decimal::new(1,0)).unwrap()
        .checked_mul(Decimal::new(100,0)).unwrap()
        .round_dp_with_strategy(digits_rounded_to, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
        .to_string();
    let result = json!({
        "date_next": date_next,
        "value_next": value_next,
        "duration_next": duration,
        "annual_return_auto_staking": total_returns,
        "annual_return_no_staking": total_amount_without,
        "difference": difference,
    });
    return result;
}