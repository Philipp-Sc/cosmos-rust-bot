<div align="center">
  
  <p>🌕🍇🤖🚀🔩🛠️</p>
  <h1>terra-rust-bot</h1> 
  <p>Get the most out of your Luna bag by automating the low hanging fruits.</p> 
    <img src="https://img.shields.io/github/languages/top/Philipp-Sc/terra-rust-bot"> 
    <img src="https://img.shields.io/github/repo-size/Philipp-Sc/terra-rust-bot"> 
    <img src="https://img.shields.io/github/commit-activity/m/Philipp-Sc/terra-rust-bot"> 
    <img src="https://img.shields.io/github/license/Philipp-Sc/terra-rust-bot">
    <img src="https://img.shields.io/twitter/follow/PSchlutermann?style=social"> 
  </div>
<br/>


> :warning: The bot needs your **seed phrase** to create and sign transactions.  
> :arrow_right_hook: You can use Terra-rust-bot without a seed phrase in view mode. Test this first.  

> :warning: No security audit has been performed. (*Disclaimer: This may steal your money. Do your own research. Take a look at the code.*)
 


## Why

* Open source bots accessible to all will empower the Terra ecosystem.
* A single executable that is easy to use with few dependencies.

 
## Features 


### Anchor Auto Farm Rewards

 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/main/terra-rust-bot_v0.3_auto_farm.png" width="380">
  
 Checks your pending borrower ANC rewards, considers the gas fees and provides them to the Astroport ANC-UST LP at Spectrum Protocol.
 
### Anchor Auto Stake Rewards


 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/main/terra-rust-bot_v0.3_auto_stake.png" width="380">
  
 Checks your pending borrower ANC rewards, considers the gas fees and stakes them automatically. 
 
### Anchor Auto Loan Repay 

 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/main/terra-rust-bot_v0.3_auto_repay.png" width="380">
 
 Keeps your loan safe by sourcing money from your balance (UST) or Anchor Deposit (aUST). 
  
### Anchor Auto Borrow

 <img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/main/terra-rust-bot_v0.3_auto_borrow.png" width="380">
 
 Optimizes your LTV by automatically borrowing additional UST and depositing it into Anchor Earn (aUST).


### Market Dashboard 


<img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.1_terra.png" width="280">
 

### Anchor Dashboard 


<img src="https://github.com/Philipp-Sc/terra-rust-bot/blob/development/terra-rust-bot_v0.1_anchor.png" width="980">
  
(* the collateral value is calculated with the max_ltv, once the max_ltv for BLUNA and BETH are different, the collateral will be incorrect, this effects some of the APYs. TODO: query collateral value for BETH and LUNA.)
 

## How it works
 

#### Security
* Sensitive information is gathered at runtime via user input. This avoids storing sensitive information within config files.
* The encrypted seed phrase is stored safely in memory with <a href="https://github.com/unrelentingtech/secstr">secstr</a>.
* The seed phrase is encrypted using a simple XOR Cipher and only decrypted when used.
* Terra-rust-bot uses <a href="https://github.com/anvie/litcrypt.rs">litcrypt</a> to hide the encryption key from naughty eyes and protect the program from illegal cracking activity.

> :arrow_down: Check out the *Additional Security Measures* described below. 


