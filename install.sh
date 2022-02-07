#!/bin/bash

sed -i 's/lc!(".*").to_string().into_bytes();/lc!("'$(openssl rand -hex 256)'").to_string().into_bytes();/g' $(find . -name "wallet.rs")
echo "updated wallet.rs"

export LITCRYPT_ENCRYPT_KEY=$(openssl rand -hex 256)

case $1 in
	"dev")
	echo "development build"
	cargo build
	mv ./target/debug/terra-rust-bot my-bot
	;;

	"prod")
	echo "release build"
	cargo build --release
	mv ./target/release/terra-rust-bot my-bot
	;;

	"native")
	echo "optimized release build"
	RUSTFLAGS="-C target-cpu=native" cargo build --release
	mv ./target/release/terra-rust-bot my-bot
	;;
	
	"")
	echo "ERROR: specify one of the following arguments: dev, prod, or native."
	exit
	;;
esac

echo "install.sh finished"

echo "terra-rust-bot executable available as ./my-bot"
echo $(ls -lh my-bot)
echo ""
echo "for convinience use ./run.sh to start/stop the bot"


