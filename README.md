<div align="center">

  <img src="https://github.com/Philipp-Sc/media/blob/main/cosmos-rust-bot/cosmos-rust-bot-icon.png" height="100">
  <h1>CRB - Cosmos Rust Bot™</h1> 
  <p>If you can't beat them join them!</p> 
    <img src="https://img.shields.io/github/languages/top/Philipp-Sc/cosmos-rust-bot"> 
    <img src="https://img.shields.io/github/repo-size/Philipp-Sc/cosmos-rust-bot"> 
    <img src="https://img.shields.io/github/commit-activity/m/Philipp-Sc/cosmos-rust-bot"> 
    <img src="https://img.shields.io/github/license/Philipp-Sc/cosmos-rust-bot">
    <a href="https://twitter.com/cosmosrustbot"><img src="https://img.shields.io/twitter/follow/CosmosRustBot?style=social"></a>
  </div>
<br/>  
<div align="center">

  <img src="https://github.com/Philipp-Sc/media/blob/main/cosmos-rust-bot/cosmos_rust_bot_telegram_start.jpeg" height="250"> 
  <img src="https://github.com/Philipp-Sc/media/blob/main/cosmos-rust-bot/cosmos_rust_bot_telegram_shortcuts_new.jpeg" height="250"> 
  <img src="https://github.com/Philipp-Sc/media/blob/main/cosmos-rust-bot/cosmos_rust_bot_telegram_help_sub.jpeg" height="250">  
  </div>



> :warning: The bot needs your **seed phrase** to create and sign transactions.  
> :arrow_right_hook: You can use Cosmos-rust-bot without a seed phrase in view mode. Test this first.

> :warning: No security audit has been performed. (*[See Disclaimer: This may steal your money. Do your own research.
Take a look at the code.](https://github.com/Philipp-Sc/cosmos-rust-bot/blob/0ecae398c80192822090598947ba7c0ee5cba562/DISCLAIMER.txt)*)

##

### [About](#about) · [Features](#features) · [Install](#install) · [Summary](#summary)

## About

### Use Case #1

- Lookup on-chain information and subscribe to get notified on state changes.
- Easy access via Telegram bot.

### Use Case #2

- Set notifications (events) to trigger/execute on-chain actions. (Send, Execute Contract,..)
- Requires: You to manage a (Linux) Server and provide a wallet seed phrase.
- Secure acces via Signal Messenger.

### Description

**CRB** is intended to be used by **coders, developers and technically-skilled users** to make use of automation.    
At the same time **CRB** can be compiled to a single executable that is easy to use with few dependencies. Making it
easy to deploy and use right away.

In short it's purpose is to have a bot on your side:

**users**

- Save the hassle of managing positions manually
- Enable strategies only bots can execute
- Receive alerts and notifications
- Send commands for the bot to execute

**devs**

- Provide insights into the Cosmos Ecosystem
- Enable developers to write their own bot
- Showcase how to use [cosmos-rust](https://github.com/cosmos/cosmos-rust/)
- Rust

## Features
> This roadmap is intended to outline the general direction for CRB. It does not represent a commitment, guarantee, obligation, or promise to deliver any feature.

### Notifications
- [x] Monitor Cosmos-Rust-Bot
- [x] Lookup Governance Proposals
- [x] Subscribe to Governance Proposal Notifications
- [x] Telegram Chat Bot 
- [ ] Lookup Protocol Governance Proposals
- [ ] Scam Detection/Classification for Governance Proposals
- [ ] Lookup Prices (Osmosiszone)
- [ ] Subscribe to Price Alerts
- [ ] Token Listings/Pools (Add/Remove/Update)
- [ ] NFT sales tracking
- [ ] Wallet actions tracking (Whale tracking)
- [ ] Validator Profile (name, comission, votes)
- [ ] Impermanent Loss tracking
- [ ] Lookup Airdrops
- [ ] Watch Wallet Networth 

### Actions
- [ ] Send
- [ ] Staking: Delegate/Re-delegate/Un-delegate
- [ ] Vote on proposals in deposit/voting period
- [ ] Buy/Sell Asset
- [ ] Auto compound staking rewards
- [ ] Transfer tokens to existing or new wallet
- [ ] Transfer IBC tokens Cross-Chain
- [ ] Swap in/out Stablecoin
- [ ] Manage Collateral / Loans
- [ ] Balance Wallet

## Install

- `cd cosmos-rust-bot`
- `./install.sh test` creates new `cosmos-rust-bot-feature-list.json`
- `./install.sh dev` builds **cosmos-rust-bot** `./my-bot`
- `./my-bot` to simply run **cosmos-rust-bot**
- see [cosmos-rust-signal-bot](https://github.com/Philipp-Sc/cosmos-rust-bot/tree/development/packages/cosmos-rust-signal-bot) to enable signal notifications.

## Summary

- I created this bot to learn about smart contracts, the Terra blockchain and to get to know the Rust programming
  language. Post-attack the journey continues with a stronger focus on the broader Cosmos ecosystem.
- Cosmos-rust-bot is a constant work in progress: **Bug Reports** and **Feature Requests** are welcome!
- Thanks to the people who helped me along the way on the terra discord. :heart:
- Special thanks to [PFC Validator](https://pfc-validator.github.io/) for being super helpful and providing their rust
  tooling open source.

##

- Grant Recipient of
  the <img src="https://uploads-ssl.webflow.com/62aba8dc00fdd48273d4c874/62b327d14f4b5887c5a0c359_osmosis-logomark-white.svg" height="12"> [Osmosis Grants Program](https://grants.osmosis.zone/)
- Many thanks to the Osmosis Grants Team for incentivizing this project. :pray:

## License

- Open Source
- Apache-2.0 license

## Dependencies

- [cosmos-rust-signal-bot](https://github.com/Philipp-Sc/cosmos-rust-bot/tree/development/packages/cosmos-rust-signal-bot) (
  optional)

- [cosmos-rust-interface](https://github.com/Philipp-Sc/cosmos-rust-interface)

## Similar Projects
- [Cosmos Gov](https://github.com/shifty11/cosmos-gov)
