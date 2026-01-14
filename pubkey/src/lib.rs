//! Trezoa account addresses.
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "frozen-abi", feature(min_specialization))]
#![allow(clippy::arithmetic_side_effects)]

// If target_os = "trezoa", then this panics so there are no dependencies.
// When target_os != "trezoa", this should be opt-in so users
// don't need the curve25519 dependency.
#[cfg(any(target_os = "trezoa", feature = "curve25519"))]
pub use trezoa_address::bytes_are_curve_point;
#[cfg(target_os = "trezoa")]
pub use trezoa_address::syscalls;
pub use trezoa_address::{
    address as pubkey, declare_deprecated_id, declare_id,
    error::{AddressError as PubkeyError, ParseAddressError as ParsePubkeyError},
    Address as Pubkey, ADDRESS_BYTES as PUBKEY_BYTES, MAX_SEEDS, MAX_SEED_LEN,
};
#[cfg(all(feature = "rand", not(target_os = "trezoa")))]
pub use trezoa_address::{
    AddressHasher as PubkeyHasher, AddressHasherBuilder as PubkeyHasherBuilder,
};

/// New random `Pubkey` for tests and benchmarks.
#[cfg(all(feature = "rand", not(target_os = "trezoa")))]
pub fn new_rand() -> Pubkey {
    Pubkey::from(rand::random::<[u8; PUBKEY_BYTES]>())
}
