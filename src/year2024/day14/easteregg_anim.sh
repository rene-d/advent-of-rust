#!/usr/bin/env bash

set -euo pipefail

i=0
declare -a frames=()
for input in ../../data/*/2024/14.in ; do
    i=$((i+1))

    ./easteregg.py $input

    mv christmastree.png frame$i.png
    frames+=(frame$i.png)
done

magick -delay 60 -loop 0 ${frames[*]} christmastree.gif
rm ${frames[*]}