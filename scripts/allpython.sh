#!/usr/bin/env bash

set -euo pipefail

script_dir=$(realpath $(dirname $0))

mkdir -p /opt/python

i()
{
    local v=$1

    local b=$(echo $v | cut -f1 -d'a')      # major.minor.micro
    local m=$(echo $v | cut -f1-2 -d'.')    # major.minor
    local url=https://www.python.org/ftp/python/$b/Python-$v.tar.xz

    rm -rf /tmp/Python-$v /opt/python/Python-$v
    curl -sL $url | tar -C /tmp -xJ
    cd /tmp/Python-$v
    ./configure --prefix=/opt/python/Python-$v --enable-optimizations
    cat config.log | grep "^py_cv_module_" | grep -Ev "=(yes|n/a)$" | ! grep -q ^
    make -j$(nproc --ignore=1)
    make altinstall
    # /opt/python/Python-$v/bin/python$m -mensurepip
    cd /tmp
    rm -rf /tmp/Python-$v

    if [ -f $script_dir/runall.py ] ; then
        $script_dir/runall.py --venv /opt/python/Python-$v/bin/python$m
    fi
}

a()
{
    i 3.10.16
    i 3.11.11
    i 3.12.8
    i 3.13.1
}

if [ ${1-} ]; then
    i $1
else
    a
fi
