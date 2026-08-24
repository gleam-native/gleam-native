// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The runtime library for the Gleam native target.
//!
//! Generated code represents every Gleam value as one 64-bit word:
//!
//! - low bit 1: a small integer, the value in the upper 63 bits (`(n << 1) | 1`)
//! - low bit 0: a pointer to a heap allocation (8-byte aligned)
//!
//! `Nil` is the small integer 0. The only heap objects so far are big
//! integers, stored as a raw `Box<BigInt>`; they are currently leaked, as
//! reference counting is not yet implemented.
//!
//! These functions use the platform C calling convention and are registered
//! with the JIT by name via [`symbols`], so they need no `#[no_mangle]`.

use num_bigint::BigInt;
use num_traits::ToPrimitive;

pub const NIL: u64 = 1;

pub const SMALL_INT_MIN: i64 = i64::MIN >> 1;
pub const SMALL_INT_MAX: i64 = i64::MAX >> 1;

pub fn tag_small_int(value: i64) -> u64 {
    ((value << 1) | 1) as u64
}

fn untag(value: u64) -> BigInt {
    if value & 1 == 1 {
        BigInt::from((value as i64) >> 1)
    } else {
        let pointer = value as *const BigInt;
        unsafe { (*pointer).clone() }
    }
}

fn retag(value: BigInt) -> u64 {
    match value.to_i64() {
        Some(small) if (SMALL_INT_MIN..=SMALL_INT_MAX).contains(&small) => tag_small_int(small),
        _ => Box::into_raw(Box::new(value)) as u64,
    }
}

/// The slow path for integer addition, called by generated code when either
/// operand is a big integer or when small-integer addition overflowed.
pub extern "C" fn gleam_native_int_add_slow(left: u64, right: u64) -> u64 {
    retag(untag(left) + untag(right))
}

/// Builds a big integer value from a signed little-endian byte string stored
/// in the compiled program's data section, for integer literals too large to
/// be tagged immediates.
///
/// # Safety
///
/// `bytes` must point to `length` readable bytes. Generated code always
/// passes a pointer into its own constant data.
pub unsafe extern "C" fn gleam_native_bigint_from_bytes(bytes: *const u8, length: u64) -> u64 {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, length as usize) };
    retag(BigInt::from_signed_bytes_le(bytes))
}

/// Prints an integer followed by a newline. The standin for a real printing
/// external until strings exist on the native target.
pub extern "C" fn print_int(value: u64) -> u64 {
    println!("{}", untag(value));
    NIL
}

/// The symbols generated code may reference, for registration with the JIT.
pub fn symbols() -> Vec<(&'static str, *const u8)> {
    vec![
        (
            "gleam_native_int_add_slow",
            gleam_native_int_add_slow as *const u8,
        ),
        (
            "gleam_native_bigint_from_bytes",
            gleam_native_bigint_from_bytes as *const u8,
        ),
        ("print_int", print_int as *const u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_int_addition() {
        let result = gleam_native_int_add_slow(tag_small_int(20), tag_small_int(22));
        assert_eq!(result, tag_small_int(42));
    }

    #[test]
    fn big_integer_from_bytes() {
        let value: BigInt = BigInt::from(i64::MAX) * 3 - BigInt::from(7);
        let bytes = value.to_signed_bytes_le();
        let result = unsafe { gleam_native_bigint_from_bytes(bytes.as_ptr(), bytes.len() as u64) };
        assert_eq!(result & 1, 0);
        assert_eq!(untag(result), value);
    }

    #[test]
    fn small_integer_from_bytes_is_tagged() {
        let bytes = BigInt::from(42).to_signed_bytes_le();
        let result = unsafe { gleam_native_bigint_from_bytes(bytes.as_ptr(), bytes.len() as u64) };
        assert_eq!(result, tag_small_int(42));
    }

    #[test]
    fn overflowing_addition_makes_a_big_integer() {
        let result = gleam_native_int_add_slow(
            tag_small_int(SMALL_INT_MAX),
            tag_small_int(SMALL_INT_MAX),
        );
        assert_eq!(result & 1, 0);
        assert_eq!(untag(result), BigInt::from(SMALL_INT_MAX) * 2);
    }
}
