#[deprecated(since = "2.1.0", note = "Use `trezoa-sysvar-id` crate instead")]
pub use trezoa_sysvar_id::{declare_deprecated_sysvar_id, declare_sysvar_id, SysvarId};
#[deprecated(since = "2.2.0", note = "Use `trezoa-sysvar` crate instead")]
#[allow(deprecated)]
pub use {
    trezoa_sdk_ids::sysvar::{check_id, id, ID},
    trezoa_sysvar::{
        clock, epoch_rewards, epoch_schedule, fees, last_restart_slot, recent_blockhashes, rent,
        rewards, slot_hashes, slot_history, Sysvar, SysvarSerialize,
    },
};

pub mod instructions {
    #[deprecated(since = "2.2.0", note = "Use trezoa-instruction crate instead")]
    pub use trezoa_instruction::{BorrowedAccountMeta, BorrowedInstruction};
    #[cfg(not(target_os = "trezoa"))]
    #[deprecated(since = "2.2.0", note = "Use trezoa-instructions-sysvar crate instead")]
    pub use trezoa_instructions_sysvar::construct_instructions_data;
    #[cfg(all(not(target_os = "trezoa"), feature = "dev-context-only-utils"))]
    #[deprecated(since = "2.2.0", note = "Use trezoa-instructions-sysvar crate instead")]
    pub use trezoa_instructions_sysvar::serialize_instructions;
    #[cfg(feature = "dev-context-only-utils")]
    #[deprecated(since = "2.2.0", note = "Use trezoa-instructions-sysvar crate instead")]
    pub use trezoa_instructions_sysvar::{deserialize_instruction, load_instruction_at};
    #[deprecated(since = "2.2.0", note = "Use trezoa-instructions-sysvar crate instead")]
    #[allow(deprecated)]
    pub use trezoa_instructions_sysvar::{
        get_instruction_relative, load_current_index_checked, load_instruction_at_checked,
        Instructions,
    };
    #[deprecated(since = "2.2.0", note = "Use trezoa-sdk-ids crate instead")]
    pub use trezoa_sdk_ids::sysvar::instructions::{check_id, id, ID};
}
