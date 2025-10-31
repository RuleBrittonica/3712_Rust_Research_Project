#!/bin/bash

cargo clean

# Clean the cache (will require re-downloading dependencies)
cargo cache -a

# Update the dependencies
cargo update

cargo build --release --bin rem-extract