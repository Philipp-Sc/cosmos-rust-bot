#!/bin/bash 

cd "$(dirname "$0")"

args=("$@")
ELEMENTS=${#args[@]} 

case $1 in
  "start")
  echo -e "Enter the arguments that you want to pass to terra-rust-bot:"
  echo -e "E.g:"
  echo -e "    [TEST]"
  echo -e "          -b anchor_auto_stake anchor_auto_repay anchor_auto_borrow -d test -w <replace_with_terra_wallet_address>"
  echo -e "    [PRODUCTION]"
  echo -e "          -b anchor_auto_stake anchor_auto_repay anchor_auto_borrow"
  IFS= read -r MYARGS < /dev/tty
  ../run.sh ${MYARGS};
  sleep 1;
  ps -p "$(cat ../my-bot.pid)";
  if [ -f ../my-bot.pid ]; then
      echo "process running with PID $(cat ../my-bot.pid)"
  else
      echo "process not running: failed to start terra-rust-bot!"
  fi
  ;;

  "stop")
  ../stop.sh
  ;;

  "show")
  cd ../packages/terra-rust-bot-output;
  ./my-bot-output local-display -m "$2"
  ;;

  "connect-signal-app")
  cd ../packages/terra-rust-signal-bot;
  echo "Open the Signal App >> Settings >> Linked Devices >> Link New Device"
  echo "Scan the QR-Code below, wait until the devices are linked."
  ./terra-rust-signal-bot link-device -s production -n terra-rust-signal-bot;
  ;;

  "start-signal-bot")
  cd ../packages/terra-rust-signal-bot;
  ./run.sh;
  ;;

  "stop-signal-bot")
  cd ../packages/terra-rust-signal-bot;
  ./stop.sh;
  ;;

  "help")
  echo "to start terra-rust-bot: './ctlscript.sh start'"
  echo "to stop terra-rust-bot: './ctlscript.sh stop'"
  echo "to view the state of terra-rust-bot: './ctlscript.sh show \"\help\"'"
  echo "to enable the signal messenger integration link your device: './ctlscript.sh connect-signal-app'"
  echo "to activate the signal bot for notifications: './ctlscript.sh start-signal-bot'"
  echo "to de-activate the signal bot: './ctlscript.sh stop-signal-bot'"
  ;;

  "")

  echo "ERROR: specify one of the following arguments: start, stop, or show."
  exit
  ;;
esac
 

   
