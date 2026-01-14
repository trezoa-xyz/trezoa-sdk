//! Re-exports the [`LamportsError`] type for backwards compatibility.
#[deprecated(
    since = "2.1.0",
    note = "Use trezoa_instruction_error::LamportsError instead"
)]
pub use trezoa_instruction_error::LamportsError;
