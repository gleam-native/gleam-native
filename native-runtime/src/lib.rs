// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The runtime library for the Gleam native target.
//!
//! Generated code represents every Gleam value as one 64-bit word:
//!
//! - low bit 1: a small integer, the value in the upper 63 bits (`(n << 1) | 1`)
//! - low bit 0: a pointer to a heap allocation (8-byte aligned)
//!
//! `Nil`, `False`/`True`, and the empty list are the small integers 0, 0/1,
//! and 0 respectively; Gleam's type system keeps them apart.
//!
//! Every heap object starts with a header word: the object kind in the low
//! 16 bits, and for records the variant tag in bits 16..32 and the field
//! count in bits 32..48. This lets polymorphic operations (deep equality,
//! `echo`) walk any value at run time. Layouts:
//!
//! - record (custom types, tuples, cons cells): `[header, fields...]`
//! - big integer / string: a header followed by the inline Rust value
//! - float: `[header, f64]` — generated code loads the payload directly
//! - closure: `[header, code pointer, captures...]`
//!
//! Every heap object also has a reference count in the word immediately
//! before the pointer generated code holds (so all payload offsets are
//! unchanged by it). Generated code calls [`gleam_native_inc`] and
//! [`gleam_native_dec`]; a count reaching zero frees the object and
//! decrements its children via a worklist, so destroying a long list does
//! not recurse.
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

pub const KIND_RECORD: u64 = 0;
pub const KIND_BIGINT: u64 = 1;
pub const KIND_FLOAT: u64 = 2;
pub const KIND_STRING: u64 = 3;
pub const KIND_CLOSURE: u64 = 4;

/// The header word of a record with the given variant tag and field count.
/// Code generation computes expected headers with this same formula, so a
/// variant check is a single word comparison.
pub const fn record_header(tag: u32, arity: u32) -> u64 {
    KIND_RECORD | ((tag as u64) << 16) | ((arity as u64) << 32)
}

const fn closure_header(captures: u32) -> u64 {
    KIND_CLOSURE | ((captures as u64) << 32)
}

fn header_kind(header: u64) -> u64 {
    header & 0xFFFF
}

fn record_tag(header: u64) -> u32 {
    ((header >> 16) & 0xFFFF) as u32
}

fn record_arity(header: u64) -> u32 {
    ((header >> 32) & 0xFFFF) as u32
}

/// A boxed Rust payload: reference count, header, value. The value word
/// generated code holds points at `header`, so the count lives at offset
/// -8 and all payload offsets are unaffected.
#[repr(C)]
struct HeapBox<T> {
    reference_count: u64,
    header: u64,
    value: T,
}

pub fn tag_small_int(value: i64) -> u64 {
    ((value << 1) | 1) as u64
}

fn box_heap<T>(kind: u64, value: T) -> u64 {
    let base = Box::into_raw(Box::new(HeapBox {
        reference_count: 1,
        header: kind,
        value,
    }));
    base as u64 + 8
}

fn container<T>(value: u64) -> *mut HeapBox<T> {
    (value - 8) as *mut HeapBox<T>
}

fn box_bigint(value: BigInt) -> u64 {
    box_heap(KIND_BIGINT, value)
}

fn box_float(value: f64) -> u64 {
    box_heap(KIND_FLOAT, value)
}

fn box_string(value: String) -> u64 {
    box_heap(KIND_STRING, value)
}

fn heap_header(value: u64) -> u64 {
    unsafe { *(value as *const u64) }
}

fn bigint_value(value: u64) -> &'static BigInt {
    unsafe { &(*container::<BigInt>(value)).value }
}

fn float_value(value: u64) -> f64 {
    unsafe { (*container::<f64>(value)).value }
}

fn string_value(value: u64) -> &'static String {
    unsafe { &(*container::<String>(value)).value }
}

fn record_field(value: u64, index: u32) -> u64 {
    unsafe { *((value as *const u64).add(1 + index as usize)) }
}

fn untag(value: u64) -> BigInt {
    if value & 1 == 1 {
        BigInt::from((value as i64) >> 1)
    } else {
        bigint_value(value).clone()
    }
}

