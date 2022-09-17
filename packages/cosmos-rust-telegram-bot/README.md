<div align="center">
  <h1>Cosmos Rust Telegram Bot</h1> 
  <p>Telegram Notifications</p> 

  </div>
<br/>

### [Install](#install) · [Usage](#usage)

> :warning: For security reasons is only possible to use cosmos-rust-telegram-bot to query or receive notifications. For other use cases setup cosmos-rust-signal-bot.
 
## How it works

#### Configuration

* **cosmos-rust-bot** writes the current data (via cosmos-rust-interface) to an embedded database.    
* Using UNIX Sockets **cosmos-rust-bot** communicates with **cosmos-rust-telegram-bot**:
  - **cosmos-rust-bot**: request notification(s) for a given query
  - **cosmos-rust-telegram-bot**: receive notification(s)   
Note: **cosmos-rust-bot** issues notifications either directly after a request or when the data related to a subscription was updated.
* **cosmos-rust-telegram-bot** manages its notification in its own embedded database.

## Install

**Build cosmos-rust-telegram-bot**

* `cd ./packages/cosmos-rust-telegram-bot`
* `RUSTFLAGS="--cfg tokio_unstable" cargo build`

## Usage

### Run cosmos-rust-signal-bot

* If you already started the cosmos-rust-bot then you can use the terra-rust-telegram-bot.

* `cd ./packages/cosmos-rust-telegram-bot`
* `RUST_LOG=error,debug,info target/debug/cosmos-rust-telegram-bot`
