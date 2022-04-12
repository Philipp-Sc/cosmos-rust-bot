#!/bin/bash

cd "$(dirname "$0")"

case $1 in
	"dev")
	echo "development build"
	cargo build
	mv ./target/debug/terra-rust-bot-output my-bot-output
	;;

	"prod")
	echo "release build"
	cargo build --release
	mv ./target/release/terra-rust-bot-output my-bot-output
	;;

	"native")
	echo "optimized release build"
	RUSTFLAGS="-C target-cpu=native" cargo build --release
	mv ./target/release/terra-rust-bot-output my-bot-output
	;;
	
	"")
	echo "ERROR: specify one of the following arguments: dev, prod, or native."
	exit
	;;
esac

echo "install.sh finished"

echo "terra-rust-bot executable available as ./my-bot-output"
echo $(ls -lh my-bot-output)