#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <crate-path>"
    exit 1
fi

CRATE_DIR="$1"

if [ ! -f "$CRATE_DIR/Cargo.toml" ]; then
    echo "Error: $CRATE_DIR does not contain a Cargo.toml (not a crate root)."
    exit 1
fi

echo "Entering crate"
cd "$CRATE_DIR"

echo "== Running Charon with cargo"
# This will produce LLBC files in target/charon/ or target/charon-debug/
charon cargo --preset=aeneas

echo "Searching for LLBC files"
LLBC_FILES=$(find . -maxdepth 1 -type f -name '*.llbc')

if [ -z "$LLBC_FILES" ]; then
    echo "Error: No .llbc files produced by Charon."
    exit 1
fi

echo "Found:"
echo "$LLBC_FILES"
echo

echo "Running Aeneas on each LLBC file"
for f in $LLBC_FILES; do
    echo "--> Processing $f"
    aeneas -backend coq "$f"
done

echo
echo "Generated Coq files"
find . -name '*.v'
