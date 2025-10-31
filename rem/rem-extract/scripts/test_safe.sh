#!/bin/bash

cargo clean
cargo check --release --bin rem-extract && cargo run --release --bin rem-extract test -v