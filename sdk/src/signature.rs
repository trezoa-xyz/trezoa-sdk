//! Functionality for public and private keys.
#![cfg(feature = "full")]

// legacy module paths
#[deprecated(
    since = "2.2.0",
    note = "Use trezoa_keypair::signable::Signable instead."
)]
pub use trezoa_keypair::signable::Signable;
pub use {
    crate::signer::{keypair::*, null_signer::*, presigner::*, *},
    trezoa_signature::{ParseSignatureError, Signature, SIGNATURE_BYTES},
};
