#!/bin/sh

set -eu

isort -l120 --profile black .
black -l120 .
ruff check --ignore E501 . ${*-}
# flake8 --max-line-length 120
