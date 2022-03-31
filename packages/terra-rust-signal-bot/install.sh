#!/bin/bash

case $1 in
	"dev")
	echo "development build"
	cargo build
	mv ./target/debug/terra-rust-signal-bot terra-rust-signal-bot
	;;

	"prod")
	echo "release build"
	cargo build --release
	mv ./target/release/terra-rust-signal-bot terra-rust-signal-bot
	;;

	"native")
	echo "optimized release build"
	RUSTFLAGS="-C target-cpu=native" cargo build --release
	mv ./target/release/terra-rust-signal-bot terra-rust-signal-bot
	;;
	
	"")
	echo "ERROR: specify one of the following arguments: dev, prod, or native."
	exit
	;;
esac

echo "install.sh finished"

echo "terra-rust-signal-bot executable available as ./terra-rust-signal-bot"
echo $(ls -lh terra-rust-signal-bot)
echo ""
echo "for convinience use ./run.sh to start the bot and ./stop.sh to stop it."
echo ""
echo "e.g. './run.sh'"
echo "e.g. './stop.sh'"
