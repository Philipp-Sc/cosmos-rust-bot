#!/bin/bash 

export CLICOLOR=0
export CLICOLOR_FORCE=0

args=("$@")

 
echo -e "checking if ./terra-rust-signal-bot is running already and stopping instance"
./stop.sh
 
nohup ./terra-rust-signal-bot $@ &  

echo $! > ./terra-rust-signal-bot.pid

echo -e "process id written to ./terra-rust-signal-bot.pid"
echo -e "to stop the bot run: ./stop.sh" 
