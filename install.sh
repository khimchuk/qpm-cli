#!/usr/bin/env sh

MYPATH=$(pwd)

cargo build
sudo cp $MYPATH/target/debug/scrappy /usr/local/bin/
