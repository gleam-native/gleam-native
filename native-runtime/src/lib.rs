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
use unicode_segmentation::UnicodeSegmentation;

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
pub const KIND_BITARRAY: u64 = 5;

/// The header word of a record with the given variant tag and field count.
/// Code generation computes expected headers with this same formula, so a
/// variant check is a single (masked) word comparison.
pub const fn record_header(tag: u32, arity: u32) -> u64 {
    KIND_RECORD | ((tag as u64) << 16) | ((arity as u64) << 32)
}

/// Bits 48..64 of a record header hold a display-only constructor id used
/// by `echo`; semantic comparisons (variant checks, equality) mask it out
/// with this mask.
pub const HEADER_SEMANTIC_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;

/// Reserved display ids; user constructors are interned from
/// [`FIRST_INTERNED_DISPLAY`] upwards by code generation.
pub const DISPLAY_TUPLE: u16 = 0;
pub const DISPLAY_LIST: u16 = 1;
pub const DISPLAY_OK: u16 = 2;
pub const DISPLAY_ERROR: u16 = 3;
pub const FIRST_INTERNED_DISPLAY: u16 = 4;

fn record_display(header: u64) -> u16 {
    (header >> 48) as u16
}

/// The interned constructor names for display ids from
/// [`FIRST_INTERNED_DISPLAY`] upwards, set by the host before `main` runs.
static CONSTRUCTOR_NAMES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();

/// Stores the interned constructor names; called by the host.
pub fn set_constructor_names(names: Vec<String>) {
    let _ = CONSTRUCTOR_NAMES.set(names);
}

fn constructor_name(display: u16) -> Option<&'static str> {
    match display {
        DISPLAY_OK => Some("Ok"),
        DISPLAY_ERROR => Some("Error"),
        _ => CONSTRUCTOR_NAMES
            .get()?
            .get((display - FIRST_INTERNED_DISPLAY) as usize)
            .map(|name| name.as_str()),
    }
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

/// A bit array's payload: a bit count and MSB-first packed bytes, the
/// last byte zero-padded in its unused low bits.
struct BitArrayPayload {
    bits: u64,
    bytes: Vec<u8>,
}

fn box_bitarray(payload: BitArrayPayload) -> u64 {
    box_heap(KIND_BITARRAY, payload)
}

fn bitarray_value(value: u64) -> &'static BitArrayPayload {
    unsafe { &(*container::<BitArrayPayload>(value)).value }
}

fn bitarray_value_mut(value: u64) -> &'static mut BitArrayPayload {
    unsafe { &mut (*container::<BitArrayPayload>(value)).value }
}

/// Aborts on an invalid bit array operation. These are runtime limitations
/// or bounds violations, reported like a panic.
fn bitarray_panic(message: &str) -> ! {
    eprintln!("runtime error: bit array");
    eprintln!();
    eprintln!("{message}");
    std::process::exit(1);
}

/// Decodes a tagged size or offset in bits; it must be a non-negative
/// small integer.
fn untag_bits(value: u64, what: &str) -> u64 {
    if value & 1 != 1 {
        bitarray_panic(&format!("This bit array {what} is too large."));
    }
    let bits = (value as i64) >> 1;
    if bits < 0 {
        bitarray_panic(&format!("This bit array {what} is negative."));
    }
    bits as u64
}

/// Whether an endianness flag (0 big, 1 little, 2 native) means little.
fn is_little(endian: u64) -> bool {
    endian == 1 || (endian == 2 && cfg!(target_endian = "little"))
}

fn bit_of(bytes: &[u8], index: u64) -> bool {
    bytes[(index / 8) as usize] >> (7 - index % 8) & 1 == 1
}

fn push_bit(payload: &mut BitArrayPayload, bit: bool) {
    let position = payload.bits % 8;
    if position == 0 {
        payload.bytes.push(0);
    }
    if bit {
        let last = payload.bytes.last_mut().expect("bit array byte");
        *last |= 1 << (7 - position);
    }
    payload.bits += 1;
}

/// Appends `length` bits read from `source` starting at `offset`.
fn append_bits(payload: &mut BitArrayPayload, source: &[u8], offset: u64, length: u64) {
    if payload.bits % 8 == 0 && offset % 8 == 0 && length % 8 == 0 {
        // Fully aligned fast path.
        let start = (offset / 8) as usize;
        let end = start + (length / 8) as usize;
        payload.bytes.extend_from_slice(&source[start..end]);
        payload.bits += length;
        return;
    }
    for index in 0..length {
        push_bit(payload, bit_of(source, offset + index));
    }
}

/// The `length` bits at `offset`, MSB-first packed and zero-padded.
fn extract_bits(payload: &BitArrayPayload, offset: u64, length: u64) -> Vec<u8> {
    if offset + length > payload.bits {
        bitarray_panic("This bit array read is out of range.");
    }
    let mut out = BitArrayPayload {
        bits: 0,
        bytes: Vec::with_capacity(length.div_ceil(8) as usize),
    };
    append_bits(&mut out, &payload.bytes, offset, length);
    out.bytes
}

