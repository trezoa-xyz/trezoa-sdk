#[cfg(feature = "full")]
pub use trezoa_pubkey::new_rand;
#[cfg(target_os = "trezoa")]
pub use trezoa_pubkey::syscalls;
pub use trezoa_pubkey::{
    bytes_are_curve_point, ParsePubkeyError, Pubkey, PubkeyError, MAX_SEEDS, MAX_SEED_LEN,
    PUBKEY_BYTES,
};
