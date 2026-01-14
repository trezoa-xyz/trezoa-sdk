#!/usr/bin/env bash

set -eo pipefail
here="$(dirname "$0")"
src_root="$(readlink -f "${here}/..")"
cd "${src_root}"

rm -rf ./target/downstream/trezoa
mkdir -p ./target/downstream
cd ./target/downstream
git clone --depth 1 https://github.com/trezoa-xyz/trezoa.git --single-branch --branch=master
cd ./trezoa

../../../scripts/patch-crates-no-header.sh . ../../..
cargo check
