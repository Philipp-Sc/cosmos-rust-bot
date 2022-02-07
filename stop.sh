#!/bin/bash

if [ -f ./my-bot.pid ]; then
    cat ./my-bot.pid
    kill `cat ./my-bot.pid`
    rm ./my-bot.pid
    echo "process stopped"
else 
    echo "process not running"
fi

