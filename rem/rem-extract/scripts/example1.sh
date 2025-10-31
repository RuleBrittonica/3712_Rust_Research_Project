#!/bin/bash

# Build the binary
cargo update
cargo build --release --bin rem-extract

# Variables
PATH="C:\Coding\rem-matt\rem-extract\examples\example1\src\main.rs"
NAME="extracted_function"
START=39
END=60

# Run the binary
./target/release/rem-extract extract $PATH $NAME $START $END