#!/bin/sh

#cc=gcc
cc=clang
w='-Wno-int-conversion -Wno-incompatible-pointer-types -Wno-return-type -Wunused-value'
o='-O2'
f='-rdynamic'
l='-ldl'
d='-g'

build() {
    $cc -shared -fpic socket.c -o socket.so
}

default() {
    time build
}

if [ $# -eq 0 ]; then
    default
else
    $1
fi
