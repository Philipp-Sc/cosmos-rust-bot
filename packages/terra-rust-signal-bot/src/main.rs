mod state;

use state::*;

use std::{path::PathBuf, time::UNIX_EPOCH};

use anyhow::{Context as _};
use directories::ProjectDirs;
use env_logger::Env;
use futures::{pin_mut, StreamExt}; 
use log::debug;
use presage::{
    prelude::{
        content::{
            Content, ContentBody, DataMessage, SyncMessage,
        },
        proto::sync_message::Sent,
        SignalServers,
    },
    prelude::{phonenumber::PhoneNumber, Uuid},
    Manager, SledConfigStore,
};
use structopt::StructOpt;
use url::Url;

use std::thread;
use std::time::Duration;

#[derive(StructOpt)]
#[structopt(about = "a basic signal CLI to try things out")]
struct Args {
    #[structopt(long = "db-path", short = "d")]
    db_path: Option<PathBuf>,

    #[structopt(flatten)]
    subcommand: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    #[structopt(about = "register a primary device using a phone number")]
    Register {
        #[structopt(long = "servers", short = "s", default_value = "staging")]
        servers: SignalServers,
        #[structopt(long, help = "Phone Number to register with in E.164 format")]
        phone_number: PhoneNumber,
        #[structopt(long)]
        use_voice_call: bool,
        #[structopt(
            long = "captcha",
            help = "Captcha obtained from https://signalcaptchas.org/registration/generate.html"
        )]
        captcha: Option<Url>,
        #[structopt(long, help = "Force to register again if already registered")]
        force: bool,
    },
    #[structopt(
        about = "generate a QR code to scan with Signal for iOS or Android to provision a secondary device on the same phone number"
    )]
    LinkDevice { 
        #[structopt(long, short = "s", default_value = "production")]
        servers: SignalServers,
        #[structopt(
            long,
            short = "n",
            help = "Name of the device to register in the primary client"
        )]
        device_name: String,
    },
    #[structopt(about = "verify the code you got from the SMS or voice-call when you registered")]
    Verify {
        #[structopt(long, short = "c", help = "SMS / Voice-call confirmation code")]
        confirmation_code: u32,
    },
    #[structopt(about = "Get information on the registered user")]
    Whoami,
    #[structopt(about = "Receives all pending messages and saves them to disk")]
    Receive, 
    #[structopt(about = "sends a message")]
    Send {
        #[structopt(long, short = "u", help = "uuid of the recipient")]
        uuid: Uuid,
        #[structopt(long, short = "m", help = "Contents of the message to send")]
        message: String,
    }, 
    #[structopt(about = "Terra-rust-bot feature: Reply with status information to incoming messages.")]
    ReceiveLoop, 
    #[structopt(about = "Terra-rust-bot feature: print information directly to the console.")]
    LocalDisplay {
        #[structopt(long, short = "m", help = "message")]
        message: String,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::from_env(
        Env::default().default_filter_or(format!("{}=info", env!("CARGO_PKG_NAME"))),
    )
    .init();

    let args = Args::from_args();

    let db_path = args.db_path.unwrap_or_else(|| {
        ProjectDirs::from("org", "whisperfish", "presage")
            .unwrap()
            .config_dir()
            .into()
    });
    debug!("opening config database from {}", db_path.display());
    let config_store = SledConfigStore::new(db_path)?;

    let csprng = rand::thread_rng();
    let mut manager = Manager::new(config_store, csprng)?;

    let millis = Duration::from_millis(1000); 

    match args.subcommand {
        Subcommand::Register {
            servers,
            phone_number,
            use_voice_call,
            captcha,
            force,
        } => {
            manager
                .register(
                    servers,
                    phone_number,
                    use_voice_call,
                    captcha.as_ref().map(|u| u.host_str().unwrap()),
                    force,
                )
                .await?;
        }
        Subcommand::LinkDevice {
            servers,
            device_name,
        } => {
            manager
                .link_secondary_device(servers, device_name.clone())
                .await?;
        }
        Subcommand::Verify { confirmation_code } => {
            manager.confirm_verification_code(confirmation_code).await?;
        }
        Subcommand::Receive => {
            let messages = manager
                .clone()
                .receive_messages()
                .await
                .context("failed to initialize messages stream")?;
            pin_mut!(messages);
            while let Some(Content { metadata, body }) = messages.next().await {
                match body {
                    ContentBody::DataMessage(message)
                    | ContentBody::SynchronizeMessage(SyncMessage {
                        sent:
                            Some(Sent {
                                message: Some(message),
                                ..
                            }),
                        ..
                    }) => {
                        if let Some(quote) = &message.quote {
                            println!(
                                "Quote from {:?}: > {:?} / {}",
                                metadata.sender,
                                quote,
                                message.body(),
                            );
                        } else if let Some(reaction) = message.reaction {
                            println!(
                                "Reaction to message sent at {:?}: {:?}",
                                reaction.target_sent_timestamp, reaction.emoji,
                            )
                        } else {
                            println!("Message from {:?}: {:?}", metadata, message); 
                        }
                    }
                    ContentBody::SynchronizeMessage(m) => {
                        eprintln!("Unhandled sync message: {:?}", m);
                    }
                    ContentBody::TypingMessage(_) => {
                        println!("{:?} is typing", metadata.sender);
                    }
                    ContentBody::CallMessage(_) => {
                        println!("{:?} is calling!", metadata.sender);
                    }
                    ContentBody::ReceiptMessage(_) => {
                        println!("Got read receipt from: {:?}", metadata.sender);
                    }
                }
            }
        }
        Subcommand::Send { uuid, message } => {
            let timestamp = std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64;

            let message = ContentBody::DataMessage(DataMessage {
                body: Some(message),
                timestamp: Some(timestamp),
                ..Default::default()
            });
            manager.send_message(uuid, message, timestamp).await?;
        } 
        Subcommand::Whoami => {
            println!("{:?}", &manager.whoami().await?)
        }
        Subcommand::LocalDisplay {message} => {
            println!("{esc}c", esc = 27 as char); 
            println!("{}", terra_rust_bot_state(&message).await);
        }
        Subcommand::ReceiveLoop => {
            println!("{}","whoami()");
            let mut my_uuid = None;
            while my_uuid.is_none() {
                match manager.whoami().await {
                    Ok(uuid) => {
                        my_uuid = Some(uuid);
                    },
                    Err(e) => {
                        println!("{:?}",e);
                        thread::sleep(millis);
                    }
                };
            }
            let my_uuid = my_uuid.unwrap().uuid;

            println!("{}","receive_messages()");
            let mut messages = None;
            while messages.is_none() {
                match manager
                .clone()
                .receive_messages()
                .await
                .context("failed to initialize messages stream") {
                    Ok(m) => {
                        messages = Some(m);
                    },
                    Err(e) => {
                        println!("{:?}",e);
                        thread::sleep(millis);
                    }
                };
            }
            let messages = messages.unwrap(); 
 
            pin_mut!(messages);

            while let Some(Content { metadata, body }) = messages.next().await {
                println!("{}","messages.next()"); 
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
                                        println!("Message from myself ({:?}): {:?}", sender_uuid, sender_message); 
                                        let timestamp = std::time::SystemTime::now()
                                            .duration_since(UNIX_EPOCH).unwrap()
                                            .as_millis() as u64;

                                        let mut msg_sent = false;
                                        while !msg_sent {
                                            let message = ContentBody::DataMessage(DataMessage {
                                                body: Some(terra_rust_bot_state(&sender_message).await),
                                                timestamp: Some(timestamp),
                                                ..Default::default()
                                            });
                                            msg_sent = match manager.send_message(my_uuid, message, timestamp).await {
                                                Ok(_) => true,
                                                _ => false,
                                            };
                                        }
                                        
                                    }  
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        } 
    };
    Ok(())
}
