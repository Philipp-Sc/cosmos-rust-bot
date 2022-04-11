#!/bin/bash

args=("$@")
nohup ./terra-rust-signal-bot $@ &

echo $! > ./signal-bot.pid

echo -e "process id written to ./signal-bot.pid"