fn retag(value: BigInt) -> u64 {
    match value.to_i64() {
        Some(small) if (SMALL_INT_MIN..=SMALL_INT_MAX).contains(&small) => tag_small_int(small),
        _ => box_bigint(value),
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
    box_float(f64::from_bits(bits))
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
    box_string(string.to_string())
}

/// Concatenates two strings into a new string, the implementation of the
/// `<>` operator.
///
/// # Safety
///
/// Both arguments must be strings created by this runtime. The Gleam type
/// system upholds this for calls from generated code.
pub unsafe extern "C" fn gleam_native_string_concat(left: u64, right: u64) -> u64 {
    let left = string_value(left);
    let right = string_value(right);
    let mut result = String::with_capacity(left.len() + right.len());
    result.push_str(left);
    result.push_str(right);
    box_string(result)
}

/// String equality by contents, returning [`TRUE`] or [`FALSE`].
///
/// # Safety
///
/// Both arguments must be strings created by this runtime. The Gleam type
/// system upholds this for calls from generated code.
pub unsafe extern "C" fn gleam_native_string_eq(left: u64, right: u64) -> u64 {
    if string_value(left) == string_value(right) {
        TRUE
    } else {
        FALSE
    }
}

/// Allocates a custom type record: a header word encoding the variant tag
/// and field count, followed by `arity` field words which generated code
/// stores immediately after this call.
pub extern "C" fn gleam_native_record_new(tag: u64, arity: u64) -> u64 {
    let value = allocate_words(1 + arity as usize);
    unsafe { *(value as *mut u64) = record_header(tag as u32, arity as u32) };
    value
}

/// Allocates a closure: a header word, a slot for the code pointer, and
/// `captures` capture words, all stored by generated code after this call.
pub extern "C" fn gleam_native_closure_new(captures: u64) -> u64 {
    let value = allocate_words(2 + captures as usize);
    unsafe { *(value as *mut u64) = closure_header(captures as u32) };
    value
}

/// Allocates `words` object words preceded by a reference count of one,
/// returning the value pointer (which points at the first object word).
fn allocate_words(words: usize) -> u64 {
    let layout = word_layout(words);
    let base = unsafe { std::alloc::alloc(layout) } as *mut u64;
    assert!(!base.is_null(), "heap allocation failed");
    unsafe { *base = 1 };
    base as u64 + 8
}

fn word_layout(words: usize) -> std::alloc::Layout {
    std::alloc::Layout::array::<u64>(1 + words).expect("heap object layout")
}

/// Increments a value's reference count. A no-op for immediates.
pub extern "C" fn gleam_native_inc(value: u64) -> u64 {
    if value & 1 == 0 {
        unsafe { *((value - 8) as *mut u64) += 1 };
    }
    value
}

/// Decrements a value's reference count, destroying the object when it
/// reaches zero. Children are processed with a worklist so that destroying
/// a long list does not overflow the stack. A no-op for immediates.
pub extern "C" fn gleam_native_dec(value: u64) -> u64 {
    let mut worklist = vec![value];
    while let Some(value) = worklist.pop() {
        if value & 1 == 1 {
            continue;
        }
        let count = (value - 8) as *mut u64;
        unsafe { *count -= 1 };
        if unsafe { *count } > 0 {
            continue;
        }
        let header = heap_header(value);
        match header_kind(header) {
            KIND_RECORD => {
                let arity = record_arity(header);
                for index in 0..arity {
                    worklist.push(record_field(value, index));
                }
                unsafe {
                    std::alloc::dealloc(count as *mut u8, word_layout(1 + arity as usize))
                };
            }
            KIND_CLOSURE => {
                let captures = record_arity(header);
                for index in 0..captures {
                    // Captures sit one word past the code pointer.
                    worklist.push(record_field(value, 1 + index));
                }
                unsafe {
                    std::alloc::dealloc(count as *mut u8, word_layout(2 + captures as usize))
                };
            }
            KIND_BIGINT => drop(unsafe { Box::from_raw(container::<BigInt>(value)) }),
            KIND_FLOAT => drop(unsafe { Box::from_raw(container::<f64>(value)) }),
            KIND_STRING => drop(unsafe { Box::from_raw(container::<String>(value)) }),
            _ => {}
        }
    }
    NIL
}

/// Structural equality between two values of the same Gleam type, returning
/// [`TRUE`] or [`FALSE`]. Closures compare by identity.
pub extern "C" fn gleam_native_eq(left: u64, right: u64) -> u64 {
    if deep_eq(left, right) { TRUE } else { FALSE }
}

fn deep_eq(left: u64, right: u64) -> bool {
    if left == right {
        return true;
    }
    // Different immediates, or an immediate against a heap value, are never
    // equal: big integers never encode small-range values.
    if left & 1 == 1 || right & 1 == 1 {
        return false;
    }
    let left_header = heap_header(left);
    if left_header != heap_header(right) {
        return false;
    }
    match header_kind(left_header) {
        KIND_BIGINT => bigint_value(left) == bigint_value(right),
        KIND_FLOAT => float_value(left) == float_value(right),
        KIND_STRING => string_value(left) == string_value(right),
        KIND_RECORD => {
            // Headers match, so tags and arities do too.
            (0..record_arity(left_header))
                .all(|index| deep_eq(record_field(left, index), record_field(right, index)))
        }
        // Closures are equal only when identical, handled above.
        _ => false,
    }
}

/// Renders a value for `echo`. Scalars print exactly; records print
/// structurally as `@tag(field, ...)` since constructor names do not exist
/// at run time. Booleans and other immediates nested inside structures
/// print as their integer encoding.
fn inspect(value: u64) -> String {
    if value & 1 == 1 {
        return format!("{}", (value as i64) >> 1);
    }
    let header = heap_header(value);
    match header_kind(header) {
        KIND_BIGINT => format!("{}", bigint_value(value)),
        KIND_FLOAT => format!("{:?}", float_value(value)),
        KIND_STRING => format!("{:?}", string_value(value)),
        KIND_CLOSURE => "//fn".to_string(),
        KIND_RECORD => {
            let fields: Vec<String> = (0..record_arity(header))
                .map(|index| inspect(record_field(value, index)))
                .collect();
            format!("@{}({})", record_tag(header), fields.join(", "))
        }
        kind => format!("<unknown kind {kind}>"),
    }
}

/// The implementation of `echo`: prints the source location and the value
/// to standard error, then returns the value. `kind` selects exact printing
/// for values whose static type the compiler knew at the echo site:
/// 0 structural, 1 int, 2 float, 3 string, 4 bool, 5 nil.
///
/// # Safety
///
/// The module name pointer must be valid as described for
/// [`gleam_native_panic`]; `message` is a string value or 0.
pub unsafe extern "C" fn gleam_native_echo(
    kind: u64,
    value: u64,
    message: u64,
    module: *const u8,
    module_length: u64,
    line: u64,
) -> u64 {
    let module = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(module, module_length as usize))
    };
    let rendered = match kind {
        1 => format!("{}", untag(value)),
        2 => format!("{:?}", float_value(value)),
        3 => format!("{:?}", string_value(value)),
        4 => (if value == TRUE { "True" } else { "False" }).to_string(),
        5 => "Nil".to_string(),
        _ => inspect(value),
    };
    eprint!("{module}:{line}");
    if message != 0 {
        eprint!(" {}", string_value(message));
    }
    eprintln!();
    eprintln!("{rendered}");
    value
}

