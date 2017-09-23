#!/bin/sh

default() {
    cd src
    ./build.sh
    cp jojo ..
    cd ..
}

fast() {
    cd src
    ./build.sh fast
    cp jojo ..
    cd ..
}

if [ $# -eq 0 ]; then
    default
else
    $1
fi
