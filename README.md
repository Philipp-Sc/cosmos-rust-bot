# cosmos-rust-bot
[A Rust Bot for the Cosmos Ecosystem.](https://github.com/Philipp-Sc/cosmos-rust-bot/tree/development/workspace/cosmos-rust-bot)

```
[rust-bert-fraud-detection, rust-link-to-text, rust-openai-gpt3-tools] <----->  FS
                        |                        
                   cosmos-rust-bot <----->  FS
                        |        \
                        |  cosmos-rust-telegram-bot <----->  FS
                        |             |
                        |          END USER 
                        |             |    
                        |             |       
                        |   nginx-reverse-proxy 
                        |          /     \
                        |         /       \
                   cosmos-rust-server     file-server (miniserve)  ---> FS
```
# Workspace Setup

```bash
git clone https://github.com/Philipp-Sc/cosmos-rust-bot.git
git submodule update --init
```
# Update Workspace

```bash
git pull
git submodule update --remote --rebase
```
```bash
# fix in case HEAD is detached
git checkout main
git submodule update --remote --rebase
```
# Docker

> Docker is not required, but it helps a lot to efficiently development and maintaining the package. Take a look at the dockerfiles to figure out how to build and run the packages directly without docker.

## Build 

### Build FRAUD DETECTION Service
```bash
cd /workspace/rust-bert-fraud-detection
docker build -t rust-bert-fraud-detection .
```

### Build GPT3 Service
```bash
cd /workspace/rust-openai-gpt3-tools
docker build -t rust-openai-gpt3-tools .
```

### Build LinkToText Service
```bash
cd /workspace/rust-link-to-text
docker build -t rust-link-to-text .
```

### Build Cosmos-Rust-Bot
```bash
docker build -t crb_build -f Dockerfile_build . # skip if already done
docker run -it --rm -v "$(pwd)/workspace":/usr/workspace -v "$(pwd)/cargo_home":/usr/cargo_home -v "$(pwd)/target":/usr/target crb_build dev
```
### Build Telegram Bot
```bash
docker build -t crb_build -f Dockerfile_build . # skip if already done
docker run -it --rm -v "$(pwd)/workspace":/usr/workspace -v "$(pwd)/cargo_home":/usr/cargo_home -v "$(pwd)/target":/usr/target crb_build tg-bot
```

### Build API (Cosmos-Rust-Server)
```bash
docker build -t crb_build -f Dockerfile_build . # skip if already done
docker run -it --rm -v "$(pwd)/workspace":/usr/workspace -v "$(pwd)/cargo_home":/usr/cargo_home -v "$(pwd)/target":/usr/target crb_build api
```

### Documentation
```bash
## the compiled package (binary) will be saved into the ./target directory.
## ./target/{debug,release}/*
```

## Prepare

### Create Features File
```bash
docker build -t crb_build -f Dockerfile_build . # skip if already done
mkdir workspace/cosmos-rust-bot/tmp
sudo docker run -it --rm -v "$(pwd)/workspace":/usr/workspace -v "$(pwd)/cargo_home":/usr/cargo_home -v "$(pwd)/target":/usr/target crb_build test 
sudo mv workspace/cosmos-rust-bot/tmp/cosmos-rust-bot-feature-list.json ./tmp/
```

## Run 

> It is recommended to start the services in this order.

### START FRAUD DETECTION Service 
```bash
cd /workspace/rust-bert-fraud-detection
sudo docker run -d --rm -v "$(pwd)/rustbert_cache":/usr/rustbert_cache -v "$(pwd)/target":/usr/target -v "$(pwd)/cargo_home":/usr/cargo_home -v "$(pwd)/package":/usr/workspace -v "$(pwd)/../../tmp":/usr/workspace/tmp -v "$(pwd)/socket_ipc":/usr/socket_ipc rust-bert-fraud-detection cargo run --release start_service
```
### START GPT3 Service
```bash
cd /workspace/rust-openai-gpt3-tools
docker run -d --rm -v "$(pwd)/rustbert_cache":/usr/rustbert_cache -v "$(pwd)/target":/usr/target -v "$(pwd)/cargo_home":/usr/cargo_home -v "$(pwd)/package":/usr/workspace -v "$(pwd)/../../tmp":/usr/workspace/tmp -v "$(pwd)/socket_ipc":/usr/socket_ipc -e OPENAI_API_KEY=12345 rust-openai-gpt-tools cargo run --release start_service
```
### START LinkToText Service
```bash
cd /workspace/rust-link-to-text
docker run -d --rm  -v "$(pwd)/target":/usr/target -v "$(pwd)/cargo_home":/usr/cargo_home -v "$(pwd)/package":/usr/workspace -v "$(pwd)/../../tmp":/usr/workspace/tmp -v "$(pwd)/socket_ipc":/usr/socket_ipc rust-link-to-text cargo run --release start_service
```
### START CosmosRustBot
```bash
docker build -t crb_run -f Dockerfile_run . # skip if already done
docker run -d --rm -v "$(pwd)/target":/usr/target:ro -v "$(pwd)/tmp":/usr/workspace/tmp -v "$(pwd)/workspace/chain-registry":/usr/workspace/chain-registry -e RUST_LOG=Error crb_run dev
```
### START Telegram Bot
```bash
docker build -t crb_run -f Dockerfile_run . # skip if already done
docker run -d --rm -v "$(pwd)/target":/usr/target:ro -v "$(pwd)/tmp":/usr/workspace/tmp -e TELOXIDE_TOKEN=12345 crb_run tg-bot
``` 

### START API
```bash
docker build -t crb_run -f Dockerfile_run . # skip if already done
docker run -d --rm -v "$(pwd)/target":/usr/target:ro -v "$(pwd)/tmp":/usr/workspace/tmp -p 444:444  crb_run api
``` 

### Documentation
```bash
# (ICP is archived via sockets that are linked between docker containers via: -v "$(pwd)/tmp":/usr/workspace/tmp)
```
