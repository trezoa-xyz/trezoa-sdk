pub use trezoa_sdk_ids::sysvar::clock::{check_id, id, ID};
use {crate::Clock, trezoa_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Clock);
