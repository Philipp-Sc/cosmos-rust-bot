#!/bin/bash

cd "$(dirname "$0")"

echo "Installing cosmos-rust-bot"

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
	MYPATH="../../cosmos-rust-bot";
	;;

	"remote")
	MYPATH="cosmos-rust-bot";
	rm -rf cosmos-rust-bot;
	git clone https://github.com/Philipp-Sc/cosmos-rust-bot.git
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
mkdir cosmos-rust-bot-output;
mkdir terra-rust-signal-bot;
cd ../../;

cp $MYPATH/ctlscript.sh ./$build_dir/;
cp -r $MYPATH/assets/cw20 ./$build_dir/assets/;


WD=$(pwd)

cargo update;
cd $MYPATH/packages/cosmos-rust-interface;cargo update;
cd ../cosmos-rust-bot-essentials;cargo update;
cd ../cosmos-rust-bot-output;cargo update;
cd ../terra-rust-signal-bot;cargo update;
cd $WD;

rm -rf $MYPATH/target;
rm -rf $MYPATH/packages/cosmos-rust-bot-output/target;
rm -rf $MYPATH/packages/cosmos-rust-bot-essentials/target;
rm -rf $MYPATH/packages/terra-rust-signal-bot/target;


case $3 in
	"all")
	# includes signal-bot, includes cosmos-rust-bot-output
	$MYPATH/install.sh $1 $2;
	$MYPATH/packages/cosmos-rust-bot-output/install.sh $1 $2;
	$MYPATH/packages/terra-rust-signal-bot/install.sh $1 $2;

	cp $MYPATH/{my-bot,run.sh,stop.sh,cosmos-rust-bot.json} ./$build_dir/;
	cp $MYPATH/packages/cosmos-rust-bot-output/my-bot-output ./$build_dir/packages/cosmos-rust-bot-output/;
	cp $MYPATH/packages/terra-rust-signal-bot/{terra-rust-signal-bot,signal-bot.sh,always-run.sh,run.sh,stop.sh} ./$build_dir/packages/terra-rust-signal-bot/;
	;;

	"default")
	# excludes signal-bot, includes cosmos-rust-bot-output
	$MYPATH/install.sh $1 $2;
	$MYPATH/packages/cosmos-rust-bot-output/install.sh $1 $2;

  	cp $MYPATH/{my-bot,run.sh,stop.sh,cosmos-rust-bot.json} ./$build_dir/;
  	cp $MYPATH/packages/cosmos-rust-bot-output/my-bot-output ./$build_dir/packages/cosmos-rust-bot-output/;
	;;

	"minimal")
	# only cosmos-rust-bot
	$MYPATH/install.sh $1 $2;
	cp $MYPATH/{my-bot,run.sh,stop.sh,cosmos-rust-bot.json} ./$build_dir/;
	;;

	"")
	echo "ERROR: specify one of the following arguments: all, default, minimal."
	exit
	;;
esac

echo "$build_dir finished!"
ls -lh ./$build_dir
echo "the next step for you is to configure the settings by editing 'cosmos-rust-bot.json'."
echo "run 'cd $build_dir;./ctlscript.sh help' to learn how to use cosmos-rust-bot."
