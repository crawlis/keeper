#!/bin/bash

if [ $TARGET == "x86_64-unknown-linux-musl" ]; then
  ./build_openssl_musl.sh
fi