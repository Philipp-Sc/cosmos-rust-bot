// RUSTFLAGS="--cfg tokio_unstable" cargo build

use cosmos_rust_telegram_bot::cosmos_rust_bot::handle_message;
use teloxide::{prelude::*, types::MessageKind};
use std::error::Error;
use chrono::Utc;
use cosmos_rust_interface::utils::entry::db::load_sled_db;
use cosmos_rust_interface::utils::entry::db::notification::socket::spawn_socket_notification_server;
use cosmos_rust_interface::utils::entry::db::notification::notify_sled_db;
use cosmos_rust_interface::utils::entry::*;
use cosmos_rust_interface::utils::entry::{CosmosRustServerValue, Notification, Notify};
use std::sync::Arc;
use teloxide::types::ParseMode;
use tokio::task::JoinSet;

// RUST_LOG=error,debug,info
#[tokio::main]
async fn main() {
    let mut join_set: JoinSet<()> = JoinSet::new();

    let tree = Arc::new(load_sled_db("cosmos_rust_telegram_bot_sled_db"));
    spawn_socket_notification_server(tree.clone().as_ref());

    pretty_env_logger::init();
    log::info!("Starting shared state bot...");

    let bot = Bot::from_env();

    // task that sends the messages.
    let tree_2 = tree.clone();
    let bot_clone = bot.clone();
    join_set.spawn(async move {
        let mut subscriber = tree_2.watch_prefix(Notify::get_prefix());
        while let Some(event) = (&mut subscriber).await {
            match event {
                sled::Event::Insert { key, value } => {
                    if let CosmosRustServerValue::Notify(notify) =
                        CosmosRustServerValue::from(value.to_vec())
                    {
                        log::info!("Status: Insert Notify Event - {}", Utc::now());
                        match tree_2.get(notify.user_hash.to_ne_bytes().to_vec()) {
                            Ok(Some(value)) => {
                                let chat_id: Option<i64> =
                                    match CosmosRustServerValue::from(value.to_vec()) {
                                        CosmosRustServerValue::UserMetaData(user_meta_data) => {
                                            Some(user_meta_data.user_chat_id)
                                        }
                                        _ => None,
                                    };
                                if let Some(id) = chat_id {
                                    log::info!(
                                        "Status: ChatID Available - {} - {}",
                                        Utc::now(),
                                        id
                                    );
                                    for msg in notify.msg {
                                        bot_clone.send_message(ChatId(id), msg)
                                            .disable_web_page_preview(true)
                                            .send().await.ok();
                                    }
                                    // TODO: remove user if not subscribed and has no pending notifications/notify
                                }
                            }
                            _ => {}
                        }
                        tree_2.remove(key).ok();
                    }
                }
                sled::Event::Remove { key } => {}
            }
        }
    });

    // task that receives the messages
    join_set.spawn(async move {
        let handler = Update::filter_message().endpoint(
            |msg: Message, bot: Bot, tree: Arc<sled::Db>| async move {
                receive(tree, bot, msg).await.ok();
                respond(())
            },
        );
        Dispatcher::builder(bot, handler)
            // Pass the shared state to the handler as a dependency.
            .dependencies(dptree::deps![tree])
            .build()
            .dispatch()
            .await;
    });

    tokio::signal::ctrl_c().await.unwrap();
    join_set.shutdown().await;
}

async fn receive(
    tree: Arc<sled::Db>,
    bot: Bot,
    message: Message,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match message.kind {
        MessageKind::Common(ref msg_common) => {
            match msg_common.from {
                None => {}
                Some(ref user) => {
                    if !user.is_bot {
                        let user_meta_data = CosmosRustServerValue::UserMetaData(UserMetaData {
                            timestamp: Utc::now().timestamp(),
                            user_id: user.id.0,
                            user_name: user.username.to_owned(),
                            first_name: Some(user.first_name.to_owned()),
                            last_name: user.last_name.to_owned(),
                            language_code: user.language_code.to_owned(),
                            user_chat_id: message.chat.id.0,
                        });
                        notify_sled_db(&tree, user_meta_data);
                        log::info!(
                            "Handle Message: Request: {} - {:?}",
                            Utc::now(),
                            &message
                        );
                        handle_message(
                            user.id.0,
                            message.text().unwrap_or("help").to_string(),
                            &tree,
                        )
                            .await;
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
