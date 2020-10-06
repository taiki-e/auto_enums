#!/bin/bash

# A script to run a simplified version of the checks done by CI.
#
# Usage:
#     bash scripts/ci.sh
#
# Note: This script requires nightly Rust, rustfmt, clippy, and cargo-expand

set -euo pipefail

if [[ "${1:-none}" == "+"* ]]; then
    toolchain="${1}"
else
    toolchain="+nightly"
fi

echo "Running 'cargo ${toolchain} fmt --all'"
cargo "${toolchain}" fmt --all

echo "Running 'cargo ${toolchain} clippy --all --all-features --all-targets'"
cargo "${toolchain}" clippy --all --all-features --all-targets -Zunstable-options

echo "Running 'cargo ${toolchain} test --all --all-features'"
TRYBUILD=overwrite cargo "${toolchain}" test --all --all-features

echo "Running 'cargo ${toolchain} doc --no-deps --all --all-features'"
cargo "${toolchain}" doc --no-deps --all --all-features
