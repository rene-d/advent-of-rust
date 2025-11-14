#!/usr/bin/env bash
set -euo pipefail

if ! command -v uv >/dev/null ; then
    echo "This script requires « uv ». You can install it with:"
    echo "curl -LsSf https://astral.sh/uv/install.sh | sh"
    exit
fi

scripts_dir=$(realpath $(dirname $0))

versions=(
    # 3.10
    3.11
    3.12
    3.13
    # 3.13t
    3.14
    # 3.14t
)

for v in ${versions[*]}; do
    venv=${AOC_TARGET_DIR:-target}/venv/py$v
    uv python install $v
    uv venv -p $v --managed-python --clear $venv
    VIRTUAL_ENV=$venv uv pip install -r $scripts_dir/requirements.txt
done

if command -v /usr/bin/python3 >/dev/null ; then
    venv=${AOC_TARGET_DIR:-target}/venv/python
    uv venv -p /usr/bin/python3 --clear $venv
    VIRTUAL_ENV=$venv uv pip install -r $scripts_dir/requirements.txt
fi
