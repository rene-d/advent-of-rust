set shell := ["bash", "-uc"]

alias r := run

run:
    ./scripts/runall.py --working-dir {{invocation_directory()}} -lrust --verified

# make

clean:
    git clean -fd

make-debug:
    @for path in crates/*/Cargo.toml ; do cargo build --manifest-path $path --quiet ; done
    cargo build --manifest-path ./Cargo.toml --quiet

make:
    @for path in crates/*/Cargo.toml ; do cargo build --manifest-path $path --quiet --release ; done
    cargo build --manifest-path ./Cargo.toml --quiet --release

# exec

timings *ARGS:
    ./scripts/timings.py {{ ARGS }}

run-all *ARGS:
    tmux new-session -d -s aoc
    for year in 20* ; do tmux new-window -t aoc:$year "./scripts/runall.py $year {{ ARGS }}" ; done
    tmux kill-window -t aoc:0
    tmux list-windows -t aoc

python *ARGS:
    ./scripts/runall.py -l py3.14 -l py3.13 -l py3.12 -l py3.11 {{ ARGS }}

# GitHub

gh-fmt:
    #!/usr/bin/env bash
    for manifest in `find . -maxdepth 3 ! -path '*/day*' -name Cargo.toml`
    do
        echo check fmt for $manifest
        cargo fmt --all --manifest-path $manifest -- --check
    done

gh-clippy:
    #!/usr/bin/env bash
    for manifest in `find . -maxdepth 3 ! -path '*/day*' -name Cargo.toml`
    do
        echo check clippy for $manifest
        cargo clippy --manifest-path $manifest -- --no-deps -D clippy::all -D clippy::pedantic -F clippy::nursery
    done

gh-test:
    #!/usr/bin/env bash
    for manifest in `find . -maxdepth 3 ! -path '*/day*' -name Cargo.toml`
    do
        echo test for $manifest
        cargo test --manifest-path $manifest -- --test-threads 4
    done

gh: gh-fmt gh-clippy gh-test

# Docker stuff

debian:
    docker build -f scripts/Dockerfile-debian -t aoc-debian scripts/
    docker run --rm -ti -v $PWD:/aoc -w /aoc aoc-debian

fedora:
    docker build -f scripts/Dockerfile-fedora -t aoc-fedora scripts/
    docker run --rm -ti -v $PWD:/aoc -w /aoc aoc-fedora

alpine:
    docker build -f scripts/Dockerfile-alpine -t aoc-alpine scripts/
    docker run --rm -ti -v $PWD:/aoc -w /aoc aoc-alpine

docker-aor:
    docker build -t aor --target aor -f scripts/Dockerfile .

docker-aoc:
    docker build -t aoc --target aoc -f scripts/Dockerfile .