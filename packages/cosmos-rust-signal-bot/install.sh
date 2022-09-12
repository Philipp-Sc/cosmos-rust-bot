#!/bin/bash

cd "$(dirname "$0")"

case $1 in
	"dev")
	echo "development build"
	cargo build
	mv ./target/debug/cosmos-rust-signal-bot cosmos-rust-signal-bot
	;;

	"prod")
	echo "release build"
	cargo build --release
	mv ./target/release/cosmos-rust-signal-bot cosmos-rust-signal-bot
	;;

	"native")
	echo "optimized release build"
	RUSTFLAGS="-C target-cpu=native" cargo build --release
	mv ./target/release/cosmos-rust-signal-bot cosmos-rust-signal-bot
	;;
	
	"")
	echo "ERROR: specify one of the following arguments: dev, prod, or native."
	exit
	;;
esac

echo "install.sh finished"

echo "cosmos-rust-signal-bot executable available as './cosmos-rust-signal-bot'"
echo $(ls -lh cosmos-rust-signal-bot)
echo ""
echo "for convenience use './run.sh' to start the bot and './stop.sh' to stop it."