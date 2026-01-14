//! Hashing with the [SHA-256] hash function, and a general [`Hash`] type.
//!
//! [SHA-256]: https://en.wikipedia.org/wiki/SHA-2
//! [`Hash`]: struct@Hash

#[cfg(not(target_os = "trezoa"))]
pub use trezoa_sha256_hasher::Hasher;
pub use {
    trezoa_hash::{Hash, ParseHashError, HASH_BYTES},
    trezoa_sha256_hasher::{hash, hashv},
};
