#[deprecated(
    since = "2.1.0",
    note = "Use trezoa-clock and trezoa-epoch-schedule crates instead."
)]
pub use {
    trezoa_clock::{Epoch, Slot, DEFAULT_SLOTS_PER_EPOCH},
    trezoa_epoch_schedule::*,
};
