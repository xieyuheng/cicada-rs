#! /bin/bash

test ()
{
    ./tangle.js
    node cicada-in-js.js
}

test
