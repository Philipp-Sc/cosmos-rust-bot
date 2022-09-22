#!/bin/bash

cd "$(dirname "$0")"

echo -e "checking if ./cosmos-rust-signal-bot is running already and stopping instance"
./stop.sh
 
nohup ./always-run.sh --volatile cosmos-rust-bot &

echo $! > ./cosmos-rust-signal-bot.pid

echo -e "process id written to ./cosmos-rust-signal-bot.pid"
echo -e "to stop the bot run: ./stop.sh" 
