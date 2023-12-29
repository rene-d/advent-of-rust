#!/usr/bin/env bash

set -euo pipefail

script_dir=$(realpath $(dirname $0))
opt_answers=

usage()
{
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  -a,--answers      check answers too"
    exit
}

parse_args()
{
    for i ; do
        case $i in
            -h|--help) usage ;;
            -a|--answers) opt_answers=1 ;;
            *) usage ;;
        esac
    done
}

c()
{
   local manifest=$1

   cargo fmt --all --manifest-path $manifest -- --check
   cargo clippy --manifest-path $manifest -- --no-deps -W clippy::all

   cargo build --manifest-path $manifest --release
   cargo test --manifest-path $manifest --quiet
}

main()
{
   if [[ ! -f Cargo.toml ]] ; then
      for manifest in `find . -maxdepth 2 -name Cargo.toml` ; do
         c $manifest
      done
   else
      c Cargo.toml
   fi

   if [[ $opt_answers ]]; then
      $script_dir/answers.py
   fi
}

parse_args "$@"
main
