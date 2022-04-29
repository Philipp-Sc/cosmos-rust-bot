#!/bin/bash

cd "$(dirname "$0")"

sed -i 's/lc!(".*").to_string().into_bytes();/lc!("'$(openssl rand -hex 256)'").to_string().into_bytes();/g' $(find . -name "wallet.rs")
echo "updated wallet.rs"

export LITCRYPT_ENCRYPT_KEY=$(openssl rand -hex 256)

case $1 in
	"dev")
	echo "development build"
	RUSTFLAGS="--cfg tokio_unstable" cargo build
	mv ./target/debug/terra-rust-bot my-bot
	;;

	"prod")
	echo "release build"
	RUSTFLAGS="--cfg tokio_unstable" cargo build --release
	mv ./target/release/terra-rust-bot my-bot
	;;

	"native")
	echo "optimized release build"
	RUSTFLAGS="--cfg tokio_unstable -C target-cpu=native" cargo build --release
	mv ./target/release/terra-rust-bot my-bot
	;;
	
	"")
	echo "ERROR: specify one of the following arguments: dev, prod, or native."
	exit
	;;
esac

echo "install.sh finished"

echo "terra-rust-bot executable available as './my-bot'"
echo $(ls -lh my-bot)
echo ""
echo "for convenience use './run.sh' to start the bot and './stop.sh' to stop the bot."
