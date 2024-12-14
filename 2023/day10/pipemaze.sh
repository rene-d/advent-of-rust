#!/usr/bin/env bash
set -euo pipefail

for i in 1 2 3 4 5 6 ; do
    ./day10.py --png --scale 15 --output test$i.png test$i.txt
done
