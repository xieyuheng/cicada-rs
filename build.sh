#!/bin/sh

#cc=gcc
cc=clang
w='-Wno-int-conversion -Wno-incompatible-pointer-types -Wno-return-type -Wunused-value'
o='-O2'
f='-rdynamic'
l='-ldl'


copy() {
    rsync --recursive --links --perms --times --group --owner --devices --specials --verbose --human-readable $@
}

tangle() {
    ./tool/tangle.js
    xxd -i core.jo > core.h
}

build() {
    $cc $w $o $f $l jojo.c -o jojo
}

fast_build() {
    $cc $w $f $l jojo.c -o jojo
}

clean() {
    rm -f jojo
    rm -f core.jojo.*
}

run() {
    ./jojo
}

default() {
    clean
    tangle
    time build
}

install_modules() {
    copy ./modules ~/.jojo/
}

test() {
    clean
    tangle
    time fast_build
    install_modules
    run
}

if [ $# -eq 0 ]; then
    default
else
    $1
fi
