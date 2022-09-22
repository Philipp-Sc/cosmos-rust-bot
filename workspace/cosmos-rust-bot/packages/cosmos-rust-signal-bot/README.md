<div align="center">


  <h1>Cosmos Rust Signal Bot</h1> 
  <p>Signal Messenger integration</p> 

  <img src="https://github.com/Philipp-Sc/media/raw/main/cosmos-rust-bot/cosmos-rust-signal-bot/gallery/signal_bot_auto_stake.png" height="250">
  <img src="https://github.com/Philipp-Sc/media/raw/main/cosmos-rust-bot/cosmos-rust-signal-bot/gallery/signal_bot_help.png" height="250">
  <img src="https://github.com/Philipp-Sc/media/raw/main/cosmos-rust-bot/cosmos-rust-signal-bot/gallery/signal_messenger.png" height="250">

  </div>
<br/>

### [Install](#install) · [Usage](#usage)

> :warning: It is recommended to use a dedicated Signal account for cosmos-rust-bot.

> :warning: The Signal Configuration is kept in-memory and secured via the [secrets crate](https://github.com/stouset/secrets)

## How it works

#### Configuration

* **cosmos-rust-bot** writes the current data (via cosmos-rust-interface) to an embedded database.    
* Using UNIX Sockets **cosmos-rust-bot** communicates with **cosmos-rust-signal-bot**:
  - **cosmos-rust-bot**: request notification(s) for a given query
  - **cosmos-rust-signal-bot**: receive notification(s)   
Note: **cosmos-rust-bot** issues notifications either directly after a request or when the data related to a subscription was updated.
* cosmos-rust-signal-bot manages its notification in its own embedded database.

## Install

**Build cosmos-rust-signal-bot**

* `cd ./packages/cosmos-rust-signal-bot`
* `./install.sh dev`

## Usage

### Run cosmos-rust-signal-bot

* If you already started the cosmos-rust-bot then you can use the cosmos-rust-signal-bot.

* `cd ./packages/cosmos-rust-signal-bot`

*The signal client is still experimental.  
The following script is configured to restart the signal-bot in case it crashes unexpectedly.*

* `./run.sh` (run the signal-bot)    
* run `cat nohup.out` to view the QR code to link device. The configuration is stored safely in-memory as long as **cosmos-rust-signal-bot** runs.    
Note: if you want a persistent device linkage edit `./run.sh` and:
  - remove `--volatile`
  - add `--try-use-linked-device` after `cosmos-rust-bot` (this will re-use your existing configuration)
* `./stop.sh` (stop the signal-bot)


* Within your **Signal Messenger App** in your Contact search bar type **"Note to Self"**.  
 
## Uninstall

* unlink your device within your Signal Messenger App.
* `ls ~/.config/presage` shows you the signal client data.
* remove it via `rm -rf ~/.config/presage`.
