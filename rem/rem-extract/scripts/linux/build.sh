#!/bin/bash

# Update the dependencies
cargo update

cargo lcheck && cargo build --release --bin rem-extract