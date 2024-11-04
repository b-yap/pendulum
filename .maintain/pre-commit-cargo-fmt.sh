#!/bin/bash

### Run formatter on all files in the repository before commit

if [ -z "$1" ]; then
  nightVersion="nightly-2024-05-30"
else
  nightVersion=$1
fi

toolchains=($(rustup toolchain list))

if [[ " ${toolchains[@]} " =~ .*"$nightVersion".* ]]; then
  cargo +$nightVersion fmt --all
else
  echo "Rust toolchain $nightVersion is not installed. Please install it using:"
  echo "rustup toolchain install $nightVersion"
  exit 1
fi
