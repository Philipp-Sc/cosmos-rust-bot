> :warning: This code is experimental. Especially the development branch. :warning:

# terra-rust-bot


> :warning: You will need to provide your **seed phrase** to let the bot create and sign transactions.

> :warning: Terra-rust-bot will not save your seed phrase, make sure you do not lose your seed phrase.

> :arrow_right: Terra-rust-bot can be used without a seed phrase, in that case you need to provide your **wallet address**.

> :arrow_right: No security audit has been performed.


## Why

* Rust is a great programming language.
* Keep It Simple, Stupid. 
* Async/Paralellism
* Easy to use. One executable.

* Extending, building on top of <a href="https://github.com/unl1k3ly/AnchorHODL">AnchorHODL</a>, adding new strategies.
* Reducing transaction fees to a minimum.
* As a failsafe against Anchor/Mirror webapp issues.


## What 
 
### Anchor Auto Stake Rewards


 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.2_auto_stake.png" width="380">
  
 
### Anchor Auto Loan Repay 


 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.2_anchor_auto_repay.png" width="380">
  
### Anchor Auto Borrow (comming soon)
### Anchor Auto Replenish (not yet implemented)
### Anchor Auto Exchange Rewards (not yet implemented)
### Anchor Auto Bid (in consideration)

### Market Dashboard 


<img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.1_terra.png" width="280">
 
 
### <a href="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.1_anchor.png"> Anchor Dashboard </a>



## How


* Sensitive information is gathered at runtime via user input. This avoids storing sensitive information within config files.
* The seed phrase is stored safely in memory with <a href="https://github.com/unrelentingtech/secstr">secstr</a>.
* Requests are either made directly to the Terra FCD or LCD. For that terra-rust-bot mainly relies on the [Terra-Rust-API](https://crates.io/crates/terra-rust-api). In some cases <a href="api.anchorprotocol.com/api/v2/distribution-apy">api.anchorprotocol.com</a> is used.
* The <a href="https://app.anchorprotocol.com/"> Anchor Web App </a> or <a href="https://github.com/unl1k3ly/AnchorHODL">AnchorHODL</a> have rather high gas fees hard coded into the application. This ensures each transaction goes through, but this also means some money is unnecessarily being spent. Looking at past transactions terra-rust-bot estimates a reasonable transaction fee. In particually by calculating a decent gas adjustment derived from past transactions. To not stale a transaction the user provides a maximum transaction fee. For each transaction the fees are simulated and double checked with the set maximum transaction fee. This way there are no static fees.
* If possible transactions are grouped together, to further reduce the gas fees.

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

 * `./terra-rust-bot -b anchor_auto_stake -d test dev` (read only, remove `test` to let the bot sign transactions)

 * `./terra-rust-bot -b anchor_auto_repay -d test dev`  

 * `./terra-rust-bot -i market` (market information)

 * `./terra-rust-bot -i anchor` (anchor information)

 * `./terra-rust-bot -i anchor -a anchor_account` (for account specific information)


 * `while sleep 0.1; do cat terra-rust-bot-display.txt; done` (to see whats happening)


### Build it yourself

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


**Command line args**

* `-i` show **info** dashboards for `market` or `anchor`. Example: `-i market anchor`. 
* `-a` show **account** dashboards for `anchor_account`.  Example: `-a anchor_account`. (Requires wallet address)
* `-b` enable **bot** for `anchor_auto_staking.`  Example: `-b anchor_auto_stake`. (Requires seed phrase)
* `-b` enable **bot** for `anchor_auto_repay.`  Example: `-b anchor_auto_repay`. (Requires seed phrase)
* `-d` enable additional development/debugging output. Currently `test` and `dev` are available. `test` will only simulate or estimate transaction fees. `dev` will output additional information to show what is happening in the background.   Example: `-d test dev`.


 
## Similar projects
- https://github.com/ALPAC-4/auto_repay
- https://github.com/RomainLanz/anchor-borrow-bot
- https://github.com/unl1k3ly/AnchorHODL
