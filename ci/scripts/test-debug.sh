#!/bin/bash

if [ $TARGET == "x86_64-unknown-linux-musl" ]
then
  ./test-musl-debug.sh
else
  cargo test --target ${TARGET} --verbose
fi