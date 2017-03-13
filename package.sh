#!/usr/bin/env sh

BASE=$(pwd)
TARGET=$BASE/target/debug

F=./examples
N=$(basename $F)
OUT=$TARGET/$N.lv2
cd $F
echo $N
cargo build
mkdir -p $OUT
cp $TARGET/*.ttl $OUT
cp ./src/*.ttl $OUT
cp ${TARGET}/lib${N}.{so,dylib,dll} "${OUT}"
cd $BASE
