//! The [secp256k1 native program][np].
//!
//! [np]: https://docs.trezoalabs.com/runtime/programs#secp256k1-program
//!
//! Constructors for secp256k1 program instructions, and documentation on the
//! program's usage can be found in [`trezoa_sdk::secp256k1_instruction`].
//!
//! [`trezoa_sdk::secp256k1_instruction`]: https://docs.rs/trezoa-sdk/latest/trezoa_sdk/secp256k1_instruction/index.html
pub use trezoa_sdk_ids::secp256k1_program::{check_id, id, ID};
