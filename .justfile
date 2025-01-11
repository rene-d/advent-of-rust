set shell := ["bash", "-uc"]

alias r := run

# export RUSTFLAGS := "-C target-cpu=native"

run:
    ./scripts/runall.py --working-dir {{invocation_directory()}} -lrust --verified

clean:
    git clean -fd

make-debug:
    cargo build --manifest-path aoc/Cargo.toml --quiet
    @for year in 20* ; do echo "cargo build $year" ; cargo build --manifest-path $year/Cargo.toml --quiet ; done

make:
    cargo build --manifest-path aoc/Cargo.toml --release --quiet
    @for year in 20* ; do echo "cargo build $year" ; cargo build --manifest-path $year/Cargo.toml --release --quiet ; done

timings *ARGS:
    ./scripts/timings.py {{ ARGS }}

run-all *ARGS:
    tmux new-session -d -s aoc
    for year in 20* ; do tmux new-window -t aoc:$year "./scripts/runall.py $year {{ ARGS }}" ; done
    tmux kill-window -t aoc:0
    tmux list-windows -t aoc

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
        cargo clippy --manifest-path $manifest -- --no-deps -D clippy::all -A clippy::pedantic  -A clippy::nursery
    done

gh-test:
    #!/usr/bin/env bash
    for manifest in `find . -maxdepth 3 ! -path '*/day*' -name Cargo.toml`
    do
        echo test for $manifest
        cargo test --manifest-path $manifest -- --test-threads 4
    done

gh: gh-fmt gh-clippy gh-test