#!/bin/sh

rm -f grid*.ppm

cargo run --release --quiet -F ascii

# sips -s format png grid.ppm --out virus.png
ffmpeg -y -i grid_%05d.ppm -vf "fps=10,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" virus.gif
rm grid*.ppm