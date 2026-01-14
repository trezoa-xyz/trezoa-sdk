#[deprecated(since = "2.2.0", note = "Use `trezoa-keypair` crate instead")]
pub use trezoa_keypair::{
    keypair_from_seed, keypair_from_seed_phrase_and_passphrase, read_keypair, read_keypair_file,
    seed_derivable::keypair_from_seed_and_derivation_path, write_keypair, write_keypair_file,
    Keypair,
};
#[deprecated(since = "2.2.0", note = "Use `trezoa-seed-phrase` crate instead")]
pub use trezoa_seed_phrase::generate_seed_from_seed_phrase_and_passphrase;
#[deprecated(since = "2.2.0", note = "Use `trezoa-signer` crate instead")]
pub use trezoa_signer::*;
