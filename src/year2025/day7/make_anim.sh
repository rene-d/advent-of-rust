#!/usr/bin/env bash
set -euo pipefail

asciinema rec --overwrite part1.cast --window-size "20x20" --command "python3 anim.py 1"
asciinema rec --overwrite part2.cast --window-size "20x20" --command "python3 anim.py 2"

asciinema convert --overwrite -f asciicast-v2 part1.cast part1-v2.cast
asciinema convert --overwrite -f asciicast-v2 part2.cast part2-v2.cast

npm install -g svg-term-cli

svg-term --window  --width 20 --height 20 --no-cursor --in part1-v2.cast --out part1.svg
svg-term --window  --width 20 --height 20 --no-cursor --in part2-v2.cast --out part2.svg

rm *.cast