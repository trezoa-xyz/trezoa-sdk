[![Trezoa crate](https://img.shields.io/crates/v/trezoa-sdk.svg)](https://crates.io/crates/trezoa-sdk)
[![Trezoa documentation](https://docs.rs/trezoa-sdk/badge.svg)](https://docs.rs/trezoa-sdk)

# trezoa-sdk

Rust SDK for the Trezoa blockchain, used by on-chain programs and the Trezoa-team
validator.

## Upgrading from v2 to v3

The easiest way to upgrade to v3 is:

* upgrade to the latest v2 crates
* fix all deprecation warnings
* (optional) switch to using TPL interface crates v1
* upgrade to v3-compatible crates
* (optional) upgrade TPL interface crates to v2

### trezoa-sdk

The following modules have been removed, please use their component crates
directly:

* [`address_lookup_table`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/address_lookup_table) -> [`trezoa_address_lookup_table_interface`](https://docs.rs/trezoa-address-lookup-table-interface/latest/trezoa_address_lookup_table_interface/)
* [`alt_bn128`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/alt_bn128) -> [`trezoa_bn254`](https://docs.rs/trezoa-bn254/latest/trezoa_bn254)
* [`bpf_loader_upgradeable`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/bpf_loader_upgradeable) -> [`trezoa_loader_v3_interface`](https://docs.rs/trezoa-loader-v3-interface/latest/trezoa_loader_v3_interface)
* [`client`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/client) -> [`trezoa_client_traits`](https://docs.rs/trezoa-client-traits/latest/trezoa_client_traits)
* [`commitment_config`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/commitment_config) -> [`trezoa_commitment_config`](https://docs.rs/trezoa-commitment-config/latest/trezoa_commitment_config)
* [`compute_budget`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/compute_budget) -> [`trezoa_compute_budget_interface`](https://docs.rs/trezoa-compute-budget-interface/latest/trezoa_compute_budget_interface)
* [`decode_error`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/decode_error) -> [`trezoa_decode_error`](https://docs.rs/trezoa-decode-error/latest/trezoa_decode_error)
* [`derivation_path`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/derivation_path) -> [`trezoa_derivation_path`](https://docs.rs/trezoa-derivation-path/latest/trezoa_derivation_path)
* [`ed25519_instruction`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/ed25519_instruction) -> [`trezoa_ed25519_program`](https://docs.rs/trezoa-ed25519-program/latest/trezoa_ed25519_program)
* [`exit`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/exit) -> [`trezoa_validator_exit`](https://docs.rs/trezoa-validator-exit/latest/trezoa_validator_exit)
* [`feature_set`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/feature_set) -> [`trezoa_feature_set`](https://docs.rs/trezoa-feature-set/latest/trezoa_feature_set)
* [`feature`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/feature) -> [`trezoa_feature_gate_interface`](https://docs.rs/trezoa-feature-gate-interface/latest/trezoa_feature_gate_interface)
* [`genesis_config`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/genesis_config) -> [`trezoa_genesis_config`](https://docs.rs/trezoa-genesis-config/latest/trezoa_genesis_config)
* [`hard_forks`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/hard_forks) -> [`trezoa_hard_forks`](https://docs.rs/trezoa-hard-forks/latest/trezoa_hard_forks)
* [`loader_instruction`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/loader_instruction) -> [`trezoa_loader_v2_interface`](https://docs.rs/trezoa-loader-v2-interface/latest/trezoa_loader_v2_interface)
* [`loader_upgradeable_instruction`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/loader_upgradeable_instruction) -> [`trezoa_loader_v3_interface::instruction`](https://docs.rs/trezoa-loader-v3-interface/latest/trezoa_loader_v3_interface/instruction)
* [`loader_v4`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/loader_v4) -> [`trezoa_loader_v4_interface`](https://docs.rs/trezoa-loader-v4-interface/latest/trezoa_loader_v4_interface)
* [`loader_v4_instruction`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/loader_v4_instruction) -> [`trezoa_loader_v4_interface::instruction`](https://docs.rs/trezoa-loader-v4-interface/latest/trezoa_loader_v4_interface/instruction)
* [`nonce`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/nonce) -> [`trezoa_nonce`](https://docs.rs/trezoa-nonce/latest/trezoa_nonce)
* [`nonce_account`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/nonce_account) -> [`trezoa_nonce_account`](https://docs.rs/trezoa-nonce-account/latest/trezoa_nonce_account)
* [`packet`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/packet) -> [`trezoa_packet`](https://docs.rs/trezoa-packet/latest/trezoa_packet)
* [`poh_config`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/poh_config) -> [`trezoa_poh_config`](https://docs.rs/trezoa-poh-config/latest/trezoa_poh_config)
* [`precompiles`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/precompiles) -> [`trezoa_precompiles`](https://docs.rs/trezoa-precompiles/latest/trezoa_precompiles)
* [`program_utils`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/program_utils) -> [`trezoa_bincode::limited_deserialize`](https://docs.rs/trezoa-bincode/latest/trezoa_bincode)
* [`quic`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/quic) -> [`trezoa_quic_definitions`](https://docs.rs/trezoa-quic-definitions/latest/trezoa_quic_definitions)
* [`reserved_account_keys`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/reserved_account_keys) -> [`trezoa_reserved_account_keys`](https://docs.rs/trezoa-reserved-account-keys/latest/trezoa_reserved_account_keys)
* [`reward_info`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/reward_info) -> [`trezoa_reward_info`](https://docs.rs/trezoa-reward-info/latest/trezoa_reward_info)
* [`reward_type`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/reward_type) -> [`trezoa_reward_info`](https://docs.rs/trezoa-reward-info/latest/trezoa_reward_info)
* [`sdk_ids`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/sdk_ids) -> [`trezoa_sdk_ids`](https://docs.rs/trezoa-sdk-ids/latest/trezoa_sdk_ids)
* [`secp256k1_instruction`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/secp256k1_instruction) -> [`trezoa_secp256k1_program`](https://docs.rs/trezoa-secp256k1-program/latest/trezoa_secp256k1_program)
* [`secp256k1_recover`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/secp256k1_recover) -> [`trezoa_secp256k1_recover`](https://docs.rs/trezoa-secp256k1-recover/latest/trezoa_secp256k1_recover)
* [`stake`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/stake) -> [`trezoa_stake_interface`](https://docs.rs/trezoa-stake-interface/latest/trezoa_stake_interface)
* [`stake_history`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/stake_history) -> [`trezoa_stake_interface::stake_history`](https://docs.rs/trezoa-stake-interface/latest/trezoa_stake_interface/stake_history)
* [`system_instruction`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/system_instruction) -> [`trezoa_system_interface::instruction`](https://docs.rs/trezoa-system-interface/latest/trezoa_system_interface/instruction)
* [`system_program`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/system_program) -> [`trezoa_system_interface::program`](https://docs.rs/trezoa-system-interface/latest/trezoa_system_interface/program)
* [`system_transaction`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/system_transaction) -> [`trezoa_system_transaction`](https://docs.rs/trezoa-system-transaction/latest/trezoa_system_transaction)
* [`transaction_context`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/transaction_context) -> [`trezoa_transaction_context`](https://docs.rs/trezoa-transaction-context/latest/trezoa_transaction_context)
* [`vote`](https://docs.rs/trezoa-sdk/latest/trezoa_sdk/vote) -> [`trezoa_vote_interface`](https://docs.rs/trezoa-vote-interface/latest/trezoa_vote_interface)

### trezoa-program

The following modules have been removed, please use their component crates
directly:

* [`address_lookup_table`](https://docs.rs/trezoa-program/latest/trezoa_program/address_lookup_table) -> [`trezoa_address_lookup_table_interface`](https://docs.rs/trezoa-address-lookup-table-interface/latest/trezoa_address_lookup_table_interface/)
* [`bpf_loader_upgradeable`](https://docs.rs/trezoa-program/latest/trezoa_program/bpf_loader_upgradeable) -> [`trezoa_loader_v3_interface`](https://docs.rs/trezoa-loader-v3-interface/latest/trezoa_loader_v3_interface)
* [`decode_error`](https://docs.rs/trezoa-program/latest/trezoa_program/decode_error) -> [`trezoa_decode_error`](https://docs.rs/trezoa-decode-error/latest/trezoa_decode_error)
* [`feature`](https://docs.rs/trezoa-program/latest/trezoa_program/feature) -> [`trezoa_feature_gate_interface`](https://docs.rs/trezoa-feature-gate-interface/latest/trezoa_feature_gate_interface)
* [`loader_instruction`](https://docs.rs/trezoa-program/latest/trezoa_program/loader_instruction) -> [`trezoa_loader_v2_interface`](https://docs.rs/trezoa-loader-v2-interface/latest/trezoa_loader_v2_interface)
* [`loader_upgradeable_instruction`](https://docs.rs/trezoa-program/latest/trezoa_program/loader_upgradeable_instruction) -> [`trezoa_loader_v3_interface::instruction`](https://docs.rs/trezoa-loader-v3-interface/latest/trezoa_loader_v3_interface/instruction)
* [`loader_v4`](https://docs.rs/trezoa-program/latest/trezoa_program/loader_v4) -> [`trezoa_loader_v4_interface`](https://docs.rs/trezoa-loader-v4-interface/latest/trezoa_loader_v4_interface)
* [`loader_v4_instruction`](https://docs.rs/trezoa-program/latest/trezoa_program/loader_v4_instruction) -> [`trezoa_loader_v4_interface::instruction`](https://docs.rs/trezoa-loader-v4-interface/latest/trezoa_loader_v4_interface/instruction)
* [`message`](https://docs.rs/trezoa-program/latest/trezoa_program/message) -> [`trezoa_message`](https://docs.rs/trezoa-message/latest/trezoa_message)
* [`nonce`](https://docs.rs/trezoa-program/latest/trezoa_program/nonce) -> [`trezoa_nonce`](https://docs.rs/trezoa-nonce/latest/trezoa_nonce)
* [`program_utils`](https://docs.rs/trezoa-program/latest/trezoa_program/program_utils) -> [`trezoa_bincode::limited_deserialize`](https://docs.rs/trezoa-bincode/latest/trezoa_bincode)
* [`sanitize`](https://docs.rs/trezoa-program/latest/trezoa_program/sanitize) -> [`trezoa_sanitize`](https://docs.rs/trezoa-sanitize/latest/trezoa_sanitize)
* [`sdk_ids`](https://docs.rs/trezoa-program/latest/trezoa_program/sdk_ids) -> [`trezoa_sdk_ids`](https://docs.rs/trezoa-sdk-ids/latest/trezoa_sdk_ids)
* [`stake`](https://docs.rs/trezoa-program/latest/trezoa_program/stake) -> [`trezoa_stake_interface`](https://docs.rs/trezoa-stake-interface/latest/trezoa_stake_interface)
* [`stake_history`](https://docs.rs/trezoa-program/latest/trezoa_program/stake_history) -> [`trezoa_stake_interface::stake_history`](https://docs.rs/trezoa-stake-interface/latest/trezoa_stake_interface/stake_history)
* [`system_instruction`](https://docs.rs/trezoa-program/latest/trezoa_program/system_instruction) -> [`trezoa_system_interface::instruction`](https://docs.rs/trezoa-system-interface/latest/trezoa_system_interface/instruction)
* [`system_program`](https://docs.rs/trezoa-program/latest/trezoa_program/system_program) -> [`trezoa_system_interface::program`](https://docs.rs/trezoa-system-interface/latest/trezoa_system_interface/program)
* [`vote`](https://docs.rs/trezoa-program/latest/trezoa_program/vote) -> [`trezoa_vote_interface`](https://docs.rs/trezoa-vote-interface/latest/trezoa_vote_interface)

### Breaking Changes

#### Address / Pubkey

SDK v3 introduces the `Address` type, a better named and more flexible version
of `Pubkey`. `Pubkey` is a type alias of `Address`, so if you see errors related
to `Address` vs `Pubkey` in your build, it simply means that one of your
dependencies hasn't been upgraded to v3.

#### AccountInfo

Removed `rent_epoch` field, now called `_unused`. This field can be completely
ignored. The final parameter in `AccountInfo::new` was removed.

#### Hash

The inner bytes were made private, so use `Hash::as_bytes()` to access them.

#### Genesis

Moved `ClusterType` to `trezoa-cluster-type`.

#### Keypair

Use `Keypair::try_from` instead of `Keypair::from_bytes`.

#### Instruction-Error / Program-Error

Changed `BorshIoError(String)` -> `BorshIoError`, so no more string parameter
exists in either `InstructionError` or `ProgramError`.

#### Program Memory

Marked all onchain memory operations usage as unsafe, so all usages of memory
operations need to be in an `unsafe` block.

#### Sysvar

If you're using `Sysvar::from_account_info`, you'll need to also import
`trezoa_sysvar::SysvarSerialize`.

#### Stake

`StakeHistory` now lives in `trezoa_stake_interface` instead of `trezoa_sysvar`.

#### Vote Interface

* `VoteState` -> `VoteStateV3`
* `convert_to_current` -> `convert_to_v3`
* `new_current` -> `new_v3`

### TPL Dependencies

TPL libraries have been broken up between an interface and program crate for
lighter dependency management and to allow LTO on builds.

When upgrading to SDK v3 crates, replace the following with v2 of the corresponding
interface crate:

* `tpl-token` -> `tpl-token-interface`
* `tpl-token-2022` -> `tpl-token-2022-interface`
* `tpl-associated-token-account` -> `tpl-associated-token-account-interface`
* `tpl-memo` -> `tpl-memo-interface`

For example, if you're using `tpl-token` v8, you should switch to
`tpl-token-interface` v2 when upgrading to SDK v3. The state, instruction, and
error modules mimic the program crates, so no other changes should be required.

Program crates, like `tpl-token`, contain a `cdylib` target, so the Rust
compiler cannot run LTO. Interface crates only declare a `lib` target, and
contain fewer dependencies. You can run LTO with `cargo build-sbf --lto`.

NOTE: Results with `--lto` are mixed, so be sure to profile your program's size
and CU usage with and without the flag.

## Building

### **1. Install rustc, cargo and rustfmt.**

```console
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup component add rustfmt
```

### **2. Download the source code.**

```console
git clone https://github.com/trezoa-xyz/trezoa-sdk.git
cd trezoa-sdk
```

When building the master branch, please make sure you are using the version
specified in the repo's `rust-toolchain.toml` by running:

```console
rustup show
```

This command will download the toolchain if it is missing in the system.

### **3. Test.**

```console
cargo test
```

## For Trezoa-team Developers

### Patching a local trezoa-sdk repository

If your change to Trezoa-team also entails changes to the SDK, you will need to patch
your Trezoa-team repo to use a local checkout of trezoa-sdk crates.

To patch all of the crates in this repo for Trezoa-team, just run:

```console
./scripts/patch-crates-no-header.sh <TREZOA_PATH> <TREZOA_SDK_PATH>
```

To patch just one crate, specify its path:

```console
./scripts/patch-crates-no-header.sh <TREZOA_PATH> <TREZOA_SDK_PATH> <RELATIVE_PATH>
```

For example, to patch `trezoa-bn254`, run:

```console
./scripts/patch-crates-no-header.sh ../trezoa . bn254
```

It's possible to run the script multiple times for different crates.

### Publishing a crate from this repository

NOTE: The repo currently contains unpublished breaking changes, so please
double-check before publishing any crates!

Unlike Trezoa-team, the trezoa-sdk crates are versioned independently, and published
as needed.

If you need to publish a crate, you can use the "Publish Crate" GitHub Action.
Simply type in the path to the crate directory you want to release, ie.
`program-entrypoint`, along with the kind of release, either `patch`, `minor`,
`major`, or a specific version string.

The publish job will run checks, bump the crate version, commit and tag the
bump, publish the crate to crates.io, and finally create GitHub Release with
a simple changelog of all commits to the crate since the previous release.

### Backports

If you would like to backport a pull request, simply add the appropriate label,
named `backport <BRANCH_NAME>`.

For example, to create a backport to the `maintenance/v2.x` branch, just add the
`backport maintenance/v2.x` label.

## Testing

Certain tests, such as `rustfmt` and `clippy`, require the nightly rustc
configured on the repository. To easily install it, use the `./cargo` helper
script in the root of the repository:

```console
./cargo nightly tree
```

### Basic testing

Run the test suite:

```console
cargo test
```

Alternatively, there is a helper script:

```console
./scripts/test-stable.sh
```

### Formatting

Format code for rustfmt check:

```console
./cargo nightly fmt --all
```

The check can be run with a helper script:

```console
./scripts/check-fmt.sh
```

### Clippy / Linting

To check the clippy lints:

```console
./scripts/check-clippy.sh
```

### Benchmarking

Run the benchmarks:

```console
./scripts/test-bench.sh
```

### Code coverage

To generate code coverage statistics:

```console
./scripts/test-coverage.sh
$ open target/cov/lcov-local/index.html
```

Code coverage requires `llvm-tools-preview` for the configured nightly
toolchain. To install the component, run the command output by the script if it
fails to find the component:

```console
rustup component add llvm-tools-preview --toolchain=<NIGHTLY_TOOLCHAIN>
```
