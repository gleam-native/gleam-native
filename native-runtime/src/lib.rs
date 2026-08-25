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
//! These functions use the platform C calling convention. The JIT registers
//! them by name via [`symbols`]; ahead-of-time compilation links against the
//! `native-runtime-static` static library, so each carries `#[no_mangle]` to
//! keep its symbol name.

use std::cell::Cell;

use compact_str::CompactString;
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
/// A standard library dict, created by the runtime's dict functions rather
/// than by generated code.
pub const KIND_DICT: u64 = 6;

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
pub const DISPLAY_SOME: u16 = 4;
pub const DISPLAY_NONE: u16 = 5;
pub const FIRST_INTERNED_DISPLAY: u16 = 6;

pub fn record_display(header: u64) -> u16 {
    (header >> 48) as u16
}

/// The interned constructor names for display ids from
/// [`FIRST_INTERNED_DISPLAY`] upwards, set by the host before `main` runs.
static CONSTRUCTOR_NAMES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();

/// Stores the interned constructor names; called by the host.
pub fn set_constructor_names(names: Vec<String>) {
    let _ = CONSTRUCTOR_NAMES.set(names);
}

pub(crate) fn constructor_name(display: u16) -> Option<&'static str> {
    match display {
        DISPLAY_OK => Some("Ok"),
        DISPLAY_ERROR => Some("Error"),
        DISPLAY_SOME => Some("Some"),
        DISPLAY_NONE => Some("None"),
        _ => CONSTRUCTOR_NAMES
            .get()?
            .get((display - FIRST_INTERNED_DISPLAY) as usize)
            .map(|name| name.as_str()),
    }
}

/// The closure header: capture count where records keep their arity, and
/// the function's own arity (without the closure argument) where records
/// keep their tag, read back by `echo`'s function rendering.
pub const fn closure_header(captures: u32, arity: u32) -> u64 {
    KIND_CLOSURE | ((arity as u64) << 16) | ((captures as u64) << 32)
}

pub fn header_kind(header: u64) -> u64 {
    header & 0xFFFF
}

pub(crate) fn record_tag(header: u64) -> u32 {
    ((header >> 16) & 0xFFFF) as u32
}

pub fn record_arity(header: u64) -> u32 {
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
    // Through the pooled allocator: the fixed-size shell recycles like any
    // record, only the payload's own buffers (if any) touch the system
    // allocator. All payload types are word-aligned.
    let words = std::mem::size_of::<HeapBox<T>>() / 8 - 1;
    let boxed = allocate_words(words);
    unsafe {
        *(boxed as *mut u64) = kind;
        std::ptr::write((boxed + 8) as *mut T, value);
    }
    boxed
}

/// Frees a [`box_heap`] allocation: drops the payload in place and
/// returns the shell to the pool.
fn free_payload<T>(value: u64) {
    unsafe {
        std::ptr::drop_in_place(&raw mut (*container::<T>(value)).value);
    }
    free_words((value - 8) as *mut u64, std::mem::size_of::<HeapBox<T>>() / 8);
}

fn container<T>(value: u64) -> *mut HeapBox<T> {
    (value - 8) as *mut HeapBox<T>
}

pub(crate) fn box_bigint(value: BigInt) -> u64 {
    box_heap(KIND_BIGINT, value)
}

pub fn box_float(value: f64) -> u64 {
    // Floats go through the pooled word allocator rather than [`box_heap`]:
    // the layouts agree ([`HeapBox<f64>`] is three words), and float churn
    // in arithmetic loops is exactly what the pool recycles best.
    let boxed = allocate_words(2);
    unsafe {
        *(boxed as *mut u64) = KIND_FLOAT;
        *((boxed as *mut u64).add(1)) = value.to_bits();
    }
    boxed
}

pub fn box_string(value: impl Into<CompactString>) -> u64 {
    box_heap(KIND_STRING, value.into())
}

pub fn heap_header(value: u64) -> u64 {
    unsafe { *(value as *const u64) }
}

pub(crate) fn bigint_value(value: u64) -> &'static BigInt {
    unsafe { &(*container::<BigInt>(value)).value }
}

pub fn float_value(value: u64) -> f64 {
    unsafe { (*container::<f64>(value)).value }
}

pub fn string_value(value: u64) -> &'static CompactString {
    unsafe { &(*container::<CompactString>(value)).value }
}

/// A bit array's payload: a bit count and MSB-first packed bytes, the
/// last byte zero-padded in its unused low bits.
pub struct BitArrayPayload {
    pub bits: u64,
    pub bytes: Vec<u8>,
}

pub fn box_bitarray(payload: BitArrayPayload) -> u64 {
    box_heap(KIND_BITARRAY, payload)
}

pub fn bitarray_value(value: u64) -> &'static BitArrayPayload {
    unsafe { &(*container::<BitArrayPayload>(value)).value }
}

pub(crate) fn bitarray_value_mut(value: u64) -> &'static mut BitArrayPayload {
    unsafe { &mut (*container::<BitArrayPayload>(value)).value }
}

