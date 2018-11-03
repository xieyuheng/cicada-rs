#!/usr/bin/env bash

# DIR=./target/release
DIR=./target/x86_64-unknown-linux-musl/release

F="-e instructions:u -F 100000 -g"

cd $DIR

perf record $F ./jojo ~/jojo/adventure/sicp/1-procedure/*

perf script > jojo.perf
stackcollapse-perf.pl jojo.perf > jojo.folded
flamegraph.pl jojo.folded > jojo.svg
firefox jojo.svg
