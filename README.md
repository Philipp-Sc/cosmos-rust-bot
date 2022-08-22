<div align="center">

  <img src="https://github.com/Philipp-Sc/media/blob/main/cosmos-rust-bot/cosmos-rust-bot-icon.png" height="100">
  <h1>CRB - Cosmos Rust Bot™</h1> 
  <p>If you can't beat them join them!</p> 
    <img src="https://img.shields.io/github/languages/top/Philipp-Sc/cosmos-rust-bot"> 
    <img src="https://img.shields.io/github/repo-size/Philipp-Sc/cosmos-rust-bot"> 
    <img src="https://img.shields.io/github/commit-activity/m/Philipp-Sc/cosmos-rust-bot"> 
    <img src="https://img.shields.io/github/license/Philipp-Sc/cosmos-rust-bot">
    <img src="https://img.shields.io/twitter/follow/PSchlutermann?style=social"> 
  </div>
<br/>
<div align="center">
  <img src="https://github.com/Philipp-Sc/media/raw/main/cosmos-rust-bot/cosmos-rust-bot-output/gallery/terminal_output_auto_stake.png" height="250">
  <img src="https://github.com/Philipp-Sc/media/raw/main/cosmos-rust-bot/cosmos-rust-bot-output/gallery/terminal_output_market.png" height="250">

  <img src="https://github.com/Philipp-Sc/media/raw/main/cosmos-rust-bot/cosmos-rust-signal-bot/gallery/signal_bot_auto_stake.png" height="250">
  <img src="https://github.com/Philipp-Sc/media/raw/main/cosmos-rust-bot/cosmos-rust-signal-bot/gallery/signal_bot_help.png" height="250">
  <img src="https://github.com/Philipp-Sc/media/raw/main/cosmos-rust-bot/cosmos-rust-signal-bot/gallery/signal_messenger.png" height="250">
</div>

##

> :warning: The bot needs your **seed phrase** to create and sign transactions.  
> :arrow_right_hook: You can use Cosmos-rust-bot without a seed phrase in view mode. Test this first.

> :warning: No security audit has been performed. (*See Disclaimer: This may steal your money. Do your own research.
Take a look at the code.*)

##

## Disclaimer

This software is for educational purposes only. Do not risk money which you are afraid to lose.
USE THE SOFTWARE AT YOUR OWN RISK. THE AUTHOR ASSUMES NO RESPONSIBILITY FOR YOUR USE OF Cosmos-Rust-Bot.

Always start by running with a dedicated test wallet and do not engage money before you understand how it works
and what results you should expect.

I strongly recommend you to have coding and Rust knowledge. Do not hesitate to read the source code and understand the
mechanism of this bot.

##

### [Install](#install) · [Config](#config) · [Usage](#usage) · [Summary](#summary)

## New Roadmap:

**Cosmos Ecosystem**:

**Analysis**

- [ ] Terra Dashboard: List Tokens, Prices & Pools (Terraswap, Astroport)
- [ ] Osmosis Dashboard: List Tokens, Prices & Pools (Osmosiszone)

**Notifications**

- [ ] Arbitrage Opportunities: Notifications, Automation (including Terra pools on Osmosiszone)
- [ ] Governance Notifications (new proposal, hit quorum, passed, rejected, executed, ..)
  -- [ ] Voting
  -- [ ] To Consider: off-chain forum activity (sentiment analysis).

**Automated Strategies**

- [ ] Arbitrage: Automation (including Terra pools on Osmosiszone)
- [ ] Impermanent Loss Protection (Osmosiszone, Astroport, Spectrum)
- [ ] DCA
- [ ] Martingale Feature (double or otherwise modify buys depending on price and tx history)
- [ ] Limit Orders
- [ ] TP and SL
- [ ] Take Profits & Diversify (reducing single point of failure risk)

**Interface**

Add commands to quickly broadcast transactions via Terminal/Signal Messenger

-
    - [ ] Show **Analysis**
-
    - [ ] Enable / Disable **Notifications**
-
    - [ ] Enable / Disable **Automated Strategies**
-
    - [ ] Dust Converter (convert small balances into a token of choice)
-
    - [ ] Abort Mission (sell selected investments and trade in for stablecoin)
-
    - [ ] Migrate Wallet (safety feature: immediately move tokens/coins to a new wallet)

## About

**CRB** is currently undergoing a rewrite. Available functionality will be described here shortly.

It is intended to be used by **coders, developers and technically-skilled users** to make use of automation.    
At the same time **CRB** can be compiled to a single executable that is easy to use with few dependencies. Making it
easy to deploy and use right away.

In short it's purpose is to have a bot on your side:

**users**

- Save you the hassle of managing your positions manually
- Enable you to run strategies only bots can execute
- Receive alerts and notifications
- Send commands for the bot to execute

**devs**

- Provide insights into the Cosmos Ecosystem
- Enable developers to write their own bot
- Showcase how to use [cosmos-rust](https://github.com/cosmos/cosmos-rust/)
- Rust

## Features

- [x] Monitor Cosmos-Rust-Bot
- [x] Lookup Governance Proposals
- [ ] Subscribe to Governance Proposal Notifications
- [ ] Vote on proposals in deposit/voting period
- [ ] Transfer tokens to existing or new wallet
- [ ] Transfer IBC tokens Cross-Chain
- [ ] Swap in/out Stablecoin
- [ ] Sentiment Analysis on Governance Proposals (scrapping news)
- [ ] Sentiment Analysis on Blockchain (e.g Osmosis, Terra, Juno)
- [ ] Sentiment Analysis on Stablecoin (e.g axelarUSDC, USK)

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
