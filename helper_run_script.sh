#!/bin/bash

case $1 in

	"dev")
	echo "./target/debug/cosmos-rust-bot"
	../target/debug/cosmos-rust-bot
	;;

	"prod")
	echo "./target/release/cosmos-rust-bot"
	../target/release/cosmos-rust-bot
	;;

	"native")
	echo "./target/release/cosmos-rust-bot"
	../target/release/cosmos-rust-bot
	;;

	"tg-bot")
	echo "./target/release/cosmos-rust-telegram-bot"
	../target/release/cosmos-rust-telegram-bot
	;;
esac

echo "exit 0"