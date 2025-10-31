#!/bin/bash

cargo clean
cargo lcheck && cargo run --release --bin rem-extract test