#! /bin/bash

test ()
{
    tangle .
    time cicada-script cicada-language.cs
}

test
