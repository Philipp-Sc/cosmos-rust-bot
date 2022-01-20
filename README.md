> :warning: This code is experimental. Especially the development branch. :warning:

# terra-rust-bot


> :warning: You will need to provide your **seed phrase** to let the bot create and sign transactions.

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

* Market Dashboard

 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.1_terra.png" width="280">
 
 
* Anchor Dashboard  


 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.1_anchor.png" width="1180">
  
 
* Anchor Auto Stake Rewards


 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.1_auto_stake.png" width="280">
  
 
* Anchor Auto Loan Repay 
 
* Anchor Auto Exchange Rewards (not yet implemented)

## How


* Sensitive information is gathered at runtime via user input. This avoids storing sensitive information within config files.
* The seed phrase is stored safely in memory with <a href="https://github.com/unrelentingtech/secstr">secstr</a>.
* Requests are either made directly to the Terra FCD or LCD. For that terra-rust-bot mainly relies on the [Terra-Rust-API](https://crates.io/crates/terra-rust-api). In some cases <a href="api.anchorprotocol.com/api/v2/distribution-apy">api.anchorprotocol.com</a> is used.
* The <a href="https://app.anchorprotocol.com/"> Anchor Web App </a> or <a href="https://github.com/unl1k3ly/AnchorHODL">AnchorHODL</a> have rather high gas fees hard coded into the application. This ensures each transaction goes through, but this also means some money is unnecessarily being spent. Looking at past transactions terra-rust-bot estimates a reasonable transaction fee. In particually by calculating a decent gas adjustment derived from past transactions. To not stale a transaction the user provides a maximum transaction fee. For each transaction the fees are simulated and double checked with the set maximum transaction fee. This way there are no static fees.
* If possible transactions are grouped together, to further reduce the gas fees.

## Disclaimer

> This may steal your money. Do your own research. Take a look at the code.

## Manual - Just Read The Instructions


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
* `-b` enable **bot** for `anchor_auto_staking.`  Example: `-b anchor_auto_staking`. (Requires seed phrase)
* `-d` enable additional development/debugging output. Currently `test` and `dev` are available. `test` will only simulate or estimate transaction fees. `dev` will output additional information to show what is happening in the background.   Example: `-d test dev`.



## Examples

* For the **Market Dashboard** run `./terra-rust-bot -i market`.
* For the **Anchor Dashboard** run `./terra-rust-bot -i anchor`.
* If you want to see your account specific data add  `-a anchor_account`.
* If you want to have Auto Staking add `-b anchor_auto_staking`

* Example: `./terra-rust-bot -i market anchor -a anchor_account`
* Market Dashboard: `./terra-rust-bot -i market`
* Anchor Dashboard 1: `./terra-rust-bot -i anchor -a anchor_account`
* Anchor Dashboard 2: `./terra-rust-bot -i anchor -a anchor_account -b anchor_auto_stake`
* Auto Staking Only: `./terra-rust-bot -b anchor_auto_stake`

 
## Similar projects
- https://github.com/ALPAC-4/auto_repay
- https://github.com/RomainLanz/anchor-borrow-bot
- https://github.com/unl1k3ly/AnchorHODL
