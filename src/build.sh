#! /bin/bash

test ()
{
    ./tangle.js
    node nymph.js
}

test
