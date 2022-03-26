#!/bin/bash

case $1 in
	"dev")
	echo "development build"
	cargo build
	mv ./target/debug/terra-rust-hook my-hook
	;;

	"prod")
	echo "release build"
	cargo build --release
	mv ./target/release/terra-rust-hook my-hook
	;;

	"native")
	echo "optimized release build"
	RUSTFLAGS="-C target-cpu=native" cargo build --release
	mv ./target/release/terra-rust-hook my-hook
	;;
	
	"")
	echo "ERROR: specify one of the following arguments: dev, prod, or native."
	exit
	;;
esac

echo "install.sh finished"

echo "terra-rust-hook executable available as ./my-hook"
echo $(ls -lh my-hook)
echo ""
echo "for convinience use ./run.sh to start the hook and ./stop.sh to stop the hook."
echo ""
echo "e.g. './run.sh'"
echo "e.g. './stop.sh'"
