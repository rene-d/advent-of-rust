#!/usr/bin/env bash
set -euo pipefail

input=${1:-}
part=${2:-1}
anim=${3:-anim.mp4}

find . -name 'frame*.png' | xargs -r -n100 rm

MAKE_ANIM=$part cargo run --release -- $input

echo "-delay 10 -loop 0" > anim.mgk
find . -name 'frame*.png' | sort >> anim.mgk
echo "-write $anim" >> anim.mgk

magick -script anim.mgk

find . -name 'frame*.png' | xargs -r -n100 rm
rm anim.mgk