/// Aborts on an invalid bit array operation. These are runtime limitations
/// or bounds violations, reported like a panic.
pub(crate) fn bitarray_panic(message: &str) -> ! {
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
pub fn append_bits(payload: &mut BitArrayPayload, source: &[u8], offset: u64, length: u64) {
    if payload.bits.is_multiple_of(8) && offset.is_multiple_of(8) && length.is_multiple_of(8) {
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
pub(crate) fn extract_bits(payload: &BitArrayPayload, offset: u64, length: u64) -> Vec<u8> {
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

pub fn record_field(value: u64, index: u32) -> u64 {
    unsafe { *((value as *const u64).add(1 + index as usize)) }
}

pub fn untag(value: u64) -> BigInt {
    if value & 1 == 1 {
        BigInt::from((value as i64) >> 1)
    } else {
        bigint_value(value).clone()
    }
}

pub fn retag(value: BigInt) -> u64 {
    match value.to_i64() {
        Some(small) if (SMALL_INT_MIN..=SMALL_INT_MAX).contains(&small) => tag_small_int(small),
        _ => box_bigint(value),
    }
}

/// The slow path for integer addition, called by generated code when either
/// operand is a big integer or when small-integer addition overflowed.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_add_slow(left: u64, right: u64) -> u64 {
    retag(untag(left) + untag(right))
}

/// The slow path for integer subtraction; see [`gleam_native_int_add_slow`].
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_sub_slow(left: u64, right: u64) -> u64 {
    retag(untag(left) - untag(right))
}

/// The slow path for integer multiplication; see
/// [`gleam_native_int_add_slow`].
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_mul_slow(left: u64, right: u64) -> u64 {
    retag(untag(left) * untag(right))
}

/// Three-way integer comparison for when either operand is a big integer:
/// returns the tagged small integer -1, 0, or 1.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_compare(left: u64, right: u64) -> u64 {
    tag_small_int(match untag(left).cmp(&untag(right)) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    })
}

/// Truncating integer division. Division by zero yields zero, following
/// Gleam's semantics on every target.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_div(left: u64, right: u64) -> u64 {
    let divisor = untag(right);
    if divisor == BigInt::ZERO {
        return tag_small_int(0);
    }
    retag(untag(left) / divisor)
}

/// Integer remainder, taking the sign of the dividend. A zero divisor
/// yields zero, following Gleam's semantics on every target.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_bigint_from_bytes(bytes: *const u8, length: u64) -> u64 {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, length as usize) };
    retag(BigInt::from_signed_bytes_le(bytes))
}

/// Boxes a float value given its IEEE 754 bit pattern. Taking the bits as an
/// integer keeps every generated call signature uniformly i64.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_from_bytes(bytes: *const u8, length: u64) -> u64 {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, length as usize) };
    let string = unsafe { std::str::from_utf8_unchecked(bytes) };
    box_string(string)
}

/// Concatenates two strings into a new string, the implementation of the
/// `<>` operator.
///
/// # Safety
///
/// Both arguments must be strings created by this runtime. The Gleam type
/// system upholds this for calls from generated code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_concat(left: u64, right: u64) -> u64 {
    let left = string_value(left);
    let right = string_value(right);
    let mut result = CompactString::with_capacity(left.len() + right.len());
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_slice_from(subject: u64, offset: u64) -> u64 {
    box_string(&string_value(subject)[offset as usize..])
}

/// Builds a two-element tuple (a record with tag 0).
pub fn make_tuple2(first: u64, second: u64) -> u64 {
    let record = gleam_native_record_new(0, 2, DISPLAY_TUPLE as u64);
    unsafe {
        *((record as *mut u64).add(1)) = first;
        *((record as *mut u64).add(2)) = second;
    }
    record
}

/// Builds an `Ok` value (variant 0 of `Result`).
pub fn make_ok(value: u64) -> u64 {
    let record = gleam_native_record_new(0, 1, DISPLAY_OK as u64);
    unsafe { *((record as *mut u64).add(1)) = value };
    record
}

/// Builds an `Error` value (variant 1 of `Result`).
pub fn make_error(value: u64) -> u64 {
    let record = gleam_native_record_new(1, 1, DISPLAY_ERROR as u64);
    unsafe { *((record as *mut u64).add(1)) = value };
    record
}

/// Builds a list (cons cells with tag 1) from already-owned values.
pub fn make_list(values: Vec<u64>) -> u64 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_byte_size(string: u64) -> u64 {
    tag_small_int(string_value(string).len() as i64)
}

/// The string's length in grapheme clusters.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_uppercase(string: u64) -> u64 {
    box_string(string_value(string).to_uppercase())
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_lowercase(string: u64) -> u64 {
    box_string(string_value(string).to_lowercase())
}

/// The string with its grapheme clusters in reverse order.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_reverse(string: u64) -> u64 {
    box_string(string_value(string).graphemes(true).rev().collect::<CompactString>())
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_contains(string: u64, needle: u64) -> u64 {
    if string_value(string).contains(string_value(needle).as_str()) {
        TRUE
    } else {
        FALSE
    }
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_ends_with(string: u64, suffix: u64) -> u64 {
    if string_value(string).ends_with(string_value(suffix).as_str()) {
        TRUE
    } else {
        FALSE
    }
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_trim(string: u64) -> u64 {
    box_string(string_value(string).trim_matches(TRIMMED_WHITESPACE))
}

/// The whitespace characters `string.trim` removes on every target: ASCII
/// whitespace plus next line and the line and paragraph separators, but
/// not the various Unicode spaces.
const TRIMMED_WHITESPACE: &[char] = &[
    ' ', '\t', '\n', '\u{B}', '\u{C}', '\r', '\u{85}', '\u{2028}', '\u{2029}',
];

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_trim_start(string: u64) -> u64 {
    box_string(string_value(string).trim_start_matches(TRIMMED_WHITESPACE))
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_trim_end(string: u64) -> u64 {
    box_string(string_value(string).trim_end_matches(TRIMMED_WHITESPACE))
}

/// The `length` grapheme clusters starting at grapheme index `start`
/// (both tagged, clamped to the string).
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_slice(string: u64, start: u64, length: u64) -> u64 {
    let start = ((start as i64) >> 1).max(0) as usize;
    let length = ((length as i64) >> 1).max(0) as usize;
    box_string(
        string_value(string)
            .graphemes(true)
            .skip(start)
            .take(length)
            .collect::<CompactString>(),
    )
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_replace(
    string: u64,
    pattern: u64,
    replacement: u64,
) -> u64 {
    box_string(string_value(string).replace(string_value(pattern).as_str(), string_value(replacement)))
}

/// Splits on a separator, returning a list of strings. An empty separator
/// yields the whole string as a single element, like the other targets.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_split(string: u64, on: u64) -> u64 {
    let string = string_value(string);
    let on = string_value(on);
    if on.is_empty() {
        return make_list(vec![box_string(string.as_str())]);
    }
    make_list(
        string
            .split(on.as_str())
            .map(box_string)
            .collect(),
    )
}

/// The first grapheme cluster and the rest: `Ok(#(head, rest))`, or
/// `Error(Nil)` for the empty string.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_pop_grapheme(string: u64) -> u64 {
    let string = string_value(string);
    match string.grapheme_indices(true).next() {
        None => make_error(NIL),
        Some((_, head)) => {
            make_ok(make_tuple2(
                box_string(head),
                box_string(&string[head.len()..]),
            ))
        }
    }
}

/// The string's grapheme clusters as a list of strings.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_graphemes(string: u64) -> u64 {
    make_list(
        string_value(string)
            .graphemes(true)
            .map(box_string)
            .collect(),
    )
}

/// The string's Unicode code points as a list of tagged integers.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_to_string(value: u64) -> u64 {
    box_string(untag(value).to_string())
}


