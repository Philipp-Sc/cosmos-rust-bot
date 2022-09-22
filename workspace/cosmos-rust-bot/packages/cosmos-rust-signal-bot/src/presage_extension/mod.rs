// extends presage example, to run in-memory after the device is linked and delegate requests to the cosmos_rust_bot controller.
use chrono::Utc;
use cosmos_rust_interface::utils::entry::db::load_sled_db;
use cosmos_rust_interface::utils::entry::db::notification::socket::spawn_socket_notification_server;
use cosmos_rust_interface::utils::entry::{CosmosRustServerValue, Notification, Notify};
use log::info;
use tokio::time::timeout;

use std::time::Duration;

use anyhow::Context as _;
use futures::{pin_mut, StreamExt};
use presage::{
    prelude::Uuid,
    prelude::{
        content::{Content, ContentBody, DataMessage, SyncMessage},
        proto::sync_message::Sent,
    },
    ConfigStore, Manager, Registered,
};
use std::collections::HashMap;
use std::thread;
use std::time::UNIX_EPOCH;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::cosmos_rust_bot::handle_message;

async fn send_message_to_self<C: ConfigStore>(
    manager: &Manager<C, Registered>,
    message: Vec<String>,
    my_uuid: Uuid,
) -> anyhow::Result<()> {
    for msg in message {
        let timestamp = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        let message = ContentBody::DataMessage(DataMessage {
            body: Some(msg),
            timestamp: Some(timestamp),
            ..Default::default()
        });

        manager.send_message(my_uuid, message, timestamp).await?;
    }
    Ok(())
}

pub async fn process_message_from_self<C: ConfigStore>(
    sender_uuid: Uuid,
    send_message: Vec<String>,
    manager: &Manager<C, Registered>,
) -> anyhow::Result<()> {
    info!(
        "Status: Processing Message From Myself - {}\n ({:?})",
        Utc::now(),
        sender_uuid
    );
    let res = send_message_to_self(manager, send_message.clone(), sender_uuid).await;
    info!(
        "Status: Processing Message From Myself Done - {}",
        Utc::now()
    );
    res
}

