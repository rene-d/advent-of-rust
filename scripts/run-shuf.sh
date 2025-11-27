#!/usr/bin/env bash
set -euo pipefail

#
# aoc run --no-build -q > before

function gen1()
{
    for l in rust ; do
        for y in {2015..2024} ; do
            for d in {1..25..5} ; do
                echo aoc run --no-build -l $l -r $y $d $(($d+1)) $(($d+2)) $(($d+3)) $(($d+4))
            done
        done
    done
}

function gen2()
{
    for l in python py3.11 py3.14 py3.14t ; do
        for y in {2015..2024} ; do
            for d in {1..25..5} ; do
                echo aoc run --no-build -l $l -r $y $d $(($d+1)) $(($d+2)) $(($d+3)) $(($d+4))
            done
        done
    done
}

function gen3()
{
    echo aoc run --no-build -lgo -r
    echo aoc run --no-build -ljava -lnodejs -llua -ltcl -l'c#' -r
    echo aoc run --no-build -lc -lc++ -lperl -lruby -lswift  -r
}

function gen()
{
    for i in $(echo "$*" | grep -o .); do
        [[ $i == r ]] && gen1
        [[ $i == p ]] && gen2
        [[ $i == o ]] && gen3
    done
    return 0
}

if [[ $# -eq 0 ]] ; then
    echo "Usage: $0 RSrpo*"
    exit
fi

if [[ "$*" == *B* ]] ; then
    aoc run --me -n -q
fi

gen $* | (
    if [[ "$*" == *S* ]] ; then
        shuf
    else
        cat
    fi
) | (
    if [[ "$*" == *R* ]] ; then
        sh
    else
        cat
    fi
)

if [[ "$*" == *T* ]] ; then
    prefix=${prefix:-stats}
    export CLICOLOR_FORCE=1
    aoc run --no-build -n -q --me | tee ${prefix}-me.txt
    aoc run --no-build -n -q -l rust | tee ${prefix}-rust.txt
    aoc run --no-build -n -q -l py3.11 | tee ${prefix}-py3.11.txt
    aoc run --no-build -n -q -l py3.14 | tee ${prefix}-py3.14.txt
    aoc run --no-build -n -q -l py3.14t | tee ${prefix}-py3.14t.txt
fi
