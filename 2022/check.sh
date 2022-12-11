#!/usr/bin/env bash

set -euo pipefail

cargo fmt
cargo clippy
cargo test
cargo build --release

days=($(find . -mindepth 2 -maxdepth 2 -name Cargo.toml | xargs dirname | xargs basename | cut -b4- | sort -n))
if [[ ${#days[@]} == 0 ]] ; then
   if [[ -f input.txt ]] ; then cargo run --offline --release -- input.txt ; fi
else
   echo ${days[@]} | xargs -n1 -I % cargo run --offline --release --bin day% -- day%/input.txt
fi
