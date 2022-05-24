
> :warning: The last tested version is at commit `12eee1b7b7d525962700c3a4bf1c861f34e651cd`.

> :information_source: Terra-rust-bot was stress tested during the UST crash, the bot successfully prevented liquidations as it was set up to do.

> :information_source: Due to the unexpected halt of the blockchain, terra-rust-bot is shifting its focus temporarily until Terra is reconstituted. 

> See <2022/5/13 - Update>

<div align="center">

  <p>🤖</p>
  <h1>CRB - Cosmos Rust Bot™</h1> 
  <p>If you can't beat them join them!</p> 
    <img src="https://img.shields.io/github/languages/top/Philipp-Sc/terra-rust-bot"> 
    <img src="https://img.shields.io/github/repo-size/Philipp-Sc/terra-rust-bot"> 
    <img src="https://img.shields.io/github/commit-activity/m/Philipp-Sc/terra-rust-bot"> 
    <img src="https://img.shields.io/github/license/Philipp-Sc/terra-rust-bot">
    <img src="https://img.shields.io/twitter/follow/PSchlutermann?style=social"> 
  </div>
<br/>
<div align="center">
  <img src="https://github.com/Philipp-Sc/media/raw/main/terra-rust-bot/terra-rust-bot-output/gallery/terminal_output_auto_stake.png" height="250">
  <img src="https://github.com/Philipp-Sc/media/raw/main/terra-rust-bot/terra-rust-bot-output/gallery/terminal_output_market.png" height="250">

  <img src="https://github.com/Philipp-Sc/media/raw/main/terra-rust-bot/terra-rust-signal-bot/gallery/signal_bot_auto_stake.png" height="250">
  <img src="https://github.com/Philipp-Sc/media/raw/main/terra-rust-bot/terra-rust-signal-bot/gallery/signal_bot_help.png" height="250">
  <img src="https://github.com/Philipp-Sc/media/raw/main/terra-rust-bot/terra-rust-signal-bot/gallery/signal_messenger.png" height="250">
</div>



> :warning: The bot needs your **seed phrase** to create and sign transactions.  
> :arrow_right_hook: You can use Cosmos-rust-bot without a seed phrase in view mode. Test this first.

> :warning: No security audit has been performed. (*Disclaimer: This may steal your money. Do your own research. Take a look at the code.*)

### [Install](#install) · [Config](#config) · [Usage](#usage) · [Summary](#summary)


## New Roadmap:

**Signal**
- [ ] In-Memory ConfigStore

