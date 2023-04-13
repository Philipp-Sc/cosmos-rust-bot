// RUSTFLAGS="--cfg tokio_unstable" cargo build

use cosmos_rust_telegram_bot::cosmos_rust_bot::handle_message;
use teloxide::{prelude::*, types::MessageKind};
use std::error::Error;
use chrono::Utc;
use cosmos_rust_interface::utils::entry::db::load_sled_db;
use cosmos_rust_interface::utils::entry::db::notification::socket::spawn_socket_notification_server;
use cosmos_rust_interface::utils::entry::db::notification::{import_user_meta_data, CRB_USER_META_DATA_STORE_JSON, notify_sled_db};
use cosmos_rust_interface::utils::entry::*;
use cosmos_rust_interface::utils::entry::{CosmosRustServerValue, Notify};
use std::sync::Arc;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};
use teloxide::types::ReplyMarkup::InlineKeyboard;
use tokio::task::JoinSet;


const NOTIFICATION_SOCKET: &str = "./tmp/cosmos_rust_bot_notification_socket";
const TG_SLED_DB: &str = "./tmp/cosmos_rust_telegram_bot_sled_db";

// RUST_LOG=error,debug,info
#[tokio::main]
async fn main() {

    let mut join_set: JoinSet<()> = JoinSet::new();

    let tree = Arc::new(load_sled_db(TG_SLED_DB));

    import_user_meta_data(&tree,CRB_USER_META_DATA_STORE_JSON);

    spawn_socket_notification_server(NOTIFICATION_SOCKET,tree.clone().as_ref());

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
                        CosmosRustServerValue::try_from(value.to_vec()).unwrap()
                    {
                        log::info!("Status: Insert Notify Event - {}", Utc::now());
                        match tree_2.get(notify.user_hash.to_ne_bytes().to_vec()) {
                            Ok(Some(value)) => {
                                let chat_id: Option<i64> =
                                    match CosmosRustServerValue::try_from(value.to_vec()).unwrap() {
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
                                    for i in 0..notify.msg.len() {

                                        let mut batch: Vec<&str> = Vec::new();

                                        let mut offset = 0;
                                        let chunk_size = 4000;
                                        while offset < notify.msg[i].len() {
                                            let chunk = &notify.msg[i][offset..std::cmp::min(offset + chunk_size, notify.msg[i].len())];
                                            batch.push(chunk);
                                            offset += chunk_size;
                                        }

                                        let batch_pop = batch.pop();

                                        for b in batch {
                                            bot_clone.send_message(ChatId(id), b)
                                                .disable_web_page_preview(true)
                                                .send().await.ok();
                                        }
                                        if let Some(last) = batch_pop {
                                            // Create the inline keyboard with the desired buttons
                                            let mut buttons: Vec<Vec<InlineKeyboardButton>> = Vec::new();
                                            if i < notify.buttons.len() {
                                                for row in &notify.buttons[i] {
                                                    buttons.push(row.iter().map(|b| {
                                                        if b.1.starts_with("https://") {
                                                            InlineKeyboardButton::new(b.0.to_owned(), InlineKeyboardButtonKind::Url(b.1.to_owned().parse().unwrap()))
                                                        } else {
                                                            InlineKeyboardButton::new(b.0.to_owned(), InlineKeyboardButtonKind::CallbackData(b.1.to_owned()))
                                                        }
                                                    }).collect());
                                                }
                                            }
                                            let keyboard = InlineKeyboardMarkup::new(buttons);

                                            bot_clone.send_message(ChatId(id), last)
                                                .disable_web_page_preview(true)
                                                .reply_markup(keyboard)
                                                .send().await.ok();
                                        }

                                    }
                                    // TODO: remove user if not subscribed and has no pending notifications/notify
                                }
                            }
                            _ => {}
                        }
                        tree_2.remove(key).ok();
                    }
                }
                sled::Event::Remove { .. } => {}
            }
        }
    });

    // task that receives the messages
    join_set.spawn(async move {

        let handler = dptree::entry()
            .branch(Update::filter_message().endpoint(
                |msg: Message, tree: Arc<sled::Db>| async move {
                    receive(tree,msg).await.ok();
                    respond(())
                },
            ))
            .branch(Update::filter_callback_query().endpoint(
                |bot: Bot, q: CallbackQuery, tree: Arc<sled::Db>,| async move {
                    callback_handler(bot,q,tree).await.ok();
                    respond(())
                },
            ));

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

/// When it receives a callback from a button it edits the message with all
/// those buttons writing a text with the selected Debian version.
///
/// **IMPORTANT**: do not send privacy-sensitive data this way!!!
/// Anyone can read data stored in the callback button.
async fn callback_handler(bot: Bot, q: CallbackQuery, tree: Arc<sled::Db>) -> Result<(), Box<dyn Error + Send + Sync>> {

    if !q.from.is_bot {
        if let Some(command) = q.data {
            if let Some(message) = q.message {

                // Tell telegram that we've seen this query, to remove 🕑 icons from the
                //
                // clients. You could also use `answer_callback_query`'s optional
                // parameters to tweak what happens on the client side.
                bot.answer_callback_query(q.id).send().await?;

                let user_meta_data = CosmosRustServerValue::UserMetaData(UserMetaData {
                    timestamp: Utc::now().timestamp(),
                    user_id: q.from.id.0,
                    user_name: q.from.username.to_owned(),
                    first_name: Some(q.from.first_name.to_owned()),
                    last_name: q.from.last_name.to_owned(),
                    language_code: q.from.language_code.to_owned(),
                    user_chat_id: message.chat.id.0,
                });
                notify_sled_db(&tree, user_meta_data);
                log::info!(
                            "Handle Message Button Callback: Request: {} - {:?},\nCommand: {:?}",
                            Utc::now(),
                            &message,
                            &command,
                        );
                handle_message(
                    q.from.id.0,
                    command,
                    &tree,
                )
                    .await;
            }
        }
    }
    Ok(())
}
