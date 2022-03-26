#!/bin/bash 

export CLICOLOR=0
export CLICOLOR_FORCE=0
 
echo -e "checking if ./my-hook is running already and stopping instance"
./stop.sh
 
nohup ./my-hook &  

echo $! > ./my-hook.pid

echo -e "process id written to ./my.hook.pid"
echo -e "to stop the bot run: ./stop.sh" 
