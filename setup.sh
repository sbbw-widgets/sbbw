#!/bin/bash

# Verify if yarn is installed
if ! command -v yarn >/dev/null 2>&1; then
  echo "Yarn is not installed"
  exit 1
fi

# Verify if cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
  echo "Cargo is not installed"
  exit 1
fi

# install dependencies with yarn in sbbw-window
cd sbbw-window && yarn

# install dependencies with cargo in sbbw-daemon
cargo install --path ./sbbw-daemon
