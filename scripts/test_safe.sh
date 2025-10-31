#!/bin/bash

cargo clean
cargo lcheck --release --bin rem-extract && cargo run --release --bin rem-extract test -v