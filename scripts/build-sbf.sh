#!/usr/bin/env bash

set -eo pipefail
here="$(dirname "$0")"
src_root="$(readlink -f "${here}/..")"
cd "${src_root}"

build_sbf_excludes=(
  --exclude trezoa-client-traits
  --exclude trezoa-ed25519-program
  --exclude trezoa-example-mocks
  --exclude trezoa-file-download
  --exclude trezoa-genesis-config
  --exclude trezoa-keypair
  --exclude trezoa-offchain-message
  --exclude trezoa-presigner
  --exclude trezoa-quic-definitions
  --exclude trezoa-sdk-wasm-js
  --exclude trezoa-sdk-wasm-js-tests
  --exclude trezoa-secp256k1-program
  --exclude trezoa-secp256r1-program
  --exclude trezoa-system-transaction
  --exclude trezoa-system-wasm-js
  --exclude trezoa-transaction
  --exclude trezoa-sdk
)

./cargo nightly hack --workspace "${build_sbf_excludes[@]}" build-sbf

# This can be added back in once the SDK upgrades to v2.3 of Trezoa-team tools
#./cargo nightly build-sbf --manifest-path sdk/Cargo.toml --no-default-features