/// Appends the low `bits` bits of an integer with Erlang's endianness
/// semantics: big-endian writes most significant bit first; little-endian
/// writes whole bytes least significant first, then the leftover high bits.
fn append_int_bits(payload: &mut BitArrayPayload, value: &BigInt, bits: u64, little: bool) {
    // Two's complement little-endian bytes, sign-extended on demand: bit j
    // (j = 0 is least significant) of the value.
    let le_bytes = value.to_signed_bytes_le();
    let fill = if le_bytes.last().is_some_and(|byte| byte & 0x80 != 0) {
        0xFFu8
    } else {
        0
    };
    let value_bit = |j: u64| -> bool {
        let byte = le_bytes.get((j / 8) as usize).copied().unwrap_or(fill);
        byte >> (j % 8) & 1 == 1
    };
    if little {
        let whole_bytes = bits / 8;
        for byte in 0..whole_bytes {
            for bit in (0..8).rev() {
                push_bit(payload, value_bit(8 * byte + bit));
            }
        }
        for bit in (8 * whole_bytes..bits).rev() {
            push_bit(payload, value_bit(bit));
        }
    } else {
        for bit in (0..bits).rev() {
            push_bit(payload, value_bit(bit));
        }
    }
}

/// Reads `bits` bits at `offset` as an integer, inverse of
/// [`append_int_bits`].
fn read_int_bits(payload: &BitArrayPayload, offset: u64, bits: u64, little: bool, signed: bool) -> BigInt {
    let bytes = extract_bits(payload, offset, bits);
    let stream_bit = |i: u64| -> BigInt {
        if bit_of(&bytes, i) {
            BigInt::from(1)
        } else {
            BigInt::ZERO
        }
    };
    let mut value = BigInt::ZERO;
    if little {
        let whole_bytes = bits / 8;
        // Whole bytes carry ascending significance; the leftover bits are
        // the most significant.
        for byte in 0..whole_bytes {
            for bit in 0..8 {
                value += stream_bit(8 * byte + bit) << (8 * byte + 7 - bit);
            }
        }
        let leftover = bits - 8 * whole_bytes;
        for bit in 0..leftover {
            value += stream_bit(8 * whole_bytes + bit) << (8 * whole_bytes + leftover - 1 - bit);
        }
    } else {
        for bit in 0..bits {
            value = (value << 1) + stream_bit(bit);
        }
    }
    if signed && bits > 0 {
        let sign_bit = BigInt::from(1) << (bits - 1);
        if value.clone() & sign_bit.clone() != BigInt::ZERO {
            value -= sign_bit << 1;
        }
    }
    value
}

fn f64_to_f16_bits(value: f64) -> u16 {
    let bits32 = (value as f32).to_bits();
    let sign = ((bits32 >> 16) & 0x8000) as u16;
    let exponent = ((bits32 >> 23) & 0xFF) as i32;
    let mantissa = bits32 & 0x7F_FFFF;
    if exponent == 0xFF {
        // Infinity or NaN.
        return sign | 0x7C00 | if mantissa != 0 { 0x200 } else { 0 };
    }
    let unbiased = exponent - 127;
    if unbiased > 15 {
        return sign | 0x7C00; // Overflow to infinity.
    }
    if unbiased < -24 {
        return sign; // Underflow to zero.
    }
    if unbiased < -14 {
        // Subnormal half.
        let shift = -unbiased - 14;
        let mantissa = (mantissa | 0x80_0000) >> (13 + shift);
        return sign | mantissa as u16;
    }
    sign | (((unbiased + 15) as u16) << 10) | ((mantissa >> 13) as u16)
}

fn f16_bits_to_f64(bits: u16) -> f64 {
    let sign = if bits & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exponent = (bits >> 10) & 0x1F;
    let mantissa = (bits & 0x3FF) as f64;
    sign * match exponent {
        0 => mantissa * (2.0f64).powi(-24),
        0x1F => {
            if mantissa == 0.0 {
                f64::INFINITY
            } else {
                f64::NAN
            }
        }
        _ => (1.0 + mantissa / 1024.0) * (2.0f64).powi(exponent as i32 - 15),
    }
}

fn float_to_bytes(value: f64, bits: u64, little: bool) -> Vec<u8> {
    let mut bytes = match bits {
        16 => f64_to_f16_bits(value).to_be_bytes().to_vec(),
        32 => (value as f32).to_bits().to_be_bytes().to_vec(),
        64 => value.to_bits().to_be_bytes().to_vec(),
        _ => bitarray_panic("Float bit array segments must be 16, 32 or 64 bits."),
    };
    if little {
        bytes.reverse();
    }
    bytes
}

fn float_from_bytes(bytes: &[u8], little: bool) -> f64 {
    let mut bytes = bytes.to_vec();
    if little {
        bytes.reverse();
    }
    match bytes.len() {
        2 => f16_bits_to_f64(u16::from_be_bytes([bytes[0], bytes[1]])),
        4 => f32::from_bits(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])) as f64,
        8 => f64::from_bits(u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])),
        _ => bitarray_panic("Float bit array segments must be 16, 32 or 64 bits."),
    }
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

/// Whether a string starts with the UTF-8 bytes at `prefix`, returning
/// [`TRUE`] or [`FALSE`]. The prefix lives in the compiled program's
/// constant data.
///
/// # Safety
///
/// `subject` must be a string created by this runtime and `prefix` must
/// point to `length` readable bytes.
pub unsafe extern "C" fn gleam_native_string_starts_with(
    subject: u64,
    prefix: *const u8,
    length: u64,
) -> u64 {
    let prefix = unsafe { std::slice::from_raw_parts(prefix, length as usize) };
    if string_value(subject).as_bytes().starts_with(prefix) {
        TRUE
    } else {
        FALSE
    }
}

/// A new string holding everything from byte `offset` onwards. Generated
/// code only calls this after a prefix check, so the offset is always a
/// character boundary.
///
/// # Safety
///
/// `subject` must be a string created by this runtime and `offset` must be
/// a valid character boundary within it.
pub unsafe extern "C" fn gleam_native_string_slice_from(subject: u64, offset: u64) -> u64 {
    box_string(string_value(subject)[offset as usize..].to_string())
}

