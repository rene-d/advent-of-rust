#!/usr/bin/env bash
set -euo pipefail

input=${1:-input.txt}

clean() {
    find . -name 'frame*.png' | xargs -r -n100 rm
    rm -f anim.mgk
}

anim() {
    local part=$1
    echo "-delay 20 -loop 0" > anim.mgk
    find . -name "frame_${part}_*.png" | sort >> anim.mgk
    echo "-write anim${part}.gif" >> anim.mgk
    magick -script anim.mgk
}

trap clean EXIT

cargo run --release --features anim -- $input

anim 1
anim 2