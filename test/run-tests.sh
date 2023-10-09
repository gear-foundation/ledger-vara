#!/bin/bash

set -e
cd "$(dirname "$0")"

echo Get Version
ledgerctl send vara_get_version.hex
echo

echo Get Public Key
ledgerctl send vara_get_pub_key.hex
echo

echo Sign Message
ledgerctl send vara_sign.hex
