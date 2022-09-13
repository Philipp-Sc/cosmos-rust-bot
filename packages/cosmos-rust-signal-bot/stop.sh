#!/bin/bash
cd "$(dirname "$0")"

if [ -f ./cosmos-rust-signal-bot.pid ]; then
    cat ./cosmos-rust-signal-bot.pid
    kill `cat ./cosmos-rust-signal-bot.pid`
    rm ./cosmos-rust-signal-bot.pid
    echo "always-run.sh process stopped"
else 
    echo "always-run.sh process not running"
fi

if [ -f ./signal-bot.pid ]; then
    cat ./signal-bot.pid
    kill `cat ./signal-bot.pid`
    rm ./signal-bot.pid
    echo "signal-bot.sh process stopped"
else
    echo "signal-bot.sh process not running"
fi