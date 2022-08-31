// extends presage example, to run in-memory after the device is linked and delegate requests to the cosmos_rust_bot controller.
use tokio::time::timeout;
use log::{debug, info};
use chrono::Utc;

use std::time::Duration;

use std::thread;
use std::{path::PathBuf, time::UNIX_EPOCH};

use anyhow::{bail, Context as _};
use chrono::Local;
use futures::{channel::oneshot, future, pin_mut, StreamExt};
use presage::{prelude::{
    content::{
        Content, ContentBody, DataMessage, GroupContext, GroupContextV2, GroupType, SyncMessage,
    },
    proto::sync_message::Sent,
    Contact, GroupMasterKey, SignalServers,
}, prelude::{phonenumber::PhoneNumber, ServiceAddress, Uuid}, ConfigStore, Manager, RegistrationOptions, SledConfigStore, SecretVolatileConfigStore, Registered};
use tokio::{
    fs,
    io::{self, AsyncBufReadExt, BufReader},
};

use crate::cosmos_rust_bot::{handle_message, handle_notifications};

async fn send_message_to_self<C: ConfigStore>(manager: &Manager<C, Registered>, message: Vec<String>, my_uuid: Uuid) -> anyhow::Result<()> {
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

pub async fn process_message_from_self<C: ConfigStore>(sender_uuid: Uuid, sender_message: String, manager: &Manager<C, Registered>) {
    info!("Status: Processing Message From Myself - {}\n ({:?}): {:?}", Utc::now(),sender_uuid, sender_message);


    let mut msg_sent = false;
    while !msg_sent {
        let text = handle_message(sender_message.to_owned()).await;
        msg_sent = match send_message_to_self(manager, text, sender_uuid).await
        {
            Ok(()) => true,
            Err(e) => {
                info!("Status: Processing Message From Myself Error (Trying again..) - {},\n{:?}", Utc::now(),e);
                false
            }
        };
    }
    info!("Status: Processing Message From Myself Done - {}", Utc::now());
}


pub async fn run_cosmos_rust_signal_bot<C: ConfigStore>(manager_option: Option<Manager<C, Registered>>) {
    match manager_option {
        Some(manager) => {
            info!("Status: Starting in Default Mode - {}", Utc::now());
            let mut timestamp_last_notification_check = Utc::now().timestamp_millis();
            info!("Status: Ready - {}", Utc::now());
            loop {
                let mut my_uuid = None;
                while my_uuid.is_none() {
                    info!("Status: Whoami - {}",Utc::now());
                    match manager.whoami().await {
                        Ok(uuid) => {
                            my_uuid = Some(uuid);
                        }
                        Err(e) => {
                            info!("Error: Whoami - {}, \n{:?}\n Waiting 1s before retrying..",Utc::now(),e);
                            let millis = Duration::from_millis(1000);
                            thread::sleep(millis);
                        }
                    };
                }

                let my_uuid = my_uuid.unwrap().uuid;
                info!("Status: Whoami Ready - {},\nUUID: {:?}",Utc::now(),my_uuid);

                info!("Status: Loading Receive Message Stream - {}", Utc::now());
                let mut messages = None;
                while messages.is_none() {
                    match manager
                        .clone()
                        .receive_messages()
                        .await
                        .context("failed to initialize messages stream") {
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

                let mut searching = false;
                info!("Status: Entering Bot Loop - {}", Utc::now());
                loop {
                    let next = timeout(Duration::from_secs(1), messages.next()).await;
                    match next {
                        Ok(Some(Content { metadata, body })) => {
                            info!("Status: Message Received - {}", Utc::now());
                            searching = false;
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
                                        if sender_uuid == my_uuid && my_uuid == Uuid::parse_str(&destination_uuid).unwrap() {
                                            // message from and to self
                                            if let Some(_quote) = &message.quote {
                                                // quote
                                            } else if let Some(_reaction) = message.reaction {
                                                // reaction
                                            } else {
                                                // default
                                                if let Some(sender_message) = message.body {
                                                    process_message_from_self(sender_uuid, sender_message, &manager).await;
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => { // no next message / irrelevant message type
                            let timestamp = Utc::now().timestamp_millis();
                            if timestamp_last_notification_check + 1000 < timestamp {
                                timestamp_last_notification_check = timestamp;
                                info!("Status: Notification Check - {}", Utc::now());
                                match handle_notifications().await {
                                    None => {}
                                    Some(text_to_send_back) => {
                                        info!("Status: Notification Available - {}\n{}", Utc::now(),text_to_send_back);
                                        let mut msg_sent = false;
                                        while !msg_sent {
                                            msg_sent = match send_message_to_self(&manager, vec![text_to_send_back.to_owned()], my_uuid).await
                                            {
                                                Ok(()) => true,
                                                Err(e) => {
                                                    info!("Status: Sending Notification Error (Trying again..) - {},\n{:?}", Utc::now(),e);
                                                    false
                                                }
                                            };
                                        }
                                        info!("Status: Notification Sent - {}", Utc::now());
                                    }
                                }
                            }
                        }
                    };
                }
            }
        }
        None => {
            info!("Status: Starting in Console Mode - {}", Utc::now());
            let stdin = io::stdin();
            let mut reader = BufReader::new(stdin).lines();
            loop {
                match &reader.next_line().await {
                    Ok(Some(line)) => {
                        let text: Vec<String> = handle_message(line.to_owned()).await;
                        for text_item in text {
                            println!("{}", text_item);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}