#!/bin/bash
if [[ `uname` == 'Darwin' ]]; then
	brew install coreutils
	SCRIPT=`greadlink -f $0`
else
	SCRIPT=`readlink -f $0`
fi
SCRIPTPATH=`dirname $SCRIPT`
PROJPATH=`dirname $SCRIPTPATH`
docker run \
-d \
--name telepost \
-v $PROJPATH:/telepost \
-v /home/shane/persistence/zeg_bot:/db \
-w /telepost \
scorpil/rust:1.19 \
/bin/bash \
-c \
"\
apt update;\
apt install libsqlite3-dev -y;\
cargo run --release;\
"
