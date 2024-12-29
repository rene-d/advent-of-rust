set shell := ["bash", "-c"]

alias r := run

# export RUSTFLAGS := "-C target-cpu=native"

run:
    ./scripts/runall.py --working-dir {{invocation_directory()}} -lrust --verified

clean:
    git clean -fd

make-debug:
    cargo build --manifest-path aoc/Cargo.toml --quiet
    cargo build --manifest-path 2015/Cargo.toml --quiet
    cargo build --manifest-path 2016/Cargo.toml --quiet
    cargo build --manifest-path 2017/Cargo.toml --quiet
    cargo build --manifest-path 2018/Cargo.toml --quiet
    cargo build --manifest-path 2019/Cargo.toml --quiet
    cargo build --manifest-path 2020/Cargo.toml --quiet
    cargo build --manifest-path 2021/Cargo.toml --quiet
    cargo build --manifest-path 2022/Cargo.toml --quiet
    cargo build --manifest-path 2023/Cargo.toml --quiet
    cargo build --manifest-path 2024/Cargo.toml --quiet


make:
    cargo build --manifest-path aoc/Cargo.toml --release --quiet
    cargo build --manifest-path 2015/Cargo.toml --release --quiet
    cargo build --manifest-path 2016/Cargo.toml --release --quiet
    cargo build --manifest-path 2017/Cargo.toml --release --quiet
    cargo build --manifest-path 2018/Cargo.toml --release --quiet
    cargo build --manifest-path 2019/Cargo.toml --release --quiet
    cargo build --manifest-path 2020/Cargo.toml --release --quiet
    cargo build --manifest-path 2021/Cargo.toml --release --quiet
    cargo build --manifest-path 2022/Cargo.toml --release --quiet
    cargo build --manifest-path 2023/Cargo.toml --release --quiet
    cargo build --manifest-path 2024/Cargo.toml --release --quiet
