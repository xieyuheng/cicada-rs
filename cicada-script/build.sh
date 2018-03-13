#! /bin/bash

test ()
{
    ./tangle.js
    time ./cicada-script
}

test
