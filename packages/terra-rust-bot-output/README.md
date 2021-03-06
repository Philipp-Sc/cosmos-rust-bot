<div align="center">

  <h1>Terra Rust Bot Output</h1> 
  <p>View the state of terra-rust-bot</p> 

  <img src="https://github.com/Philipp-Sc/media/raw/main/terra-rust-bot/terra-rust-bot-output/gallery/terminal_output_auto_stake.png" height="350">
  <img src="https://github.com/Philipp-Sc/media/raw/main/terra-rust-bot/terra-rust-bot-output/gallery/terminal_output_market.png" height="350">
  </div>
<br/>

### [Install](#install) · [Usage](#usage) · [Errors](#errors)

This package can be used to view the latest informaction directly from the terra-rust-bot.

## How it works

#### Configuration

* The current state from terra-rust-bot is written to **./packages/terra-rust-bot-output/terra-rust-bot-state.json**.
  You can use this package **terra-rust-bot-output** directly to view all the information in a human readable format.

## Install

**Build**

* `cd ./packages/terra-rust-bot-output`
* `cargo build`

## Usage

* After that you should be able to run:
* `./target/debug/terra-rust-bot-output local-display -m "\help"  `
* This will show you all possible commands.

 ```
 [Available Commands]
UPDATE SETTINGS
    \set <field> <value>
SYSTEM TIME AND LATEST TIMESTAMP 
    \ping      
MARKET INFO    
    \market         
ANCHOR INFO    
    \anchor info    
ANCHOR ACCOUNT INFO    
    \anchor account  
EVERY AUTOMATION
    \auto        
AUTO REPAY INFO
    \auto repay  
AUTO BORROW INFO 
    \auto borrow  
AUTO STAKE INFO
    \auto stake   
AUTO FARM INFO 
    \auto farm   
SHOW ALL ERRORS
    \errors  
SHOW LOGS OF RECENT TRANSACTIONS
    \logs  
TASK COUNT (failed,pending,upcoming,all)
    \task count         
TASK LIST (failed,pending,upcoming,all)
    \task list          
TIMESTAMPS WHEN TASKS WERE RESOLVED
    \task history     
TIMESTAMPS WHEN ENTRIES WERE WRITTEN TO STATE
    \state history        
DEBUG (SHOWS ABSOLUTLY EVERYTHING)
    \debug
```

## Errors

This section explains possible errors that might pop up.

**Minor**

- Msg:
  `account sequence mismatch, expected 644, got 643: incorrect account sequence: invalid request`

- Description: To sign transactions the account number and sequence is needed. If the account sequence (# of
  transactions) is outdated the request fails. The bot will retry asap.

- Cause: This error might occur when the bot sends multiple transactions at about the same time. It is a sign of
  congestion.

**Major**

- Msg:
  ` Overflow: Cannot Sub with 5608138 and 5639945: execute wasm contract failed: invalid request`

- Description: The wrong amount of tokens specified, exceeding the maximum available value.

- Cause: This error can be a result of a calculation/rounding error. If this error persists please open a bug report.

