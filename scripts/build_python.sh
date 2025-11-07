#!/usr/bin/env bash

set -euo pipefail

script_dir=$(realpath $(dirname $0))

mkdir -p /opt/python

prerequesites()
{
    case $(uname -s) in
        Darwin)
            # TODO
            brew install --quiet tcl-tk
            ;;
        Linux)
            if [[ ! -f /.dockerenv ]] ; then
                return
            fi

            if [[ -f /etc/os-release ]] ; then
                source /etc/os-release
                case $ID in
                    debian|ubuntu)
                        apt-get install -y curl gcc libssl-dev liblzma-dev libreadline-dev libncursesw5-dev libsqlite3-dev tk-dev libgdbm-dev libgdbm-compat-dev libbz2-dev libffi-dev zlib1g-dev libzstd-dev
                        ;;
                    fedora)
                        dnf install -y curl gcc openssl-devel libuuid-devel sqlite-devel ncurses-devel bzip2-devel gdbm-devel libffi-devel readline-devel tk-devel libzstd-devel mpdecimal-devel
                        ;;
                    *)
                        echo "Unknown Linux distro $ID"
                        exit 1
                        ;;
                esac
            else
                echo "Unknown Linux distro"
                exit 1
            fi
            ;;
        *)
            echo "Unknown system: $(uname -s)"
            exit 1
            ;;
    esac

}

i()
{
    local v=$1
    local b=$(echo $v | cut -f1 -d'a')      # major.minor.micro
    local m=$(echo $v | cut -f1-2 -d'.')    # major.minor
    local url=https://www.python.org/ftp/python/$b/Python-$v.tar.xz

    prerequesites

    rm -rf /tmp/Python-$v /opt/python/Python-$v
    curl -sL $url | tar -C /tmp -xJ
    cd /tmp/Python-$v

    ./configure --prefix=/opt/python/Python-$(sed -E '/\bPY_VERSION\b/s/.*"([0-9.a-z]+)".*/\1/p;d' Include/patchlevel.h) --enable-optimizations
    echo
    echo "********************************"
    cat config.log | grep "^py_cv_module_" | grep -Ev "=yes$"
    echo "********************************"
    echo
    ! cat config.log | grep "^py_cv_module_" | grep -Ev "=(yes|n/a)$" | grep -q ^

    make -j$(nproc --ignore=1)
    make altinstall

    cd /tmp
    rm -rf /tmp/Python-$v
}

a()
{
    # i 3.10.18
    # i 3.11.14
    i 3.12.12
    i 3.13.9
    i 3.14.0
    # i 3.15.0a1
}

if [ ${1-} ]; then
    i $1
else
    a
fi
