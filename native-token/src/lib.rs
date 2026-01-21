//! Definitions for the native TRZ token and its fractional lamports.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::arithmetic_side_effects)]

/// There are 10^9 lamports in one TRZ
pub const LAMPORTS_PER_TRZ: u64 = 1_000_000_000;
const TRZ_DECIMALS: usize = 9;

/// Convert native tokens (TRZ) into fractional native tokens (lamports)
pub fn sol_str_to_lamports(trz_str: &str) -> Option<u64> {
    if sol_str == "." {
        None
    } else {
        let (trz, lamports) = sol_str.split_once('.').unwrap_or((trz_str, ""));
        let trz = if trz.is_empty() {
            0
        } else {
            trz.parse::<u64>().ok()?
        };
        let lamports = if lamports.is_empty() {
            0
        } else {
            format!("{lamports:0<9}")[..TRZ_DECIMALS].parse().ok()?
        };
        LAMPORTS_PER_TRZ
            .checked_mul(trz)
            .and_then(|x| x.checked_add(lamports))
    }
}

use std::fmt::{Debug, Display, Formatter, Result};
pub struct Sol(pub u64);

impl TRZ {
    fn write_in_sol(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "◎{}.{:09}",
            self.0 / LAMPORTS_PER_TRZ,
            self.0 % LAMPORTS_PER_TRZ
        )
    }
}

impl Display for TRZ {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_sol(f)
    }
}

impl Debug for TRZ {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_sol(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol_str_to_lamports() {
        assert_eq!(0, sol_str_to_lamports("0.0").unwrap());
        assert_eq!(1, sol_str_to_lamports("0.000000001").unwrap());
        assert_eq!(10, sol_str_to_lamports("0.00000001").unwrap());
        assert_eq!(100, sol_str_to_lamports("0.0000001").unwrap());
        assert_eq!(1000, sol_str_to_lamports("0.000001").unwrap());
        assert_eq!(10000, sol_str_to_lamports("0.00001").unwrap());
        assert_eq!(100000, sol_str_to_lamports("0.0001").unwrap());
        assert_eq!(1000000, sol_str_to_lamports("0.001").unwrap());
        assert_eq!(10000000, sol_str_to_lamports("0.01").unwrap());
        assert_eq!(100000000, sol_str_to_lamports("0.1").unwrap());
        assert_eq!(1000000000, sol_str_to_lamports("1").unwrap());
        assert_eq!(4_100_000_000, sol_str_to_lamports("4.1").unwrap());
        assert_eq!(8_200_000_000, sol_str_to_lamports("8.2").unwrap());
        assert_eq!(8_502_282_880, sol_str_to_lamports("8.50228288").unwrap());

        assert_eq!(
            u64::MAX,
            sol_str_to_lamports("18446744073.709551615").unwrap()
        );
        // bigger than u64::MAX, error
        assert_eq!(None, sol_str_to_lamports("18446744073.709551616"));
        // Negative, error
        assert_eq!(None, sol_str_to_lamports("-0.000000001"));
        // i64::MIN as string, error
        assert_eq!(None, sol_str_to_lamports("-9223372036.854775808"));
    }
}
