pub use trezoa_sdk_ids::sysvar::slot_hashes::{check_id, id, ID};
use {crate::SlotHashes, trezoa_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(SlotHashes);
