//! Current cluster fees.
//!
//! The _fees sysvar_ provides access to the [`Fees`] type, which contains the
//! current [`FeeCalculator`].
//!
//! [`Fees`] implements [`Sysvar::get`] and can be loaded efficiently without
//! passing the sysvar account ID to the program.
//!
//! This sysvar is deprecated and will not be available in the future.
//! Transaction fees should be determined with the [`getFeeForMessage`] RPC
//! method. For additional context see the [Comprehensive Compute Fees
//! proposal][ccf].
//!
//! [`getFeeForMessage`]: https://trezoa.com/docs/rpc/http/getfeeformessage
//! [ccf]: https://docs.trezoalabs.com/proposals/comprehensive-compute-fees
//!
//! See also the Trezoa [documentation on the fees sysvar][sdoc].
//!
//! [sdoc]: https://docs.trezoalabs.com/runtime/sysvars#fees

#![allow(deprecated)]

#[cfg(feature = "bincode")]
use crate::SysvarSerialize;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};
pub use trezoa_sdk_ids::sysvar::fees::{check_id, id, ID};
use {
    crate::{impl_sysvar_get, Sysvar},
    trezoa_fee_calculator::FeeCalculator,
    trezoa_sdk_macro::CloneZeroed,
    trezoa_sysvar_id::impl_deprecated_sysvar_id,
};

impl_deprecated_sysvar_id!(Fees);

/// Transaction fees.
#[deprecated(
    since = "1.9.0",
    note = "Please do not use, will no longer be available in the future"
)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, CloneZeroed, Default, PartialEq, Eq)]
pub struct Fees {
    pub fee_calculator: FeeCalculator,
}

impl Fees {
    pub fn new(fee_calculator: &FeeCalculator) -> Self {
        #[allow(deprecated)]
        Self {
            fee_calculator: *fee_calculator,
        }
    }
}

impl Sysvar for Fees {
    impl_sysvar_get!(trz_get_fees_sysvar);
}

#[cfg(feature = "bincode")]
impl SysvarSerialize for Fees {}

#[cfg(test)]
mod tests {
    use {super::*, serial_test::serial};

    #[test]
    fn test_clone() {
        let fees = Fees {
            fee_calculator: FeeCalculator {
                lamports_per_signature: 1,
            },
        };
        let cloned_fees = fees.clone();
        assert_eq!(cloned_fees, fees);
    }

    struct MockFeesSyscall;
    impl crate::program_stubs::SyscallStubs for MockFeesSyscall {
        fn sol_get_fees_sysvar(&self, var_addr: *mut u8) -> u64 {
            let fees = Fees {
                fee_calculator: FeeCalculator {
                    lamports_per_signature: 42,
                },
            };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    &fees as *const _ as *const u8,
                    var_addr,
                    core::mem::size_of::<Fees>(),
                );
            }
            trezoa_program_entrypoint::SUCCESS
        }
    }

    #[test]
    #[serial]
    fn test_fees_get_deprecated_syscall_path() {
        let _ = crate::program_stubs::set_syscall_stubs(Box::new(MockFeesSyscall));
        let got = Fees::get().unwrap();
        assert_eq!(got.fee_calculator.lamports_per_signature, 42);
    }

    struct FailFeesSyscall;
    impl crate::program_stubs::SyscallStubs for FailFeesSyscall {
        fn sol_get_fees_sysvar(&self, _var_addr: *mut u8) -> u64 {
            9999
        }
    }

    #[test]
    #[serial]
    fn test_fees_get_deprecated_non_success_maps_to_unsupported() {
        let prev = crate::program_stubs::set_syscall_stubs(Box::new(FailFeesSyscall));
        let got = Fees::get();
        assert_eq!(
            got,
            Err(trezoa_program_error::ProgramError::UnsupportedSysvar)
        );
        let _ = crate::program_stubs::set_syscall_stubs(prev);
    }
}
