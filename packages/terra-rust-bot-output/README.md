<div align="center">
   
  <h1>Terra Rust Bot Output</h1> 
  <p>View the state of terra-rust-bot</p> 
  </div>
<br/>

### [Install](#install) · [Usage](#usage)  
 
This package can be used to view the latest informaction directly from the terra-rust-bot. 


## How it works
 
#### Configuration
* The current state from terra-rust-bot is written to **./packages/terra-rust-bot-output/terra-rust-bot-state.json**. You can use this package **terra-rust-bot-output** directly to view all the information in a human readable format.
 

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
