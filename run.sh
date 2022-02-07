#!/bin/bash 

export CLICOLOR=1
export CLICOLOR_FORCE=1

BLUE='\033[0;34m'
PURPLE='\033[0;35m' 
YELLOW='\033[0;33m'
NC='\033[0m'

echo -e "${PURPLE}checking if ./my-bot is running already and stopping instance ${BLUE}"
#echo -e "(if you want to run multiple instance, you will need to update ./run.sh and ./stop.sh)${BLUE}"
./stop.sh

# store arguments in a special array 
args=("$@") 
# get number of elements 
ELEMENTS=${#args[@]} 

IS_BOT=false
IS_TEST=false
HAS_WALLET=false 

IS_GENERAL=true

# echo each element in array  
# for loop 
for (( i=0;i<$ELEMENTS;i++)); do 
    if [ ${args[${i}]} = "-b" ]; then
	    IS_BOT=true
        IS_GENERAL=false
    fi 
    if [ ${args[${i}]} = "-a" ]; then 
        IS_GENERAL=false
    fi 

    if [ ${args[${i}]} = "test" ]; then
        IS_TEST=true
    fi 

    if [ ${args[${i}]} = "-w" ]; then
        HAS_WALLET=true
        IS_GENERAL=false
    fi 

done

#echo "IS_BOT:${IS_BOT}"
#echo "IS_TEST:${IS_TEST}"
#echo "HAS_WALLET:${HAS_WALLET}"

NEED_INPUT=false

if [ "$IS_BOT" = true ] && ( ( [ "$IS_TEST" = true ] && [ "$HAS_WALLET" = false ] ) || [ "$IS_TEST" = false ] ); then
    echo -e "${BLUE}Enter your seed phrase ${PURPLE}(press enter if you want to continue without your seed phrase)"
    NEED_INPUT=true
fi

if [ "$IS_BOT" = false ] && [ "$HAS_WALLET" = false ] && [ "$IS_GENERAL" = false ]; then
    echo -e "${BLUE}Enter your wallet address ${PURPLE}(press enter if you want to continue withoug your wallet address)"
    NEED_INPUT=true
fi

if [ "$IS_GENERAL" = false ] && [ "$NEED_INPUT" = true ]; then
    IFS= read -rs INPUT < /dev/tty
    nohup ./my-bot $@  <<< "$INPUT" &
else
    nohup ./my-bot $@ & 
fi

echo $! > ./my-bot.pid

echo -e "${PURPLE}process id written to ./my.bot.pid${PURPLE}"
echo -e "to stop the bot run: ${YELLOW}./stop.sh${PURPLE}"
echo ""
echo "to view the current state of the bot run the following command"
echo -e "${YELLOW}while sleep 0.1; do cat terra-rust-bot-display.txt; done${PURPLE}"
echo "(CTRL + C to exit)"
