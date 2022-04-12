#!/bin/bash

cd "$(dirname "$0")"

echo -e "checking if ./terra-rust-signal-bot is running already and stopping instance"
./stop.sh
 
nohup ./always-run.sh activate &

echo $! > ./terra-rust-signal-bot.pid

echo -e "process id written to ./terra-rust-signal-bot.pid"
echo -e "to stop the bot run: ./stop.sh" 
