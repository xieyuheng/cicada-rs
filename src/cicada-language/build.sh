#! /bin/bash

test ()
{
    ./tangle.js
    time ./cs cicada-language.cs
}

test