/// Reports a `panic`, `todo`, or failed `let assert` and aborts the program
/// with exit code 1.
///
/// `kind` is 0 for `panic`, 1 for `todo`, and 2 for `let assert`. `message`
/// is a string value or 0 when the source gave no message. The module and
/// function names arrive as pointers into the compiled program's constant
/// data. Declared as returning a value so generated code can treat it as an
/// ordinary call, but it never returns.
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
        string_value(message)
    };
    eprintln!("runtime error: {name}");
    eprintln!();
    eprintln!("{message}");
    eprintln!();
    eprintln!("    {module}.{function}:{line}");
    std::process::exit(1);
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

/// Prints a float followed by a newline, formatted the way Gleam floats are
/// written (always with a decimal point or exponent).
///
/// # Safety
///
/// `value` must be a float created by this runtime. The Gleam type system
/// upholds this for calls from generated code.
pub unsafe extern "C" fn print_float(value: u64) -> u64 {
    println!("{:?}", float_value(value));
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
    println!("{}", string_value(value));
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
        (
            "gleam_native_closure_new",
            gleam_native_closure_new as *const u8,
        ),
        ("gleam_native_eq", gleam_native_eq as *const u8),
        ("gleam_native_inc", gleam_native_inc as *const u8),
        ("gleam_native_dec", gleam_native_dec as *const u8),
        ("gleam_native_echo", gleam_native_echo as *const u8),
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

    fn make_string(content: &str) -> u64 {
        unsafe { gleam_native_string_from_bytes(content.as_ptr(), content.len() as u64) }
    }

    fn make_record(tag: u64, fields: &[u64]) -> u64 {
        let record = gleam_native_record_new(tag, fields.len() as u64);
        for (index, field) in fields.iter().enumerate() {
            unsafe { *((record as *mut u64).add(1 + index)) = *field };
        }
        record
    }

    #[test]
    fn reference_counting_lifecycle() {
        // A record holding its own reference to a string.
        let string = make_string("shared");
        let record = make_record(0, &[gleam_native_inc(string)]);
        // Drop our reference; the record still holds one.
        let _ = gleam_native_dec(string);
        assert_eq!(string_value(record_field(record, 0)), "shared");
        // Destroying the record frees the string too.
        let _ = gleam_native_dec(record);

        // A long list must not overflow the stack when destroyed.
        let mut list = NIL;
        for n in 0..200_000 {
            list = make_record(1, &[tag_small_int(n), list]);
        }
        let _ = gleam_native_dec(list);
    }

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
        assert_eq!(div(-7, 2), tag_small_int(-3));
        assert_eq!(rem(-7, 2), tag_small_int(-1));
        assert_eq!(rem(7, -2), tag_small_int(1));
        assert_eq!(div(1, 0), tag_small_int(0));
        assert_eq!(rem(1, 0), tag_small_int(0));
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
        let overflowed = gleam_native_int_mul_slow(tag_small_int(SMALL_INT_MAX), tag_small_int(2));
        assert_eq!(overflowed & 1, 0);
        assert_eq!(untag(overflowed), BigInt::from(SMALL_INT_MAX) * 2);
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

    #[test]
    fn float_round_trip() {
        let boxed = gleam_native_float_from_bits(1.5_f64.to_bits());
        assert_eq!(boxed & 1, 0);
        assert_eq!(float_value(boxed), 1.5);
    }

    #[test]
    fn string_round_trip() {
        let content = "Hello, 🌍!";
        let boxed = make_string(content);
        assert_eq!(boxed & 1, 0);
        assert_eq!(string_value(boxed), content);
    }

    #[test]
    fn string_equality() {
        assert_eq!(
            unsafe { gleam_native_string_eq(make_string("ab"), make_string("ab")) },
            TRUE
        );
        assert_eq!(
            unsafe { gleam_native_string_eq(make_string("ab"), make_string("ac")) },
            FALSE
        );
        assert_eq!(
            unsafe { gleam_native_string_eq(make_string(""), make_string("")) },
            TRUE
        );
    }

    #[test]
    fn string_concatenation() {
        let result =
            unsafe { gleam_native_string_concat(make_string("Hello, "), make_string("🌍!")) };
        assert_eq!(string_value(result), "Hello, 🌍!");
    }

    #[test]
    fn deep_equality() {
        // Identical immediates and different immediates.
        assert_eq!(gleam_native_eq(tag_small_int(4), tag_small_int(4)), TRUE);
        assert_eq!(gleam_native_eq(tag_small_int(4), tag_small_int(5)), FALSE);

        // Records compare structurally, including nested strings.
        let a = make_record(1, &[tag_small_int(7), make_string("x")]);
        let b = make_record(1, &[tag_small_int(7), make_string("x")]);
        let c = make_record(1, &[tag_small_int(8), make_string("x")]);
        let d = make_record(2, &[tag_small_int(7), make_string("x")]);
        assert_eq!(gleam_native_eq(a, b), TRUE);
        assert_eq!(gleam_native_eq(a, c), FALSE);
        assert_eq!(gleam_native_eq(a, d), FALSE);

        // Nested records (a cons list of records).
        let list_a = make_record(1, &[a, NIL]);
        let list_b = make_record(1, &[b, NIL]);
        assert_eq!(gleam_native_eq(list_a, list_b), TRUE);

        // Immediate against heap value.
        assert_eq!(gleam_native_eq(NIL, a), FALSE);

        // Floats and big integers by value.
        let big_a = gleam_native_int_add_slow(tag_small_int(SMALL_INT_MAX), tag_small_int(1));
        let big_b = gleam_native_int_add_slow(tag_small_int(SMALL_INT_MAX), tag_small_int(1));
        assert_eq!(gleam_native_eq(big_a, big_b), TRUE);
        let float_a = gleam_native_float_from_bits(1.5_f64.to_bits());
        let float_b = gleam_native_float_from_bits(1.5_f64.to_bits());
        assert_eq!(gleam_native_eq(float_a, float_b), TRUE);
    }

    #[test]
    fn inspect_renders_structurally() {
        let record = make_record(1, &[tag_small_int(7), make_string("hi")]);
        assert_eq!(inspect(record), "@1(7, \"hi\")");
        assert_eq!(inspect(tag_small_int(-3)), "-3");
        assert_eq!(inspect(gleam_native_float_from_bits(2.5_f64.to_bits())), "2.5");
    }
}