pub async fn run_cosmos_rust_signal_bot<C: ConfigStore>(
    manager_option: Option<Manager<C, Registered>>,
) {
    // sled db to store notifications and user meta data.
    let tree = load_sled_db("cosmos_rust_signal_bot_sled_db");
    spawn_socket_notification_server(&tree);
    /*    let tree_2 = tree.clone();

    let _thread = tokio::spawn(async move {
        let mut subscriber = tree_2.watch_prefix(Notification::get_prefix());
        while let Some(event) = (&mut subscriber).await {
            match event {
                sled::Event::Remove { key } => {}
                sled::Event::Insert { key, value } => {
                    match CosmosRustServerValue::from(value.to_vec()) {
                        CosmosRustServerValue::Notification(n) => {}
                        _ => {}
                    }
                }
            }
        }
    });*/

    match manager_option {
        // current architecture suboptimal: recieving messages has a higher priority than sending results.
        // could be abused with spam
        Some(manager) => {
            info!("Status: Starting in Default Mode - {}", Utc::now());
            info!("Status: Ready - {}", Utc::now());
            let mut my_uuid = None;
            while my_uuid.is_none() {
                info!("Status: Whoami - {}", Utc::now());
                match manager.whoami().await {
                    Ok(uuid) => {
                        my_uuid = Some(uuid);
                    }
                    Err(e) => {
                        info!(
                            "Error: Whoami - {}, \n{:?}\n Waiting 1s before retrying..",
                            Utc::now(),
                            e
                        );
                        let millis = Duration::from_millis(1000);
                        thread::sleep(millis);
                    }
                };
            }

            let my_uuid = my_uuid.unwrap().uuid;
            info!(
                "Status: Whoami Ready - {},\nUUID: {:?}",
                Utc::now(),
                my_uuid
            );
            loop {
                info!("Status: Loading Receive Message Stream - {}", Utc::now());
                let mut messages = None;
                while messages.is_none() {
                    match manager
                        .clone()
                        .receive_messages()
                        .await
                        .context("failed to initialize messages stream")
                    {
                        Ok(m) => {
                            messages = Some(m);
                        }
                        Err(e) => {
                            info!("Error: Loading Receive Message Stream - {}, \n{:?}\n Waiting 1s before retrying..",Utc::now(),e);
                            let millis = Duration::from_millis(1000);
                            thread::sleep(millis);
                        }
                    };
                }
                let messages = messages.unwrap();
                pin_mut!(messages);
                info!("Status: Receive Message Stream Ready - {}", Utc::now());

                info!("Status: Entering Bot Loop - {}", Utc::now());
                loop {
                    // TODO spam protection, rate limit incoming messages.
                    let next = timeout(Duration::from_secs(1), messages.next()).await;
                    match next {
                        Ok(Some(Content { metadata, body })) => {
                            info!("Status: Message Received - {}", Utc::now());
                            match body {
                                // ContentBody::DataMessage(message) |
                                ContentBody::SynchronizeMessage(SyncMessage {
                                    sent:
                                        Some(Sent {
                                            destination_uuid: Some(destination_uuid),
                                            message: Some(message),
                                            ..
                                        }),
                                    ..
                                }) => {
                                    if let Some(sender_uuid) = metadata.sender.uuid {
                                        if sender_uuid == my_uuid
                                            && my_uuid
                                                == Uuid::parse_str(&destination_uuid).unwrap()
                                        {
                                            // message from and to self
                                            if let Some(_quote) = &message.quote {
                                                // quote
                                            } else if let Some(_reaction) = message.reaction {
                                                // reaction
                                            } else {
                                                // default
                                                if let Some(sender_message) = message.body {
                                                    handle_message(sender_message, &tree).await;
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            // timeout: message stream empty
                            // send notifications
                            info!("Status: Notification Check - {}", Utc::now());
                            let mut r = tree.scan_prefix(Notify::get_prefix());
                            if let Some(Ok((key, val))) = r.next() {
                                info!("Status: Notifications Available - {}", Utc::now());
                                if let CosmosRustServerValue::Notify(notify) =
                                    CosmosRustServerValue::from(val.to_vec())
                                {
                                    match process_message_from_self(my_uuid, notify.msg, &manager)
                                        .await
                                    {
                                        Ok(_) => {
                                            tree.remove(key).ok();
                                        }
                                        Err(_) => {
                                            // do not remove entry
                                        }
                                    }
                                }
                            }
                        }
                    };
                }
            }
        }
        None => {
            let tree_2 = tree.clone();
            let _thread = std::thread::spawn(move || loop {
                let r = tree_2.scan_prefix(Notify::get_prefix());
                let mut once = true;
                for item in r {
                    if once {
                        print!("\x1B[2J");
                        once = false;
                    }
                    if let Ok((key, val)) = item {
                        info!("Status: Notifications Available - {}", Utc::now());
                        if let CosmosRustServerValue::Notify(notify) =
                            CosmosRustServerValue::from(val.to_vec())
                        {
                            for msg in notify.msg {
                                println!("{}", msg);
                            }
                            tree_2.remove(key).ok();
                        }
                    }
                }
            });

            info!("Status: Starting in Console Mode - {}", Utc::now());
            let stdin = io::stdin();
            let mut reader = BufReader::new(stdin).lines();
            loop {
                let next = timeout(Duration::from_secs(2), reader.next_line());
                match next.await {
                    Ok(Ok(Some(line))) => {
                        handle_message(line.to_owned(), &tree).await;
                    }
                    Err(_) => {
                        // timeout: no console input
                        // send notification handled in thread
                    }
                    _ => {}
                }
            }
        }
    }
}
