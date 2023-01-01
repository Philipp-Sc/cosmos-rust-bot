<div align="center">

  <img src="https://github.com/Philipp-Sc/media/blob/main/cosmos-rust-bot/cosmos-rust-bot-icon.png" height="100">
  <h1>CRB - Cosmos Rust Bot™</h1> 
  <p> A Rust Bot for the Cosmos Ecosystem. </p> 
    <img src="https://img.shields.io/github/languages/top/Philipp-Sc/cosmos-rust-bot"> 
    <img src="https://img.shields.io/github/repo-size/Philipp-Sc/cosmos-rust-bot"> 
    <img src="https://img.shields.io/github/commit-activity/m/Philipp-Sc/cosmos-rust-bot"> 
    <img src="https://img.shields.io/github/license/Philipp-Sc/cosmos-rust-bot">
    <a href="https://twitter.com/cosmosrustbot"><img src="https://img.shields.io/twitter/follow/CosmosRustBot?style=social"></a>
  </div>
<br/> 



> :warning: No security audit has been performed. (*[See Disclaimer: Do your own research. Take a look at the code.](https://github.com/Philipp-Sc/cosmos-rust-bot/blob/0ecae398c80192822090598947ba7c0ee5cba562/DISCLAIMER.txt)*)

##

### [About](#about) · [Install](#install) · [Summary](#summary)

## About

> Cosmos-Rust-Bot refers to this repository and it's code which enables others to build services on top of CRB.


### Description

> Cosmos-Rust-Bot queries the blockchain nodes via **gRPC** (a remote procedure call protocol that uses protobufs for serializing structured data) for the best performance due to its binary encoding format. Also syncing directly to blockchain nodes using gRPC allows for efficient access to data and functionality, useful for tasks such as submitting transactions or querying the blockchain state. This avoids going through an additional layer of abstraction (e.g LCD/FCD). 

> Cosmos-Rust-Bot integrates **CosmWasm 1.0.0** via [cosmos-rust](https://github.com/cosmos/cosmos-rust) as well as providing its own adjustments (including additional types and osmosis-proto definitions) [here](https://github.com/Philipp-Sc/cosmos-rust-development). The resulting core is provided as simple rust crate [cosmos-rust-package](https://github.com/Philipp-Sc/cosmos-rust-package), making it easy to use in your own project.

> Cosmos-Rust-Bot aims to be modular and therefore the following features/programs are integrated via **UNIX Sockets**, enabling them to be developed and updated sepearatly: [Fraud Detection](https://github.com/Philipp-Sc/rust-bert-fraud-detection), [LinkToText](https://github.com/Philipp-Sc/rust-link-to-text) and the [OpenAI API](https://github.com/Philipp-Sc/rust-openai-gpt-tools).

> **Telegram** and **Signal Messenger** integrations are also available as separate package.

> To bring everything together **Dockerfiles** are provided for easy setup and development.

## Install

> dockerfiles available [here](https://github.com/Philipp-Sc/cosmos-rust-bot).

# Use Case #1 
## Cosmos Governance Briefings 

[@cosmos_governance_briefings_bot](https://t.me/cosmos_governance_briefings_bot) provides alerts and analysis on governance proposals, making it easier for users to navigate the process and stay informed about the proposals that are being considered.

One of the unique features is the integrated fraud detection, which helps users identify potential scams or malicious proposals. This is particularly important in the cryptocurrency space, where scams and fraud can be a significant issue. I also use open source technology and AI to generate brief explanations of key aspects of governance proposals, helping users better understand technical language and make informed voting decisions. This is particularly useful for users who may be intimidated by the complexity or unfamiliar with the governance process, as it provides a more accessible and user-friendly way to participate.

In addition to providing notifications and analysis, the bot also allows users to easily access and review proposals, staying up-to-date on the latest governance developments in the Cosmos ecosystem. This can be particularly helpful for users who may not have the time or resources to keep track of all the proposals on their own. 

## Summary

> I created this bot to learn about smart contracts, the Terra blockchain and to get to know the Rust programming
  language. Post-attack the journey continues with a stronger focus on the broader Cosmos ecosystem.
  
> Cosmos-rust-bot is a constant work in progress: **Bug Reports** and **Feature Requests** are welcome!

> Thanks to the people who helped me along the way on the terra discord. :heart:

> Special thanks to [PFC Validator](https://pfc-validator.github.io/) for being super helpful and providing their rust
  tooling open source.

##

> Grant Recipient of
  the <img src="https://uploads-ssl.webflow.com/62aba8dc00fdd48273d4c874/62b327d14f4b5887c5a0c359_osmosis-logomark-white.svg" height="12"> [Osmosis Grants Program](https://grants.osmosis.zone/)
  
> Many thanks to the Osmosis Grants Team for incentivizing this project. :pray:

## License

> Open Source
> Apache-2.0 license
