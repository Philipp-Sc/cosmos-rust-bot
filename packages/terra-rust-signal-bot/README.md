<div align="center">
   
  <h1>Terra Rust Signal Bot</h1> 
  <p>Signal Messenger integration</p> 
  </div>
<br/>

### [Install](#install) · [Usage](#usage)  
 
This package includes a Signal Messenger integration that can be used to obtain the latest informaction directly from the terra-rust-bot. 

> :warning: It is recommended to use a dedicated Signal account for terra-rust-bot.

> :warning: Currently there is no local encryption for the linked Signal account. If the system is compromised your Signal account is vulnerable. This will be addressed in upcoming releases. 

\help
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


## How it works
 
#### Configuration
* The current state from terra-rust-bot is written to **./packages/terra-rust-signal-bot/terra-rust-bot-state.json**. Terra-rust-signal-bot reads the file and extracts the information needed.	 
 

## Install

> Tested on Linux.
> Tested on Windows Subsystem for Linux / Ubuntu.

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
 
* If you already started the terra-rust-bot then you can use the terra-rust-signal-bot to either directly view the state or to activate the signal-bot.

* `cd ./packages/terra-rust-signal-bot`
* `./run.sh receive-loop` (run the signal-bot)
* `./stop.sh` (stop the signal-bot)

* `./terra-rust-signal-bot local-display -m "\help"  ` (to view the state directly)
* replace `"\help"` with any other command to view the state.
