
> :warning: The last (tested) version of Terra-rust-bot can be found [here](https://github.com/Philipp-Sc/terra-rust-bot/tree/d9499d182c6e627a5dedfab15f0ee0fdc698b994).

> :information_source: The bot was stress tested during the UST crash, it successfully prevented liquidations as it was set up to do.

> :information_source: As a result of the crash, this bot is shifting its focus to Cosmos. Terra related functionalities will be restored the days following the Terra 2.0 rebirth. 

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

**Technical**
- [ ] Refactor to Cosmos-rust-bot 
- [ ] CosmWasm v.1.0.0: Use gRPC instead of LCD/FCD using [cosmos-rust](https://github.com/cosmos/cosmos-rust/).
- [ ] Signal: In-Memory ConfigStore  

**Cosmos Ecosystem**: 

**Analysis**
- [ ] Terra Dashboard: List Tokens, Prices & Pools (Terraswap, Astroport) 
- [ ] Osmosis Dashboard: List Tokens, Prices & Pools (Osmosiszone)

**Notifications**
- [ ] Arbitrage Opportunities: Notifications, Automation (including Terra pools on Osmosiszone)
- [ ] Governance Notifications (new proposal, hit quorum, passed, rejected, executed, ..) 
- - [ ] To Consider: off-chain forum activity.

**Automated Strategies**

- [ ] Arbitrage: Automation (including Terra pools on Osmosiszone)
- [ ] Impermanent Loss Protection (Osmosiszone, Astroport, Spectrum) 
- [ ] DCA
  - - [ ] Martingale Feature (double or otherwise modify buys depending on price and tx history)
- [ ] Limit Orders 
  - - [ ] TP and SL
- [ ] Take Profits & Diversify (reducing single point of failure risk)


**Interface**

Add commands to quickly broadcast transactions via Terminal/Signal Messenger
- - [ ] Show **Analysis**
- - [ ] Enable / Disable **Notifications**
- - [ ] Enable / Disable **Automated Strategies**
- - [ ] Dust Converter (convert small balances into a token of choice)
- - [ ] Abort Mission (sell selected investments and trade in for stablecoin)
- - [ ] Migrate Wallet (safety feature: immediately move tokens/coins to a new wallet)



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

Feature list:

- to be updated after CosmWasm v.1.0.0 refactoring.
 
## Summary

- I created this bot to learn about smart contracts, the Terra blockchain and to get to know the Rust programming
  language. Post-attack the journey continues with a stronger focus on the broader Cosmos ecosystem.
- Cosmos-rust-bot is a constant work in progress: **Bug Reports** and **Feature Requests** are welcome!
- Thanks to the people who helped me along the way on the <a href="https://discord.com/invite/EuKCeGFb93">terra
  discord </a>. :heart:
- Special thanks to [PFC Validator](https://pfc-validator.github.io/) for being super helpful and providing their rust tooling open source.

## Support!

You do not need to donate to me, but I will be very happy if you can.

- `terra1q0n5e43mmz8ddra8554xkxsvelnz4evwukxkht`
- `osmo10885ryvnfvu7hjt8lqvge77uderycqcu50nmmh`

Thanks a lot.

## Dependencies

- [cosmos-rust-interface](https://github.com/Philipp-Sc/cosmos-rust-interface)

## Similar projects

- https://github.com/ALPAC-4/auto_repay
- https://github.com/RomainLanz/anchor-borrow-bot
- https://github.com/unl1k3ly/AnchorHODL
