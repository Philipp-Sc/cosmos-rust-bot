# cosmos-rust-bot
[A Rust Bot for the Cosmos Ecosystem.](https://github.com/Philipp-Sc/cosmos-rust-bot/tree/development/workspace/cosmos-rust-bot)

# Workspace Setup

```
git clone https://github.com/Philipp-Sc/cosmos-rust-bot.git

git submodule update --init
```
# Update Workspace

```
git pull

git submodule update
```

# Build Cosmos-Rust-Bot
```
cd workspace/cosmos-rust-bot

./install.sh native
```

# Run Cosmos-Rust-Bot 
```
nohup ./my-bot &
```

# Build Cosmos-Rust-Telegram-Bot
```
cd workspace/cosmos-rust-bot/packages/cosmos-rust-telegram-bot

RUSTFLAGS="--cfg tokio_unstable -C target-cpu=native" cargo build --release
```

# Run Cosmos-Rust-Telegram-Bot 
```
export TELOXIDE_TOKEN=<your_telegram_access_token>
nohup ../../../target/release/cosmos-rust-telegram-bot &
```

