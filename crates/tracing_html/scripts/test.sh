#!/usr/bin/env bash
set -eu

(cd ../../apps/tracing-html-viewer && npx webpack)

cargo test $@