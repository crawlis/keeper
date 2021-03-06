#!/bin/bash

set -euo pipefail

if [ $TARGET == "x86_64-unknown-linux-musl" ]
then
  ./ci/scripts/build-musl-debug.sh "$1"
else
  cargo build --target ${TARGET} --verbose
fi