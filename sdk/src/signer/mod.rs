#![cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `trezoa-presigner` crate instead")]
pub use trezoa_presigner as presigner;
#[deprecated(since = "2.2.0", note = "Use `trezoa-seed-derivable` crate instead")]
pub use trezoa_seed_derivable::SeedDerivable;
#[deprecated(since = "2.2.0", note = "Use `trezoa-signer` crate instead")]
pub use trezoa_signer::{
    null_signer, signers, unique_signers, EncodableKey, EncodableKeypair, Signer, SignerError,
};
pub mod keypair;
