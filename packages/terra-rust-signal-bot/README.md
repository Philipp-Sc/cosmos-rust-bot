<div align="center">

      
  <h1>Terra Rust Signal Bot</h1> 
  <p>Signal Messenger integration</p> 
   
  <img src="gallery/signal_bot_auto_stake.png" width="200">
  <img src="gallery/signal_bot_help.png" width="200">
  <img src="gallery/signal_messenger.png" width="200">
  </div>
<br/>

 


### [Install](#install) · [Usage](#usage)  
 
This package includes a Signal Messenger integration that can be used to obtain the latest informaction directly from the terra-rust-bot. 

> :warning: It is recommended to use a dedicated Signal account for terra-rust-bot.

> :warning: Currently there is no local encryption for the linked Signal account. If the system is compromised your Signal account is vulnerable. This will be addressed in upcoming releases. 


## How it works
 
#### Configuration
* The current state from terra-rust-bot is written to **./packages/terra-rust-bot-output/terra-rust-bot-state.json**. Terra-rust-signal-bot reads the file and extracts the information needed.	 
 

## Install
 

**Build terra-rust-signal-bot**

* `cd ./packages/terra-rust-signal-bot`
* `./install.sh dev`

* After that you should be able to run:  
* `./terra-rust-signal-bot help` 

* To link your signal account you will need to scan a QR-Code with your phone.   
* `./terra-rust-signal-bot link-device -s production -n terra-rust-signal-bot`
* You can use the *send* and *receive* command to test the connection.
 
 
## Usage

### Run terra-rust-signal-bot
 
* If you already started the terra-rust-bot then you can use the terra-rust-signal-bot.

* `cd ./packages/terra-rust-signal-bot`
* `./run.sh activate` (run the signal-bot)
* `./stop.sh` (stop the signal-bot)

* Within your **Signal Messenger App** in your Contact search bar type **"Notes to Self"**. Open the channel and write "\help" to see a list of all available commands.


 ```
 [Available Commands]
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

## Uninstall

* unlink your device within your Signal Messenger App.
* `ls ~/.config/presage` shows you the signal client data.
* remove it via `rm -rf ~/.config/presage`.
