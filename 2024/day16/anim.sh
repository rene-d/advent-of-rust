#!/usr/bin/env bash
set -euo pipefail

input=${1:-input.txt}
anim=${2:-anim.gif}

find . -name 'frame*.png' | xargs -r -n100 rm

cargo run --release --features anim -- $input

echo "-delay 20 -loop 0" > anim.mgk
find . -name 'frame*.png' | sort >> anim.mgk
echo "-write $anim" >> anim.mgk

magick -script anim.mgk

find . -name 'frame*.png' | xargs -r -n100 rm
rm anim.mgk
