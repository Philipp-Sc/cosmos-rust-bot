#!/bin/bash 

cd "$(dirname "$0")"

export CLICOLOR=1
export CLICOLOR_FORCE=1

BLUE='\033[0;34m'
PURPLE='\033[0;35m' 
YELLOW='\033[0;33m'
NC='\033[0m'

echo -e "${PURPLE}checking if ./my-bot is running already and stopping instance ${BLUE}"
#echo -e "(if you want to run multiple instance, you will need to update ./run.sh and ./stop.sh)${BLUE}"
./stop.sh


IS_BOT=$(grep -oP '\"anchor_protocol_auto_repay\":( )*(true|false)' terra-rust-bot.json | grep -oP '(true|false)')
if [ "$IS_BOT" = "false" ]; then
 IS_BOT=$(grep -oP '\"anchor_protocol_auto_borrow\":( )*(true|false)' terra-rust-bot.json | grep -oP '(true|false)')
fi
if [ "$IS_BOT" = "false" ]; then
 IS_BOT=$(grep -oP '\"anchor_protocol_auto_stake\":( )*(true|false)' terra-rust-bot.json | grep -oP '(true|false)')
fi
if [ "$IS_BOT" = "false" ]; then
 IS_BOT=$(grep -oP '\"anchor_protocol_auto_farm\":( )*(true|false)' terra-rust-bot.json | grep -oP '(true|false)')
fi

IS_TEST=$(grep -oP '\"read_only_mode\":( )*(true|false)' terra-rust-bot.json | grep -oP '(true|false)')
HAS_WALLET=$(grep -oP '\"terra_wallet_address\":( )*(null|\"terra)' terra-rust-bot.json | grep -oP '(null| "terra)' | grep -oP '(null|terra)')


NEED_ACCOUNT=$(grep -oP '\"anchor_account_info\":( )*(true|false)' terra-rust-bot.json | grep -oP '(true|false)')

#echo "IS_BOT:${IS_BOT}"
#echo "IS_TEST:${IS_TEST}"
#echo "HAS_WALLET:${HAS_WALLET}"

NEED_INPUT=false

if [ "$IS_BOT" = "true" ] && ( ( [ "$IS_TEST" = "true" ] && [ "$HAS_WALLET" = "null" ] ) || [ "$IS_TEST" = "false" ] ); then
    echo -e "${BLUE}Enter your seed phrase ${PURPLE}(press enter if you want to continue without your seed phrase)"
    NEED_INPUT=true
elif [ "$IS_BOT" = "false" ] && [ "$HAS_WALLET" = "null" ] && [ "$NEED_ACCOUNT" = "true" ]; then
    echo -e "${BLUE}Enter your wallet address ${PURPLE}(press enter if you want to continue without your wallet address)"
    NEED_INPUT=true
fi

if [ "$NEED_INPUT" = true ]; then
    IFS= read -r INPUT < /dev/tty
    printf '\033[1A\033[K'
    echo -n -e "${YELLOW}"
    if [ "$IS_BOT" = "true" ]; then
      for i in {1..24}
      do
          printf "***** "
      done
    else
      for i in {1..44}
      do
          printf "*"
      done
    fi
    echo -e "${PURPLE}"
    nohup ./my-bot  <<< "$INPUT" &
else
    nohup ./my-bot &
fi

echo $! > ./my-bot.pid

echo -e "${PURPLE}process id written to ./my.bot.pid${PURPLE}"
echo -e "to stop the bot run: ${YELLOW}./stop.sh${NC}"