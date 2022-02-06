#!/bin/sh
set -e
cargo fmt
cargo clippy -- -D clippy::pedantic
cargo doc --open --workspace --no-deps --bins --lib
