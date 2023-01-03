# Grants Overview

## Osmosis Grants Program


- Cosmos-Rust-Bot is a grant recipient (Batch #4) of
  the <img src="https://uploads-ssl.webflow.com/62aba8dc00fdd48273d4c874/62b327d14f4b5887c5a0c359_osmosis-logomark-white.svg" height="12"> [Osmosis Grants Program](https://grants.osmosis.zone/)
- Many thanks to the Osmosis Grants Team for incentivizing this project.

### Batch #4 - Cosmos-Rust-Bot

#### Funding Amount

> $4,750 (in OSMO)   
> **Payment Structure**
> 30% upfront / 70% at completion of the bot

#### Description

> Cosmos-Rust-Bot (which will support features such as governance proposal notifications, scam detection, summaries, topic extraction) will be able to support multiple Cosmos chains without the need to completely rewrite the API, even with the update to CosmWasm v.1.0.0, as it will use gRPC instead of LCD/FCD using cosmos-rust.

#### Purpose
> The code will be insightful for new and existing developers who want to build applications in Rust that interact directly with the Cosmos / Osmosis nodes. The advantage of Cosmos-Rust-Bot not being an on-chain protocol is that features such as those listed above can be implemented rather quickly.

### Introduction
 
Previously known as the Terra-Rust-Bot, this project was originally focused on providing auto compounding, yield optimization, and automation for tasks such as borrowing, repaying, and staking. However, due to unforeseen circumstances involving the crash of the Terra blockchain, I decided to pivot and seek a new home in the Cosmos ecosystem. By leveraging the existing code and working to improve user adoption, I hope to avoid the risk of being tied to a single chain.

In transitioning to the Cosmos ecosystem, the goals of the project also changed. Rather than focusing on yield optimization and auto compounding, I decided to prioritize governance features such as notifications, fraud detection, proposal summaries, and briefings. These features are designed to increase awareness and improve the overall experience of participating in governance within the Cosmos ecosystem.

Thanks to recent advances in artificial intelligence and natural language processing, it is now possible to extract valuable insights and summaries from long and complex governance proposals. This can help users stay informed and make informed decisions, especially when multiple proposals are up for vote at the same time. The Cosmos-Rust-Bot's fraud detection features can also assist users in avoiding scams and phishing attacks, which are a common problem in the cryptocurrency world.

I hope that the Cosmos-Rust-Bot will be a valuable resource for anyone interested in staying engaged with the governance process in the Cosmos ecosystem. It will also serve as a useful example for developers looking to build similar applications using Rust and the cosmos-rust library.

### Milestones

#### Updated Architecture with Multi-chain Support (Refactoring: CosmWasm, gRPC)
- [x] Ensuring that the bot is compatible with multiple Cosmos chains and can be easily adapted to new versions of CosmWasm
- > Cosmos-Rust-Bot queries the blockchain nodes via **gRPC** (a remote procedure call protocol that uses protobufs for serializing structured data) for the best performance due to its binary encoding format. Also syncing directly to blockchain nodes using gRPC allows for efficient access to data and functionality, useful for tasks such as submitting transactions or querying the blockchain state. This avoids going through an additional layer of abstraction (e.g LCD/FCD).
- > Cosmos-Rust-Bot integrates **CosmWasm 1.0.0** via [cosmos-rust](https://github.com/cosmos/cosmos-rust) as well as providing its own adjustments (including additional types and osmosis-proto definitions) [here](https://github.com/Philipp-Sc/cosmos-rust-development). The resulting core is provided as simple rust crate [cosmos-rust-package](https://github.com/Philipp-Sc/cosmos-rust-package), making it easy to use in your own project.
- [x] Multi-chain Support (tested):
  - 🧪 Osmosis (including osmosis specific governance proposals)
  - 🌏 Terra 2.0
  - 🪐 Juno
  - 🌌 Cosmos Hub
  - 🐋 Kujira
  - (Standard Cosmos related governance proposals are supported for any feather chain!)

#### Features
> Cosmos-Rust-Bot aims to be modular and therefore the following features/programs are integrated via **UNIX Sockets**, enabling them to be developed and updated sepearatly: [Fraud Detection](https://github.com/Philipp-Sc/rust-bert-fraud-detection), [LinkToText](https://github.com/Philipp-Sc/rust-link-to-text) and the [OpenAI API](https://github.com/Philipp-Sc/rust-openai-gpt-tools).    

> Live [@cosmos_governance_briefings_bot](https://t.me/cosmos_governance_briefings_bot) (Telegram)
#### Governance Proposal Notifications
- [x] Sending notifications to users when new governance proposals are submitted, when existing proposals are up for vote, pass, fail, are rejected
- [x] Allowing users to customize their notification preferences, including the types of notifications they receive
- [x] Allowing users to search and view specific proposals.

#### Governance Proposal Fraud Detection
- [x] I implemented a robust semi-supervised spam detection using Rust native (state-of-the-art) NLP pipelines.
- This currently provides a decent fraud/spam/scam detection (accuracy >=90%), and it can be further improved via fine-tuning.
- Details can be found here [rust-bert-fraud-detection](https://github.com/Philipp-Sc/rust-bert-fraud-detection), it's open source.

#### GPT-3
> I developed a tool [LinkToText](https://github.com/Philipp-Sc/rust-link-to-text) that uses a headless browser to follow links (within the proposal description) and extract the main text content from the websites.

> Note: This feature is limited to legitimate proposals in voting period. (This is to reduce costs.)

#### Governance Proposal Summaries 
- [x] Generating useful summaries of governance proposals using artificial intelligence and natural language processing techniques. Providing a brief overview of the proposal, including its main points and key arguments.

#### Governance Proposal Briefings
- [x] Generating briefings for governance proposals using GPT-3, a state-of-the-art language generation model. Providing a more detailed analysis of the proposal, including its potential implications and possible consequences
- [x] Using LinkToText to follow links within the proposal description and extract relevant information from external sources.

- Exploring the following topics via a question interface.
  - 🛠️ Feasibility and technical viability
  - 💸 Economic impact
  - ⚖️ Legal and regulatory compliance
  - 🌿 Long-term sustainability
  - 🔎 Transparency & Accountability
  - 👥 Community Support 

### What is next?

> Multiple validators and cosmos users have already created various Governance Notification Bots. Nevertheless, my project is overtaking them both in features and development process. Additionally, Cosmos-Rust-Bot is 100% open source!
> However it is important to be aware of the features users actually value and use, therefore I will continue to gather feedback from users and regularly update and improve the Cosmos-Rust-Bot based on their needs and preferences.

> I plan to continue expanding the bot's capabilities and adding new features that will enhance the user experience and facilitate engagement with the governance process.
- Adding support for more Cosmos chains, DAOs, Protocols and potentially other blockchain networks.
- Improving the user interface and making it more user-friendly (Website Integration!)
- Adding new features and functionality based on user feedback and requests

Overall, my goal is to make the Cosmos-Rust-Bot a valuable and indispensable tool for anyone interested in staying engaged with governance in the Cosmos ecosystem.

#### But wait, this is not all!

Notifications can be difficult to monetize, and relying on donations or grants is not a sustainable business model. Therefore, I am also exploring other ways to monetize the Cosmos-Rust-Bot and ensure its long-term viability.

One potential avenue for monetization is through the creation of premium features or subscriptions. These could include additional notifications, more detailed summaries and briefings, or access to advanced tools and resources. By offering a range of pricing options, I hope to make the Cosmos-Rust-Bot accessible to a wide range of users while also providing additional value to those who are willing to pay for it.

Another potential monetization strategy is through partnerships and collaborations with other organizations or individuals within the Cosmos ecosystem. This could include working with validators to provide customized notifications and tools to their delegators, or collaborating with other projects to provide additional value to their users.

Ultimately, the goal is to create a self-sustaining business model that allows the Cosmos-Rust-Bot to continue to grow and evolve over time, while also providing value to users and contributing to the development of the Cosmos ecosystem.

In my opinion the mentioned monetization strategies can only be successful by providing maximum value. For that I think one additional key feature is needed to be explored. Current developments in AI and Natural Language Processing allow for the creation of chatbots and virtual assistants that can interact with users in a natural, human-like way. By integrating these capabilities into the Cosmos-Rust-Bot, I believe it could become a valuable resource for users looking for guidance, support, and information about the Cosmos ecosystem.

For example, a chatbot could answer questions about:

- How to set up a wallet?
- How do I set up an account on a DEX or exchange?
- How do I swap tokens on osmosis.zone?
- How to delegate to a validator?
- How to participate in governance?
- How to submit a governance proposal?
- What dapps are available within the Osmosis Ecosystem?
- How do I use a dapp?
- Are there any fees associated with using a dapp? If so, how are they calculated?
- What precautions should I take when using a dapp to ensure the security of my assets?

Overall, I believe that integrating chatbot capabilities into the Cosmos-Rust-Bot could be a valuable addition and could help drive adoption and engagement with the tool. It could also potentially provide additional monetization opportunities, such as through the creation of subscription-based virtual assistant services or through partnerships with organizations looking to provide **customer support** and education to their users.