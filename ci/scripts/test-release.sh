#!/bin/bash

if [ $TARGET == "x86_64-unknown-linux-musl" ]
then
  ./test-musl-release.sh
else
  cargo test --target ${TARGET} --verbose --release
fi