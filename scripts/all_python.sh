#!/usr/bin/env bash
set -euo pipefail

scripts_dir=$(realpath $(dirname $0))

versions=(
    3.10
    3.11
    3.12
    3.13
    # 3.13t
    3.14
    # 3.14t
)

for v in ${versions[*]}; do
    uv python install $v
    uv venv -p $v --managed-python --clear .venv/py$v
    VIRTUAL_ENV=.venv/py$v uv pip install -r $scripts_dir/requirements.txt
done