/// Renders a float the way Gleam writes floats: always with a decimal
/// point, keeping exponent notation, and naming the non-finite values.
pub(crate) fn format_float(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let rendered = format!("{value:?}");
    match rendered.find('e') {
        Some(position) if !rendered[..position].contains('.') => {
            format!("{}.0{}", &rendered[..position], &rendered[position..])
        }
        _ => rendered,
    }
}

/// A float rendered the way Gleam writes floats.
///
/// # Safety
///
/// The argument must be a float created by this runtime.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_float_to_string(value: u64) -> u64 {
    box_string(format_float(float_value(value)))
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

/// Runs the program's entry wrapper on the dedicated "gleam-main" thread
/// with the given stack size (in megabytes) and the stack overflow handler
/// installed. Tail calls run in constant space, and deep non-tail recursion
/// gets generous room before the overflow handler reports it. A minimum of
/// one megabyte keeps a misconfigured project able to reach `main` at all.
pub fn run_program_thread(
    stack_size_megabytes: u64,
    entry: extern "C" fn() -> u64,
) -> Result<(), String> {
    run_program_thread_with(stack_size_megabytes, move || {
        let _ = entry();
    })
}

/// [`run_program_thread`] for an arbitrary body: used by the test runner,
/// whose entry is a Rust loop over the compiled test functions.
pub fn run_program_thread_with(
    stack_size_megabytes: u64,
    body: impl FnOnce() + Send + 'static,
) -> Result<(), String> {
    let stack_size = usize::try_from(stack_size_megabytes.max(1))
        .unwrap_or(usize::MAX)
        .saturating_mul(1024 * 1024);
    std::thread::Builder::new()
        .name("gleam-main".into())
        .stack_size(stack_size)
        .spawn(move || {
            publish_pool();
            install_stack_overflow_handler();
            body();
        })
        .map_err(|error| format!("could not start the program thread: {error}"))?
        .join()
        .map_err(|_| "the program crashed".to_string())?;
    Ok(())
}

/// The state of a `gleam test` run: the discovered tests and the run's
/// progress. Present only while tests are executing; its presence is what
/// switches [`gleam_native_panic`] into recoverable test mode.
struct TestRun {
    /// Test display names and their C-convention entry wrappers.
    tests: Vec<(String, extern "C" fn() -> u64)>,
    /// The index of the next test to start.
    next: std::sync::atomic::AtomicUsize,
    /// How many tests have failed so far.
    failed: std::sync::atomic::AtomicUsize,
}

static TEST_RUN: std::sync::OnceLock<TestRun> = std::sync::OnceLock::new();

/// Runs the given tests in order on the current thread, reporting each
/// outcome, then exits the process: 0 if every test passed, 1 otherwise.
///
/// A failing test panics into [`gleam_native_panic`], which reports the
/// failure and resumes the run by calling [`continue_test_run`] rather than
/// returning through the failed test's stack frames. Those frames (and the
/// failed test's allocations) are simply abandoned — the process exits when
/// the run finishes, so the leak is harmless. A stack overflow is not
/// recoverable this way and still aborts the whole run.
pub fn run_tests(tests: Vec<(String, extern "C" fn() -> u64)>) -> ! {
    let _ = TEST_RUN.set(TestRun {
        tests,
        next: std::sync::atomic::AtomicUsize::new(0),
        failed: std::sync::atomic::AtomicUsize::new(0),
    });
    continue_test_run()
}

/// Runs every not-yet-started test to completion, then reports the summary
/// and exits the process. Called from [`run_tests`] and re-entered by
/// [`gleam_native_panic`] after a test fails.
fn continue_test_run() -> ! {
    use std::sync::atomic::Ordering;
    let run = TEST_RUN.get().expect("a test run is in progress");
    loop {
        let index = run.next.fetch_add(1, Ordering::SeqCst);
        let Some((name, function)) = run.tests.get(index) else {
            break;
        };
        let _ = function();
        println!("  PASS {name}");
    }
    let total = run.tests.len();
    let failed = run.failed.load(Ordering::SeqCst);
    let plural = if failed == 1 { "failure" } else { "failures" };
    println!();
    println!("Ran {total} tests, {failed} {plural}");
    std::process::exit(if failed == 0 { 0 } else { 1 });
}

/// The symbol of the program data blob that ahead-of-time compilation embeds
/// in the executable, decoded by [`start`] before the program runs.
pub const PROGRAM_DATA_SYMBOL: &str = "gleam_native_program_data";

/// Encodes the program data blob for [`PROGRAM_DATA_SYMBOL`]. Layout, with
/// all integers little-endian: the stack size in megabytes (u64), the
/// constructor name count (u32), then each name as a u32 byte length
/// followed by that many bytes of UTF-8.
pub fn encode_program_data(stack_size_megabytes: u64, constructor_names: &[String]) -> Vec<u8> {
    let mut data = stack_size_megabytes.to_le_bytes().to_vec();
    let count = u32::try_from(constructor_names.len()).expect("constructor count fits in u32");
    data.extend_from_slice(&count.to_le_bytes());
    for name in constructor_names {
        let length = u32::try_from(name.len()).expect("constructor name fits in u32");
        data.extend_from_slice(&length.to_le_bytes());
        data.extend_from_slice(name.as_bytes());
    }
    data
}

