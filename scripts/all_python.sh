#!/usr/bin/env bash
set -euo pipefail

if ! command -v uv >/dev/null ; then
    echo "This script requires « uv ». You can install it with:"
    echo "curl -LsSf https://astral.sh/uv/install.sh | sh"
    exit
fi

scripts_dir=$(realpath $(dirname $0))

versions=(
    3.11
    3.12
    3.13
    3.14
)

function inst_bin()
{
    if ! command -v $1 >/dev/null ; then
        echo "$1 not found."
    elif [[ $($1 -c "import sys;print(sys.hexversion>=0x30A0000)" 2>/dev/null) != "True" ]] ; then
        echo "$($1 -V) is too old."
    else
        venv=${AOC_TARGET_DIR:-target}/venv/python
        uv venv -p $1 --clear $venv
        VIRTUAL_ENV=$venv uv pip install -r $scripts_dir/requirements.txt
    fi
}

function inst_py()
{
    local version=$1
    if [[ $version == system ]] ; then
        inst_bin /usr/bin/python3
    elif [[ ! $version =~ 3\..* ]] ; then
        inst_bin $version
    else
        local venv=${AOC_TARGET_DIR:-target}/venv/py$version
        uv python install $version
        uv venv -p $version --managed-python --clear $venv
        VIRTUAL_ENV=$venv uv pip install -r $scripts_dir/requirements.txt
    fi
}

if [[ $# == 0 ]] ; then
    set -- ${versions[*]}
fi

for version ; do
    inst_py $version
done
