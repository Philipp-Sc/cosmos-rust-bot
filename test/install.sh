#!/bin/bash

cd "$(dirname "$0")"

echo -n "Checking dependencies... "
for name in git openssl cargo
do
  [[ $(which $name 2>/dev/null) ]] || { echo -en "\n$name needs to be installed. ";deps=1; }
done
[[ $deps -ne 1 ]] && echo "OK" || { echo -en "\nInstall the above and rerun this script\n";exit 1; }

echo "Installing Terra-rust-bot"

args=("$@")

case $1 in
	"dev")
	;;

	"prod")
	;;

	"native")
	;;

	"")
	echo "ERROR: specify one of the following arguments: dev, prod, or native."
	exit
	;;
esac


case $2 in
	"local")
	MYPATH="../../terra-rust-bot";
	;;

	"remote")
	MYPATH="terra-rust-bot";
	git clone https://github.com/Philipp-Sc/terra-rust-bot.git
	;;

	"")
	echo "ERROR: specify one of the following arguments: local, remote."
	exit
	;;
esac

$MYPATH/install.sh $@;
$MYPATH/packages/terra-rust-bot-output/install.sh $@;
$MYPATH/packages/terra-rust-signal-bot/install.sh $@;

rm -rf build;
mkdir build;

cd build;
mkdir packages;
mkdir bin;
cd packages;
mkdir terra-rust-bot-output;
mkdir terra-rust-signal-bot;
cd ../../;

cp $MYPATH/{my-bot,run.sh,run_show_user_input.sh,stop.sh,terra-rust-bot.json} ./build/;

cp $MYPATH/packages/terra-rust-bot-output/my-bot-output ./build/packages/terra-rust-bot-output/;
cp $MYPATH/packages/terra-rust-signal-bot/{terra-rust-signal-bot,signal-bot.sh,always-run.sh,run.sh,stop.sh} ./build/packages/terra-rust-signal-bot/;

cp $MYPATH/bin/ctlscript.sh ./build/bin/;

echo "build successful!"
ls -lh ./build
echo "the next step for you is to configure the settings by editing 'terra-rust-bot.json'."
echo "run './build/bin/ctlscript.sh help' if you need more info on how to start/stop the bot."