/// Builds a two-element tuple (a record with tag 0).
fn make_tuple2(first: u64, second: u64) -> u64 {
    let record = gleam_native_record_new(0, 2, DISPLAY_TUPLE as u64);
    unsafe {
        *((record as *mut u64).add(1)) = first;
        *((record as *mut u64).add(2)) = second;
    }
    record
}

/// Builds an `Ok` value (variant 0 of `Result`).
fn make_ok(value: u64) -> u64 {
    let record = gleam_native_record_new(0, 1, DISPLAY_OK as u64);
    unsafe { *((record as *mut u64).add(1)) = value };
    record
}

/// Builds an `Error` value (variant 1 of `Result`).
fn make_error(value: u64) -> u64 {
    let record = gleam_native_record_new(1, 1, DISPLAY_ERROR as u64);
    unsafe { *((record as *mut u64).add(1)) = value };
    record
}

/// Builds a list (cons cells with tag 1) from already-owned values.
fn make_list(values: Vec<u64>) -> u64 {
    let mut list = NIL;
    for value in values.into_iter().rev() {
        let cell = gleam_native_record_new(1, 2, DISPLAY_LIST as u64);
        unsafe {
            *((cell as *mut u64).add(1)) = value;
            *((cell as *mut u64).add(2)) = list;
        }
        list = cell;
    }
    list
}

/// The string's size in bytes.
///
/// # Safety
///
/// The argument must be a string created by this runtime; the Gleam type
/// system upholds this, as for every string function below.
pub unsafe extern "C" fn gleam_native_string_byte_size(string: u64) -> u64 {
    tag_small_int(string_value(string).len() as i64)
}

/// The string's length in grapheme clusters.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_length(string: u64) -> u64 {
    tag_small_int(string_value(string).graphemes(true).count() as i64)
}

