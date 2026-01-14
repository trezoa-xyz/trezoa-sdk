pub use trezoa_sdk_ids::sysvar::epoch_rewards::{check_id, id, ID};
use {crate::EpochRewards, trezoa_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(EpochRewards);
