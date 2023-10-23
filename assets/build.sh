#!/bin/sh

set -e
cd "$(dirname "$0")/.."

cargo ledger build nanos
cargo ledger build nanosplus
cargo ledger build nanox

cp -vf target/nanos/release/app.hex assets/app_nanos.hex
cp -vf target/nanosplus/release/app.hex assets/app_nanosplus.hex
cp -vf target/nanox/release/app.hex assets/app_nanox.hex
