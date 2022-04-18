#!/bin/bash

cd "$(dirname "$0")"

echo "Installing Terra-rust-bot"

echo -n "Checking dependencies... "
for name in git openssl cargo
do
  [[ $(which $name 2>/dev/null) ]] || { echo -en "\n$name needs to be installed. ";deps=1; }
done
[[ $deps -ne 1 ]] && echo "OK" || { echo -en "\nInstall the above and rerun this script\n";exit 1; }

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

rm -rf build;
mkdir build;

cd build;
mkdir packages;
mkdir bin;
cd packages;
mkdir terra-rust-bot-output;
mkdir terra-rust-signal-bot;
cd ../../;

cp $MYPATH/bin/ctlscript.sh ./build/bin/;

case $3 in
	"all")
	# includes signal-bot, includes terra-rust-bot-output
  $MYPATH/install.sh $1 $2;
  rm -rf $MYPATH/target;
  $MYPATH/packages/terra-rust-bot-output/install.sh $1 $2;
  rm -rf $MYPATH/packages/terra-rust-bot-output/target;
  $MYPATH/packages/terra-rust-signal-bot/install.sh $1 $2;
  rm -rf $MYPATH/packages/terra-rust-signal-bot/target;

  cp $MYPATH/{my-bot,run.sh,stop.sh,terra-rust-bot.json} ./build/;
  cp $MYPATH/packages/terra-rust-bot-output/my-bot-output ./build/packages/terra-rust-bot-output/;
  cp $MYPATH/packages/terra-rust-signal-bot/{terra-rust-signal-bot,signal-bot.sh,always-run.sh,run.sh,stop.sh} ./build/packages/terra-rust-signal-bot/;
	;;

	"default")
	# excludes signal-bot, includes terra-rust-bot-output
  $MYPATH/install.sh $1 $2;
  rm -rf $MYPATH/target;
  $MYPATH/packages/terra-rust-bot-output/install.sh $1 $2;
  rm -rf $MYPATH/packages/terra-rust-bot-output/target;

  cp $MYPATH/{my-bot,run.sh,stop.sh,terra-rust-bot.json} ./build/;
  cp $MYPATH/packages/terra-rust-bot-output/my-bot-output ./build/packages/terra-rust-bot-output/;
	;;

	"minimal")
	# only terra-rust-bot
	$MYPATH/install.sh $1 $2;
  rm -rf $MYPATH/target;
  cp $MYPATH/{my-bot,run.sh,stop.sh,terra-rust-bot.json} ./build/;
	;;

	"")
	echo "ERROR: specify one of the following arguments: all, default, minimal."
	exit
	;;
esac

echo "build finished!"
ls -lh ./build
echo "the next step for you is to configure the settings by editing 'terra-rust-bot.json'."
echo "run 'cd build/bin;./ctlscript.sh help' to learn how to use terra-rust-bot."