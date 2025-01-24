#!/usr/bin/env bash
# [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

filename=${1:-input.txt}

left=($(cat "$filename" | awk '{print $1}' | sort))
right=($(cat "$filename" | awk '{print $2}' | sort))

n=$(( ${#left[@]} - 1 ))
i=0
part1=0
for i in $(seq 0 $n) ; do
    l=${left[$i]}
    r=${right[$i]}

    d=$(($l - $r))
    if [[ $d -lt 0 ]] ; then
        part1=$(($part1 - $d))
    else
        part1=$(($part1 + $d))
    fi

    counter[$r]=$(( ${counter[$r]} + 1 ))
done
echo $part1

part2=0
for i in $(seq 0 $n) ; do
    l=${left[$i]}

    if [[ ${counter[$l]} ]] ; then
        for j in $(seq ${counter[$l]}) ; do
            part2=$(($part2 + $l  ))
        done
    fi
done
echo $part2
