> :warning: This code is pre alpha. You have been warned. :warning:

# terra-rust-bot


> :warning: You will need to **seed phrase** to let the bot create and sign transactions.

> :warning: No security audit has been performed.


## Why

* Rust is a great programming language. It is also the language terra/cosmos smart contracts are written in.
* Keep It Simple, Stupid. 
* No unnecessary libraries or overhead.
* Easy to use. Just run the executable in the terminal.

* Extending, building on top of <a href="https://github.com/unl1k3ly/AnchorHODL">AnchorHODL</a>, adding new strategies.
* As a failsafe against Anchor/Mirror webapp issues.

## What 

* Market Dashboard
* Anchor Dashboard
* Anchor Auto Stake Rewards
* Anchor Auto Exchange Rewards (not yet implemented)
* Anchor Auto Loan Repay (not yet implemented)

## How


* Sensitive information is gathered at runtime via user input. This avoids storing sensitive information within config files.
* The seed phrase is stored safely in memory with <a href="https://github.com/unrelentingtech/secstr">secstr</a>.
* Requests are either made directly to the Terra FCD or LCD. If possible the LCD is prefered. For that terra-rust-bot mainly relies on the [Terra-Rust-API](https://crates.io/crates/terra-rust-api).

## Disclaimer

> This may steal your money. This is not investment advice. Do you own research.


## Manual - Just Read The Instructions

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
* `-d` enable additional development/debugging output.
* Example: `./terra-rust-bot -i market anchor -a anchor_account -b anchor_auto_stake`

## Similar projects
- https://github.com/ALPAC-4/auto_repay
- https://github.com/RomainLanz/anchor-borrow-bot
- https://github.com/unl1k3ly/AnchorHODL
