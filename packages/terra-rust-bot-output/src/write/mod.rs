// here load terra-rust-bot.json
// change parameter
// save

use terra_rust_bot_essentials::shared::{UserSettings,load_user_settings};
use serde_json::Value;
use serde_json::{to_value,from_value};
use serde_json::json;
use std::fs;


pub fn save_user_settings(user_settings: &UserSettings,path: &str) -> anyhow::Result<()> {
    let line = format!("{}", serde_json::to_string(user_settings)?);
    fs::write(path, &line)?;
    Ok(())
}

pub async fn update_user_settings(path: &str,field: &str, new_value: &str) -> anyhow::Result<()> {

    let mut user_settings: UserSettings = load_user_settings(path);
    if user_settings.hot_reload {
        let mut value: Value = to_value(user_settings)?;
        let new_value = match new_value {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => json!(new_value),
        };
        *value.get_mut(field).unwrap() = new_value;
        user_settings = from_value(value)?;
        save_user_settings(&user_settings, path)?;
    }
    Ok(())

}