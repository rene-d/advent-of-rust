#!/usr/bin/env bash

cargo fmt

pedantic()
{
    local manifest=$1
    local day=$2

    if [[ -f $manifest ]] ; then

        if ! cargo clippy --manifest-path $manifest -- -F clippy::pedantic -F clippy::all -F clippy::nursery 2>&1 | grep -qE '^error:|^warning:' ; then
            echo -e "Day $day is \033[1;35mEPIC PEDANTIC AND NURSERY\033[0m. Amen 🙏"

        elif ! cargo clippy --manifest-path $manifest -- -F clippy::pedantic 2>&1 | grep -qE '^error:|^warning:' ; then
            echo -e "Day $day is \033[1;35mEPIC PEDANTIC\033[0m. Roxxor 🦾"

        elif ! cargo clippy --manifest-path $manifest -- -D clippy::pedantic 2>&1 | grep -qE '^error:|^warning:' ; then
            echo -e "Day $day is \033[1;34mMAX PEDANTIC\033[0m. Congratz 🤓"

        elif ! cargo clippy --manifest-path $manifest -- -W clippy::pedantic 2>&1 | grep -qE '^error:|^warning:' ; then
            echo -e "Day $day is \033[94mVERY PEDANTIC\033[0m. Good game 👍"

        elif ! cargo clippy --manifest-path $manifest -- -A clippy::pedantic 2>&1 | grep -qE '^error:|^warning:' ; then
            echo -e "Day $day is \033[92mPEDANTIC\033[0m. Well done 😃"

        elif ! cargo clippy --manifest-path $manifest -- -A clippy::all 2>&1 | grep -qE '^error:|^warning:' ; then
            echo -e "Day $day is \033[1;37mjust good enough\033[0m 😐"

        else
            echo -e "Day $day is \033[1;31mWTF\033[0m 🤬. Go to https://www.rust-lang.org/learn."
        fi

    fi
}

if [[ -f Cargo.toml ]] && grep -q "[[bin]]" Cargo.toml ; then
    cargo clippy -- -W clippy::pedantic
else
    for day in {1..25} ; do
        pedantic day$day/Cargo.toml $day
    done
fi
