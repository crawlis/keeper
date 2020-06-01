#!/bin/bash

set -euo pipefail

if [ $TARGET == "x86_64-unknown-linux-musl" ]
then
  ./build-musl-debug.sh
else
  cargo build --target ${TARGET} --verbose
fi