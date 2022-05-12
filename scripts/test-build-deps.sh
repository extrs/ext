#!/usr/bin/env bash
set -eu

cargo install --debug --path .

cd $1 && cargo ext build-deps