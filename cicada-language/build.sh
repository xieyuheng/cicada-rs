#! /bin/bash

test ()
{
    ./tangle.js
    time cicada-script cicada-language.cs
}

test
