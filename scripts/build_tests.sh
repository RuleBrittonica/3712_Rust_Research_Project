#!/bin/bash

# First we need to remove the old test files if they exist
rm -rf ./input
rm -rf ./output
rm -rf ./correct_output

# Fixup the Cargo.toml file
python ./scripts/fixup_cargotoml.py

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
