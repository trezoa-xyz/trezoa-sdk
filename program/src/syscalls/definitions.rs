#[deprecated(since = "2.1.0", note = "Use `trezoa_cpi::syscalls` instead")]
pub use trezoa_cpi::syscalls::{
    sol_get_return_data, sol_invoke_signed_c, sol_invoke_signed_rust, sol_set_return_data,
};
#[deprecated(
    since = "2.2.0",
    note = "Use `trezoa_define_syscall::definitions` instead"
)]
pub use trezoa_define_syscall::definitions::{
    sol_alt_bn128_compression, sol_alt_bn128_group_op, sol_big_mod_exp, sol_blake3,
    sol_curve_group_op, sol_curve_multiscalar_mul, sol_curve_pairing_map, sol_curve_validate_point,
    sol_get_clock_sysvar, sol_get_epoch_rewards_sysvar, sol_get_epoch_schedule_sysvar,
    sol_get_epoch_stake, sol_get_fees_sysvar, sol_get_last_restart_slot, sol_get_rent_sysvar,
    sol_get_sysvar, sol_keccak256, sol_remaining_compute_units,
};
#[cfg(target_feature = "static-syscalls")]
pub use trezoa_define_syscall::sys_hash;
#[deprecated(since = "2.1.0", note = "Use `trezoa_instruction::syscalls` instead")]
pub use trezoa_instruction::syscalls::{
    sol_get_processed_sibling_instruction, sol_get_stack_height,
};
#[deprecated(since = "2.1.0", note = "Use `trezoa_msg::syscalls` instead.")]
pub use trezoa_msg::syscalls::{sol_log_, sol_log_64_, sol_log_compute_units_, sol_log_data};
#[deprecated(
    since = "2.1.0",
    note = "Use `trezoa_program_memory::syscalls` instead"
)]
pub use trezoa_program_memory::syscalls::{sol_memcmp_, sol_memcpy_, sol_memmove_, sol_memset_};
#[deprecated(since = "2.1.0", note = "Use `trezoa_pubkey::syscalls` instead")]
pub use trezoa_pubkey::syscalls::{
    sol_create_program_address, sol_log_pubkey, sol_try_find_program_address,
};
#[deprecated(
    since = "2.1.0",
    note = "Use `trezoa_secp256k1_recover::sol_secp256k1_recover` instead"
)]
pub use trezoa_secp256k1_recover::sol_secp256k1_recover;
#[deprecated(since = "2.1.0", note = "Use trezoa_sha256_hasher::sol_sha256 instead")]
pub use trezoa_sha256_hasher::sol_sha256;