/// The entry point for ahead-of-time compiled programs, called from the C
/// `main` that the `native-runtime-static` library provides. Stores the
/// command line arguments (without the program name), decodes the embedded
/// program data, and runs the entry wrapper on the program thread. Returns
/// the process exit code.
///
/// # Safety
///
/// `argc`/`argv` must be the values C `main` received, and `program_data`
/// must point at a blob produced by [`encode_program_data`].
pub unsafe fn start(
    argc: i32,
    argv: *const *const std::ffi::c_char,
    program_data: *const u8,
    entry: extern "C" fn() -> u64,
) -> i32 {
    let mut arguments = Vec::new();
    for index in 1..argc.max(0) {
        let argument = unsafe { std::ffi::CStr::from_ptr(*argv.add(index as usize)) };
        arguments.push(argument.to_string_lossy().into_owned());
    }
    set_start_arguments(arguments);

    unsafe fn read_u32(cursor: &mut *const u8) -> u32 {
        let value = unsafe { std::ptr::read_unaligned(*cursor as *const u32) };
        *cursor = unsafe { cursor.add(4) };
        u32::from_le(value)
    }
    let mut cursor = program_data;
    let stack_size_megabytes =
        u64::from_le(unsafe { std::ptr::read_unaligned(cursor as *const u64) });
    cursor = unsafe { cursor.add(8) };
    let count = unsafe { read_u32(&mut cursor) };
    let mut names = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let length = unsafe { read_u32(&mut cursor) } as usize;
        let bytes = unsafe { std::slice::from_raw_parts(cursor, length) };
        cursor = unsafe { cursor.add(length) };
        names.push(String::from_utf8_lossy(bytes).into_owned());
    }
    set_constructor_names(names);

    match run_program_thread(stack_size_megabytes, entry) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("error: {error}");
            1
        }
    }
}

/// The command line arguments as a list of strings.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_start_arguments() -> u64 {
    let arguments = START_ARGUMENTS.get().cloned().unwrap_or_default();
    make_list(
        arguments
            .into_iter()
            .map(box_string)
            .collect(),
    )
}

/// Ends the program immediately with the given exit code.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_exit(code: u64) -> u64 {
    let code = untag(code).to_i32().unwrap_or(1);
    std::process::exit(code);
}

/// A new empty bit array, the start of a construction chain.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_empty() -> u64 {
    box_bitarray(BitArrayPayload {
        bits: 0,
        bytes: Vec::new(),
    })
}

/// Appends an integer segment of `bits` (tagged) bits, truncating the value
/// to the segment size. Construction chains own their array uniquely, so
/// the array is mutated in place and returned.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_append_int(
    array: u64,
    value: u64,
    bits: u64,
    endian: u64,
) -> u64 {
    // A negative size appends nothing, matching Erlang: `<<0:-8>>` is `<<>>`.
    if bits & 1 == 1 && (bits as i64) < 0 {
        return array;
    }
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_bitarray_append_float(
    array: u64,
    value: u64,
    bits: u64,
    endian: u64,
) -> u64 {
    // A negative size appends nothing; see the integer segment case.
    if bits & 1 == 1 && (bits as i64) < 0 {
        return array;
    }
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_rest_is_bytes(array: u64, offset: u64) -> u64 {
    let offset = untag_bits(offset, "offset");
    if bitarray_value(array)
        .bits
        .saturating_sub(offset)
        .is_multiple_of(8)
    {
        TRUE
    } else {
        FALSE
    }
}

/// Reads an integer of `bits` (tagged) bits at (tagged) bit offset
/// `offset`.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_record_new(tag: u64, arity: u64, display: u64) -> u64 {
    let value = allocate_words(1 + arity as usize);
    unsafe {
        *(value as *mut u64) = record_header(tag as u32, arity as u32) | (display << 48)
    };
    value
}

/// Allocates a closure: a header word, a slot for the code pointer, and
/// `captures` capture words, all stored by generated code after this call.
/// `arity` is the function's parameter count, kept for `echo`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_closure_new(captures: u64, arity: u64) -> u64 {
    let value = allocate_words(2 + captures as usize);
    unsafe { *(value as *mut u64) = closure_header(captures as u32, arity as u32) };
    value
}

/// Allocates `words` object words preceded by a reference count of one,
/// returning the value pointer (which points at the first object word).
/// Recycles a freed allocation of the same size from the pool when one is
/// available, falling back to the system allocator.
pub(crate) fn allocate_words(words: usize) -> u64 {
    let total = 1 + words;
    if total < POOL_CLASSES {
        let recycled = POOL.with(|pool| {
            let head = pool.heads[total].get();
            if head != 0 {
                // The count word of a pooled block holds the next block.
                pool.heads[total].set(unsafe { *(head as *const u64) });
                pool.counts[total].set(pool.counts[total].get() - 1);
            }
            head
        });
        if recycled != 0 {
            unsafe { *(recycled as *mut u64) = 1 };
            return recycled + 8;
        }
    }
    let layout = word_layout(words);
    let base = unsafe { std::alloc::alloc(layout) } as *mut u64;
    assert!(!base.is_null(), "heap allocation failed");
    unsafe { *base = 1 };
    base as u64 + 8
}

fn word_layout(words: usize) -> std::alloc::Layout {
    std::alloc::Layout::array::<u64>(1 + words).expect("heap object layout")
}

