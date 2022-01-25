# terra-rust-bot


> :warning: You will need to provide your **seed phrase** to let the bot create and sign transactions.

> :warning: Terra-rust-bot will not save your **seed phrase**, make sure you do not lose it. Make sure you clear your copy/paste clipboard.

> :arrow_right: Terra-rust-bot can be used without a seed phrase, view only mode. It is recommended to test this first :arrow_left:

> :arrow_right: It is always a good idea to use a dedicated wallet. :arrow_left:
 
> :warning: No security audit has been performed.


## Why

* Rust is a great programming language.
* Keep It Simple, Stupid. 
* Power to the people. 
* Reducing transaction fees to a minimum. 
* Easy to use. One executable.


## What 
 
### Anchor Auto Stake Rewards


 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/main/terra-rust-bot_v0.3_auto_stake.png" width="380">
  
 
### Anchor Auto Loan Repay 


 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/main/terra-rust-bot_v0.3_auto_repay.png" width="380">
  
### Anchor Auto Borrow

 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/main/terra-rust-bot_v0.3_auto_borrow.png" width="380">



### Anchor Auto Replenish (not yet implemented)
### Anchor Auto Exchange Rewards (not yet implemented)
### Anchor aUST <--> bLuna Strategy (not yet implemented)
### Anchor Auto Bid (in consideration)

### Market Dashboard 


<img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.1_terra.png" width="280">
 

### Anchor Dashboard 


<img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.1_anchor.png" width="980">
  


## How


* Sensitive information is gathered at runtime via user input. This avoids storing sensitive information within config files.
* The seed phrase is stored safely in memory with <a href="https://github.com/unrelentingtech/secstr">secstr</a>.
* Requests are either made directly to the Terra FCD or LCD. For that terra-rust-bot mainly relies on the [Terra-Rust-API](https://crates.io/crates/terra-rust-api). In some cases <a href="api.anchorprotocol.com/api/v2/distribution-apy">api.anchorprotocol.com</a> is used.
* The <a href="https://app.anchorprotocol.com/"> Anchor Web App </a> or <a href="https://github.com/unl1k3ly/AnchorHODL">AnchorHODL</a> have rather high gas fees hard coded into the application. This ensures each transaction goes through, but this also means some money is unnecessarily being spent. Looking at past transactions terra-rust-bot estimates a reasonable transaction fee. In particually by calculating a decent gas adjustment derived from past transactions. To not stale a transaction the user provides a maximum transaction fee. For each transaction the fees are simulated and double checked with the set maximum transaction fee. This way there are no static fees.
* If possible transactions are grouped together, to further reduce the gas fees.
* The configuration can be customized via the **terra-rust-bot.json** file. It will be loaded at startup.
* The current state is written to **terra-rust-bot-display.txt** instead of the console.

## Disclaimer

> This may steal your money. Do your own research. Take a look at the code.

## Manual - Just Read The Instructions

### Run terra-rust-bot

 **Configue terra-rust-bot.json**

 > :warning: If this file does not exist hard coded values from the terra-rust-bot implementation are used.
 
 > :arrow_right: This file needs to be in the working directory, from where you execute the command to run terra-rust-bot.

 * `trigger_percentage:` at which point you want to repay (1 equals a LTV of 60%).
 * `borrow_percentage:` at which point you want to borrow (1 equals a LTV of 60%).
 * `target_percentage:` the LTV do you want to maintain (1 equals a LTV of 60%).
 * `max_tx_fee:` the maximum UST amount you want to spend per transaction per fee.
 * `max_gas_adjustment:` the maximum gas_adjustment you are willing to use.
 * `gas_adjustment_preference:` has an influence on the gas_adjustment you end up with.
 * `min_ust_balance:` the minimum UST balance, if below this value no further transactions will be made.
 * `wallet_acc_address:` if empty you may be asked at runtime to provide a wallet address.


 **Provide command line arguments and run**
 
**Command line args**

 * `-i` show **info** dashboards: `market` or `anchor`.  
 * `-a` show **account** dashboards: `anchor_account`.  
 * `-b` enable **bot**: `anchor_auto_stake`, `anchor_auto_borrow`  or `anchor_auto_repay`.
 * `-d` enable additional development/debugging output. Currently `test` and `dev` are available. `test` will only simulate or estimate transaction fees. `dev` will output additional information to show what is happening in the background. 

 **Examples**

 * `./terra-rust-bot -b anchor_auto_stake -d test dev` (read only, remove `test` to let the bot sign transactions)

 * `./terra-rust-bot -b anchor_auto_repay -d test dev`  
 
 * `./terra-rust-bot -b anchor_auto_borrow -d test dev` 

 * `./terra-rust-bot -b anchor_auto_stake anchor_auto_repay anchor_auto_borrow -d test dev`

 * `./terra-rust-bot -i market` (market information)

 * `./terra-rust-bot -i anchor` (anchor information)

 * `./terra-rust-bot -i anchor -a anchor_account` (for account specific information)

 **View the current state**

 * `while sleep 0.1; do cat terra-rust-bot-display.txt; done` 


### Build it yourself

> Tested on Linux, if you have issues on macOS or Windows please let me know.


**Install Rust**

* <a href="https://doc.rust-lang.org/book/ch01-00-getting-started.html">Get started here.</a>

**Clone the repository**

* `git clone https://github.com/Philipp-Sc/terra-rust-bot.git`

* `cd terra-rust-bot`


**Build**

* `cargo build` or
* `cargo build --release`

**Run**

* `./target/debug/terra-rust-bot ` or
* `./target/release/terra-rust-bot `

* Command line arguments are discribed in the text above.


 
## Similar projects
- https://github.com/ALPAC-4/auto_repay
- https://github.com/RomainLanz/anchor-borrow-bot
- https://github.com/unl1k3ly/AnchorHODL
