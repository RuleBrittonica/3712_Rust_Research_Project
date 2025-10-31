#!/bin/bash

set -euo pipefail
# delete the testdata directory
rm -rf ./testdata
# re-clone the testdata
./scripts/clone_tests.sh

cargo clean

# Clean the cache (will require re-downloading dependencies)
cargo cache -a

# Update the dependencies
cargo update

# Now we need to build the project
cargo lcheck && cargo build --release --bin rem-extract

# Now we need to run the project
cargo run --release --bin rem-extract test -v