/// Reference counting frees objects deterministically, and hot loops free
/// and re-request the same shapes over and over (cons cells above all), so
/// freed fixed-size allocations sit on per-size free lists for
/// [`allocate_words`] to hand straight back instead of going through the
/// system allocator each time. A freed block stores the next free block's
/// address in its count word, so the pool needs no memory of its own.
///
/// The layout is part of the code generation contract: generated code pops
/// free lists inline through [`gleam_native_pool`], reading `heads[total]`
/// at byte offset `8 * total` and `counts[total]` at byte offset
/// `8 * (POOL_CLASSES + total)`.
#[repr(C)]
pub struct Pool {
    /// The first free block of each size class (0 when empty), indexed by
    /// the block's total size in words, count word included.
    heads: [Cell<u64>; POOL_CLASSES],
    /// How many blocks each class holds, enforcing
    /// [`POOL_CLASS_CAPACITY`].
    counts: [Cell<u64>; POOL_CLASSES],
}

/// One class per total word count: count and header words plus up to 32
/// fields. Larger objects use the system allocator directly.
pub const POOL_CLASSES: usize = 35;

/// The most blocks a size class retains. Frees beyond this go back to the
/// system allocator, bounding how much freed memory the pool can hold onto
/// after a large working set shrinks.
const POOL_CLASS_CAPACITY: u64 = 4096;

/// The address of the program thread's [`Pool`], published by
/// [`run_program_thread_with`] before any Gleam code runs so that generated
/// code (which runs only on that thread) can pop pooled allocations inline.
/// Zero until published; inline fast paths check and fall back to the
/// runtime's allocation call.
#[unsafe(no_mangle)]
#[allow(non_upper_case_globals)]
pub static gleam_native_pool: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

/// The symbol generated code reads the program thread's pool through.
pub const POOL_SYMBOL: &str = "gleam_native_pool";

/// Publishes the calling thread's pool for generated code.
fn publish_pool() {
    POOL.with(|pool| {
        gleam_native_pool.store(
            pool as *const Pool as u64,
            std::sync::atomic::Ordering::Release,
        )
    });
}

thread_local! {
    static POOL: Pool = const {
        Pool {
            heads: [const { Cell::new(0) }; POOL_CLASSES],
            counts: [const { Cell::new(0) }; POOL_CLASSES],
        }
    };
}

/// Frees an allocation of `total` words (count word included): into the
/// pool when its size class has room, back to the system allocator
/// otherwise.
fn free_words(base: *mut u64, total: usize) {
    let pooled = total < POOL_CLASSES
        && POOL.with(|pool| {
            let count = pool.counts[total].get();
            if count >= POOL_CLASS_CAPACITY {
                return false;
            }
            unsafe { *base = pool.heads[total].get() };
            pool.heads[total].set(base as u64);
            pool.counts[total].set(count + 1);
            true
        });
    if !pooled {
        unsafe {
            std::alloc::dealloc(
                base as *mut u8,
                std::alloc::Layout::array::<u64>(total).expect("heap object layout"),
            )
        };
    }
}

/// Increments a value's reference count. A no-op for immediates.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_inc(value: u64) -> u64 {
    if value & 1 == 0 {
        unsafe { *((value - 8) as *mut u64) += 1 };
    }
    value
}

/// Decrements a value's reference count, destroying the object when it
/// reaches zero. A no-op for immediates. The common cases — an immediate,
/// or a count that stays positive — touch nothing but the count word.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dec(value: u64) -> u64 {
    if value & 1 == 1 {
        return NIL;
    }
    let count = (value - 8) as *mut u64;
    unsafe {
        *count -= 1;
        if *count > 0 {
            return NIL;
        }
    }
    destroy(value);
    NIL
}

/// Destroys an object whose reference count has already reached zero: the
/// slow path behind the decrement generated code emits inline.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_destroy(value: u64) -> u64 {
    destroy(value);
    NIL
}

thread_local! {
    /// The reused worklist buffer for [`destroy`], so a destroy performs no
    /// allocation of its own. Destruction can re-enter itself (a dict's
    /// drop releases its keys and entries); a nested destroy finds the cell
    /// empty and works with a fresh vector.
    static WORKLIST: Cell<Vec<u64>> = const { Cell::new(Vec::new()) };
}

/// Destroys an object whose reference count has just reached zero,
/// decrementing its children via a worklist so that destroying a long list
/// does not recurse.
#[cold]
fn destroy(first: u64) {
    // Only a record or closure with fields holds references the worklist
    // must walk; everything else — strings and other leaf payloads, dicts
    // (whose entries release through their `Drop`), and field-less records
    // — frees directly. The empty vector never allocates: nothing is
    // pushed onto it.
    let header = heap_header(first);
    let kind = header_kind(header);
    if (kind != KIND_RECORD && kind != KIND_CLOSURE) || record_arity(header) == 0 {
        free_object(first, &mut Vec::new());
        return;
    }
    let mut worklist = WORKLIST.take();
    free_object(first, &mut worklist);
    while let Some(value) = worklist.pop() {
        if value & 1 == 1 {
            continue;
        }
        let count = (value - 8) as *mut u64;
        unsafe {
            *count -= 1;
            if *count > 0 {
                continue;
            }
        }
        free_object(value, &mut worklist);
    }
    WORKLIST.set(worklist);
}

/// Frees one object whose count has reached zero, pushing the children it
/// owned onto the worklist.
fn free_object(value: u64, worklist: &mut Vec<u64>) {
    let count = (value - 8) as *mut u64;
    let header = heap_header(value);
    match header_kind(header) {
        KIND_RECORD => {
            let arity = record_arity(header);
            for index in 0..arity {
                worklist.push(record_field(value, index));
            }
            free_words(count, 2 + arity as usize);
        }
        KIND_CLOSURE => {
            let captures = record_arity(header);
            for index in 0..captures {
                // Captures sit one word past the code pointer.
                worklist.push(record_field(value, 1 + index));
            }
            free_words(count, 3 + captures as usize);
        }
        KIND_BIGINT => free_payload::<BigInt>(value),
        // A float box is count, header, and payload: the three-word class.
        KIND_FLOAT => free_words(count, 3),
        KIND_STRING => free_payload::<CompactString>(value),
        KIND_BITARRAY => free_payload::<BitArrayPayload>(value),
        KIND_DICT => {
            // Dropping the persistent map releases exactly the tree
            // nodes no other dict shares; their keys and entries
            // release their references through [`DictKey`] and
            // [`DictEntry`]'s `Drop` implementations.
            free_payload::<DictPayload>(value);
        }
        _ => {}
    }
}


