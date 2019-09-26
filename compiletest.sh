#!/bin/bash

# A script to run compile tests with the same condition of the checks done by CI.
#
# Usage
#
# ```sh
# . ./compiletest.sh
# ```

TRYBUILD=overwrite RUSTFLAGS='--cfg compiletest' cargo +nightly test -p auto_enums --all-features --test compiletest
# RUSTFLAGS='--cfg compiletest' cargo +nightly test -p auto_enums --all-features --test compiletest
