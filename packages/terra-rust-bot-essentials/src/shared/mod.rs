use serde::Deserialize;
use serde::Serialize;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::fs;
use std::io;
use std::hash::{Hash};
use serde_json::{Value};

#[derive(Debug)]
pub struct Maybe<T> {
    pub data: anyhow::Result<T>,
    pub timestamp: i64,
}

impl<T: Clone> Clone for Maybe<T> {
    fn clone(&self) -> Maybe<T> {
        match self {
            Maybe { data: Err(err), timestamp } => Maybe { data: Err(anyhow::anyhow!(err.to_string())), timestamp: *timestamp },
            Maybe { data: Ok(value), timestamp } => Maybe { data: Ok(value.clone()), timestamp: *timestamp },
        }
    }
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Clone)]
pub struct Entry {
    pub timestamp: i64,
    pub key: String,
    pub prefix: Option<String>,
    pub value: String,
    pub suffix: Option<String>,
    pub index: Option<i32>,
    pub group: Option<String>,
}

pub type State = Vec<Option<Entry>>;

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

pub fn load_asset_whitelist(path: &str) -> Value {
    serde_json::json!({
        "contracts": serde_json::from_str::<Value>(&fs::read_to_string(format!("{}contracts.json", path)).unwrap()).unwrap(),
        "pairs_dex": serde_json::from_str::<Value>(&fs::read_to_string(format!("{}pairs.dex.json", path)).unwrap()).unwrap(),
        "pairs": serde_json::from_str::<Value>(&fs::read_to_string(format!("{}pairs.json", path)).unwrap()).unwrap(),
        "tokens": serde_json::from_str::<Value>(&fs::read_to_string(format!("{}tokens.json", path)).unwrap()).unwrap(),
        "custom": serde_json::from_str::<Value>(&fs::read_to_string(format!("{}custom.json", path)).unwrap()).unwrap(),
    })
}

pub fn load_user_settings(path: &str) -> UserSettings {
    let user_settings: UserSettings = match fs::read_to_string(path) {
        Ok(file) => {
            match serde_json::from_str(&file) {
                Ok(res) => {
                    res
                }
                Err(err) => {
                    println!("{:?}", err);
                    Default::default()
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
            Default::default()
        }
    };
    if user_settings.remove {
        let res = fs::remove_file(path);
        println!("{:?}", res);
    }
    user_settings
}

pub async fn load_state(path: &str) -> Option<State> {
    let mut state: Option<State> = None;
    let mut try_counter = 0;
    while state.is_none() && try_counter < 3 {
        match fs::read_to_string(path) {
            Ok(file) => {
                match serde_json::from_str(&file) {
                    Ok(res) => { state = Some(res); }
                    Err(_) => { try_counter = try_counter + 1; }
                };
            }
            Err(_) => {
                try_counter = try_counter + 1;
            }
        }
    }
    if let Some(mut s) = state {
        s.sort_by(|a, b| a.as_ref().unwrap().index.unwrap_or(0i32).cmp(&b.as_ref().unwrap().index.unwrap_or(0i32)));
        return Some(s);
    }
    state
}