// ---------------------------------------------------------------------------
// Dicts and string trees
//
// These value shapes are part of the core value model — structural
// equality, destruction, and `echo` must understand them — while the
// `native-runtime-stdlib` crate provides the standard library's entry
// points over them.

/// The hasher dict maps use: SipHash with fixed keys, so hashes — and
/// with them the map's unspecified iteration order — are deterministic
/// within a build.
pub type DictHasher = std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>;

/// The map a dict holds: a persistent hash-array-mapped trie whose clones
/// share structure, so an immutable insert path-copies O(log n) nodes
/// instead of copying the map.
pub type DictMap = im_rc::HashMap<DictKey, DictEntry, DictHasher>;

/// The payload of a dict (kind [`KIND_DICT`]). Gleam dicts are unordered,
/// so keys hash structurally (consistent with [`deep_eq`]) rather than
/// sorting by [`cmp_values`]. Keys and values own a reference each,
/// managed by their wrappers' `Clone` and `Drop`: when a path copy clones
/// a shared node its entries take new references, and when the last map
/// holding a node drops it its entries release theirs.
pub struct DictPayload {
    pub map: DictMap,
}

/// A dict key: an owned reference to a Gleam value, hashed and compared
/// structurally.
#[repr(transparent)]
pub struct DictKey(pub u64);

impl Clone for DictKey {
    fn clone(&self) -> Self {
        DictKey(gleam_native_inc(self.0))
    }
}

impl Drop for DictKey {
    fn drop(&mut self) {
        let _ = gleam_native_dec(self.0);
    }
}

impl std::hash::Hash for DictKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        hash_value(self.0, state);
    }
}

impl PartialEq for DictKey {
    fn eq(&self, other: &Self) -> bool {
        deep_eq(self.0, other.0)
    }
}

impl Eq for DictKey {}

/// A stored dict value: an owned reference to a Gleam value, like
/// [`DictKey`].
#[repr(transparent)]
pub struct DictEntry(pub u64);

impl Clone for DictEntry {
    fn clone(&self) -> Self {
        DictEntry(gleam_native_inc(self.0))
    }
}

impl Drop for DictEntry {
    fn drop(&mut self) {
        let _ = gleam_native_dec(self.0);
    }
}

/// A borrowed lookup key: hashed and compared like [`DictKey`] but
/// holding no reference, so probing a map does not touch reference
/// counts.
#[repr(transparent)]
pub struct DictKeyRef(pub u64);

impl std::borrow::Borrow<DictKeyRef> for DictKey {
    fn borrow(&self) -> &DictKeyRef {
        // Both types are transparent wrappers around the value word.
        unsafe { &*(self as *const DictKey as *const DictKeyRef) }
    }
}

impl std::hash::Hash for DictKeyRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        hash_value(self.0, state);
    }
}

impl PartialEq for DictKeyRef {
    fn eq(&self, other: &Self) -> bool {
        deep_eq(self.0, other.0)
    }
}

impl Eq for DictKeyRef {}

/// A structural hash consistent with [`deep_eq`]: equal values hash
/// equally. Values of different kinds are never equal, so their hashes
/// may collide freely. Negative zero hashes as zero (they compare equal);
/// dict entries combine order-independently, since equal dicts may
/// iterate differently.
fn hash_value(value: u64, state: &mut impl std::hash::Hasher) {
    use std::hash::Hash;
    if value & 1 == 1 {
        ((value as i64) >> 1).hash(state);
        return;
    }
    let header = heap_header(value);
    match header_kind(header) {
        KIND_BIGINT => bigint_value(value).hash(state),
        KIND_FLOAT => {
            let float = float_value(value);
            let float = if float == 0.0 { 0.0 } else { float };
            float.to_bits().hash(state);
        }
        KIND_STRING => string_value(value).hash(state),
        KIND_RECORD => {
            (header & HEADER_SEMANTIC_MASK).hash(state);
            for index in 0..record_arity(header) {
                hash_value(record_field(value, index), state);
            }
        }
        KIND_BITARRAY => {
            let payload = bitarray_value(value);
            payload.bits.hash(state);
            payload.bytes.hash(state);
        }
        KIND_DICT => {
            let map = &dict_payload(value).map;
            map.len().hash(state);
            let mut combined: u64 = 0;
            for (key, entry) in map.iter() {
                let mut entry_state = std::collections::hash_map::DefaultHasher::new();
                hash_value(key.0, &mut entry_state);
                hash_value(entry.0, &mut entry_state);
                combined = combined.wrapping_add(std::hash::Hasher::finish(&entry_state));
            }
            combined.hash(state);
        }
        // Closures and unknown kinds compare by identity.
        _ => value.hash(state),
    }
}

