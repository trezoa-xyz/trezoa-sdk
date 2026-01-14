pub use trezoa_sdk_ids::sysvar::rent::{check_id, id, ID};
use {crate::Rent, trezoa_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Rent);
