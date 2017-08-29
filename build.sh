#!/bin/sh

clang -rdynamic -Wno-int-conversion -Wno-incompatible-pointer-types -Wno-return-type -Wunused-value jojo.c -o jojo
