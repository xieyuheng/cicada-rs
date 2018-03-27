#! /bin/bash

test ()
{
    ./tangle.js
    time cicada-script microkanren.cs
}

test
