#!/bin/bash

if [ -f ./terra-rust-signal-bot.pid ]; then
    cat ./terra-rust-signal-bot.pid
    kill `cat ./terra-rust-signal-bot.pid`
    rm ./terra-rust-signal-bot.pid
    echo "process stopped"
else 
    echo "process not running"
fi

