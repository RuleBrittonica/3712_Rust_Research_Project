#!/bin/bash

# First Remove all of the old files and directories
rm -rf ./input
rm -rf ./output
rm -rf ./correct_output

python ./scripts/fixup_cargotoml.py
rm -f ./0_test_info.csv
rm -f ./rem-extract/src/test_details.rs

cargo clean

# Clean the cache (will require re-downloading dependencies)
cargo cache -a

# Update the dependencies
cargo update

# Then make the starting dirs
mkdir -p ./input
mkdir -p ./output
mkdir -p ./correct_output

# Now we need to generate the input files, using the python scripts
python ./1_make_rust_toolchaintoml.py
python ./2_extract_tests.py
python ./3_make_test_details_rs.py
python ./4_convert_to_project.py
python ./5_fixup_semicolons.py

# Now we need to build the project
cargo lcheck && cargo build --release --bin rem-extract

# Now we need to run the project
cargo run --release --bin rem-extract test -v