#!/usr/bin/env bash
set -euo pipefail

# Build the binary
cargo update
cargo build --release --bin rem-extract

# Variables
FILE_PATH="examples/example1/src/main.rs"
NEW_FN_NAME="extracted_function"
START_INDEX=39
END_INDEX=60

echo "Running: rem-extract extract -m \"$FILE_PATH\" \"$NEW_FN_NAME\" $START_INDEX $END_INDEX"

# Run the binary
./../target/release/rem-extract \
  extract -m \
  "$FILE_PATH" "$NEW_FN_NAME" "$START_INDEX" "$END_INDEX"
