/*
 * Terra Contract Addresses
 *
 */

use serde_json::{json, Value};
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetWhitelist {
    pub contracts: Value,
    pub pairs_dex: Value,
    pub pairs: Value,
    pub tokens: Value,
    pub custom: Value,
}

pub fn contracts(list: &AssetWhitelist, protocol: &str, name: &str) -> Option<String> {
    let none_value = json!("None");
    let contract_list = list.contracts.as_object().unwrap().get("mainnet").unwrap().as_object().unwrap();
    for (key, value) in contract_list {
        let value_name = value.get("name").unwrap_or(&none_value).as_str().unwrap();
        let value_protocol = value.get("protocol").unwrap_or(&none_value).as_str().unwrap();
        if value_name == name && value_protocol.to_uppercase() == protocol.to_uppercase() {
            return Some(key.to_string());
        }
    }
    None
}

pub fn pairs_dex(list: &AssetWhitelist, assets: [&str; 2], dex: &str) -> Option<String> {
    let none_value = json!("None");
    let pairs_list = list.pairs_dex.as_object().unwrap().get("mainnet").unwrap().as_object().unwrap();
    for (key, value) in pairs_list {
        let value_dex = value.get("dex").unwrap_or(&none_value).as_str().unwrap();
        if value_dex.to_uppercase() == dex.to_uppercase() {
            let value_assets: Vec<&str> = value.get("assets").unwrap().as_array().unwrap().iter().map(|x| x.as_str().unwrap()).collect();
            if value_assets.contains(&assets[0]) && value_assets.contains(&assets[1]) && value_assets.len() == 2 {
                return Some(key.to_string());
            }
        }
    }
    None
}

pub fn tokens(list: &AssetWhitelist, protocol: &str, symbol: &str) -> Option<String> {
    let none_value = json!("None");
    let tokens_list = list.tokens.as_object().unwrap().get("mainnet").unwrap().as_object().unwrap();
    for (key, value) in tokens_list {
        let value_symbol = value.get("symbol").unwrap_or(&none_value).as_str().unwrap();
        let value_protocol = value.get("protocol").unwrap_or(&none_value).as_str().unwrap();
        if value_symbol == symbol && value_protocol.to_uppercase() == protocol.to_uppercase() {
            return Some(key.to_string());
        }
    }
    None
}

// this is for contracts/tokens/pairs that are not whitelisted for whatever reason but needed for terra-ruts-bot
pub fn custom(list: &AssetWhitelist, protocol: &str, symbol: &str) -> Option<String> {
    let none_value = json!("None");
    let custom_list = list.custom.as_object().unwrap().get("mainnet").unwrap().as_object().unwrap();
    for (key, value) in custom_list {
        let value_symbol = value.get("symbol").unwrap_or(&none_value).as_str().unwrap();
        let value_protocol = value.get("protocol").unwrap_or(&none_value).as_str().unwrap();
        if value_symbol == symbol && value_protocol.to_uppercase() == protocol.to_uppercase() {
            return Some(key.to_string());
        }
    }
    None
}