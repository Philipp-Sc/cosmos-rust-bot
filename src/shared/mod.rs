use serde::Deserialize;
use serde::Serialize;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::fs;
use std::io;
use std::hash::{Hash};
use serde_json::{Value};

// todo: add functionality to add/remove/edit settings in-memory & the settings file on disk (load, edit, save).

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSettings {
    pub governance_proposal_notifications: bool,
    pub pause_requested: bool,
    pub hot_reload: bool,
    pub remove: bool,
    pub test: bool,
    pub terra_wallet_address: Option<String>,
}

impl Default for UserSettings {
    fn default() -> UserSettings {
        UserSettings {
            governance_proposal_notifications: true,
            pause_requested: false,
            hot_reload: false,
            remove: false,
            test: true,
            terra_wallet_address: None,
        }
    }
}

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
