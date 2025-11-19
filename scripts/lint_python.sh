#!/bin/sh

set -eu

isort -l120 --profile black .
black -l120 .
ruff check . ${*-}
# flake8 --max-line-length 120
