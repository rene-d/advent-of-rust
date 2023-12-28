#!/bin/sh
set -e
cargo fmt
cargo clippy -- -W clippy::pedantic
cargo doc --open --workspace --no-deps --bins --lib
