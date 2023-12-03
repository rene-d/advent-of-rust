#!/usr/bin/env bash

cargo fmt
cargo clippy -- -A clippy::pedantic -A clippy::all
