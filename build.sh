#! /bin/bash

echo "rebuilding jojo..."

gcc src/env.c src/obj.c src/ins.c src/jojo.c -o src/jojo # -lczmq
# src/gdom_selftest
