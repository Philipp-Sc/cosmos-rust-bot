#!/bin/bash
cd "$(dirname "$0")"

args=("$@")

./signal-bot.sh $@;
sleep 10;

while true
do
if ! ps -p $(cat ./signal-bot.pid) > /dev/null
then
    ./signal-bot.sh $@;
    sleep 10
fi
sleep 1
done
