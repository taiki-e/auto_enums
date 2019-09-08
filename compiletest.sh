#!/bin/bash

# A script to run compile tests with the same condition of the checks done by CI.
#
# Usage
#
# ```sh
# . ./compiletest.sh
# ```

rm -rf target/debug/deps/libauto_enums* && RUSTFLAGS='--cfg compiletest' cargo test -p auto_enums --all-features --test compiletest
