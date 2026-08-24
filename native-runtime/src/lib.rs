// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The runtime library for the Gleam native target.
//!
//! Generated code represents every Gleam value as one 64-bit word:
//!
//! - low bit 1: a small integer, the value in the upper 63 bits (`(n << 1) | 1`)
//! - low bit 0: a pointer to a heap allocation (8-byte aligned)
//!
//! `Nil` is the small integer 0. Heap objects so far are big integers
//! (a raw `Box<BigInt>`), floats (a raw `Box<f64>`), strings (a raw
//! `Box<String>`, always valid UTF-8), and custom type records (a variant
//! tag word followed by the field values, allocated by
//! [`gleam_native_record_new`] and written by generated code); they are
//! currently leaked, as reference counting is not yet implemented. Heap
//! objects carry no kind header yet: Gleam's type system statically
//! separates which functions receive which types, so none is needed until
//! polymorphic runtime services (structural equality, `echo`) exist.
//!
//! These functions use the platform C calling convention and are registered
//! with the JIT by name via [`symbols`], so they need no `#[no_mangle]`.

use num_bigint::BigInt;
use num_traits::ToPrimitive;

pub const NIL: u64 = 1;

/// `False` and `True` are the tagged small integers 0 and 1.
pub const FALSE: u64 = 1;
pub const TRUE: u64 = 3;

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

/// The slow path for integer subtraction; see [`gleam_native_int_add_slow`].
pub extern "C" fn gleam_native_int_sub_slow(left: u64, right: u64) -> u64 {
    retag(untag(left) - untag(right))
}

/// The slow path for integer multiplication; see
/// [`gleam_native_int_add_slow`].
pub extern "C" fn gleam_native_int_mul_slow(left: u64, right: u64) -> u64 {
    retag(untag(left) * untag(right))
}

/// Three-way integer comparison for when either operand is a big integer:
/// returns the tagged small integer -1, 0, or 1.
pub extern "C" fn gleam_native_int_compare(left: u64, right: u64) -> u64 {
    tag_small_int(match untag(left).cmp(&untag(right)) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    })
}

/// Truncating integer division. Division by zero yields zero, following
/// Gleam's semantics on every target.
pub extern "C" fn gleam_native_int_div(left: u64, right: u64) -> u64 {
    let divisor = untag(right);
    if divisor == BigInt::ZERO {
        return tag_small_int(0);
    }
    retag(untag(left) / divisor)
}

/// Integer remainder, taking the sign of the dividend. A zero divisor
/// yields zero, following Gleam's semantics on every target.
pub extern "C" fn gleam_native_int_rem(left: u64, right: u64) -> u64 {
    let divisor = untag(right);
    if divisor == BigInt::ZERO {
        return tag_small_int(0);
    }
    retag(untag(left) % divisor)
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

/// Boxes a float value given its IEEE 754 bit pattern. Taking the bits as an
/// integer keeps every generated call signature uniformly i64.
pub extern "C" fn gleam_native_float_from_bits(bits: u64) -> u64 {
    Box::into_raw(Box::new(f64::from_bits(bits))) as u64
}

/// Builds a string value from UTF-8 bytes stored in the compiled program's
/// data section.
///
/// # Safety
///
/// `bytes` must point to `length` readable bytes of valid UTF-8. Generated
/// code always passes a pointer into its own constant data, containing
/// compiler-validated string contents.
pub unsafe extern "C" fn gleam_native_string_from_bytes(bytes: *const u8, length: u64) -> u64 {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, length as usize) };
    let string = unsafe { std::str::from_utf8_unchecked(bytes) };
    Box::into_raw(Box::new(string.to_string())) as u64
}

/// Concatenates two strings into a new string, the implementation of the
/// `<>` operator.
///
/// # Safety
///
/// Both arguments must be strings created by this runtime. The Gleam type
/// system upholds this for calls from generated code.
pub unsafe extern "C" fn gleam_native_string_concat(left: u64, right: u64) -> u64 {
    let left = unsafe { &*(left as *const String) };
    let right = unsafe { &*(right as *const String) };
    let mut result = String::with_capacity(left.len() + right.len());
    result.push_str(left);
    result.push_str(right);
    Box::into_raw(Box::new(result)) as u64
}

