use serde::Deserialize;
use serde::Serialize;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::fs;

#[derive(Debug)]
pub struct Maybe<T> {
    pub data: anyhow::Result<T>,   
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSettings {
    pub pause_requested: bool,
    pub hot_reload: bool,
    pub remove: bool,
    pub test: bool,
    pub terra_wallet_address: Option<String>,
    pub anchor_protocol_auto_repay: bool,
    pub anchor_protocol_auto_borrow: bool,
    pub anchor_protocol_auto_stake: bool,
    pub anchor_protocol_auto_farm: bool,
    pub terra_market_info: bool,
    pub anchor_general_info: bool,
    pub anchor_account_info: bool,
    pub trigger_percentage: Decimal,
    pub target_percentage: Decimal,
    pub borrow_percentage: Decimal,
    pub min_ust_balance: Decimal,
    pub gas_adjustment_preference: Decimal,
    pub max_tx_fee: Decimal,
    pub ust_balance_preference: Decimal,
}
impl Default for UserSettings {
    fn default() -> UserSettings {
        UserSettings {
            pause_requested: false,
            hot_reload: false,
            remove: false,
            test: true,
            terra_wallet_address: None,
            anchor_protocol_auto_repay: false,
            anchor_protocol_auto_borrow: false,
            anchor_protocol_auto_stake: false,
            anchor_protocol_auto_farm: false,
            terra_market_info: true,
            anchor_general_info: true,
            anchor_account_info: false,
            trigger_percentage: Decimal::from_str("0.9").unwrap(),
            target_percentage: Decimal::from_str("0.72").unwrap(),
            borrow_percentage: Decimal::from_str("0.5").unwrap(),
            max_tx_fee: Decimal::from_str("5").unwrap(),
            gas_adjustment_preference: Decimal::from_str("1.2").unwrap(),
            min_ust_balance: Decimal::from_str("10").unwrap(),
            ust_balance_preference: Decimal::from_str("20").unwrap(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entry {
    pub timestamp: i64,
    pub key: String,
    pub prefix: Option<String>,
    pub value: String,
    pub suffix: Option<String>,
    pub group: Option<String>,
}

pub type State = Vec<Option<Entry>>;


pub fn load_user_settings(path: &str) -> UserSettings {

    let user_settings: UserSettings = match fs::read_to_string(path) {
        Ok(file) => {
            match serde_json::from_str(&file) {
                Ok(res) => {
                    res
                },
                Err(err) => {
                    println!("{:?}",err);
                    Default::default()
                }
            }
        },
        Err(err) => {
            println!("{:?}",err);
            Default::default()
        }
    };
    user_settings
}

pub async fn load_state(path: &str) -> Option<State> {
    let mut state: Option<State> = None;
    let mut try_counter = 0;
    while state.is_none() && try_counter<3 {
        match fs::read_to_string(path) {
            Ok(file) => {
                match serde_json::from_str(&file) {
                    Ok(res) => { state = Some(res); },
                    Err(_) => { try_counter = try_counter + 1; },
                };
            },
            Err(_) => {
                try_counter = try_counter + 1;
            }
        }
    }
    state
}