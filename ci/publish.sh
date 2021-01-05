#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

cd "$(cd "$(dirname "${0}")" && pwd)"/..

set -x

(
  cd core
  cargo publish
)
(
  cd derive
  cargo publish
)

sleep 30
cargo publish