**Terra**
- [ ] Terra 2.0: use gRPC instead of LCD/FCD (cosmos-rust)[https://github.com/cosmos/cosmos-rust/] 
- [ ] Impermanent Loss Protection

**Osmosis**
- [ ] Market Dashboard: (assets,pools,prices)
- [ ] DCA  

**Cosmos**
- [ ] Market Dashboard: List prices (Terraswap, Astroport, Osmosiszone)
- [ ] Arbitrage: Notifications, Automation
- [ ] Take Profits & Diversify Cross-chain (Terra) <-> (Osmosis)


### < 2022/5/13 - Update >

Terra-rust-bot will continue to keep the commits rolling and be ready to serve when Terra is reborn from its ashes, the
bot will now focus on the following:

1. Integrate Osmosis
2. Possibly integrate the Juno Network

New functionalities will be whatever is useful to increase and protect your wallet.

This includes protection against Black Swan events, by exiting positions and transferring tokens to a safe haven. Let's
keep building!

### </>

**TRB** can be used to connect with a terra wallet to keep potential loans safe *(Auto Repay)*, maximise their
utility *(Auto Borrow)* and more *(Auto Stake, Auto Farm,..)*. This repository includes a Signal Messenger integration
that can be used to obtain the latest informaction directly from the bot.

It is intended to be used by **coders, developers and technically-skilled users** to make use of automation.    
At the same time **TRB** can be compiled to a single executable that is easy to use with few dependencies. Making it
easy to use.

In short it's purpose is to:

- Save you the hassle of managing your positions manually
- Enable you to run strategies only bots can execute
- Take commands
- Send alerts and notifications
- Provide insights into the Terra Ecosystem

Feature list:

**Automation**

- Anchor Protocol Auto Repay/Borrow:  
  Keeps your loan safe by sourcing money from your balance (UST) or Anchor Deposit (aUST) and automatically borrows
  additional UST and depositing it into Anchor Earn (aUST).
- Anchor Protocol Auto Stake/Farm:  
  Checks your pending borrower ANC rewards, considers the gas fees and stakes them automatically or provides them to the
  Astroport ANC-UST LP at Spectrum Protocol.

**View Token Prices, Collateral Information, Pending Rewards, etc.**

- Market, Anchor
- Anchor Account

## How it works

#### Security

* Sensitive information is gathered at runtime via user input. This avoids storing sensitive information within config
  files.
* The encrypted seed phrase is stored safely in memory with <a href="https://github.com/unrelentingtech/secstr">
  secstr</a>.
* The seed phrase is encrypted using a simple XOR Cipher and only decrypted when used.
* Terra-rust-bot uses <a href="https://github.com/anvie/litcrypt.rs">litcrypt</a> to hide the encryption key from
  naughty eyes and protect the program from illegal cracking activity.

#### Additional Security Measures

> There is no easy way for an attacker to extract your seed phrase, BUT given enough time and root access to your system it is certainly possible someone experienced can hack their way into the RAM, modify the code or introduce memory leaks to steal the seed. Everything CAN be hacked. Here are some security measures you might want to consider.

- Always clear your copy/paste clipboard.
- Use a dedicated wallet.
- Avoid vserver and use a dedicated root server. (RAM snapshots are a security risk)
- Harden your system. (Firewall, SSH, SELinux, Filesystem Encryption, VPN)

#### Requests

* Requests are either made directly to the Terra FCD or LCD. For that terra-rust-bot mainly relies on
  the [Terra-Rust-API](https://crates.io/crates/terra-rust-api). Click here to see all
  custom [API Endpoints](https://github.com/Philipp-Sc/terra-rust-bot/blob/main/packages/terra-rust-api-layer/src/services/blockchain/smart_contracts/objects/meta/api/data/endpoints.rs)
  that might be used, depending on the setup.

#### Fees

* Looking at past transactions terra-rust-bot estimates a reasonable transaction fee. In particually by looking at the
  actual gas amounts that were used in past transactions. This estimate can be used to offset the fee, keeping the
  account balance stable. For each transaction the fees are simulated using the prefered gas adjustment and double
  checked with the set maximum transaction fee.
* If possible transactions are grouped together, to further reduce the gas fees.

#### Configuration

* The configuration can be customized via the **terra-rust-bot.json** file.
* The current state is written to **./packages/terra-rust-bot-output/terra-rust-bot-state.json**.

## Install

* See [Install](https://github.com/Philipp-Sc/terra-rust-bot/tree/main/install/)

## Config

### terra-rust-bot.json

* `pause_requested:` if `true` terra-rust-bot will pause (all running tasks will be aborted) until `pause_requested` is
  set to `false`. (`hot_reload: true` required)
* `hot_reload`: if `true` and the terra-rust-bot.json file gets updated terra-rust-bot will reload it. This gives you
  live control over the bot. (`remove: false` required)

*Note: only with `hot_reload: true` you can use the `\set <field> <value>` command*

* `remove:` security feature, if `true` the terra-rust-bot.json file will be deleted after it has been loaded.
* `test:` if `true` no transactions will be executed, useful for testing or view only, will simulate or estimate
  transaction fees.
* `terra_wallet_address:` convenience feature: default `null`, if set for example
  to `"terra1q0n5e43mmz8ddra8554xkxsvelnz4evwukxkht"` terra-rust-bot will not ask you for your wallet address. (can **
  not** be updated once terra-rust-bot is started)


* `anchor_protocol_auto_repay:` if `true` saves you from being liquidated triggered by the *trigger_percentage*.
* `anchor_protocol_auto_borrow:` if `true` and current borrow limit grows (see *borrow_percentage*), an auto borrow will
  be triggered increasing the loan to the *target_percent*.
* `anchor_protocol_auto_stake:` if `true` ANC rewards will be claimed automatically and
  staked. (`anchor_protocol_auto_farm` needs to be `false`!)
* `anchor_protocol_auto_farm:` if `true` ANC rewards will be claimed automatically and provided to the Spectrum Protocol
  ANC-UST-LP Auto Compound Vault. (`anchor_protocol_auto_stake` needs to be `false`!)


* `anchor_account_info:` if `true` queries data for the **Anchor Account Dashboard**.


* `terra_market_info:` if `true` queries data for the **Market Dashboard**.
* `anchor_general_info:` if `true` queries data for the **Anchor Dashboard**.


* `trigger_percentage:` recommended value not greater than 0.95 (= trigger repay at 95% of the borrow limit).
* `borrow_percentage:` recommended value around 0.7 (= trigger borrow at 70% of the borrow limit).
* `target_percentage:` recommended value 0.8 (= repay position to 80% of the borrow limit).


* `max_tx_fee:` safeguard parameter: max. UST amount to spend per transaction for the fees. recommended value 5.
* `gas_adjustment_preference:` the gas_adjustment you want to use, recommended value is "1.2" or higher to ensure
  transactions go through.


* `min_ust_balance:` min. UST balance, if below this value no further transactions will be made. If min_ust_balance is
  10 UST then you should have more than that deposited for the bot to be able to execute transactions, around 15 or 20
  UST. It is your job to make sure the balance is sufficient.
* `ust_balance_preference:` greater than min_ust_balance, the bot will try to maintain the balance at the given value.

**Note**: the difference between `min_ust_balance` and `ust_balance_preference` is important to consider. In case
transactions fail because the `gas_adjustment_preference` is to low, terra-rust-bot will try again to broadcast the
transaction as long as there are UST to spend or it reached `min_ust_balance`. This prevents potential infinite (failed)
transactions that might drain your account.

## Usage

### Run terra-rust-bot

> :arrow_right: To optimize terra-rust-bot's response time run it on a multi-core system for the best performance.


**Location of the executables**

**terra-rust-bot** (the actual terra-rust-bot)

* `./terra-rust-bot/test/build/my-bot`  
  *The following two bash scripts are available: ./run.sh and ./stop.sh*
  *./run.sh makes sure the seed phrase stays secure and starts the bot as a background process*
  *It will also automatically stop any running instance of ./my-bot first before creating a new process*

**terra-rust-bot-output** (program to show the state of terra-rust-bot)

* `./terra-rust-bot/test/build/packages/terra-rust-bot-output/my-bot-output`

**terra-rust-signal-bot** (signal messenger bot)

* `./terra-rust-bot/test/build/packages/terra-rust-signal-bot/terra-rust-signal-bot`  
  *The following two bash scripts are available: ./run.sh and ./stop.sh*
  *./run.sh starts the bot as a background process and makes sure the terra-rust-signal-bot gets restarted if it
  crashed*
  *It will also automatically stop any running instance of first before creating a new process*

**To view the current state and/or use the `\set <field> <value>` commands:**

* Either use the
  package [terra-rust-bot-output](https://github.com/Philipp-Sc/terra-rust-bot/tree/main/packages/terra-rust-bot-output)
  to view the state in the terminal.

* Or use [terra-rust-signal-bot](https://github.com/Philipp-Sc/terra-rust-bot/tree/main/packages/terra-rust-signal-bot)
  to view the state via Signal.

**To bring everything together smoothly `./terra-rust-bot/test/build/ctlscript.sh` handles all interaction with the
above-mentioned executables and bash scripts.**

*Make sure you used the installation script as described in the build section above.*

* `cd test/build`
* run `./ctlscript.sh help` to learn how to use terra-rust-bot.

## Summary

- I created this bot to learn about smart contracts, the terra blockchain and to get to know the Rust programming
  language.
- Terra-rust-bot is a constant work in progress: **Bug Reports** and **Feature Requests** are welcome!
- Thanks to the people who helped me along the way on the <a href="https://discord.com/invite/EuKCeGFb93">terra
  discord </a>. :heart:

## Support!

You do not need to donate to me, but I will be very happy if you can.

- `terra1q0n5e43mmz8ddra8554xkxsvelnz4evwukxkht`

Thanks a lot.

## Similar projects

- https://github.com/ALPAC-4/auto_repay
- https://github.com/RomainLanz/anchor-borrow-bot
- https://github.com/unl1k3ly/AnchorHODL
