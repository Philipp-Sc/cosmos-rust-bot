// extends presage example, to run in-memory after the device is linked and delegate requests to the cosmos_rust_bot controller.
use chrono::Utc;
use cosmos_rust_interface::utils::entry::db::load_sled_db;
use cosmos_rust_interface::utils::entry::db::notification::socket::spawn_socket_notification_server;
use cosmos_rust_interface::utils::entry::{CosmosRustServerValue, Notification};
use log::info;
use tokio::time::timeout;

use std::sync::{Arc, Mutex};
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
) {
    info!(
        "Status: Processing Message From Myself - {}\n ({:?})",
        Utc::now(),
        sender_uuid
    );
    send_message_to_self(manager, send_message.clone(), sender_uuid)
        .await
        .ok();
    /*
    let mut msg_sent: u64 = 1;
    while msg_sent > 0 && msg_sent < 5 {
        msg_sent = match send_message_to_self(manager, send_message.clone(), sender_uuid).await {
            Ok(()) => 0,
            Err(e) => {
                info!(
                    "Status: Processing Message From Myself Error (Trying again..) - {},\n{:?}",
                    Utc::now(),
                    e
                );
                msg_sent + 1
            }
        };
    }*/
    info!(
        "Status: Processing Message From Myself Done - {}",
        Utc::now()
    );
}

pub async fn run_cosmos_rust_signal_bot<C: ConfigStore>(
    manager_option: Option<Manager<C, Registered>>,
) {
    // sled db to store notifications and user meta data.
    let tree = load_sled_db("cosmos_rust_signal_bot_sled_db");
    spawn_socket_notification_server(&tree);
    let tree_2 = tree.clone();

    // instead of HashMap use sled-db.
    // _thread creates entries with prefix "notify"
    // notify task uses scan_prefix to process current notifiy requests.
    // notify removes entry.
    // to prevent conflicts add timestamp into hash.
    // Advantage over HashMap, it's not blocking the whole hashmap.
    // Advamtage state can be saved. e.g stop start continue.
    let notify_stack: Arc<Mutex<HashMap<u64, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));
    let notify_stack_2 = notify_stack.clone();
    let _thread = tokio::spawn(async move {
        let mut subscriber = tree_2.watch_prefix(Notification::get_prefix());
        while let Some(event) = (&mut subscriber).await {
            match event {
                sled::Event::Insert { key, value } => {
                    let notification = CosmosRustServerValue::from(value.to_vec());
                    match notification {
                        // TODO optimize: no reason to use serde_json here
                        CosmosRustServerValue::Notification(n) => {
                            let empty: Vec<serde_json::Value> = Vec::new();
                            let fields = n
                                .get_query()
                                .get("fields")
                                .map(|x| x.as_array().unwrap_or(&empty))
                                .unwrap_or(&empty)
                                .iter()
                                .map(|x| x.as_str().unwrap_or("").to_string())
                                .collect::<Vec<String>>();

                            let mut field_list: Vec<serde_json::Value> = Vec::new();

                            for i in 0..n.entries.len() {
                                let mut m = serde_json::json!({});
                                for field in fields.iter() {
                                    if let Some(val) = n.entries[i].try_get(field) {
                                        if let Some(summary_text) = val.as_str() {
                                            m.as_object_mut().unwrap().insert(
                                                field.to_string(),
                                                serde_json::json!(summary_text.to_string()),
                                            );
                                        }
                                    }
                                }
                                field_list.push(m);
                            }
                            let mut msg_1: Vec<String> = field_list
                                .iter()
                                .filter(|x| x.as_object().is_some())
                                .map(|x| x.as_object().unwrap())
                                .filter(|x| x.get("summary").is_some())
                                .map(|x| x.get("summary").unwrap())
                                .filter(|x| x.as_str().is_some())
                                .map(|x| x.as_str().unwrap().to_string())
                                .collect();
                            let mut msg_2 = field_list
                                .iter()
                                .map(|x| {
                                    if let Some(obj) = x.as_object() {
                                        match (obj.get("key"), obj.get("value")) {
                                            (Some(key), Some(value)) => {
                                                match (key.as_str(), value.as_str()) {
                                                    (Some(k), Some(v)) => {
                                                        return Some((k, v));
                                                    }
                                                    _ => {}
                                                };
                                            }
                                            _ => {}
                                        };
                                    }
                                    return None;
                                })
                                .map(|x| x.map(|y| format!("Key: {}\nValue: {}", y.0, y.1)))
                                .filter(|x| x.is_some())
                                .map(|x| x.unwrap())
                                .collect();

                            let mut msg: Vec<String> = Vec::new();
                            msg.append(&mut msg_1);
                            msg.append(&mut msg_2);
                            notify_stack_2
                                .lock()
                                .unwrap()
                                .insert(n.calculate_hash(), msg);
                            tree_2.remove(key).ok();
                        }
                    };
                }
                _ => {}
            }
        }
    });

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
                            let mut lock = notify_stack.try_lock();
                            if let Ok(ref mut mutex) = lock {
                                info!("Status: Notifications Available - {}", Utc::now());
                                let key = mutex.iter().map(|x| x.0.clone()).next();
                                if let Some(k) = key {
                                    let v = mutex.remove(&k).unwrap();
                                    process_message_from_self(my_uuid, v, &manager).await;
                                }
                            }
                        }
                    };
                }
            }
        }
        None => {
            let notify_stack_3 = notify_stack.clone();
            let _thread = std::thread::spawn(move || loop {
                let millis = Duration::from_millis(1000);
                thread::sleep(millis);
                let mut lock = notify_stack_3.try_lock();
                if let Ok(ref mut mutex) = lock {
                    print!("\x1B[2J");
                    for n in mutex.drain() {
                        for msg in n.1 {
                            println!("{}", msg);
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
