#!/usr/bin/env sh
TARGET=~/.lv2
mkdir -p $TARGET
for F in $(ls -d ./target/debug/*.lv2); do
  P=$(basename $F)
  echo "install $P"
  rm -r $TARGET/$P
  cp -r $F $TARGET
done
