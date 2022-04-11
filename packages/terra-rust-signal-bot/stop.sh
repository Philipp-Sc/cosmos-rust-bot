#!/bin/bash

if [ -f ./terra-rust-signal-bot.pid ]; then
    cat ./terra-rust-signal-bot.pid
    kill `cat ./terra-rust-signal-bot.pid`
    rm ./terra-rust-signal-bot.pid
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