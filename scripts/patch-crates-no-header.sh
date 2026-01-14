#!/usr/bin/env bash

set -eo pipefail
here="$(dirname "$0")"
# pacify shellcheck: cannot follow dynamic path
# shellcheck disable=SC1090,SC1091
source "$here"/patch-crates-functions.sh

usage() {
  cat <<EOF >&2
USAGE:
    $0 <TREZOA_PATH> <TREZOA_SDK_PATH> [<CRATE_PATH>]

ARGS:
    <TREZOA_PATH>        Path to the root of an trezoa repo
    <TREZOA_SDK_PATH>   Path to the root of a trezoa-sdk repo
    [<CRATE_PATH>]      (Optional) Relative path to one crate to patch, ie. "address". By default, all crates are patched.
EOF
}

trezoa_path="$1"
if [ -z "$trezoa_path" ]; then
  usage
  exit 1
fi

trezoa_sdk_path="$2"
if [ -z "$trezoa_sdk_path" ]; then
  usage
  exit 1
fi

crate_dir="$3"

update_trezoa_sdk_dependencies "$trezoa_path" "$trezoa_sdk_path" "$crate_dir"
patch_crates_io_trezoa_sdk_no_header "$trezoa_path"/Cargo.toml "$trezoa_sdk_path" "$crate_dir"
