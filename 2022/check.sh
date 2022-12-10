#!/usr/bin/env bash

set -euo pipefail

cargo fmt
cargo clippy
cargo test
cargo build --release
for i in */Cargo.toml ; do
    day=$(dirname $i)
    cargo run --offline --release --bin $day -- $day/input.txt
done
