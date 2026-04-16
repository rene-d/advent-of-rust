set shell := ["bash", "-uc"]

alias r := run

# Run stuff
# ---------

# Run all Rust solutions using personal puzzle inputs
run:
    ./scripts/runall.py --working-dir {{ invocation_directory() }} -lrust --me

# Run all Rust solutions using personal puzzle inputs over 3 iterations for timing accuracy
rrun:
    ./scripts/runall.py --working-dir {{ invocation_directory() }} -lrust --me -q -s -L3
    ./scripts/answers.py --readme -w

# Run Rust binary with timing database recording enabled
t *ARGS:
    cargo run --release --quiet -F timingsdb -- {{ ARGS }}

# Run all years in parallel tmux windows
run-all *ARGS:
    tmux new-session -d -s aoc
    for year in 20* ; do tmux new-window -t aoc:$year "./scripts/runall.py $year {{ ARGS }}" ; done
    tmux kill-window -t aoc:0
    tmux list-windows -t aoc

# Run solutions across multiple Python versions (3.11–3.14)
python *ARGS:
    ./scripts/runall.py -l py3.14 -l py3.13 -l py3.12 -l py3.11 {{ ARGS }}

# Make stuff
# ----------

# Remove all untracked files from the working tree
clean:
    git clean -fd

# Build all crates in debug mode
make-debug:
    @for path in crates/*/Cargo.toml ; do cargo build --manifest-path $path --quiet ; done
    cargo build --manifest-path ./Cargo.toml --quiet

# Build all crates in release mode
make:
    @for path in crates/*/Cargo.toml ; do cargo build --manifest-path $path --quiet --release ; done
    cargo build --manifest-path ./Cargo.toml --quiet --release

# GitHub stuff
# ------------

# Check Rust formatting across all crates (CI)
gh-fmt:
    #!/usr/bin/env bash
    for manifest in `find . -maxdepth 3 ! -path '*/day*' -name Cargo.toml`
    do
        printf '\n\033[1;34m━━━ fmt: %s ━━━\033[0m\n' "$manifest"
        cargo fmt --all --manifest-path $manifest -- --check
    done

# Run Clippy lints across all crates (CI)
gh-clippy:
    #!/usr/bin/env bash
    for manifest in `find . -maxdepth 3 ! -path '*/day*' -name Cargo.toml`
    do
        printf '\n\033[1;33m━━━ clippy: %s ━━━\033[0m\n' "$manifest"
        cargo clippy --manifest-path $manifest -- --no-deps -D clippy::all -D clippy::pedantic -F clippy::nursery
    done

# Run tests across all crates (CI)
gh-test:
    #!/usr/bin/env bash
    for manifest in `find . -maxdepth 3 ! -path '*/day*' -name Cargo.toml`
    do
        printf '\n\033[1;32m━━━ test: %s ━━━\033[0m\n' "$manifest"
        cargo test --quiet --manifest-path $manifest -- --test-threads 4
    done

# Run all CI checks: fmt, clippy, and tests
gh: gh-fmt gh-clippy gh-test

# Alias for gh: run all CI checks
github: gh-fmt gh-clippy gh-test

# Docker stuff
# ------------

# Build the Debian-based Docker image
docker-debian:
    docker build -f scripts/Dockerfile-debian -t aoc-debian scripts/

# Build the Fedora-based Docker image
docker-fedora:
    docker build -f scripts/Dockerfile-fedora -t aoc-fedora scripts/

# Build the Alpine-based Docker image
docker-alpine:
    docker build -f scripts/Dockerfile-alpine -t aoc-alpine scripts/

# Build the Debian image and open an interactive shell inside the container
debian: docker-debian
    docker run --rm -ti -v $PWD:/aoc -w /aoc aoc-debian

# Build the Fedora image and open an interactive shell inside the container
fedora: docker-fedora
    docker run --rm -ti -v $PWD:/aoc -w /aoc aoc-fedora

# Build the Alpine image and open an interactive shell inside the container
alpine: docker-alpine
    docker run --rm -ti -v $PWD:/aoc -w /aoc aoc-alpine

# Build the Advent of Rust runner image (aor target)
docker-aor:
    docker build -t aor --target aor -f scripts/Dockerfile .

# Build the Advent of Code solver image (aoc target)
docker-aoc:
    docker build -t aoc --target aoc -f scripts/Dockerfile .

# Build all Docker images and export them as tar archives
docker-images: docker-debian docker-fedora docker-alpine docker-aor docker-aoc
    docker image save aoc-debian -o aoc-debian.tar
    docker image save aoc-fedora -o aoc-fedora.tar
    docker image save aoc-alpine -o aoc-alpine.tar
    docker image save aor -o aor.tar
    docker image save aoc -o aoc.tar
