#!/bin/bash

# Update the dependencies
cargo update

cargo lcheck --bin rem-extract && cargo build --release --bin rem-extract