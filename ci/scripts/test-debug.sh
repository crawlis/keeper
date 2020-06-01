#!/bin/bash

if [ $TARGET == "x86_64-unknown-linux-musl" ]
then
  .ci/scripts/test-musl-debug.sh "$1"
else
  cargo test --target ${TARGET} --verbose
fi