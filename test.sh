#!/bin/bash

# Verify if cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
  echo "Cargo or Rust is not installed"
  exit 1
fi

cat << EOF >> /tmp/test_config.toml
name = "Test"
class_name = "Test_Class"
width = "200.0"
height = "Max"
x = 0.0
y = 0.0
transparent = true
blur = true
always_on_top = true
EOF

cargo test
