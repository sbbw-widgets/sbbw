#!/bin/bash

# Verify if cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
  echo "Cargo or Rust is not installed"
  exit 1
fi

# install dependencies with cargo in sbbw-daemon
cargo build --release
