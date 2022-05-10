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
	rm -rf terra-rust-bot;
	git clone https://github.com/Philipp-Sc/terra-rust-bot.git
	;;

	"")
	echo "ERROR: specify one of the following arguments: local, remote."
	exit
	;;
esac

build_dir="build_"$(date +%Y%m%d_%H%M%S)

mkdir $build_dir;

cd $build_dir;
mkdir packages;
mkdir assets;
cd packages;
mkdir terra-rust-bot-output;
mkdir terra-rust-signal-bot;
cd ../../;

cp $MYPATH/ctlscript.sh ./$build_dir/;
cp -r $MYPATH/assets/cw20 ./$build_dir/assets/;


WD=$(pwd)

cargo update;
cd $MYPATH/packages/terra-rust-api-layer;cargo update;
cd ../terra-rust-bot-essentials;cargo update;
cd ../terra-rust-bot-output;cargo update;
cd ../terra-rust-signal-bot;cargo update;
cd $WD;

rm -rf $MYPATH/target;
rm -rf $MYPATH/packages/terra-rust-bot-output/target;
rm -rf $MYPATH/packages/terra-rust-bot-essentials/target;
rm -rf $MYPATH/packages/terra-rust-signal-bot/target;


case $3 in
	"all")
	# includes signal-bot, includes terra-rust-bot-output
	$MYPATH/install.sh $1 $2;
	$MYPATH/packages/terra-rust-bot-output/install.sh $1 $2;
	$MYPATH/packages/terra-rust-signal-bot/install.sh $1 $2;

	cp $MYPATH/{my-bot,run.sh,stop.sh,terra-rust-bot.json} ./$build_dir/;
	cp $MYPATH/packages/terra-rust-bot-output/my-bot-output ./$build_dir/packages/terra-rust-bot-output/;
	cp $MYPATH/packages/terra-rust-signal-bot/{terra-rust-signal-bot,signal-bot.sh,always-run.sh,run.sh,stop.sh} ./$build_dir/packages/terra-rust-signal-bot/;
	;;

	"default")
	# excludes signal-bot, includes terra-rust-bot-output
	$MYPATH/install.sh $1 $2;
	$MYPATH/packages/terra-rust-bot-output/install.sh $1 $2;

  	cp $MYPATH/{my-bot,run.sh,stop.sh,terra-rust-bot.json} ./$build_dir/;
  	cp $MYPATH/packages/terra-rust-bot-output/my-bot-output ./$build_dir/packages/terra-rust-bot-output/;
	;;

	"minimal")
	# only terra-rust-bot
	$MYPATH/install.sh $1 $2;
	cp $MYPATH/{my-bot,run.sh,stop.sh,terra-rust-bot.json} ./$build_dir/;
	;;

	"")
	echo "ERROR: specify one of the following arguments: all, default, minimal."
	exit
	;;
esac

echo "$build_dir finished!"
ls -lh ./$build_dir
echo "the next step for you is to configure the settings by editing 'terra-rust-bot.json'."
echo "run 'cd $build_dir;./ctlscript.sh help' to learn how to use terra-rust-bot."
