#!/bin/bash

WALLET_RS=$(find . -name "wallet.rs")
sed -i 's/lc!(".*").to_string().into_bytes();/lc!("'$(openssl rand -hex 256)'").to_string().into_bytes();/g' $WALLET_RS
echo "updated secret in $WALLET_RS"

export LITCRYPT_ENCRYPT_KEY=$(openssl rand -hex 256)
echo "created secret LITCRYPT_ENCRYPT_KEY"

case $1 in

       "update")
        echo "update"
        cargo update
        ;;

        "test")
        echo "test"
  	cargo test --package cosmos-rust-bot --features build-binary -- --nocapture
        ;;

	"dev")
	echo "cargo build --features build-binary"
	cargo build --package cosmos-rust-bot --features build-binary
	;;

	"prod")
	echo "cargo build --features build-binary --release"
	cargo build --package cosmos-rust-bot --features build-binary --release
	;;

	"native")
	echo "RUSTFLAGS=""$RUSTFLAGS" -C target-cpu=native" cargo build --package cosmos-rust-bot --features build-binary --release "
	RUSTFLAGS="$RUSTFLAGS -C target-cpu=native" cargo build --package cosmos-rust-bot --features build-binary --release
	;;

	"tg-bot")
        echo "RUSTFLAGS=""$RUSTFLAGS" -C target-cpu=native" cargo build --package cosmos-rust-telegram-bot --release "
        RUSTFLAGS="$RUSTFLAGS -C target-cpu=native" cargo build --package cosmos-rust-telegram-bot --release
        ;;

	"api")
	echo "RUSTFLAGS=""$RUSTFLAGS" -C target-cpu=native" cargo build --package cosmos-rust-server --release "
	RUSTFLAGS="$RUSTFLAGS -C target-cpu=native" cargo build --package cosmos-rust-server --release
	;;
esac

echo "exit 0"
