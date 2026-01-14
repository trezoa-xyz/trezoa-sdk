pub use trezoa_sdk_ids::sysvar::slot_history::{check_id, id, ID};
use {crate::SlotHistory, trezoa_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(SlotHistory);
