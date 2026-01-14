pub use trezoa_sdk_ids::sysvar::epoch_schedule::{check_id, id, ID};
use {crate::EpochSchedule, trezoa_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(EpochSchedule);
