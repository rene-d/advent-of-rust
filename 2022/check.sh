#!/usr/bin/env bash

set -euo pipefail

cargo fmt
cargo clippy
cargo test
cargo build --release
days=($(for i in */Cargo.toml ; do echo $(dirname $i) | cut -b4- ; done | sort -n))
echo ${days[@]} | xargs -n1 -I % cargo run --offline --release --bin day% -- day%/input.txt