/// Reports a `panic` or `todo` and aborts the program with exit code 1.
///
/// `kind` is 0 for `panic` and 1 for `todo`. `message` is a string value or
/// 0 when the source gave no message. The module and function names arrive
/// as pointers into the compiled program's constant data. Declared as
/// returning a value so generated code can treat it as an ordinary call, but
/// it never returns.
///
/// # Safety
///
/// The pointer arguments must be valid as described above. Generated code
/// always passes pointers into its own constant data and runtime-created
/// strings.
pub unsafe extern "C" fn gleam_native_panic(
    kind: u64,
    message: u64,
    module: *const u8,
    module_length: u64,
    function: *const u8,
    function_length: u64,
    line: u64,
) -> u64 {
    let module = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(module, module_length as usize))
    };
    let function = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(
            function,
            function_length as usize,
        ))
    };
    let (name, default_message) = match kind {
        1 => ("todo", "This has not yet been implemented"),
        2 => (
            "let assert",
            "Pattern match failed, no pattern matched the value",
        ),
        _ => ("panic", "`panic` expression evaluated"),
    };
    let message = if message == 0 {
        default_message
    } else {
        unsafe { &*(message as *const String) }
    };
    eprintln!("runtime error: {name}");
    eprintln!();
    eprintln!("{message}");
    eprintln!();
    eprintln!("    {module}.{function}:{line}");
    std::process::exit(1);
}

/// Allocates a custom type record: one word for the variant tag followed by
/// `arity` words for the fields, which generated code stores immediately
/// after this call.
pub extern "C" fn gleam_native_record_new(tag: u64, arity: u64) -> u64 {
    let words = 1 + arity as usize;
    let layout = std::alloc::Layout::array::<u64>(words).expect("record layout");
    let pointer = unsafe { std::alloc::alloc(layout) } as *mut u64;
    assert!(!pointer.is_null(), "record allocation failed");
    unsafe { *pointer = tag };
    pointer as u64
}

/// String equality by contents, returning [`TRUE`] or [`FALSE`].
///
/// # Safety
///
/// Both arguments must be strings created by this runtime. The Gleam type
/// system upholds this for calls from generated code.
pub unsafe extern "C" fn gleam_native_string_eq(left: u64, right: u64) -> u64 {
    let left = unsafe { &*(left as *const String) };
    let right = unsafe { &*(right as *const String) };
    if left == right { TRUE } else { FALSE }
}

/// Prints an integer followed by a newline. The standin for a real printing
/// external until strings exist on the native target.
pub extern "C" fn print_int(value: u64) -> u64 {
    println!("{}", untag(value));
    NIL
}

/// Prints a boolean as Gleam writes it, `True` or `False`, followed by a
/// newline.
pub extern "C" fn print_bool(value: u64) -> u64 {
    println!("{}", if value == TRUE { "True" } else { "False" });
    NIL
}

/// Prints a string followed by a newline, the native implementation for a
/// `gleam/io.println`-style external.
///
/// # Safety
///
/// `value` must be a string created by this runtime. The Gleam type system
/// upholds this for calls from generated code.
pub unsafe extern "C" fn println(value: u64) -> u64 {
    let string = unsafe { &*(value as *const String) };
    println!("{string}");
    NIL
}

