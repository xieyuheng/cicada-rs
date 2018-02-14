#! /bin/bash

test ()
{
    ./tangle.js
    node cicada.js
}

test
