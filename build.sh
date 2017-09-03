#!/bin/sh

#cc=gcc
cc=clang
w='-Wno-int-conversion -Wno-incompatible-pointer-types -Wno-return-type -Wunused-value'
o=''
f='-rdynamic'
l='-ldl'

tangle() {
    echo tangle
    ./tool/tangle.js
    xxd -i core.jo > core.h
}

build() {
    echo build
    $cc $w $o $f $l jojo.c -o jojo
}

default() {
    tangle
    build
}


if [ $# -eq 0 ]; then
    default
else
    $1
fi