/// Prints a float followed by a newline, formatted the way Gleam floats are
/// written (always with a decimal point or exponent).
///
/// # Safety
///
/// `value` must be a float created by this runtime. The Gleam type system
/// upholds this for calls from generated code.
pub unsafe extern "C" fn print_float(value: u64) -> u64 {
    let float = unsafe { *(value as *const f64) };
    println!("{float:?}");
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
            "gleam_native_int_sub_slow",
            gleam_native_int_sub_slow as *const u8,
        ),
        (
            "gleam_native_int_mul_slow",
            gleam_native_int_mul_slow as *const u8,
        ),
        (
            "gleam_native_int_compare",
            gleam_native_int_compare as *const u8,
        ),
        ("gleam_native_int_div", gleam_native_int_div as *const u8),
        ("gleam_native_int_rem", gleam_native_int_rem as *const u8),
        (
            "gleam_native_bigint_from_bytes",
            gleam_native_bigint_from_bytes as *const u8,
        ),
        (
            "gleam_native_float_from_bits",
            gleam_native_float_from_bits as *const u8,
        ),
        (
            "gleam_native_string_from_bytes",
            gleam_native_string_from_bytes as *const u8,
        ),
        (
            "gleam_native_string_concat",
            gleam_native_string_concat as *const u8,
        ),
        (
            "gleam_native_string_eq",
            gleam_native_string_eq as *const u8,
        ),
        (
            "gleam_native_record_new",
            gleam_native_record_new as *const u8,
        ),
        ("gleam_native_panic", gleam_native_panic as *const u8),
        ("print_int", print_int as *const u8),
        ("print_bool", print_bool as *const u8),
        ("print_float", print_float as *const u8),
        ("println", println as *const u8),
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
    fn float_round_trip() {
        let boxed = gleam_native_float_from_bits(1.5_f64.to_bits());
        assert_eq!(boxed & 1, 0);
        assert_eq!(unsafe { *(boxed as *const f64) }, 1.5);
    }

    #[test]
    fn string_round_trip() {
        let content = "Hello, 🌍!";
        let boxed = unsafe {
            gleam_native_string_from_bytes(content.as_ptr(), content.len() as u64)
        };
        assert_eq!(boxed & 1, 0);
        assert_eq!(unsafe { &*(boxed as *const String) }, content);
    }

    #[test]
    fn string_equality() {
        let make = |content: &str| unsafe {
            gleam_native_string_from_bytes(content.as_ptr(), content.len() as u64)
        };
        assert_eq!(unsafe { gleam_native_string_eq(make("ab"), make("ab")) }, TRUE);
        assert_eq!(unsafe { gleam_native_string_eq(make("ab"), make("ac")) }, FALSE);
        assert_eq!(unsafe { gleam_native_string_eq(make(""), make("")) }, TRUE);
    }

    #[test]
    fn string_concatenation() {
        let make = |content: &str| unsafe {
            gleam_native_string_from_bytes(content.as_ptr(), content.len() as u64)
        };
        let result = unsafe { gleam_native_string_concat(make("Hello, "), make("🌍!")) };
        assert_eq!(unsafe { &*(result as *const String) }, "Hello, 🌍!");
    }

    #[test]
    fn subtraction_and_multiplication() {
        assert_eq!(
            gleam_native_int_sub_slow(tag_small_int(40), tag_small_int(-2)),
            tag_small_int(42)
        );
        assert_eq!(
            gleam_native_int_mul_slow(tag_small_int(6), tag_small_int(7)),
            tag_small_int(42)
        );
        let overflowed =
            gleam_native_int_mul_slow(tag_small_int(SMALL_INT_MAX), tag_small_int(2));
        assert_eq!(overflowed & 1, 0);
        assert_eq!(untag(overflowed), BigInt::from(SMALL_INT_MAX) * 2);
    }

    #[test]
    fn integer_comparison() {
        let compare = |a: u64, b: u64| gleam_native_int_compare(a, b);
        assert_eq!(
            compare(tag_small_int(1), tag_small_int(2)),
            tag_small_int(-1)
        );
        assert_eq!(
            compare(tag_small_int(2), tag_small_int(2)),
            tag_small_int(0)
        );
        let big = gleam_native_int_add_slow(tag_small_int(SMALL_INT_MAX), tag_small_int(1));
        assert_eq!(compare(big, tag_small_int(5)), tag_small_int(1));
        assert_eq!(compare(tag_small_int(-5), big), tag_small_int(-1));
    }

    #[test]
    fn division_and_remainder() {
        let div = |a: i64, b: i64| gleam_native_int_div(tag_small_int(a), tag_small_int(b));
        let rem = |a: i64, b: i64| gleam_native_int_rem(tag_small_int(a), tag_small_int(b));
        assert_eq!(div(84, 2), tag_small_int(42));
        // Truncation toward zero, remainder takes the dividend's sign.
        assert_eq!(div(-7, 2), tag_small_int(-3));
        assert_eq!(rem(-7, 2), tag_small_int(-1));
        assert_eq!(rem(7, -2), tag_small_int(1));
        // Division by zero yields zero.
        assert_eq!(div(1, 0), tag_small_int(0));
        assert_eq!(rem(1, 0), tag_small_int(0));
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