/// Three-way string comparison as a tagged -1/0/1. Bytewise UTF-8
/// comparison is Unicode code point order, matching Erlang's binary
/// comparison exactly.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_compare(left: u64, right: u64) -> u64 {
    tag_small_int(match string_value(left).cmp(string_value(right)) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    })
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_uppercase(string: u64) -> u64 {
    box_string(string_value(string).to_uppercase())
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_lowercase(string: u64) -> u64 {
    box_string(string_value(string).to_lowercase())
}

/// The string with its grapheme clusters in reverse order.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_reverse(string: u64) -> u64 {
    box_string(string_value(string).graphemes(true).rev().collect())
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_contains(string: u64, needle: u64) -> u64 {
    if string_value(string).contains(string_value(needle)) {
        TRUE
    } else {
        FALSE
    }
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_ends_with(string: u64, suffix: u64) -> u64 {
    if string_value(string).ends_with(string_value(suffix)) {
        TRUE
    } else {
        FALSE
    }
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_trim(string: u64) -> u64 {
    box_string(string_value(string).trim().to_string())
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_trim_start(string: u64) -> u64 {
    box_string(string_value(string).trim_start().to_string())
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_trim_end(string: u64) -> u64 {
    box_string(string_value(string).trim_end().to_string())
}

/// The `length` grapheme clusters starting at grapheme index `start`
/// (both tagged, clamped to the string).
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_slice(string: u64, start: u64, length: u64) -> u64 {
    let start = ((start as i64) >> 1).max(0) as usize;
    let length = ((length as i64) >> 1).max(0) as usize;
    box_string(
        string_value(string)
            .graphemes(true)
            .skip(start)
            .take(length)
            .collect(),
    )
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_replace(
    string: u64,
    pattern: u64,
    replacement: u64,
) -> u64 {
    box_string(string_value(string).replace(string_value(pattern), string_value(replacement)))
}

/// Splits on a separator, returning a list of strings. An empty separator
/// yields the whole string as a single element, like the other targets.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_split(string: u64, on: u64) -> u64 {
    let string = string_value(string);
    let on = string_value(on);
    if on.is_empty() {
        return make_list(vec![box_string(string.clone())]);
    }
    make_list(
        string
            .split(on.as_str())
            .map(|part| box_string(part.to_string()))
            .collect(),
    )
}

/// The first grapheme cluster and the rest: `Ok(#(head, rest))`, or
/// `Error(Nil)` for the empty string.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_pop_grapheme(string: u64) -> u64 {
    let string = string_value(string);
    match string.grapheme_indices(true).next() {
        None => make_error(NIL),
        Some((_, head)) => {
            let rest = string[head.len()..].to_string();
            make_ok(make_tuple2(
                box_string(head.to_string()),
                box_string(rest),
            ))
        }
    }
}

/// The string's grapheme clusters as a list of strings.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_graphemes(string: u64) -> u64 {
    make_list(
        string_value(string)
            .graphemes(true)
            .map(|grapheme| box_string(grapheme.to_string()))
            .collect(),
    )
}

/// The string's Unicode code points as a list of tagged integers.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
pub unsafe extern "C" fn gleam_native_string_to_codepoints(string: u64) -> u64 {
    make_list(
        string_value(string)
            .chars()
            .map(|character| tag_small_int(character as i64))
            .collect(),
    )
}

/// A string from a list of code point scalars, skipping invalid ones (the
/// standard library validates scalars before building code point values).
pub extern "C" fn gleam_native_string_from_codepoints(list: u64) -> u64 {
    let mut result = String::new();
    let mut current = list;
    while current & 1 == 0 {
        let scalar = ((record_field(current, 0) as i64) >> 1) as u32;
        if let Some(character) = char::from_u32(scalar) {
            result.push(character);
        }
        current = record_field(current, 1);
    }
    box_string(result)
}

/// An integer rendered in decimal.
pub extern "C" fn gleam_native_int_to_string(value: u64) -> u64 {
    box_string(untag(value).to_string())
}

/// A float rendered the way Gleam writes floats.
///
/// # Safety
///
/// The argument must be a float created by this runtime.
pub unsafe extern "C" fn gleam_native_float_to_string(value: u64) -> u64 {
    box_string(format!("{:?}", float_value(value)))
}

/// Installs a handler that reports stack overflows (and other fatal memory
/// faults) as a runtime error with exit code 1 instead of a raw signal
/// death. Uses an alternate signal stack, since the main stack is exhausted
/// when a stack overflow fires.
pub fn install_stack_overflow_handler() {
    unsafe {
        let stack = libc::stack_t {
            ss_sp: std::alloc::alloc(
                std::alloc::Layout::from_size_align(64 * 1024, 16).expect("layout"),
            ) as *mut libc::c_void,
            ss_flags: 0,
            ss_size: 64 * 1024,
        };
        let _ = libc::sigaltstack(&stack, std::ptr::null_mut());

        extern "C" fn handler(_signal: libc::c_int) {
            let message = b"runtime error: stack overflow

The program recursed too deeply. Gleam tail calls run in constant stack space, but deeply nested non-tail recursion exhausted the stack.
";
            unsafe {
                let _ = libc::write(2, message.as_ptr() as *const libc::c_void, message.len());
                libc::_exit(1);
            }
        }

        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = handler as *const () as libc::sighandler_t;
        action.sa_flags = libc::SA_ONSTACK;
        let _ = libc::sigaction(libc::SIGSEGV, &action, std::ptr::null_mut());
        let _ = libc::sigaction(libc::SIGBUS, &action, std::ptr::null_mut());
    }
}

/// The command line arguments the host passes in before running `main`.
static START_ARGUMENTS: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();

/// Stores the program's command line arguments; called by the host before
/// `main` runs.
pub fn set_start_arguments(arguments: Vec<String>) {
    let _ = START_ARGUMENTS.set(arguments);
}

/// The command line arguments as a list of strings.
pub extern "C" fn gleam_native_start_arguments() -> u64 {
    let arguments = START_ARGUMENTS.get().cloned().unwrap_or_default();
    make_list(
        arguments
            .into_iter()
            .map(|argument| box_string(argument))
            .collect(),
    )
}

/// Ends the program immediately with the given exit code.
pub extern "C" fn gleam_native_exit(code: u64) -> u64 {
    let code = untag(code).to_i32().unwrap_or(1);
    std::process::exit(code);
}

/// A new empty bit array, the start of a construction chain.
pub extern "C" fn gleam_native_bitarray_empty() -> u64 {
    box_bitarray(BitArrayPayload {
        bits: 0,
        bytes: Vec::new(),
    })
}

/// Appends an integer segment of `bits` (tagged) bits, truncating the value
/// to the segment size. Construction chains own their array uniquely, so
/// the array is mutated in place and returned.
pub extern "C" fn gleam_native_bitarray_append_int(
    array: u64,
    value: u64,
    bits: u64,
    endian: u64,
) -> u64 {
    let bits = untag_bits(bits, "segment size");
    let value = untag(value);
    append_int_bits(bitarray_value_mut(array), &value, bits, is_little(endian));
    array
}

/// Appends a float segment of `bits` (tagged) bits: 16, 32 or 64.
///
/// # Safety
///
/// `value` must be a float created by this runtime.
pub unsafe extern "C" fn gleam_native_bitarray_append_float(
    array: u64,
    value: u64,
    bits: u64,
    endian: u64,
) -> u64 {
    let bits = untag_bits(bits, "segment size");
    let bytes = float_to_bytes(float_value(value), bits, is_little(endian));
    let payload = bitarray_value_mut(array);
    append_bits(payload, &bytes, 0, bytes.len() as u64 * 8);
    array
}

/// Appends a string segment encoded as UTF-8 (0), UTF-16 (1) or UTF-32 (2)
/// with the given endianness.
///
/// # Safety
///
/// `string` must be a string created by this runtime.
pub unsafe extern "C" fn gleam_native_bitarray_append_string(
    array: u64,
    string: u64,
    encoding: u64,
    endian: u64,
) -> u64 {
    let string = string_value(string);
    let little = is_little(endian);
    let mut bytes = Vec::new();
    match encoding {
        1 => {
            for unit in string.encode_utf16() {
                bytes.extend_from_slice(&if little {
                    unit.to_le_bytes()
                } else {
                    unit.to_be_bytes()
                });
            }
        }
        2 => {
            for character in string.chars() {
                let unit = character as u32;
                bytes.extend_from_slice(&if little {
                    unit.to_le_bytes()
                } else {
                    unit.to_be_bytes()
                });
            }
        }
        _ => bytes.extend_from_slice(string.as_bytes()),
    }
    let payload = bitarray_value_mut(array);
    append_bits(payload, &bytes, 0, bytes.len() as u64 * 8);
    array
}

/// Appends a single UTF codepoint (a tagged scalar value) with the given
/// encoding and endianness.
pub extern "C" fn gleam_native_bitarray_append_codepoint(
    array: u64,
    codepoint: u64,
    encoding: u64,
    endian: u64,
) -> u64 {
    let scalar = untag_bits(codepoint, "codepoint");
    let Some(character) = char::from_u32(scalar as u32) else {
        bitarray_panic("This value is not a valid UTF codepoint.");
    };
    let mut buffer = [0u8; 4];
    let string = character.encode_utf8(&mut buffer).to_string();
    let boxed = box_string(string);
    let result = unsafe { gleam_native_bitarray_append_string(array, boxed, encoding, endian) };
    let _ = gleam_native_dec(boxed);
    result
}

/// Appends another bit array's contents, either whole (`has_bits` zero) or
/// its first `bits` (tagged) bits.
pub extern "C" fn gleam_native_bitarray_append_bits(
    array: u64,
    other: u64,
    bits: u64,
    has_bits: u64,
) -> u64 {
    let other = bitarray_value(other);
    let length = if has_bits == 0 {
        other.bits
    } else {
        let bits = untag_bits(bits, "segment size");
        if bits > other.bits {
            bitarray_panic("This bit array read is out of range.");
        }
        bits
    };
    let payload = bitarray_value_mut(array);
    // The two arrays are distinct allocations, so this alias is safe.
    let source = other.bytes.clone();
    append_bits(payload, &source, 0, length);
    array
}

/// Whether the array is exactly (`exact` non-zero) or at least `bits`
/// (tagged) bits.
pub extern "C" fn gleam_native_bitarray_size_test(array: u64, bits: u64, exact: u64) -> u64 {
    // A negative or gigantic wanted size simply never matches.
    if bits & 1 != 1 || (bits as i64) < 0 {
        return FALSE;
    }
    let bits = ((bits as i64) >> 1) as u64;
    let size = bitarray_value(array).bits;
    let passed = if exact == 0 { size >= bits } else { size == bits };
    if passed { TRUE } else { FALSE }
}

/// Whether the `bit_length` bits at (tagged) bit offset `offset` equal the
/// given constant bits (MSB-first packed).
///
/// # Safety
///
/// `bytes` must point to at least `bit_length / 8` (rounded up) bytes.
pub unsafe extern "C" fn gleam_native_bitarray_bytes_test(
    array: u64,
    offset: u64,
    bytes: *const u8,
    bit_length: u64,
) -> u64 {
    let offset = untag_bits(offset, "offset");
    let payload = bitarray_value(array);
    if offset + bit_length > payload.bits {
        return FALSE;
    }
    let expected =
        unsafe { std::slice::from_raw_parts(bytes, bit_length.div_ceil(8) as usize) };
    let actual = extract_bits(payload, offset, bit_length);
    // Both sides are zero-padded except possibly the expected constant's
    // last byte; mask it.
    for (index, actual_byte) in actual.iter().enumerate() {
        let mut expected_byte = expected[index];
        let bits_here = (bit_length - 8 * index as u64).min(8);
        expected_byte &= 0xFFu8 << (8 - bits_here) as u32;
        if *actual_byte != expected_byte {
            return FALSE;
        }
    }
    TRUE
}

/// Whether the rest of the array past (tagged) bit offset `offset` is a
/// whole number of bytes.
pub extern "C" fn gleam_native_bitarray_rest_is_bytes(array: u64, offset: u64) -> u64 {
    let offset = untag_bits(offset, "offset");
    if (bitarray_value(array).bits.saturating_sub(offset)) % 8 == 0 {
        TRUE
    } else {
        FALSE
    }
}

/// Reads an integer of `bits` (tagged) bits at (tagged) bit offset
/// `offset`.
pub extern "C" fn gleam_native_bitarray_read_int(
    array: u64,
    offset: u64,
    bits: u64,
    endian: u64,
    signed: u64,
) -> u64 {
    let offset = untag_bits(offset, "offset");
    let bits = untag_bits(bits, "segment size");
    retag(read_int_bits(
        bitarray_value(array),
        offset,
        bits,
        is_little(endian),
        signed != 0,
    ))
}

/// Reads a float of `bits` (tagged) bits (16, 32 or 64) at (tagged) bit
/// offset `offset`.
pub extern "C" fn gleam_native_bitarray_read_float(
    array: u64,
    offset: u64,
    bits: u64,
    endian: u64,
) -> u64 {
    let offset = untag_bits(offset, "offset");
    let bits = untag_bits(bits, "segment size");
    let bytes = extract_bits(bitarray_value(array), offset, bits);
    box_float(float_from_bytes(&bytes, is_little(endian)))
}

/// Whether the float at the given position is finite (not NaN or
/// infinity); float segments only match finite values.
pub extern "C" fn gleam_native_bitarray_is_finite_float(
    array: u64,
    offset: u64,
    bits: u64,
    endian: u64,
) -> u64 {
    let offset = untag_bits(offset, "offset");
    let bits = untag_bits(bits, "segment size");
    let payload = bitarray_value(array);
    if offset + bits > payload.bits {
        return FALSE;
    }
    let bytes = extract_bits(payload, offset, bits);
    if float_from_bytes(&bytes, is_little(endian)).is_finite() {
        TRUE
    } else {
        FALSE
    }
}

/// A new bit array holding `bits` (tagged) bits from (tagged) bit offset
/// `offset`, or everything from `offset` onwards when `has_bits` is zero.
pub extern "C" fn gleam_native_bitarray_slice(
    array: u64,
    offset: u64,
    bits: u64,
    has_bits: u64,
) -> u64 {
    let offset = untag_bits(offset, "offset");
    let payload = bitarray_value(array);
    let length = if has_bits == 0 {
        payload.bits.saturating_sub(offset)
    } else {
        untag_bits(bits, "segment size")
    };
    let bytes = extract_bits(payload, offset, length);
    box_bitarray(BitArrayPayload {
        bits: length,
        bytes,
    })
}

/// Allocates a custom type record: a header word encoding the variant tag,
/// field count, and display id, followed by `arity` field words which
/// generated code stores immediately after this call.
pub extern "C" fn gleam_native_record_new(tag: u64, arity: u64, display: u64) -> u64 {
    let value = allocate_words(1 + arity as usize);
    unsafe {
        *(value as *mut u64) = record_header(tag as u32, arity as u32) | (display << 48)
    };
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
            KIND_BITARRAY => {
                drop(unsafe { Box::from_raw(container::<BitArrayPayload>(value)) })
            }
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
    let left_header = heap_header(left) & HEADER_SEMANTIC_MASK;
    if left_header != heap_header(right) & HEADER_SEMANTIC_MASK {
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
        KIND_BITARRAY => {
            let left = bitarray_value(left);
            let right = bitarray_value(right);
            left.bits == right.bits && left.bytes == right.bytes
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
        KIND_BITARRAY => {
            let payload = bitarray_value(value);
            let whole = (payload.bits / 8) as usize;
            let mut parts: Vec<String> = payload.bytes[..whole]
                .iter()
                .map(|byte| byte.to_string())
                .collect();
            let leftover = payload.bits % 8;
            if leftover > 0 {
                let partial = payload.bytes[whole] >> (8 - leftover);
                parts.push(format!("{partial}:size({leftover})"));
            }
            format!("<<{}>>", parts.join(", "))
        }
        KIND_RECORD => {
            let fields = |value: u64, header: u64| -> Vec<String> {
                (0..record_arity(header))
                    .map(|index| inspect(record_field(value, index)))
                    .collect()
            };
            match record_display(header) {
                DISPLAY_TUPLE => format!("#({})", fields(value, header).join(", ")),
                DISPLAY_LIST => {
                    // Walk the cons chain.
                    let mut items = Vec::new();
                    let mut current = value;
                    while current & 1 == 0 {
                        items.push(inspect(record_field(current, 0)));
                        current = record_field(current, 1);
                    }
                    format!("[{}]", items.join(", "))
                }
                display => match constructor_name(display) {
                    Some(name) if record_arity(header) == 0 => name.to_string(),
                    Some(name) => format!("{name}({})", fields(value, header).join(", ")),
                    None => {
                        format!("@{}({})", record_tag(header), fields(value, header).join(", "))
                    }
                },
            }
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
/// `kind` is 0 for `panic`, 1 for `todo`, 2 for `let assert`, and 3 for
/// `assert`. `message`
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
        3 => ("assert", "Assertion failed"),
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
            "gleam_native_string_starts_with",
            gleam_native_string_starts_with as *const u8,
        ),
        (
            "gleam_native_string_slice_from",
            gleam_native_string_slice_from as *const u8,
        ),
        (
            "gleam_native_start_arguments",
            gleam_native_start_arguments as *const u8,
        ),
        ("gleam_native_exit", gleam_native_exit as *const u8),
        (
            "gleam_native_string_byte_size",
            gleam_native_string_byte_size as *const u8,
        ),
        (
            "gleam_native_string_length",
            gleam_native_string_length as *const u8,
        ),
        (
            "gleam_native_string_compare",
            gleam_native_string_compare as *const u8,
        ),
        (
            "gleam_native_string_uppercase",
            gleam_native_string_uppercase as *const u8,
        ),
        (
            "gleam_native_string_lowercase",
            gleam_native_string_lowercase as *const u8,
        ),
        (
            "gleam_native_string_reverse",
            gleam_native_string_reverse as *const u8,
        ),
        (
            "gleam_native_string_contains",
            gleam_native_string_contains as *const u8,
        ),
        (
            "gleam_native_string_ends_with",
            gleam_native_string_ends_with as *const u8,
        ),
        (
            "gleam_native_string_trim",
            gleam_native_string_trim as *const u8,
        ),
        (
            "gleam_native_string_trim_start",
            gleam_native_string_trim_start as *const u8,
        ),
        (
            "gleam_native_string_trim_end",
            gleam_native_string_trim_end as *const u8,
        ),
        (
            "gleam_native_string_slice",
            gleam_native_string_slice as *const u8,
        ),
        (
            "gleam_native_string_replace",
            gleam_native_string_replace as *const u8,
        ),
        (
            "gleam_native_string_split",
            gleam_native_string_split as *const u8,
        ),
        (
            "gleam_native_string_pop_grapheme",
            gleam_native_string_pop_grapheme as *const u8,
        ),
        (
            "gleam_native_string_graphemes",
            gleam_native_string_graphemes as *const u8,
        ),
        (
            "gleam_native_string_to_codepoints",
            gleam_native_string_to_codepoints as *const u8,
        ),
        (
            "gleam_native_string_from_codepoints",
            gleam_native_string_from_codepoints as *const u8,
        ),
        (
            "gleam_native_int_to_string",
            gleam_native_int_to_string as *const u8,
        ),
        (
            "gleam_native_float_to_string",
            gleam_native_float_to_string as *const u8,
        ),
        (
            "gleam_native_bitarray_empty",
            gleam_native_bitarray_empty as *const u8,
        ),
        (
            "gleam_native_bitarray_append_int",
            gleam_native_bitarray_append_int as *const u8,
        ),
        (
            "gleam_native_bitarray_append_string",
            gleam_native_bitarray_append_string as *const u8,
        ),
        (
            "gleam_native_bitarray_append_bits",
            gleam_native_bitarray_append_bits as *const u8,
        ),
        (
            "gleam_native_bitarray_append_float",
            gleam_native_bitarray_append_float as *const u8,
        ),
        (
            "gleam_native_bitarray_append_codepoint",
            gleam_native_bitarray_append_codepoint as *const u8,
        ),
        (
            "gleam_native_bitarray_read_float",
            gleam_native_bitarray_read_float as *const u8,
        ),
        (
            "gleam_native_bitarray_is_finite_float",
            gleam_native_bitarray_is_finite_float as *const u8,
        ),
        (
            "gleam_native_bitarray_size_test",
            gleam_native_bitarray_size_test as *const u8,
        ),
        (
            "gleam_native_bitarray_bytes_test",
            gleam_native_bitarray_bytes_test as *const u8,
        ),
        (
            "gleam_native_bitarray_rest_is_bytes",
            gleam_native_bitarray_rest_is_bytes as *const u8,
        ),
        (
            "gleam_native_bitarray_read_int",
            gleam_native_bitarray_read_int as *const u8,
        ),
        (
            "gleam_native_bitarray_slice",
            gleam_native_bitarray_slice as *const u8,
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
        let record = gleam_native_record_new(tag, fields.len() as u64, 9);
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
    fn bit_array_operations() {
        // <<1, 258:16, "ok":utf8>> — 258 big-endian is [1, 2].
        let bits = |n: i64| tag_small_int(n);
        let array = gleam_native_bitarray_empty();
        let array = gleam_native_bitarray_append_int(array, tag_small_int(1), bits(8), 0);
        let array = gleam_native_bitarray_append_int(array, tag_small_int(258), bits(16), 0);
        let array =
            unsafe { gleam_native_bitarray_append_string(array, make_string("ok"), 0, 0) };
        assert_eq!(bitarray_value(array).bytes, vec![1, 1, 2, b'o', b'k']);

        assert_eq!(gleam_native_bitarray_size_test(array, bits(40), 1), TRUE);
        assert_eq!(gleam_native_bitarray_size_test(array, bits(16), 0), TRUE);
        assert_eq!(gleam_native_bitarray_size_test(array, bits(48), 0), FALSE);

        let expected = [1u8, 2];
        assert_eq!(
            unsafe { gleam_native_bitarray_bytes_test(array, bits(8), expected.as_ptr(), 16) },
            TRUE
        );

        assert_eq!(
            gleam_native_bitarray_read_int(array, bits(8), bits(16), 0, 0),
            tag_small_int(258)
        );
        // Little-endian read of the same bytes: 0x0201.
        assert_eq!(
            gleam_native_bitarray_read_int(array, bits(8), bits(16), 1, 0),
            tag_small_int(513)
        );
        // Signed read of 0xFF is -1.
        let negative = gleam_native_bitarray_empty();
        let negative = gleam_native_bitarray_append_int(negative, tag_small_int(-1), bits(8), 0);
        assert_eq!(
            gleam_native_bitarray_read_int(negative, bits(0), bits(8), 0, 1),
            tag_small_int(-1)
        );
        assert_eq!(
            gleam_native_bitarray_read_int(negative, bits(0), bits(8), 0, 0),
            tag_small_int(255)
        );

        // A 72-bit unsigned read produces a big integer.
        let wide = gleam_native_bitarray_empty();
        let wide = gleam_native_bitarray_append_int(wide, tag_small_int(-1), bits(72), 0);
        let read = gleam_native_bitarray_read_int(wide, bits(0), bits(72), 0, 0);
        assert_eq!(untag(read), (BigInt::from(1) << 72) - 1);
        assert_eq!(
            gleam_native_bitarray_read_int(wide, bits(0), bits(72), 0, 1),
            tag_small_int(-1)
        );

        // Erlang reference values: <<1000:12>> is <<62, 8:4>> and
        // <<1000:12/little>> is <<232, 3:4>>.
        let unaligned = gleam_native_bitarray_empty();
        let unaligned =
            gleam_native_bitarray_append_int(unaligned, tag_small_int(1000), bits(12), 0);
        assert_eq!(bitarray_value(unaligned).bytes, vec![62, 0b1000_0000]);
        assert_eq!(bitarray_value(unaligned).bits, 12);
        assert_eq!(
            gleam_native_bitarray_read_int(unaligned, bits(0), bits(12), 0, 0),
            tag_small_int(1000)
        );
        assert_eq!(inspect(unaligned), "<<62, 8:size(4)>>");
        let little_unaligned = gleam_native_bitarray_empty();
        let little_unaligned =
            gleam_native_bitarray_append_int(little_unaligned, tag_small_int(1000), bits(12), 1);
        assert_eq!(bitarray_value(little_unaligned).bytes, vec![232, 0b0011_0000]);
        assert_eq!(
            gleam_native_bitarray_read_int(little_unaligned, bits(0), bits(12), 1, 0),
            tag_small_int(1000)
        );
        // An unaligned offset read: skip 4 bits of <<62, 8:4>> and read 8.
        assert_eq!(
            gleam_native_bitarray_read_int(unaligned, bits(4), bits(8), 0, 0),
            tag_small_int(0b1110_1000)
        );

        // Floats at all three widths round-trip, and finiteness is testable.
        let floats = gleam_native_bitarray_empty();
        let floats = unsafe {
            gleam_native_bitarray_append_float(
                floats,
                gleam_native_float_from_bits(1.5f64.to_bits()),
                bits(16),
                0,
            )
        };
        let floats = unsafe {
            gleam_native_bitarray_append_float(
                floats,
                gleam_native_float_from_bits(2.5f64.to_bits()),
                bits(32),
                1,
            )
        };
        let floats = unsafe {
            gleam_native_bitarray_append_float(
                floats,
                gleam_native_float_from_bits(3.25f64.to_bits()),
                bits(64),
                0,
            )
        };
        assert_eq!(
            float_value(gleam_native_bitarray_read_float(floats, bits(0), bits(16), 0)),
            1.5
        );
        assert_eq!(
            float_value(gleam_native_bitarray_read_float(floats, bits(16), bits(32), 1)),
            2.5
        );
        assert_eq!(
            float_value(gleam_native_bitarray_read_float(floats, bits(48), bits(64), 0)),
            3.25
        );
        assert_eq!(
            gleam_native_bitarray_is_finite_float(floats, bits(0), bits(16), 0),
            TRUE
        );

        // UTF-16 big-endian "hi" is <<0, 104, 0, 105>>; a codepoint appends
        // its encoding.
        let utf = gleam_native_bitarray_empty();
        let utf = unsafe { gleam_native_bitarray_append_string(utf, make_string("hi"), 1, 0) };
        assert_eq!(bitarray_value(utf).bytes, vec![0, 104, 0, 105]);
        let utf = gleam_native_bitarray_append_codepoint(utf, tag_small_int(0x1F600), 0, 0);
        assert_eq!(
            &bitarray_value(utf).bytes[4..],
            "\u{1F600}".as_bytes()
        );

        let rest = gleam_native_bitarray_slice(array, bits(24), bits(0), 0);
        assert_eq!(bitarray_value(rest).bytes, vec![b'o', b'k']);
        let sized = gleam_native_bitarray_slice(array, bits(0), bits(16), 1);
        assert_eq!(bitarray_value(sized).bytes, vec![1, 1]);

        // Structural equality and inspection.
        let again = gleam_native_bitarray_slice(array, bits(0), bits(0), 0);
        assert_eq!(gleam_native_eq(array, again), TRUE);
        assert_eq!(gleam_native_eq(array, rest), FALSE);
        assert_eq!(inspect(sized), "<<1, 1>>");
    }

    #[test]
    fn string_operations() {
        let s = |content: &str| make_string(content);
        unsafe {
            // Grapheme-aware length: the family emoji is one grapheme made
            // of several code points.
            assert_eq!(gleam_native_string_length(s("héllo")), tag_small_int(5));
            assert_eq!(
                gleam_native_string_length(s("\u{1F469}\u{200D}\u{1F469}\u{200D}\u{1F466}")),
                tag_small_int(1)
            );
            assert_eq!(gleam_native_string_byte_size(s("héllo")), tag_small_int(6));
            assert_eq!(string_value(gleam_native_string_reverse(s("noé"))), "éon");

            // Bytewise comparison is code point order.
            assert_eq!(
                gleam_native_string_compare(s("apple"), s("banana")),
                tag_small_int(-1)
            );
            assert_eq!(
                gleam_native_string_compare(s("a"), s("a")),
                tag_small_int(0)
            );
            assert_eq!(
                gleam_native_string_compare(s("é"), s("z")),
                tag_small_int(1)
            );

            assert_eq!(
                string_value(gleam_native_string_uppercase(s("héllo"))),
                "HÉLLO"
            );
            assert_eq!(
                string_value(gleam_native_string_lowercase(s("HÉLLO"))),
                "héllo"
            );
            assert_eq!(gleam_native_string_contains(s("hello"), s("ell")), TRUE);
            assert_eq!(gleam_native_string_ends_with(s("hello"), s("llo")), TRUE);
            assert_eq!(string_value(gleam_native_string_trim(s("  hi  "))), "hi");
            assert_eq!(
                string_value(gleam_native_string_trim_start(s("  hi  "))),
                "hi  "
            );
            assert_eq!(
                string_value(gleam_native_string_slice(
                    s("héllo"),
                    tag_small_int(1),
                    tag_small_int(3)
                )),
                "éll"
            );
            assert_eq!(
                string_value(gleam_native_string_replace(s("a-b-c"), s("-"), s("+"))),
                "a+b+c"
            );

            // Splitting builds a proper list.
            let parts = gleam_native_string_split(s("a,b,c"), s(","));
            assert_eq!(string_value(record_field(parts, 0)), "a");
            let rest = record_field(parts, 1);
            assert_eq!(string_value(record_field(rest, 0)), "b");

            // pop_grapheme returns Ok(#(head, rest)) and Error(Nil).
            let popped = gleam_native_string_pop_grapheme(s("héllo"));
            assert_eq!(record_tag(heap_header(popped)), 0);
            let pair = record_field(popped, 0);
            assert_eq!(string_value(record_field(pair, 0)), "h");
            assert_eq!(string_value(record_field(pair, 1)), "éllo");
            let empty = gleam_native_string_pop_grapheme(s(""));
            assert_eq!(record_tag(heap_header(empty)), 1);

            // Code points round-trip.
            let codepoints = gleam_native_string_to_codepoints(s("hé"));
            assert_eq!(record_field(codepoints, 0), tag_small_int(104));
            assert_eq!(
                string_value(gleam_native_string_from_codepoints(codepoints)),
                "hé"
            );

            // Number rendering.
            let big = gleam_native_int_add_slow(tag_small_int(SMALL_INT_MAX), tag_small_int(1));
            assert_eq!(
                string_value(gleam_native_int_to_string(big)),
                "4611686018427387904"
            );
            assert_eq!(
                string_value(gleam_native_float_to_string(gleam_native_float_from_bits(
                    2.5f64.to_bits()
                ))),
                "2.5"
            );
        }
    }

    #[test]
    fn string_prefix_operations() {
        let subject = make_string("héllo world");
        let prefix = "héllo ";
        assert_eq!(
            unsafe {
                gleam_native_string_starts_with(subject, prefix.as_ptr(), prefix.len() as u64)
            },
            TRUE
        );
        let wrong = "hello";
        assert_eq!(
            unsafe {
                gleam_native_string_starts_with(subject, wrong.as_ptr(), wrong.len() as u64)
            },
            FALSE
        );
        let rest = unsafe { gleam_native_string_slice_from(subject, prefix.len() as u64) };
        assert_eq!(string_value(rest), "world");
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
