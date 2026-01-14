#!/usr/bin/env bash

set -eo pipefail

here="$(dirname "$0")"
src_root="$(readlink -f "${here}/..")"

cd "${src_root}"

no_std_crates=(
  -p trezoa-address
  -p trezoa-account-view
  -p trezoa-blake3-hasher
  -p trezoa-clock
  -p trezoa-cluster-type
  -p trezoa-commitment-config
  -p trezoa-define-syscall
  -p trezoa-epoch-info
  -p trezoa-epoch-rewards
  -p trezoa-epoch-schedule
  -p trezoa-epoch-stake
  -p trezoa-fee-calculator
  -p trezoa-hash
  -p trezoa-instruction-view
  -p trezoa-keccak-hasher
  -p trezoa-msg
  -p trezoa-program-error
  -p trezoa-program-log
  -p trezoa-program-log-macro
  -p trezoa-program-memory
  -p trezoa-program-pack
  -p trezoa-pubkey
  -p trezoa-rent
  -p trezoa-sanitize
  -p trezoa-sdk-ids
  -p trezoa-sha256-hasher
  -p trezoa-signature
  -p trezoa-sysvar-id
  -p trezoa-system-interface
)
# Use the upstream BPF target, which doesn't support std, to make sure that our
# no_std support really works.
target="bpfel-unknown-none"

# These features require alloc
exclude_features_no_alloc="alloc,borsh,curve25519,serde,slice-cpi"
# These features never work on upstream BPF
exclude_features="atomic,bincode,default,dev-context-only-utils,frozen-abi,rand,std,verify"

./cargo nightly hack check \
  -Zbuild-std=core \
  "--target=$target" \
  "--exclude-features=${exclude_features},${exclude_features_no_alloc}" \
  --each-feature \
  "${no_std_crates[@]}"

# Check that all crates with features that work with no_std + alloc still work!
./cargo nightly hack check \
  -Zbuild-std=alloc,core \
  "--target=${target}" \
  "--exclude-features=${exclude_features}" \
  --each-feature \
  "${no_std_crates[@]}"
