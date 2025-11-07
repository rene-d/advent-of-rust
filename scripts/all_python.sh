#!/usr/bin/env bash
set -euo pipefail

scripts_dir=$(realpath $(dirname $0))

for v in 3.11 3.12 3.13 3.14 ; do
    uv python install $v
    uv venv -p $v --clear .venv/py$v
    VIRTUAL_ENV=.venv/py$v uv pip install -r $scripts_dir/requirements.txt
done
