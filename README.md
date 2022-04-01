<div align="center">
  
  <p>🌕🍇🤖</p>
  <h1>TRB - Terra Rust Bot</h1> 
  <p>Get the most bang for your buck by automating the low hanging fruits of the Terra Ecosystem.</p> 
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
  
### [Install](#install) · [Config](#config) · [Usage](#usage) · [Summary](#summary)
 

**TRB** can be used to connect with a terra wallet tgo keep potential loans safe *(Auto Repay)*, maximise their utility *(Auto Borrow)* and more *(Auto Stake, Auto Farm,..)*. This repository includes a Signal Messenger integration that can be used to obtain the latest informaction directly from the bot. 


It is intended to be used by **coders, developers and technically-skilled users** to make use of automation.    
At the same time **TRB** can be compiled to a single executable that is easy to use with few dependencies. Making it easy to use.    

Current feature list:    

- Anchor Protocol: Auto Repay (1), Auto Borrow (2), Auto Stake (3), Auto Farm (4).
- (1) Keeps your loan safe by sourcing money from your balance (UST) or Anchor Deposit (aUST).     
- (2) Optimizes your LTV by automatically borrowing additional UST and depositing it into Anchor Earn (aUST).
- (3) Checks your pending borrower ANC rewards, considers the gas fees and stakes them automatically. 
- (4) Checks your pending borrower ANC rewards, considers the gas fees and provides them to the Astroport ANC-UST LP at Spectrum Protocol.


- Dashboards: Market, Anchor Info, Anchor Account


  

## How it works
 

#### Security
* Sensitive information is gathered at runtime via user input. This avoids storing sensitive information within config files.
* The encrypted seed phrase is stored safely in memory with <a href="https://github.com/unrelentingtech/secstr">secstr</a>.
* The seed phrase is encrypted using a simple XOR Cipher and only decrypted when used.
* Terra-rust-bot uses <a href="https://github.com/anvie/litcrypt.rs">litcrypt</a> to hide the encryption key from naughty eyes and protect the program from illegal cracking activity.   

#### Additional Security Measures

> There is no easy way for an attacker to extract your seed phrase, BUT given enough time and root access to your system it is certainly possible someone experienced can hack their way into the RAM, modify the code or introduce memory leaks to steal the seed. Everything CAN be hacked. Here are some security measures you might want to consider.

- Always clear your copy/paste clipboard.
- Use a dedicated wallet.
- Avoid vserver and use a dedicated root server. (RAM snapshots are a security risk)
- Harden your system. (Firewall, SSH, SELinux, Filesystem Encryption, VPN)
- Hide the fact that you are using terra-rust-bot: Rename the executable to something (un)expected.  
 


#### Requests
* Requests are either made directly to the Terra FCD or LCD. For that terra-rust-bot mainly relies on the [Terra-Rust-API](https://crates.io/crates/terra-rust-api). In some cases <a href="api.anchorprotocol.com/api/v2/distribution-apy">api.anchorprotocol.com</a> is used.

#### Fees
* Looking at past transactions terra-rust-bot estimates a reasonable transaction fee. In particually by looking at the actual gas amounts that were used in past transactions. This estimate can be used to offset the fee, keeping the account balance stable. For each transaction the fees are simulated using the prefered gas adjustment and double checked with the set maximum transaction fee.  
* If possible transactions are grouped together, to further reduce the gas fees.

#### Configuration
* The configuration can be customized via the **terra-rust-bot.json** file.
* The current state is written to **./packages/terra-rust-signal-bot/terra-rust-bot-state.json**.
 


## Install

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


**Build terra-rust-bot**

 *To make the process as simple as possiblethe following bash script is: ./install.sh*
 *You can choose between three different build options:*
* `./install.sh dev` fast build
* `./install.sh prod` optimized build
* `./install.sh native` optimize the build for your CPU    

## Update

* To update save your local changes with  `git stash`.
* Get the latest code with `git pull`.
* Then build the package again.
 

## Config

### terra-rust-bot.json

 > :arrow_right: This file needs to be in the working directory, from where you execute the command to run terra-rust-bot.
 

 * `trigger_percentage:` recommended value not greater than 0.95 (= trigger repay at 95% of the borrow limit).
 * `borrow_percentage:` recommended value around 0.7 (= trigger borrow at 70% of the borrow limit).
 * `target_percentage:` recommended value 0.8 (= repay position to 80% of the borrow limit).
 * `max_tx_fee:` safeguard parameter: max. UST amount to spend per transaction for the fees. recommended value 5.
 * `gas_adjustment_preference:` the gas_adjustment you want to use, recommended value is "1.2" or higher to ensure transactions go through.
 * `min_ust_balance:` min. UST balance, if below this value no further transactions will be made. If min_ust_balance is 10 UST then you should have more than that deposited for the bot to be able to execute transactions, around 15 or 20 UST. It is your job to make sure the balance is sufficient.
 * `ust_balance_preference:` greater than min_ust_balance, the bot will try to maintain the balance at the given value.


## Usage

### Run terra-rust-bot


> :arrow_right: To optimize terra-rust-bot's response time run it on a multi-core system for the best performance.  
 

**Location of the executable**

* `./my-bot` (if you used the install script)

**Server Environment**

*For convinience the following two bash scripts are available: ./run.sh and ./stop.sh*
*./run.sh makes sure the seed phrase stays secure and starts the bot as a background process*
*It will also automatically stop any running instance of ./my-bot first before creating a new process*

 **Common Use Cases**
* Auto Repay/Borrow `./run.sh -b anchor_auto_repay anchor_auto_borrow -d test`    
* Auto Repay/Borrow + Auto Staking `./run.sh -b anchor_auto_stake anchor_auto_repay anchor_auto_borrow -d test`    
* Everything `./run.sh -i market anchor -a anchor_account -b anchor_auto_stake anchor_auto_repay anchor_auto_borrow -d test`    

*the above commands are read only, remove `-d test` to let the bot sign transactions*

**Command line args**

 * `-i` show **info** dashboards: `market` or `anchor`.  
 * `-a` show **account** dashboards: `anchor_account`.  
 * `-b` enable **bot**: `anchor_auto_lp`,`anchor_auto_stake`, `anchor_auto_borrow`  or `anchor_auto_repay`.
 * `-d` enable additional development/debugging output. Currently only `test` is available. `test` will only simulate or estimate transaction fees.
 
 **Process**

*Only works if you used the ./run.sh script*    
 * `ps -p $(cat my-bot.pid)` check if the bot is still running.
 * `kill -TSTP $(cat my-bot.pid) ` pause the bot
 * `kill -CONT $(cat my-bot.pid) ` resume the bot

 **View the current state**

* Use the [terra-rust-signal-bot](https://github.com/Philipp-Sc/terra-rust-bot/tree/main/packages/terra-rust-signal-bot) to view the state. 

 
## Summary

- I created this bot to learn about smart contracts, the terra blockchain and to get to know the Rust programming language.
- Terra-rust-bot is a constant work in progress: **Bug Reports** and **Feature Requests** are welcome!
- Thanks to the people who helped me along the way on the <a href="https://discord.com/invite/EuKCeGFb93">terra discord </a>. :heart: 
 
## Support!

You do not need to donate to me, but I will be very happy if you can.  

- `terra1q0n5e43mmz8ddra8554xkxsvelnz4evwukxkht`

Thanks a lot,     
Philipp.
 
 
## Similar projects
- https://github.com/ALPAC-4/auto_repay
- https://github.com/RomainLanz/anchor-borrow-bot
- https://github.com/unl1k3ly/AnchorHODL
