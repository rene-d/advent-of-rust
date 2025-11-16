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

function inst_py()
{
    local version=$1
    if [[ $version == "system" ]] ; then
        if command -v /usr/bin/python3 >/dev/null \
            && [[ $(/usr/bin/python3 -c "import sys;print(sys.hexversion>=0x30A0000)") == "True" ]]
        then
            venv=${AOC_TARGET_DIR:-target}/venv/python
            uv venv -p /usr/bin/python3 --clear $venv
            VIRTUAL_ENV=$venv uv pip install -r $scripts_dir/requirements.txt
        else
            echo "$(/usr/bin/python3 -V) is too old."
        fi
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
