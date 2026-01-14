//! Defines the [`TransportError`] type.

#![cfg(feature = "full")]
#[deprecated(since = "2.1.0", note = "Use trezoa_transaction_error crate instead")]
pub use trezoa_transaction_error::{TransportError, TransportResult as Result};
