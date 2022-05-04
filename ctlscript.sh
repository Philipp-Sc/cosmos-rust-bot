#!/bin/bash 

cd "$(dirname "$0")"

args=("$@")
ELEMENTS=${#args[@]} 

case $1 in
  "start")
  ./run.sh;
  sleep 1;
  ps -p "$(cat ./my-bot.pid)";
  if [ -f ./my-bot.pid ]; then
      echo "process running with PID $(cat ./my-bot.pid)"
  else
      echo "process not running: failed to start terra-rust-bot!"
  fi
  ;;

  "stop")
  ./stop.sh
  ;;

  "show")
  cd ./packages/terra-rust-bot-output;
  ./my-bot-output local-display -m "$2"
  ;;

  "utils")
  cd ./packages/terra-rust-bot-output;
  ./my-bot-output local-utils
  ;;

  "connect-signal-app")
  cd ./packages/terra-rust-signal-bot;
  echo "Open the Signal App >> Settings >> Linked Devices >> Link New Device"
  echo "Scan the QR-Code below, wait until the devices are linked."
  ./terra-rust-signal-bot link-device -s production -n terra-rust-signal-bot;
  ;;

  "start-signal-bot")
  cd ./packages/terra-rust-signal-bot;
  ./run.sh;
  ;;

  "stop-signal-bot")
  cd ./packages/terra-rust-signal-bot;
  ./stop.sh;
  ;;

  "help")
  echo "to enable different features and configure terra-rust-bot: 'nano ./terra-rust-bot.json'"
  echo "to start terra-rust-bot: './ctlscript.sh start'"
  echo "to stop terra-rust-bot: './ctlscript.sh stop'"
  echo "to view the state of terra-rust-bot: './ctlscript.sh show \"\help\"'"
  echo "to enable the signal messenger integration link your device: './ctlscript.sh connect-signal-app'"
  echo "to activate the signal bot for notifications: './ctlscript.sh start-signal-bot'"
  echo "to stop the signal bot: './ctlscript.sh stop-signal-bot'"
  ;;

  "")

  echo "ERROR: specify one of the following arguments: start, stop, or show."
  exit
  ;;
esac
 

   