/// A total order over Gleam values, consistent with structural equality:
/// `Equal` exactly when [`deep_eq`] holds (floats excepted for NaN, which
/// Gleam code cannot produce as a dict key in practice). Values of
/// different types order by an arbitrary type rank; Gleam's type system
/// keeps mixed-type keys to `Dynamic` dicts only.
pub fn cmp_values(left: u64, right: u64) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    let rank = |value: u64| -> u8 {
        if value & 1 == 1 {
            return 0;
        }
        match header_kind(heap_header(value)) {
            KIND_BIGINT => 0,
            KIND_FLOAT => 1,
            KIND_STRING => 2,
            KIND_BITARRAY => 3,
            KIND_RECORD => 4,
            KIND_DICT => 5,
            _ => 6,
        }
    };
    let left_rank = rank(left);
    match left_rank.cmp(&rank(right)) {
        Ordering::Equal => {}
        ordering => return ordering,
    }
    match left_rank {
        // Integers compare numerically whether small or big.
        0 => {
            if left & 1 == 1 && right & 1 == 1 {
                ((left as i64) >> 1).cmp(&((right as i64) >> 1))
            } else {
                untag(left).cmp(&untag(right))
            }
        }
        1 => {
            let left = float_value(left);
            let right = float_value(right);
            left.partial_cmp(&right).unwrap_or_else(|| left.total_cmp(&right))
        }
        2 => string_value(left).cmp(string_value(right)),
        3 => {
            let left = bitarray_value(left);
            let right = bitarray_value(right);
            (left.bits, &left.bytes).cmp(&(right.bits, &right.bytes))
        }
        4 => {
            let left_header = heap_header(left) & HEADER_SEMANTIC_MASK;
            let right_header = heap_header(right) & HEADER_SEMANTIC_MASK;
            match left_header.cmp(&right_header) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
            for index in 0..record_arity(left_header) {
                match cmp_values(record_field(left, index), record_field(right, index)) {
                    Ordering::Equal => {}
                    ordering => return ordering,
                }
            }
            Ordering::Equal
        }
        5 => {
            let left = dict_payload(left);
            let right = dict_payload(right);
            match left.map.len().cmp(&right.map.len()) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
            // Iteration order is unspecified, so a total order needs each
            // side's entries sorted by key first. Dicts nested inside dict
            // keys are rare enough for the sort not to matter.
            let sorted = |payload: &DictPayload| -> Vec<(u64, u64)> {
                let mut entries: Vec<(u64, u64)> = payload
                    .map
                    .iter()
                    .map(|(key, entry)| (key.0, entry.0))
                    .collect();
                entries.sort_by(|(left, _), (right, _)| cmp_values(*left, *right));
                entries
            };
            for ((left_key, left_value), (right_key, right_value)) in
                sorted(left).into_iter().zip(sorted(right))
            {
                match cmp_values(left_key, right_key) {
                    Ordering::Equal => {}
                    ordering => return ordering,
                }
                match cmp_values(left_value, right_value) {
                    Ordering::Equal => {}
                    ordering => return ordering,
                }
            }
            Ordering::Equal
        }
        // Closures and unknown kinds: identity order.
        _ => left.cmp(&right),
    }
}

pub fn dict_payload(value: u64) -> &'static DictPayload {
    unsafe { &(*container::<DictPayload>(value)).value }
}

/// The dict's map, mutably; only sound for transient dicts, which the
/// standard library uses linearly.
pub fn dict_payload_mut(value: u64) -> &'static mut DictPayload {
    unsafe { &mut (*container::<DictPayload>(value)).value }
}

pub fn box_dict(payload: DictPayload) -> u64 {
    box_heap(KIND_DICT, payload)
}

/// Appends a string tree's text to the buffer with a worklist, so deep
/// trees cannot overflow the stack. A native string tree is either a
/// string or a (possibly nested) list of string trees, mirroring Erlang's
/// iodata.
fn flatten_tree(tree: u64, buffer: &mut String) {
    let mut worklist = vec![tree];
    while let Some(value) = worklist.pop() {
        if value & 1 == 1 {
            // The empty list: nothing to add.
            continue;
        }
        let header = heap_header(value);
        match header_kind(header) {
            KIND_STRING => buffer.push_str(string_value(value)),
            KIND_RECORD => {
                // A cons cell: process the head before the tail.
                worklist.push(record_field(value, 1));
                worklist.push(record_field(value, 0));
            }
            _ => {}
        }
    }
}

/// Flattens a string tree into its text.
pub fn tree_to_string(tree: u64) -> String {
    let mut buffer = String::new();
    flatten_tree(tree, &mut buffer);
    buffer
}

/// Whether two string trees flatten to the same text; used by structural
/// equality when the two sides have different shapes.
pub(crate) fn trees_hold_equal_text(left: u64, right: u64) -> bool {
    tree_to_string(left) == tree_to_string(right)
}

/// Structural equality between two values of the same Gleam type, returning
/// [`TRUE`] or [`FALSE`]. Closures compare by identity.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_eq(left: u64, right: u64) -> u64 {
    if deep_eq(left, right) { TRUE } else { FALSE }
}

pub(crate) fn deep_eq(left: u64, right: u64) -> bool {
    if left == right {
        return true;
    }
    // Different immediates, or an immediate against a heap value, are never
    // equal: big integers never encode small-range values.
    if left & 1 == 1 || right & 1 == 1 {
        return false;
    }
    let left_header = heap_header(left) & HEADER_SEMANTIC_MASK;
    let right_header = heap_header(right) & HEADER_SEMANTIC_MASK;
    if left_header != right_header {
        // A string against a list can only be two string trees of different
        // shapes (no other value of one type has both representations), so
        // they compare by their flattened text.
        let kinds = (header_kind(left_header), header_kind(right_header));
        let list_display = |value: u64| record_display(heap_header(value)) == DISPLAY_LIST;
        if kinds == (KIND_STRING, KIND_RECORD) && list_display(right)
            || kinds == (KIND_RECORD, KIND_STRING) && list_display(left)
        {
            return trees_hold_equal_text(left, right);
        }
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
        KIND_DICT => {
            // Equal dicts may iterate in different orders (hash collisions
            // keep insertion order), so compare by lookup.
            let left = dict_payload(left);
            let right = dict_payload(right);
            left.map.len() == right.map.len()
                && left.map.iter().all(|(key, entry)| {
                    right
                        .map
                        .get(key)
                        .is_some_and(|other| deep_eq(entry.0, other.0))
                })
        }
        // Closures are equal only when identical, handled above.
        _ => false,
    }
}

