#! /bin/bash

test ()
{
    ./tangle.js
    node cicada-script.js
}

test
