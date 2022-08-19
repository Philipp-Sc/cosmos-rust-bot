use bot_library::*;
// to get control over the settings
use cosmos_rust_interface::utils::entry::db::*; // to get control over the data

// this is the controller for the commands or notification gathering.
// most of the heavy lifting will be taken from the two imports already.

pub async fn process_message(msg: String) -> String {
    "Hi, I read your message. ~ Your Cosmos-Rust-Bot".to_string()
}

pub async fn handle_notifications() -> Option<String> {
    None
}