/// Renders a value for `echo`. Scalars print exactly; records print
/// structurally as `@tag(field, ...)` since constructor names do not exist
/// at run time. Booleans and other immediates nested inside structures
/// print as their integer encoding.
/// Renders a string the way `echo` does on every target: the common
/// escapes by name, other control characters (and the C1 range) as
/// zero-padded uppercase `\u{XXXX}`, everything else literally.
fn inspect_string(string: &str) -> String {
    let mut out = String::with_capacity(string.len() + 2);
    out.push('"');
    for character in string.chars() {
        match character {
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{c}' => out.push_str("\\f"),
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            character if character < ' ' || ('\u{7f}'..'\u{a0}').contains(&character) => {
                out.push_str(&format!("\\u{{{:04X}}}", character as u32));
            }
            character => out.push(character),
        }
    }
    out.push('"');
    out
}

pub fn inspect(value: u64) -> String {
    if value & 1 == 1 {
        return format!("{}", (value as i64) >> 1);
    }
    let header = heap_header(value);
    match header_kind(header) {
        KIND_BIGINT => format!("{}", bigint_value(value)),
        KIND_FLOAT => format_float(float_value(value)),
        KIND_STRING => inspect_string(string_value(value)),
        KIND_CLOSURE => {
            // Parameters render as `a`, `b`, ... from the arity stored in
            // the header, matching the other targets.
            let arity = record_tag(header) as u64;
            let parameters: Vec<String> = (0..arity)
                .map(|index| {
                    char::from_u32(('a' as u32) + (index % 26) as u32)
                        .expect("letter")
                        .to_string()
                })
                .collect();
            format!("//fn({}) {{ ... }}", parameters.join(", "))
        }
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
                    // Walk the cons chain. A non-empty list whose elements
                    // are all integers in the printable ASCII range renders
                    // as a charlist, as on the other targets.
                    let mut items = Vec::new();
                    let mut chars = Some(String::new());
                    let mut current = value;
                    while current & 1 == 0 {
                        let element = record_field(current, 0);
                        if let Some(text) = &mut chars {
                            let printable = element & 1 == 1
                                && (32..=126).contains(&((element as i64) >> 1));
                            if printable {
                                text.push((((element as i64) >> 1) as u8) as char);
                            } else {
                                chars = None;
                            }
                        }
                        items.push(inspect(element));
                        current = record_field(current, 1);
                    }
                    match chars {
                        Some(text) if !items.is_empty() => {
                            format!("charlist.from_string(\"{text}\")")
                        }
                        _ => format!("[{}]", items.join(", ")),
                    }
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
        KIND_DICT => {
            // Sorted by key, so `echo` output stays deterministic (and
            // matches what the ordered map used to print) even though the
            // map itself iterates in hash order.
            let mut pairs: Vec<(u64, u64)> = dict_payload(value)
                .map
                .iter()
                .map(|(key, entry)| (key.0, entry.0))
                .collect();
            pairs.sort_by(|(left, _), (right, _)| cmp_values(*left, *right));
            let entries: Vec<String> = pairs
                .into_iter()
                .map(|(key, entry)| format!("#({}, {})", inspect(key), inspect(entry)))
                .collect();
            format!("dict.from_list([{}])", entries.join(", "))
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_echo(
    kind: u64,
    value: u64,
    message: u64,
    path: *const u8,
    path_length: u64,
    line: u64,
) -> u64 {
    let path = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(path, path_length as usize))
    };
    let rendered = match kind {
        1 => format!("{}", untag(value)),
        2 => format_float(float_value(value)),
        3 => inspect_string(string_value(value)),
        4 => (if value == TRUE { "True" } else { "False" }).to_string(),
        5 => "Nil".to_string(),
        // A statically-known list: the empty list is a bare tagged integer
        // that structural inspection cannot identify.
        6 if value == NIL => "[]".to_string(),
        _ => inspect(value),
    };
    // The location is greyed with the same ANSI codes the other targets
    // use, matching their output exactly.
    eprint!("\u{1b}[90m{path}:{line}\u{1b}[39m");
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
#[unsafe(no_mangle)]
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
    // Under `gleam test` the panic is a test failure: name the test on
    // standard output alongside the PASS lines before the report.
    let failed_test = TEST_RUN.get().and_then(|run| {
        let index = run
            .next
            .load(std::sync::atomic::Ordering::SeqCst)
            .checked_sub(1)?;
        let (test_name, _) = run.tests.get(index)?;
        Some((run, test_name))
    });
    if let Some((_, test_name)) = &failed_test {
        println!("  FAIL {test_name}");
    }
    eprintln!("runtime error: {name}");
    eprintln!();
    eprintln!("{message}");
    eprintln!();
    eprintln!("    {module}.{function}:{line}");
    if let Some((run, _)) = failed_test {
        let _ = run
            .failed
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        // Resume with the next test instead of exiting; the failed test's
        // frames below this one are abandoned.
        continue_test_run();
    }
    std::process::exit(1);
}

/// Prints an integer followed by a newline. The standin for a real printing
/// external until strings exist on the native target.
#[unsafe(no_mangle)]
pub extern "C" fn print_int(value: u64) -> u64 {
    println!("{}", untag(value));
    NIL
}

/// Prints a boolean as Gleam writes it, `True` or `False`, followed by a
/// newline.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_float(value: u64) -> u64 {
    println!("{}", format_float(float_value(value)));
    NIL
}

/// Prints a string followed by a newline, the native implementation for a
/// `gleam/io.println`-style external.
///
/// # Safety
///
/// `value` must be a string created by this runtime. The Gleam type system
/// upholds this for calls from generated code.
#[unsafe(no_mangle)]
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
        (
            "gleam_native_destroy",
            gleam_native_destroy as *const u8,
        ),
        // A data symbol, not a function: generated code loads the program
        // thread's pool address through it for inline pooled allocation.
        (
            POOL_SYMBOL,
            (&raw const gleam_native_pool) as *const u8,
        ),
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
