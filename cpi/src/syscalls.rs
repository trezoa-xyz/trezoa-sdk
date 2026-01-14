/// Syscall definitions used by `trezoa_cpi`.
pub use trezoa_define_syscall::definitions::{
    sol_invoke_signed_c, sol_invoke_signed_rust, sol_set_return_data,
};
use trezoa_pubkey::Pubkey;

#[deprecated(
    since = "3.1.0",
    note = "Use `trezoa_define_syscall::definitions::sol_get_return_data` instead"
)]
pub unsafe fn sol_get_return_data(data: *mut u8, length: u64, program_id: *mut Pubkey) -> u64 {
    trezoa_define_syscall::definitions::sol_get_return_data(data, length, program_id as *mut u8)
}