#### Requests
* Requests are either made directly to the Terra FCD or LCD. For that terra-rust-bot mainly relies on the [Terra-Rust-API](https://crates.io/crates/terra-rust-api). In some cases <a href="api.anchorprotocol.com/api/v2/distribution-apy">api.anchorprotocol.com</a> is used.

#### Fees
* Looking at past transactions terra-rust-bot estimates a reasonable transaction fee. In particually by looking at the actual gas amounts that were used in past transactions. This estimate can be used to offset the fee, keeping the account balance stable. For each transaction the fees are simulated using the prefered gas adjustment and double checked with the set maximum transaction fee.  
* If possible transactions are grouped together, to further reduce the gas fees.

### Configuration
* The configuration can be customized via the **terra-rust-bot.json** file.
* The current state is written to **./packages/terra-rust-hook/terra-rust-bot-display.txt** instead of the console.


### Notifications/ Slack Webhook
* In addition to the terra-rust-bot a notification package is now available. **./packages/terra-rust-hook/** You will need to add your secret webhook URL into the file **main.rs**, then run the install script with `./install.sh native` afterwards you can run the program with `./run.sh` and stop it with `./stop.sh`. This notification bot will run independently of terra-rust-bot.
 

## Manual - Just Read The Instructions

### Step 1: Build

> Tested on Linux.
> Tested on Windows Subsystem for Linux / Ubuntu.


**Install Rust**

* <a href="https://doc.rust-lang.org/book/ch01-00-getting-started.html">Get started here.</a>
* On Linux: Download the file with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rust.sh`, view it: `less ./rust.sh`, and run the script `./rust.sh` to start rustup installation. The script makes PATH changes only to login shell configuration files. You need to `source ~/.cargo/env` until you logout and login back into the system. To update rustup afterwards, run `rustup self update`.
* Note: Works both with edition = "2018" and edition = "2021" (Cargo.toml). If you do not want to use the nightly version, just edit the config (Cargo.toml).
* To use the nightly edition (edition = "2021") install it with: `rustup default nightly && rustup update`.

* On WSL: You may need to install the following packages first:
* `sudo apt-get install build-essential libssl-dev pkg-config`

**Clone the repository**

* `git clone https://github.com/Philipp-Sc/terra-rust-bot.git`

* `cd terra-rust-bot`


**Build**

 *To make the process as simple as possible I created a bash script: ./install.sh*
 *You can choose between three different build options:*
* `./install.sh dev` fast build
* `./install.sh prod` optimized build
* `./install.sh native` optimize the build for your CPU
 

### Step 2: terra-rust-bot.json

 > :warning: If this file does not exist hard coded values from the terra-rust-bot implementation are used.
 
 > :arrow_right: This file needs to be in the working directory, from where you execute the command to run terra-rust-bot.

 * `trigger_percentage:` at which point you want to repay (1 equals a LTV of 60%).
 * `borrow_percentage:` at which point you want to borrow (1 equals a LTV of 60%).
 * `target_percentage:` the LTV do you want to maintain (1 equals a LTV of 60%).
 * `max_tx_fee:` the maximum UST amount you want to spend per transaction per fee.
 * `max_gas_adjustment:` currently not used by the bot.
 * `gas_adjustment_preference:` the gas_adjustment you want to use, recommended value is "1.2" or higher.
 * `min_ust_balance:` the minimum UST balance, if below this value no further transactions will be made. If min_ust_balance is 10 UST then you should have more than that deposited for the bot to be able to execute transactions, around 15 or 20 UST. It is your job to make sure the balance is sufficient.
 * `ust_balance_preference:` should be higher than min_ust_balance. For example 20 UST. Auto Repay will try to maintain the balance at that value.

### Step 3: Run terra-rust-bot


> :arrow_right: To optimize terra-rust-bot's response time run it on a multi-core system for the best performance.  
 

**Location of the executable**

* `./target/debug/terra-rust-bot `,
* `./target/release/terra-rust-bot ` or
* `./my-bot` (if you use the install script)

**Command line args**

 * `-i` show **info** dashboards: `market` or `anchor`.  
 * `-a` show **account** dashboards: `anchor_account`.  
 * `-b` enable **bot**: `anchor_auto_stake`, `anchor_auto_borrow`  or `anchor_auto_repay`.
 * `-d` enable additional development/debugging output. Currently `test` and `dev` are available. `test` will only simulate or estimate transaction fees. `dev` will output additional information to show what is happening in the background. 

 **Examples**

 * `./terra-rust-bot -b anchor_auto_stake -d test dev` (read only, remove `test` to let the bot sign transactions)

 * `./terra-rust-bot -b anchor_auto_lp -d test dev`  
 
 * `./terra-rust-bot -b anchor_auto_repay -d test dev`  
 
 * `./terra-rust-bot -b anchor_auto_borrow -d test dev` 

 * `./terra-rust-bot -b anchor_auto_stake anchor_auto_repay anchor_auto_borrow -d test dev`

 * `./terra-rust-bot -i market` (market information)

 * `./terra-rust-bot -i anchor` (anchor information)

 * `./terra-rust-bot -i anchor -a anchor_account` (for account specific information)

 **View the current state**

* `while sleep 0.1; do cat ./packages/terra-rust-hook/terra-rust-bot-display.txt; done` (watch the display output of terra-rust-bot)


**Server Environment Example**

*For convinience the following two bash scripts are available: ./run.sh and ./stop.sh*
*./run.sh makes sure the seed phrase stays secure and starts the bot as a background process*
*It will also automatically stop any running instance of ./my-bot first before creating a new process*
 **Common Use Cases**
* Auto Repay/Borrow `./run.sh -b anchor_auto_repay anchor_auto_borrow -d test dev` (read only, remove `test` to let the bot sign transactions)
* Auto Repay/Borrow + Auto Staking `./run.sh -b anchor_auto_stake anchor_auto_repay anchor_auto_borrow -d test dev` (read only, remove `test` to let the bot sign transactions)


*The ./stop.sh script simply stops the bot.*
* `./stop.sh` (stops the bot)

 

## Additional Security Measures

> There is no easy way for an attacker to extract your seed phrase, BUT given enough time and root access to your system it is certainly possible someone experienced can hack their way into the RAM, modify the code or introduce memory leaks to steal the seed. Everything CAN be hacked. Here are some security measures you might want to consider.

- Always clear your copy/paste clipboard.
- Use a dedicated wallet.
- Avoid vserver and use a dedicated root server. (RAM snapshots are a security risk)
- Harden your system. (Firewall, SSH, SELinux, Filesystem Encryption, VPN)
- Minimize your attack surface.
- Hide the fact that you are using terra-rust-bot: Rename the executable to something (un)expected. 
- Prepare a Honeypot/Decoy
 

## Summary

- I created this bot to learn about smart contracts, the terra blockchain and to get to know the Rust programming language.
- This is a constant work in progress: **Bug Reports** and **Feature Requests** are welcome!
- Thanks to the people who helped me along the way on the <a href="https://discord.com/invite/EuKCeGFb93">terra discord </a>. :heart: 
 

## Similar projects
- https://github.com/ALPAC-4/auto_repay
- https://github.com/RomainLanz/anchor-borrow-bot
- https://github.com/unl1k3ly/AnchorHODL