#!/usr/bin/env bash

set -uo pipefail

for year in {2015..2021}; do
    for day in {1..25}; do
	if [[ -d $year/day$day ]] && ! [[ -f $year/day$day/input.txt ]]; then
        echo $year $day
        curl -s --cookie session=$1 -o ${year}/day${day}/input.txt https://adventofcode.com/${year}/day/${day}/input
	fi
    done
done