#!/bin/bash

# A script to run compile tests with the same condition of the checks done by CI.
#
# Usage
#
# ```sh
# . ./compiletest.sh
# ```

TRYBUILD=overwrite RUSTFLAGS='--cfg auto_enums_def_site_enum_ident' cargo +nightly test -p auto_enums --all-features --test compiletest
