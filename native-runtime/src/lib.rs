// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The runtime library for the Gleam native target.
//!
//! Generated code represents every Gleam value as one 64-bit word:
//!
//! - low bit 1: a small integer, the value in the upper 63 bits (`(n << 1) | 1`)
//! - low three bits 000: a pointer to a heap allocation (8-byte aligned)
//! - low three bits 010, 100, or 110: a special immediate constant —
//!   `Nil` (2), `False` (4), `True` (6), and the empty list (10), each
//!   distinct so runtime-polymorphic operations (`echo`, `inspect`,
//!   `dynamic` classification) can tell them apart from integers
//!
//! `True` is `False | 2`, and bit 1 distinguishes them, so generated
//! boolean tests mask bit 1 and negation flips it. No value encodes as 0:
//! generated code and the runtime reserve 0 as a sentinel.
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

mod process;
pub use process::{
    SubjectPayload, exit_current_abnormally, in_child_process, monitor_arc, process_symbols,
    receive_tags, spawn_child,
};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use unicode_segmentation::UnicodeSegmentation;

/// The special immediate constants: words whose low three bits are neither
/// a small integer's (bit 0 set) nor a heap pointer's (all zero). `TRUE`
/// must stay `FALSE | 2` — generated code tests booleans by masking bit 1
/// and negates by flipping it.
pub const NIL: u64 = 0b010;
pub const FALSE: u64 = 0b100;
pub const TRUE: u64 = 0b110;
pub const EMPTY_LIST: u64 = 0b1010;

/// Whether a value word is an immediate (small integer or special
/// constant) rather than a heap pointer. Heap pointers are 8-byte aligned,
/// so any set low bit marks an immediate; no value word is 0.
pub fn is_immediate(value: u64) -> bool {
    value & 0b111 != 0
}

/// Whether a value word is a tagged small integer.
pub fn is_small_int(value: u64) -> bool {
    value & 1 == 1
}

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
/// A process subject: the handle messages are sent and received on,
/// created by the process runtime.
pub const KIND_SUBJECT: u64 = 7;

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

pub fn record_tag(header: u64) -> u32 {
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
    free_words(
        (value - 8) as *mut u64,
        std::mem::size_of::<HeapBox<T>>() / 8,
    );
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
    box_heap(KIND_STRING, StringPayload::Owned(value.into()))
}

/// A string's payload: its own bytes, or a view into another string's.
/// A view keeps an owned reference to its parent — always an `Owned`
/// string — so `rest`-of-string values from pattern matches and the
/// standard library's parsing loops share one buffer instead of copying
/// the remainder each step.
pub enum StringPayload {
    Owned(CompactString),
    View(StringView),
}

/// A view's fields; dropping one releases the parent reference.
pub struct StringView {
    parent: u64,
    offset: u32,
    length: u32,
}

impl Drop for StringView {
    fn drop(&mut self) {
        let _ = gleam_native_dec(self.parent);
    }
}

/// Slices at or below this many bytes are copied inline rather than
/// viewed: the copy is as cheap as the view and never pins the parent
/// buffer.
const MAX_INLINE_SLICE: usize = 24;

/// A string holding `parent`'s bytes from `offset` for `length` bytes,
/// both on character boundaries: inline-copied when short, otherwise a
/// view sharing the (root) parent's buffer.
pub fn box_string_slice(parent: u64, offset: usize, length: usize) -> u64 {
    if length <= MAX_INLINE_SLICE {
        return box_string(&string_value(parent)[offset..offset + length]);
    }
    // Point at the root so view chains stay one level deep.
    let (root, base) = match unsafe { &(*container::<StringPayload>(parent)).value } {
        StringPayload::Owned(_) => (parent, 0),
        StringPayload::View(view) => (view.parent, view.offset as usize),
    };
    let (Ok(offset), Ok(length)) = (u32::try_from(base + offset), u32::try_from(length)) else {
        return box_string(&string_value(parent)[offset..offset + length]);
    };
    box_heap(
        KIND_STRING,
        StringPayload::View(StringView {
            parent: gleam_native_inc(root),
            offset,
            length,
        }),
    )
}

/// A slice of `parent` given as a subslice of its own text, as the string
/// functions returning `&str` regions (trims, split parts) produce.
fn slice_of(parent: u64, sub: &str) -> u64 {
    let offset = sub.as_ptr() as usize - string_value(parent).as_ptr() as usize;
    box_string_slice(parent, offset, sub.len())
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

#[inline]
pub fn string_value(value: u64) -> &'static str {
    match unsafe { &(*container::<StringPayload>(value)).value } {
        StringPayload::Owned(string) => string.as_str(),
        StringPayload::View(view) => {
            let StringPayload::Owned(parent) =
                (unsafe { &(*container::<StringPayload>(view.parent)).value })
            else {
                unreachable!("view parents are owned strings");
            };
            // Views are constructed on character boundaries within the
            // parent, so resolution skips the boundary re-checks.
            unsafe {
                std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                    parent.as_ptr().add(view.offset as usize),
                    view.length as usize,
                ))
            }
        }
    }
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
    exit_current_abnormally(format!("bit array error: {message}"));
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
fn read_int_bits(
    payload: &BitArrayPayload,
    offset: u64,
    bits: u64,
    little: bool,
    signed: bool,
) -> BigInt {
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
    // A uniquely-referenced owned left operand — the accumulator in an
    // `acc <> piece` fold — appends in place, making such folds amortized
    // linear instead of quadratic. A count of one also rules out every
    // alias: `s <> s` holds two references, a view of the left string
    // holds a parent reference, and an interned literal is held by its
    // cache slot. Views themselves are excluded — their buffer belongs to
    // the parent.
    if sole_reference(left)
        && let StringPayload::Owned(string) =
            (unsafe { &mut (*container::<StringPayload>(left)).value })
    {
        string.push_str(string_value(right));
        return gleam_native_inc(left);
    }
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
    let length = string_value(subject).len() - offset as usize;
    box_string_slice(subject, offset as usize, length)
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
    let mut list = EMPTY_LIST;
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
    box_string(
        string_value(string)
            .graphemes(true)
            .rev()
            .collect::<CompactString>(),
    )
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_trim(string: u64) -> u64 {
    slice_of(
        string,
        string_value(string).trim_matches(TRIMMED_WHITESPACE),
    )
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
    slice_of(
        string,
        string_value(string).trim_start_matches(TRIMMED_WHITESPACE),
    )
}

/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_trim_end(string: u64) -> u64 {
    slice_of(
        string,
        string_value(string).trim_end_matches(TRIMMED_WHITESPACE),
    )
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
    box_string(string_value(string).replace(string_value(pattern), string_value(replacement)))
}

/// Splits on a separator, returning a list of strings. An empty separator
/// yields the whole string as a single element, like the other targets.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_split(string: u64, on: u64) -> u64 {
    let text = string_value(string);
    let on = string_value(on);
    if on.is_empty() {
        return make_list(vec![gleam_native_inc(string)]);
    }
    make_list(text.split(on).map(|part| slice_of(string, part)).collect())
}

/// The first grapheme cluster and the rest: `Ok(#(head, rest))`, or
/// `Error(Nil)` for the empty string.
///
/// # Safety
///
/// See [`gleam_native_string_byte_size`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_string_pop_grapheme(string: u64) -> u64 {
    let text = string_value(string);
    match text.grapheme_indices(true).next() {
        None => make_error(NIL),
        Some((_, head)) => make_ok(make_tuple2(
            box_string(head),
            box_string_slice(string, head.len(), text.len() - head.len()),
        )),
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
    while !is_immediate(current) {
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

// ---------------------------------------------------------------------------
// Panic stack traces
//
// Generated code preserves frame pointers, so at panic time the runtime can
// walk the frame-pointer chain and map each return address to a compiled
// Gleam function through a table of code ranges built from the same
// per-function line tables that back the DWARF debug info. Tail calls
// replace their caller's frame, so tail-called frames do not appear — the
// same elision Erlang's last-call optimization produces.

/// One compiled Gleam function's code range and line table, registered by
/// the host before the program runs (the JIT from finalized addresses, an
/// ahead-of-time executable from the table embedded in its object file).
pub struct FrameInfo {
    /// The function's first instruction address.
    pub start: usize,
    /// The function's code length in bytes.
    pub length: u32,
    /// The `module.function` display name.
    pub name: String,
    /// The function's module source path, package-root relative.
    pub path: String,
    /// Sorted (code offset, source line) pairs: where each source line's
    /// machine code begins.
    pub rows: Vec<(u32, u32)>,
}

static FRAME_TABLE: std::sync::OnceLock<Vec<FrameInfo>> = std::sync::OnceLock::new();

/// Stores the compiled functions' code ranges for stack traces; called by
/// the host before the program runs.
pub fn set_frame_table(mut table: Vec<FrameInfo>) {
    table.sort_by_key(|frame| frame.start);
    let _ = FRAME_TABLE.set(table);
}

/// The symbol of the frame table blob that ahead-of-time compilation embeds
/// in the executable, decoded by [`start`] before the program runs.
pub const FRAME_TABLE_SYMBOL: &str = "gleam_native_frame_table";

/// Decodes the frame table blob ahead-of-time compilation embeds under
/// [`FRAME_TABLE_SYMBOL`] and registers it. Layout, all integers
/// little-endian, every entry 8-byte aligned: a u64 entry count, then per
/// entry a u64 function address (a linker-resolved relocation), u32 code
/// length, u32 name length, u32 path length, u32 row count, the name and
/// path bytes, then the (u32 offset, u32 line) rows, padded to 8 bytes.
///
/// # Safety
///
/// `data` must point at a blob with the layout above whose relocations the
/// linker has resolved.
pub unsafe fn decode_frame_table(data: *const u8) -> Vec<FrameInfo> {
    unsafe fn read_u32(cursor: &mut *const u8) -> u32 {
        let value = unsafe { std::ptr::read_unaligned(*cursor as *const u32) };
        *cursor = unsafe { cursor.add(4) };
        u32::from_le(value)
    }
    unsafe fn read_bytes<'a>(cursor: &mut *const u8, length: usize) -> &'a [u8] {
        let bytes = unsafe { std::slice::from_raw_parts(*cursor, length) };
        *cursor = unsafe { cursor.add(length) };
        bytes
    }
    let mut cursor = data;
    let count = u64::from_le(unsafe { std::ptr::read_unaligned(cursor as *const u64) });
    cursor = unsafe { cursor.add(8) };
    let mut table = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let start =
            u64::from_le(unsafe { std::ptr::read_unaligned(cursor as *const u64) }) as usize;
        cursor = unsafe { cursor.add(8) };
        let length = unsafe { read_u32(&mut cursor) };
        let name_length = unsafe { read_u32(&mut cursor) } as usize;
        let path_length = unsafe { read_u32(&mut cursor) } as usize;
        let row_count = unsafe { read_u32(&mut cursor) } as usize;
        let name =
            String::from_utf8_lossy(unsafe { read_bytes(&mut cursor, name_length) }).into_owned();
        let path =
            String::from_utf8_lossy(unsafe { read_bytes(&mut cursor, path_length) }).into_owned();
        let mut rows = Vec::with_capacity(row_count);
        for _ in 0..row_count {
            let offset = unsafe { read_u32(&mut cursor) };
            let line = unsafe { read_u32(&mut cursor) };
            rows.push((offset, line));
        }
        let position = cursor as usize - data as usize;
        cursor = unsafe { cursor.add(position.next_multiple_of(8) - position) };
        table.push(FrameInfo {
            start,
            length,
            name,
            path,
            rows,
        });
    }
    table
}

/// The Gleam function containing an address, and the source line of the
/// instruction before it (return addresses point after their call).
fn resolve_frame(return_address: usize) -> Option<(&'static FrameInfo, u32)> {
    let table = FRAME_TABLE.get()?;
    let address = return_address.checked_sub(1)?;
    let index = table
        .partition_point(|frame| frame.start <= address)
        .checked_sub(1)?;
    let frame = table.get(index)?;
    let offset = address - frame.start;
    if offset >= frame.length as usize {
        return None;
    }
    let offset = offset as u32;
    let row = frame
        .rows
        .partition_point(|(row_offset, _)| *row_offset <= offset)
        .checked_sub(1)?;
    let (_, line) = *frame.rows.get(row)?;
    Some((frame, line))
}

/// The caller's frame pointer register. Even when this function's caller
/// set up no frame of its own, the register still holds the most recent
/// frame in the chain.
#[inline(always)]
fn current_frame_pointer() -> usize {
    let frame_pointer: usize;
    unsafe {
        #[cfg(target_arch = "aarch64")]
        std::arch::asm!("mov {}, x29", out(reg) frame_pointer);
        #[cfg(target_arch = "x86_64")]
        std::arch::asm!("mov {}, rbp", out(reg) frame_pointer);
        #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
        {
            frame_pointer = 0;
        }
    }
    frame_pointer
}

/// Removes the pointer-authentication signature from a return address:
/// generated aarch64 code signs return addresses when the CPU supports
/// it, hiding the real address from the frame-table lookup. The feature
/// probe caches in an atomic; [`install_stack_overflow_handler`] warms it
/// so the signal handler's use stays async-signal-safe.
#[cfg(target_arch = "aarch64")]
fn strip_pointer_authentication(pointer: usize) -> usize {
    if std::arch::is_aarch64_feature_detected!("paca") {
        let mut value = pointer;
        unsafe { std::arch::asm!("xpaci {0}", inout(reg) value) };
        value
    } else {
        pointer
    }
}

#[cfg(not(target_arch = "aarch64"))]
fn strip_pointer_authentication(pointer: usize) -> usize {
    pointer
}

/// How many resolved Gleam frames a trace prints at most, and how many raw
/// frame records the walk visits at most (an overflowed stack can hold
/// millions).
const MAX_TRACE_FRAMES: usize = 64;
const MAX_TRACE_STEPS: usize = 512;

/// Walks the frame-pointer chain upward from `frame_pointer`, writing an
/// Erlang-style stack trace through `write` — deepest frame first, one
/// `  module.function src/module.gleam:line` row per Gleam frame with the
/// location greyed, consecutive identical frames collapsed, and a greyed
/// `...` row when the walk is cut short. Frames that are not compiled
/// Gleam code (runtime internals between generated frames) are skipped.
/// Returns the number of rows written; writes nothing (and returns zero)
/// when no frame resolves.
///
/// Performs no allocation, so the stack overflow signal handler can use it;
/// every frame pointer is bounds-checked against the program thread's stack
/// before it is dereferenced.
fn write_stack_trace(mut frame_pointer: usize, top: usize, write: &mut dyn FnMut(&[u8])) -> usize {
    fn write_row(write: &mut dyn FnMut(&[u8]), frame: &FrameInfo, line: u32, repeats: usize) {
        let mut digits = [0u8; 20];
        write(b"  ");
        write(frame.name.as_bytes());
        write(b"\x1b[90m ");
        write(frame.path.as_bytes());
        write(b":");
        write(format_int(line as usize, &mut digits));
        write(b"\x1b[0m\n");
        if repeats > 1 {
            write(b"  \x1b[90m... (frame repeated ");
            write(format_int(repeats - 1, &mut digits));
            write(b" more times)\x1b[0m\n");
        }
    }
    /// Formats an integer into the buffer without allocating.
    fn format_int(mut value: usize, digits: &mut [u8; 20]) -> &[u8] {
        let mut index = digits.len();
        loop {
            index -= 1;
            digits[index] = b'0' + (value % 10) as u8;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        &digits[index..]
    }

    let mut pending: Option<(&FrameInfo, u32, usize)> = None;
    let mut rows = 0;
    let mut truncated = false;
    for step in 0..MAX_TRACE_STEPS {
        if frame_pointer == 0 || frame_pointer & 7 != 0 || top == 0 || frame_pointer + 16 > top {
            break;
        }
        if step + 1 == MAX_TRACE_STEPS {
            truncated = true;
            break;
        }
        // A frame record is [previous frame pointer, return address] on
        // both supported architectures.
        let next = unsafe { *(frame_pointer as *const usize) };
        let return_address = unsafe { *((frame_pointer + 8) as *const usize) };
        let return_address = strip_pointer_authentication(return_address);
        if let Some((frame, line)) = resolve_frame(return_address) {
            match &mut pending {
                Some((held, held_line, repeats))
                    if std::ptr::eq(*held, frame) && *held_line == line =>
                {
                    *repeats += 1;
                }
                _ => {
                    if let Some((held, held_line, repeats)) = pending.take() {
                        write_row(write, held, held_line, repeats);
                        rows += 1;
                    }
                    if rows == MAX_TRACE_FRAMES {
                        truncated = true;
                        break;
                    }
                    pending = Some((frame, line, 1));
                }
            }
        }
        // The chain must move strictly upward to terminate.
        if next <= frame_pointer {
            break;
        }
        frame_pointer = next;
    }
    if let Some((held, held_line, repeats)) = pending {
        write_row(write, held, held_line, repeats);
        rows += 1;
    }
    if truncated && rows > 0 {
        write(b"  \x1b[90m...\x1b[0m\n");
    }
    rows
}

/// Installs a handler that reports stack overflows (and other fatal memory
/// faults) as a runtime error with exit code 1 instead of a raw signal
/// death. Uses an alternate signal stack, since the main stack is exhausted
/// when a stack overflow fires; the report walks the exhausted stack's
/// frame pointers (read out of the signal context) for a trace of the
/// runaway recursion.
pub fn install_stack_overflow_handler() {
    install_signal_stack();
    install_fault_handler();
}

/// Gives the calling thread an alternate signal stack, so the fault handler
/// can run when the fault is the thread's own stack overflowing. Called on
/// every thread that executes Gleam code: the program thread, or each
/// scheduler worker under the fiber runner.
pub fn install_signal_stack() {
    unsafe {
        let stack = libc::stack_t {
            ss_sp: std::alloc::alloc(
                std::alloc::Layout::from_size_align(64 * 1024, 16).expect("layout"),
            ) as *mut libc::c_void,
            ss_flags: 0,
            ss_size: 64 * 1024,
        };
        let _ = libc::sigaltstack(&stack, std::ptr::null_mut());
    }
}

/// Installs the process-wide SIGSEGV/SIGBUS action that reports fatal
/// memory faults; the calling thread still needs [`install_signal_stack`]
/// for the handler to survive that thread's own stack overflowing.
fn install_fault_handler() {
    // Warm the pointer-authentication feature probe's cache, so the signal
    // handler's stack walk performs no first-use system call.
    let _ = strip_pointer_authentication(0);
    unsafe {
        /// The faulting thread's frame pointer register, from the signal's
        /// machine context.
        unsafe fn context_frame_pointer(context: *mut libc::c_void) -> usize {
            if context.is_null() {
                return 0;
            }
            let context = context as *mut libc::ucontext_t;
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            unsafe {
                let state = (*context).uc_mcontext;
                if state.is_null() {
                    return 0;
                }
                (*state).__ss.__fp as usize
            }
            #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
            unsafe {
                let state = (*context).uc_mcontext;
                if state.is_null() {
                    return 0;
                }
                (*state).__ss.__rbp as usize
            }
            #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
            unsafe {
                (*context).uc_mcontext.regs[29] as usize
            }
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            unsafe {
                (*context).uc_mcontext.gregs[libc::REG_RBP as usize] as usize
            }
            #[cfg(not(any(target_os = "macos", target_os = "linux")))]
            {
                let _ = context;
                0
            }
        }

        // Everything in the handler is async-signal-safe: raw writes to
        // standard error, reads of the (already published) frame table,
        // and bounds-checked reads of the faulting thread's stack.
        extern "C" fn handler(
            _signal: libc::c_int,
            _info: *mut libc::siginfo_t,
            context: *mut libc::c_void,
        ) {
            fn write_bytes(bytes: &[u8]) {
                unsafe {
                    let _ = libc::write(2, bytes.as_ptr() as *const libc::c_void, bytes.len());
                }
            }
            write_bytes(
                b"runtime error: stack overflow

The program recursed too deeply. Gleam tail calls run in constant stack space, but deeply nested non-tail recursion exhausted the stack.
",
            );
            let frame_pointer = unsafe { context_frame_pointer(context) };
            if frame_pointer != 0 {
                let mut wrote_header = false;
                let _ = write_stack_trace(frame_pointer, current_stack_top(), &mut |bytes| {
                    if !wrote_header {
                        wrote_header = true;
                        write_bytes(b"\nstacktrace:\n");
                    }
                    write_bytes(bytes);
                });
            }
            unsafe {
                libc::_exit(1);
            }
        }

        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = handler as *const () as libc::sighandler_t;
        action.sa_flags = libc::SA_ONSTACK | libc::SA_SIGINFO;
        let _ = libc::sigaction(libc::SIGSEGV, &action, std::ptr::null_mut());
        let _ = libc::sigaction(libc::SIGBUS, &action, std::ptr::null_mut());
    }
}

/// The closure-invocation thunks, registered before Gleam code runs: one
/// C-convention entry per argument count, generated alongside the program
/// (Gleam closures use a calling convention Rust cannot call directly).
/// The JIT registers finalized pointers; ahead-of-time executables
/// register the linked symbols from their C `main`.
static INVOKERS: [std::sync::atomic::AtomicUsize; 7] = [
    std::sync::atomic::AtomicUsize::new(0),
    std::sync::atomic::AtomicUsize::new(0),
    std::sync::atomic::AtomicUsize::new(0),
    std::sync::atomic::AtomicUsize::new(0),
    std::sync::atomic::AtomicUsize::new(0),
    std::sync::atomic::AtomicUsize::new(0),
    std::sync::atomic::AtomicUsize::new(0),
];

/// Registers the closure-invocation thunk for one argument count; called
/// by the host before the program runs.
pub fn set_invoker(arity: usize, pointer: *const u8) {
    INVOKERS[arity].store(pointer as usize, std::sync::atomic::Ordering::Release);
}

/// Calls a Gleam closure. The closure is borrowed (a reference is taken
/// for the call, which the callee releases); the arguments are consumed —
/// the callee owns them. Returns an owned result.
pub fn call_closure(closure: u64, arguments: &[u64]) -> u64 {
    let pointer = INVOKERS[arguments.len()].load(std::sync::atomic::Ordering::Acquire);
    assert!(pointer != 0, "closure invoker not registered");
    let closure = gleam_native_inc(closure);
    unsafe {
        match arguments {
            [] => std::mem::transmute::<usize, extern "C" fn(u64) -> u64>(pointer)(closure),
            [a] => {
                std::mem::transmute::<usize, extern "C" fn(u64, u64) -> u64>(pointer)(closure, *a)
            }
            [a, b] => std::mem::transmute::<usize, extern "C" fn(u64, u64, u64) -> u64>(pointer)(
                closure, *a, *b,
            ),
            [a, b, c] => std::mem::transmute::<usize, extern "C" fn(u64, u64, u64, u64) -> u64>(
                pointer,
            )(closure, *a, *b, *c),
            [a, b, c, d] => std::mem::transmute::<
                usize,
                extern "C" fn(u64, u64, u64, u64, u64) -> u64,
            >(pointer)(closure, *a, *b, *c, *d),
            [a, b, c, d, e] => std::mem::transmute::<
                usize,
                extern "C" fn(u64, u64, u64, u64, u64, u64) -> u64,
            >(pointer)(closure, *a, *b, *c, *d, *e),
            [a, b, c, d, e, f] => std::mem::transmute::<
                usize,
                extern "C" fn(u64, u64, u64, u64, u64, u64, u64) -> u64,
            >(pointer)(closure, *a, *b, *c, *d, *e, *f),
            _ => unreachable!("unsupported closure arity"),
        }
    }
}

/// The command line arguments the host passes in before running `main`.
static START_ARGUMENTS: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();

/// Stores the program's command line arguments; called by the host before
/// `main` runs.
pub fn set_start_arguments(arguments: Vec<String>) {
    let _ = START_ARGUMENTS.set(arguments);
}

/// Runs the program's entry wrapper as the root process: a fiber with its
/// own guard-paged stack, scheduled (with every process it spawns) on a
/// tokio multi-thread runtime's worker threads. Returns when the root
/// process finishes; remaining processes are abandoned, matching the
/// other targets' behavior when `main` returns. Used by `gleam run`,
/// `gleam test`, and ahead-of-time compiled executables alike.
pub fn run_program_fiber(
    stack_size_megabytes: u64,
    entry: extern "C" fn() -> u64,
) -> Result<(), String> {
    run_program_fiber_with(stack_size_megabytes, move || {
        let _ = entry();
    })
}

/// [`run_program_fiber`] for an arbitrary body: used by the test runner.
pub fn run_program_fiber_with(
    stack_size_megabytes: u64,
    body: impl FnOnce() + Send + 'static,
) -> Result<(), String> {
    // Stacks are committed lazily by the OS, so a generous size reserves
    // address space, not memory; the guard page below each turns overflow
    // into a fault the handler reports.
    let stack_size = usize::try_from(stack_size_megabytes.max(1))
        .unwrap_or(usize::MAX)
        .saturating_mul(1024 * 1024);
    process::PROCESS_STACK_SIZE.store(stack_size, std::sync::atomic::Ordering::Relaxed);
    arm_rc_debugging();
    install_fault_handler();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("gleam-scheduler")
        .on_thread_start(install_signal_stack)
        .enable_time()
        .build()
        .map_err(|error| format!("could not start the scheduler: {error}"))?;
    let (_shared, future) = process::new_process(true, body)?;
    let result = runtime.block_on(runtime.spawn(future));
    if std::env::var_os("GLEAM_DEBUG_SEND").is_some() {
        eprintln!(
            "sends: {} moved roots, {} copied roots",
            TRANSFER_MOVED.load(std::sync::atomic::Ordering::Relaxed),
            TRANSFER_COPIED.load(std::sync::atomic::Ordering::Relaxed),
        );
    }
    match result {
        Ok(()) => Ok(()),
        Err(error) if error.is_panic() => std::panic::resume_unwind(error.into_panic()),
        Err(error) => Err(format!("the program crashed: {error}")),
    }
}

/// Runs the given tests in order — each as its own process, monitored by
/// this one — reporting each outcome, then exits: 0 if every test passed,
/// 1 otherwise. Called inside the root fiber.
///
/// A failing test panics into [`gleam_native_panic`], which reports the
/// failure and terminates the test's process; its fiber is abandoned
/// without unwinding and the runner (observing the abnormal exit through
/// its monitor) records the failure and moves on — the same mechanism any
/// crashing process uses. A test process's *descendants* may crash freely
/// without failing the test, as long as the test process itself finishes
/// normally. A stack overflow still aborts the whole run.
pub fn run_tests(tests: Vec<(String, extern "C" fn() -> u64)>) -> ! {
    let mut failed = 0usize;
    for (name, function) in &tests {
        let function = *function;
        let (_test_process, down) = process::spawn_child_monitored(move || {
            let _ = function();
        });
        let envelope = receive_tags(&[down], None).expect("a monitored test reports down");
        let reason = envelope.take_value();
        let passed = string_value(reason) == "normal";
        if passed {
            println!("  PASS {name}");
        } else {
            failed += 1;
            println!("  FAIL {name}");
            // The reason names what ended the process when it wasn't a
            // reported panic (a kill, or an exit signal from a link).
            eprintln!("  (test process exited with reason: {})", string_value(reason));
        }
        let _ = gleam_native_dec(reason);
    }
    let total = tests.len();
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
/// program data and frame table, and runs the entry wrapper on the program
/// thread. Returns the process exit code.
///
/// # Safety
///
/// `argc`/`argv` must be the values C `main` received, `program_data`
/// must point at a blob produced by [`encode_program_data`], and
/// `frame_table` at a linker-resolved blob with [`decode_frame_table`]'s
/// layout.
pub unsafe fn start(
    argc: i32,
    argv: *const *const std::ffi::c_char,
    program_data: *const u8,
    frame_table: *const u8,
    literal_init: extern "C" fn() -> u64,
    entry: extern "C" fn() -> u64,
) -> i32 {
    set_frame_table(unsafe { decode_frame_table(frame_table) });
    // Build every interned literal (marked permanent) before any Gleam
    // code runs; single-threaded, so the slots need no synchronization.
    let _ = literal_init();
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

    match run_program_fiber(stack_size_megabytes, entry) {
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
    make_list(arguments.into_iter().map(box_string).collect())
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
    let passed = if exact == 0 {
        size >= bits
    } else {
        size == bits
    };
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
    let expected = unsafe { std::slice::from_raw_parts(bytes, bit_length.div_ceil(8) as usize) };
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

/// Whether the integer of `bits` (tagged) bits at (tagged) bit offset
/// `offset` equals `expected` (a tagged small integer or heap big
/// integer), compared numerically; literal integer patterns whose size is
/// only known at run time match this way. A negative or non-small size,
/// or a read past the end of the array, never matches.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_int_equals(
    array: u64,
    offset: u64,
    bits: u64,
    endian: u64,
    signed: u64,
    expected: u64,
) -> u64 {
    if !is_small_int(bits) || (bits as i64) < 0 {
        return FALSE;
    }
    let bits = ((bits as i64) >> 1) as u64;
    let offset = untag_bits(offset, "offset");
    let payload = bitarray_value(array);
    if offset + bits > payload.bits {
        return FALSE;
    }
    let value = read_int_bits(payload, offset, bits, is_little(endian), signed != 0);
    if value == untag(expected) {
        TRUE
    } else {
        FALSE
    }
}

/// Whether the float of `bits` (tagged) bits at (tagged) bit offset
/// `offset` equals `expected` (an f64 passed as its bits), compared
/// numerically like the other targets: NaN data never matches, and
/// negative zero equals zero. Sizes other than 16, 32, and 64 bits, and
/// reads past the end of the array, never match.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_float_equals(
    array: u64,
    offset: u64,
    bits: u64,
    endian: u64,
    expected: u64,
) -> u64 {
    if !is_small_int(bits) {
        return FALSE;
    }
    let bits = (bits as i64) >> 1;
    if !matches!(bits, 16 | 32 | 64) {
        return FALSE;
    }
    let bits = bits as u64;
    let offset = untag_bits(offset, "offset");
    let payload = bitarray_value(array);
    if offset + bits > payload.bits {
        return FALSE;
    }
    let bytes = extract_bits(payload, offset, bits);
    let value = float_from_bytes(&bytes, is_little(endian));
    if value == f64::from_bits(expected) {
        TRUE
    } else {
        FALSE
    }
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
    unsafe { *(value as *mut u64) = record_header(tag as u32, arity as u32) | (display << 48) };
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
    POOL.with(|pool| allocate_words_in(pool, words))
}

/// [`allocate_words`] against an already-resolved pool, so a walk
/// allocating many objects (a deep copy) looks the thread-local up once.
pub(crate) fn allocate_words_in(pool: &Pool, words: usize) -> u64 {
    if rc_stats() {
        let _ = RC_ALLOCATIONS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    let total = 1 + words;
    if total < POOL_CLASSES {
        let head = pool.heads[total].get();
        if head != 0 {
            // The count word of a pooled block holds the next block.
            pool.heads[total].set(unsafe { *(head as *const u64) });
            pool.counts[total].set(pool.counts[total].get() - 1);
            unsafe { *(head as *mut u64) = 1 };
            return head + 8;
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

/// The per-execution context generated code reaches through its hidden
/// leading `env` parameter: every tail-convention function receives a
/// pointer to the running program's (later: process's) context. The layout
/// is part of the code generation contract — generated code loads `pool`
/// at byte offset 0; the remaining fields are read by the runtime.
#[repr(C)]
pub struct ProcessContext {
    /// The pool generated code pops inline allocations from: always the
    /// pool of the worker thread currently running this context's code,
    /// rewritten by the scheduler on every resume.
    pub pool: *const Pool,
    /// The bounds of the stack the context's code runs on (a fiber stack,
    /// or the program thread's stack under the legacy runner): the
    /// stack-trace walk stops at `stack_high` and never dereferences a
    /// frame pointer outside them.
    pub stack_low: usize,
    pub stack_high: usize,
    /// Reserved for cooperative preemption: a budget the generated code
    /// will eventually decrement and yield on.
    pub reductions: i64,
}

thread_local! {
    /// The context of the code currently running on this thread, set by
    /// the runners before Gleam code executes. The C-convention entry
    /// points generated code exposes (the entry wrapper, closure-invoke
    /// thunks, test wrappers) read it through
    /// [`gleam_native_current_context`] to pass as the hidden `env`
    /// argument. Const-initialized so reads never allocate — the fault
    /// handler reads it during a stack overflow.
    static CURRENT_CONTEXT: Cell<*mut ProcessContext> =
        const { Cell::new(std::ptr::null_mut()) };
}

/// The running context's address, for the generated C-convention wrappers
/// to pass as the hidden `env` argument when crossing into tail-convention
/// code.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_current_context() -> u64 {
    CURRENT_CONTEXT.with(|context| context.get()) as u64
}

/// The symbol of [`gleam_native_current_context`], imported by generated
/// C-convention wrappers.
pub const CURRENT_CONTEXT_SYMBOL: &str = "gleam_native_current_context";

/// Sets the thread's running context; used by the process scheduler
/// around every fiber resume.
pub(crate) fn set_current_context(context: *mut ProcessContext) {
    CURRENT_CONTEXT.with(|current| current.set(context));
}

/// The thread's running context pointer, or null. The process module
/// recovers the whole process state from it by field offset, so one
/// thread-local serves both generated code and the runtime.
pub(crate) fn current_context_pointer() -> *mut ProcessContext {
    CURRENT_CONTEXT.with(|current| current.get())
}

/// The calling thread's allocation pool, for pinning a process context to
/// the worker about to run it.
pub(crate) fn current_pool() -> *const Pool {
    POOL.with(|pool| pool as *const Pool)
}

/// The stack top recorded in the running context, or zero when no context
/// is set: the stack-trace walk's upper bound.
fn current_stack_top() -> usize {
    let context = CURRENT_CONTEXT.with(|context| context.get());
    if context.is_null() {
        0
    } else {
        unsafe { (*context).stack_high }
    }
}

/// The sign bit of a count word marks a permanent object — an interned
/// literal or zero-arity constructor, shared by every process for the
/// program's whole life. Count operations (inline and runtime) test the
/// bit and touch nothing when it is set, so permanent objects are safe to
/// share across scheduler threads without atomics.
pub const PERMANENT_COUNT: u64 = 1 << 63;

/// Whether a heap value's count word marks it permanent. A relaxed
/// atomic load: atomically counted words (which also carry the sign bit)
/// are modified concurrently, and the flag bits themselves never change
/// after marking.
fn is_permanent(value: u64) -> bool {
    atomic_count(value).load(std::sync::atomic::Ordering::Relaxed) & PERMANENT_COUNT != 0
}

/// Bit 62 of a count word marks an atomically counted *leaf* — a value
/// whose fields are all immediates or permanents, shared across process
/// boundaries by reference instead of being copied per send (subjects
/// are the motivating case). The sign bit is set as well, so every
/// inline fast-path check that skips plain counting still applies; the
/// cold negative-count path tests this bit and performs the count
/// operation atomically instead of skipping it. The real count lives in
/// the low bits and the object is freed normally when it reaches zero —
/// unlike permanents, atomic values do not leak.
pub const ATOMIC_COUNT: u64 = 1 << 62;

/// The low bits of an atomic count word: the actual reference count.
pub const ATOMIC_COUNT_MASK: u64 = ATOMIC_COUNT - 1;

/// Whether a heap value's count word marks it atomically counted; a
/// relaxed load, as in [`is_permanent`].
fn is_atomic(value: u64) -> bool {
    atomic_count(value).load(std::sync::atomic::Ordering::Relaxed) & ATOMIC_COUNT != 0
}

/// The count word as an atomic, for values marked [`ATOMIC_COUNT`].
fn atomic_count(value: u64) -> &'static std::sync::atomic::AtomicU64 {
    unsafe { &*((value - 8) as *const std::sync::atomic::AtomicU64) }
}

/// Adds one reference to an atomically counted value.
fn atomic_inc(value: u64) {
    let _ = atomic_count(value).fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

/// Drops one reference from an atomically counted value, destroying it
/// when the count reaches zero (the release/acquire pair orders the
/// dropping thread's writes before the free, as `Arc` does).
fn atomic_dec(value: u64) -> bool {
    let previous = atomic_count(value).fetch_sub(1, std::sync::atomic::Ordering::Release);
    if previous & ATOMIC_COUNT_MASK == 1 {
        std::sync::atomic::fence(std::sync::atomic::Ordering::Acquire);
        return true;
    }
    false
}

/// Marks a heap value atomically counted so it can be shared across
/// process boundaries by reference: sends deliver the same box instead
/// of a copy, and it is freed when its last reference anywhere drops.
/// Only a *leaf* qualifies — every field an immediate or permanent —
/// since sharing must not drag plainly counted children across threads;
/// anything else (or an already marked value) is left as it was, which
/// is always correct, just copied on send. Returns a new reference,
/// since externals borrow their argument.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_make_shared(value: u64) -> u64 {
    if is_immediate(value) {
        return value;
    }
    let count = unsafe { *((value - 8) as *const u64) };
    if count & (PERMANENT_COUNT | ATOMIC_COUNT) == 0 {
        let header = heap_header(value);
        let leaf = match header_kind(header) {
            KIND_RECORD => (0..record_arity(header)).all(|index| {
                let field = record_field(value, index);
                is_immediate(field) || is_permanent(field)
            }),
            _ => false,
        };
        if leaf {
            unsafe {
                *((value - 8) as *mut u64) = PERMANENT_COUNT | ATOMIC_COUNT | count;
            }
        }
    }
    gleam_native_inc(value)
}

/// Marks a heap value permanent: its count operations become no-ops and it
/// is never freed. Called by the generated literal-init function on each
/// interned literal before the program runs; returns the value. A no-op
/// for immediates.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_make_permanent(value: u64) -> u64 {
    if !is_immediate(value) {
        unsafe { *((value - 8) as *mut u64) = PERMANENT_COUNT };
    }
    value
}

/// The symbol of [`gleam_native_make_permanent`], called by the generated
/// literal-init function.
pub const MAKE_PERMANENT_SYMBOL: &str = "gleam_native_make_permanent";

/// The symbol of the generated literal-init function, which builds every
/// interned literal into its slot (marked permanent) before the program
/// runs; called by the host once, single-threaded.
pub const LITERAL_INIT_SYMBOL: &str = "gleam_native_literal_init";

/// Reference-count debugging, armed by environment variables read before
/// the program thread starts (compiling with `GLEAM_DEBUG_RC=1` also
/// routes generated code's inline count operations through the checked
/// runtime entry points):
///
/// - `GLEAM_DEBUG_RC=1`: validate every count the runtime touches, abort
///   with a diagnostic on a dead or garbage count, and poison freed pool
///   blocks so use-after-free names itself.
/// - `GLEAM_RC_STATS=1`: count allocations and frees, reporting them (and
///   the objects still live) when the process exits. Interned literals
///   and zero-arity constructors are expected to remain live.
/// - `GLEAM_TRACE_RC=<kind|all>`: print every checked count operation on
///   objects of the given kind.
static RC_FLAGS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static RC_TRACE_KIND: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
static RC_ALLOCATIONS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static RC_FREES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

const RC_FLAG_DEBUG: u64 = 1;
const RC_FLAG_STATS: u64 = 2;

/// The header written over a freed pooled block under `GLEAM_DEBUG_RC`,
/// so a use-after-free reads as this kind instead of stale data.
const POISON_KIND: u64 = 0xDEAD;

#[inline]
fn rc_debug() -> bool {
    RC_FLAGS.load(std::sync::atomic::Ordering::Relaxed) & RC_FLAG_DEBUG != 0
}

#[inline]
fn rc_stats() -> bool {
    RC_FLAGS.load(std::sync::atomic::Ordering::Relaxed) & RC_FLAG_STATS != 0
}

/// Validates a heap value's count word before an operation on it.
/// Permanent objects are exempt: their count word is the permanent bit,
/// not a live count.
fn rc_check(value: u64, operation: &str) {
    let count = unsafe { *((value - 8) as *const u64) };
    if count & PERMANENT_COUNT != 0 {
        return;
    }
    let header = heap_header(value);
    if header_kind(header) == POISON_KIND {
        eprintln!("RC BUG: {operation} of freed (pooled) value {value:#x}, count word {count:#x}");
        eprintln!("{}", std::backtrace::Backtrace::force_capture());
        std::process::abort();
    }
    if count == 0 || count > 1 << 40 {
        eprintln!(
            "RC BUG: {operation} of dead/garbage value {value:#x}, count {count:#x}, header {header:#x}"
        );
        std::process::abort();
    }
    let traced = RC_TRACE_KIND.load(std::sync::atomic::Ordering::Relaxed);
    if traced == header_kind(header) || traced == u64::MAX - 1 {
        eprintln!(
            "TRACE {operation} {value:#x} kind {} count {count}",
            header_kind(header)
        );

    }
}

/// The process's peak resident set size in bytes, from `getrusage`
/// (macOS reports bytes, Linux kilobytes).
pub fn peak_rss_bytes() -> u64 {
    unsafe {
        let mut usage: libc::rusage = std::mem::zeroed();
        if libc::getrusage(libc::RUSAGE_SELF, &mut usage) != 0 {
            return 0;
        }
        let maximum = usage.ru_maxrss as u64;
        if cfg!(target_os = "macos") {
            maximum
        } else {
            maximum * 1024
        }
    }
}

extern "C" fn report_rc_stats() {
    let allocations = RC_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed);
    let frees = RC_FREES.load(std::sync::atomic::Ordering::Relaxed);
    eprintln!(
        "RC STATS: {allocations} allocations, {frees} frees, {} live at exit \
         (interned literals and constructors are expected to remain), \
         peak RSS {:.1} MB",
        allocations - frees,
        peak_rss_bytes() as f64 / (1024.0 * 1024.0),
    );
}

/// Reads the debug environment; called once before the program thread
/// runs Gleam code.
fn arm_rc_debugging() {
    use std::sync::atomic::Ordering;
    let mut flags = 0;
    if std::env::var_os("GLEAM_DEBUG_RC").is_some() {
        flags |= RC_FLAG_DEBUG;
    }
    if std::env::var_os("GLEAM_RC_STATS").is_some() {
        flags |= RC_FLAG_STATS;
        unsafe { libc::atexit(report_rc_stats) };
    }
    if let Some(kind) = std::env::var_os("GLEAM_TRACE_RC") {
        let kind = kind
            .to_string_lossy()
            .parse::<u64>()
            .unwrap_or(u64::MAX - 1);
        RC_TRACE_KIND.store(kind, Ordering::Relaxed);
        flags |= RC_FLAG_DEBUG;
    }
    RC_FLAGS.store(flags, Ordering::Relaxed);
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
    POOL.with(|pool| free_words_in(pool, base, total))
}

/// [`free_words`] against an already-resolved pool, so a walk freeing
/// many objects (a destroy) looks the thread-local up once.
fn free_words_in(pool: &Pool, base: *mut u64, total: usize) {
    if rc_stats() {
        let _ = RC_FREES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    if rc_debug() {
        // Poison the header so a stale reference reads as freed.
        unsafe { *base.add(1) = POISON_KIND };
    }
    let pooled = total < POOL_CLASSES && {
        let count = pool.counts[total].get();
        if count >= POOL_CLASS_CAPACITY {
            false
        } else {
            unsafe { *base = pool.heads[total].get() };
            pool.heads[total].set(base as u64);
            pool.counts[total].set(count + 1);
            true
        }
    };
    if !pooled {
        unsafe {
            std::alloc::dealloc(
                base as *mut u8,
                std::alloc::Layout::array::<u64>(total).expect("heap object layout"),
            )
        };
    }
}

/// Increments a value's reference count. A no-op for immediates and
/// permanent objects.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_inc(value: u64) -> u64 {
    if is_immediate(value) {
        return value;
    }
    if is_atomic(value) {
        atomic_inc(value);
        return value;
    }
    if !is_permanent(value) {
        if rc_debug() {
            rc_check(value, "inc");
        }
        unsafe { *((value - 8) as *mut u64) += 1 };
    }
    value
}

/// Decrements a value's reference count, destroying the object when it
/// reaches zero. A no-op for immediates and permanent objects. The common
/// cases — an immediate, or a count that stays positive — touch nothing
/// but the count word.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dec(value: u64) -> u64 {
    if is_immediate(value) {
        return NIL;
    }
    if is_atomic(value) {
        if atomic_dec(value) {
            destroy(value);
        }
        return NIL;
    }
    if is_permanent(value) {
        return NIL;
    }
    if rc_debug() {
        rc_check(value, "dec");
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
    // pushed onto it. The thread's pool is resolved once for the whole
    // walk.
    POOL.with(|pool| {
        let header = heap_header(first);
        let kind = header_kind(header);
        if (kind != KIND_RECORD && kind != KIND_CLOSURE) || record_arity(header) == 0 {
            free_object(pool, first, &mut Vec::new());
            return;
        }
        let mut worklist = WORKLIST.take();
        free_object(pool, first, &mut worklist);
        while let Some(value) = worklist.pop() {
            if is_immediate(value) {
                continue;
            }
            if is_atomic(value) {
                if atomic_dec(value) {
                    free_object(pool, value, &mut worklist);
                }
                continue;
            }
            if is_permanent(value) {
                continue;
            }
            if rc_debug() {
                rc_check(value, "dec (destroy)");
            }
            let count = (value - 8) as *mut u64;
            unsafe {
                *count -= 1;
                if *count > 0 {
                    continue;
                }
            }
            free_object(pool, value, &mut worklist);
        }
        WORKLIST.set(worklist);
    })
}

/// Frees one object whose count has reached zero, pushing the children it
/// owned onto the worklist.
fn free_object(pool: &Pool, value: u64, worklist: &mut Vec<u64>) {
    let count = (value - 8) as *mut u64;
    let header = heap_header(value);
    match header_kind(header) {
        KIND_RECORD => {
            let arity = record_arity(header);
            for index in 0..arity {
                worklist.push(record_field(value, index));
            }
            free_words_in(pool, count, 2 + arity as usize);
        }
        KIND_CLOSURE => {
            let captures = record_arity(header);
            for index in 0..captures {
                // Captures sit one word past the code pointer.
                worklist.push(record_field(value, 1 + index));
            }
            free_words_in(pool, count, 3 + captures as usize);
        }
        KIND_BIGINT => free_payload::<BigInt>(value),
        // A float box is count, header, and payload: the three-word class.
        KIND_FLOAT => free_words_in(pool, count, 3),
        KIND_STRING => free_payload::<StringPayload>(value),
        KIND_BITARRAY => free_payload::<BitArrayPayload>(value),
        KIND_DICT => {
            // Dropping the persistent map releases exactly the tree
            // nodes no other dict shares; their keys and entries
            // release their references through [`DictKey`] and
            // [`DictEntry`]'s `Drop` implementations.
            free_payload::<DictPayload>(value);
        }
        KIND_SUBJECT => free_payload::<SubjectPayload>(value),
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
    if is_small_int(value) {
        ((value as i64) >> 1).hash(state);
        return;
    }
    if is_immediate(value) {
        // Nil, booleans, and the empty list: distinct words, equal only to
        // themselves, so the word is the hash.
        value.hash(state);
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
        // Subjects compare by channel identity, so they hash by it too
        // (copies of one subject are equal but live at different
        // addresses).
        KIND_SUBJECT => subject_payload(value).tag.hash(state),
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
        if is_small_int(value) {
            return 0;
        }
        if is_immediate(value) {
            return match value {
                // Booleans order `False < True`, like Erlang's atoms.
                FALSE | TRUE => 4,
                NIL => 5,
                // The empty list sorts before every non-empty list (a
                // cons cell is a record, ranked higher).
                _ => 6,
            };
        }
        match header_kind(heap_header(value)) {
            KIND_BIGINT => 0,
            KIND_FLOAT => 1,
            KIND_STRING => 2,
            KIND_BITARRAY => 3,
            KIND_RECORD => 7,
            KIND_DICT => 8,
            _ => 9,
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
            if is_small_int(left) && is_small_int(right) {
                ((left as i64) >> 1).cmp(&((right as i64) >> 1))
            } else {
                untag(left).cmp(&untag(right))
            }
        }
        1 => {
            let left = float_value(left);
            let right = float_value(right);
            left.partial_cmp(&right)
                .unwrap_or_else(|| left.total_cmp(&right))
        }
        2 => string_value(left).cmp(string_value(right)),
        3 => {
            let left = bitarray_value(left);
            let right = bitarray_value(right);
            (left.bits, &left.bytes).cmp(&(right.bits, &right.bytes))
        }
        // Booleans compare by word (`FALSE < TRUE`); `Nil` and the empty
        // list are single-valued ranks.
        4 => left.cmp(&right),
        5 | 6 => Ordering::Equal,
        7 => {
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
        8 => {
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

/// Whether an external's heap argument is the last reference to its
/// value. The caller passes arguments as owned temporaries and releases
/// them right after the call, so a count of one means nothing else can
/// observe the value: an external may recycle it in place and return a
/// fresh reference to it instead of building a copy.
#[inline]
pub fn sole_reference(value: u64) -> bool {
    unsafe { *((value - 8) as *const u64) == 1 }
}

pub fn dict_payload(value: u64) -> &'static DictPayload {
    unsafe { &(*container::<DictPayload>(value)).value }
}

/// The dict's map, mutably; only sound for transient dicts, which the
/// standard library uses linearly.
pub fn dict_payload_mut(value: u64) -> &'static mut DictPayload {
    unsafe { &mut (*container::<DictPayload>(value)).value }
}

/// Boxes a subject payload as a Gleam value.
pub(crate) fn box_subject(payload: SubjectPayload) -> u64 {
    box_heap(KIND_SUBJECT, payload)
}

/// The payload of a subject value.
pub(crate) fn subject_payload(value: u64) -> &'static SubjectPayload {
    unsafe { &(*container::<SubjectPayload>(value)).value }
}

/// Deep-copies a value for delivery to another process: the copy shares
/// nothing with the original except permanent objects (interned literals,
/// safe to share because count operations skip them) and the
/// internally-synchronized process handles inside subjects. Record and
/// list spines copy iteratively, so long lists cannot overflow the stack;
/// recursion happens only through dict entries.
/// Prepares a message this caller *owns* for delivery to another process:
/// a single walk that moves every exclusively-owned node (reference count
/// one) as it stands — the receiver inherits the memory — and replaces
/// each shared subtree with a deep copy, releasing the shared reference.
/// Either way the caller's reference is consumed and the result is
/// reachable only by the receiver.
///
/// Moving is sound because a count of one proves nothing else can ever
/// observe the node: any live binding or borrowed read in the sender
/// shows up as an extra count (or prevents consumption at the call site
/// altogether), which routes that subtree through the copy branch. Dicts
/// always copy — their persistent maps share tree nodes through
/// non-atomic `Rc`s that box counts cannot see. A moved node was
/// allocated from this thread's pool and will be freed into the
/// receiver's; pools recycle free memory, not live values, so that is
/// already sound. String views keep their parent reference, so the
/// parent slot is walked like a child; every other kind is a leaf whose
/// internals (system allocations, atomically counted process handles)
/// move safely between threads.
pub static TRANSFER_MOVED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static TRANSFER_COPIED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

thread_local! {
    /// The reused slot buffer for [`transfer_for_send`], so a send
    /// performs no allocation of its own (mirroring [`destroy`]'s
    /// worklist). A transfer never re-enters itself.
    static TRANSFER_SLOTS: Cell<Vec<*mut u64>> = const { Cell::new(Vec::new()) };
}

pub fn transfer_for_send(value: u64) -> u64 {
    if !is_immediate(value) && !is_permanent(value) {
        let counter = if unsafe { *((value - 8) as *const u64) } == 1 {
            &TRANSFER_MOVED
        } else {
            &TRANSFER_COPIED
        };
        let _ = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    let mut root = value;
    let mut slots = TRANSFER_SLOTS.take();
    slots.push(&mut root);
    while let Some(slot) = slots.pop() {
        let value = unsafe { *slot };
        if is_immediate(value) || is_permanent(value) {
            continue;
        }
        let header = heap_header(value);
        let kind = header_kind(header);
        if unsafe { *((value - 8) as *const u64) } != 1 || kind == KIND_DICT {
            let copy = deep_copy(value);
            let _ = gleam_native_dec(value);
            unsafe { *slot = copy };
            continue;
        }
        match kind {
            KIND_RECORD => {
                for index in 0..record_arity(header) as usize {
                    slots.push((value as *mut u64).wrapping_add(1 + index));
                }
            }
            KIND_CLOSURE => {
                for index in 0..record_arity(header) as usize {
                    slots.push((value as *mut u64).wrapping_add(2 + index));
                }
            }
            KIND_STRING => {
                if let StringPayload::View(view) =
                    unsafe { &mut (*container::<StringPayload>(value)).value }
                {
                    slots.push(&mut view.parent);
                }
            }
            _ => {}
        }
    }
    TRANSFER_SLOTS.set(slots);
    root
}

pub fn deep_copy(value: u64) -> u64 {
    fn copy_node(pool: &Pool, value: u64, worklist: &mut Vec<u64>) -> u64 {
        let header = heap_header(value);
        match header_kind(header) {
            KIND_RECORD => {
                let arity = record_arity(header) as usize;
                let copy = allocate_words_in(pool, 1 + arity);
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        value as *const u64,
                        copy as *mut u64,
                        1 + arity,
                    );
                }
                // The copy's fields still point at the original's
                // children; the worklist rewrites them.
                if arity > 0 {
                    worklist.push(copy);
                }
                copy
            }
            KIND_CLOSURE => {
                let captures = record_arity(header) as usize;
                let copy = allocate_words_in(pool, 2 + captures);
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        value as *const u64,
                        copy as *mut u64,
                        2 + captures,
                    );
                }
                if captures > 0 {
                    worklist.push(copy);
                }
                copy
            }
            KIND_BIGINT => box_bigint(bigint_value(value).clone()),
            KIND_FLOAT => box_float(float_value(value)),
            // Views flatten into owned strings on copy.
            KIND_STRING => box_string(string_value(value)),
            KIND_BITARRAY => {
                let payload = bitarray_value(value);
                box_bitarray(BitArrayPayload {
                    bits: payload.bits,
                    bytes: payload.bytes.clone(),
                })
            }
            KIND_DICT => {
                // Structural sharing must not cross the process boundary:
                // rebuild the map, deep-copying every key and entry.
                let payload = dict_payload(value);
                let mut map = DictMap::default();
                for (key, entry) in payload.map.iter() {
                    let _ = map.insert(DictKey(deep_copy(key.0)), DictEntry(deep_copy(entry.0)));
                }
                box_dict(DictPayload { map })
            }
            KIND_SUBJECT => box_subject(subject_payload(value).clone()),
            kind => unreachable!("deep copy of unknown kind {kind}"),
        }
    }

    if is_immediate(value) {
        return value;
    }
    if is_permanent(value) {
        // An atomically counted value is shared, not copied: the copy is
        // one more reference to the same box.
        if is_atomic(value) {
            atomic_inc(value);
        }
        return value;
    }
    POOL.with(|pool| {
        let mut worklist = Vec::new();
        let root = copy_node(pool, value, &mut worklist);
        while let Some(node) = worklist.pop() {
            let header = heap_header(node);
            let (first, count) = match header_kind(header) {
                KIND_RECORD => (1, record_arity(header) as usize),
                KIND_CLOSURE => (2, record_arity(header) as usize),
                _ => unreachable!("only records and closures carry child slots"),
            };
            for index in first..first + count {
                let slot = (node as *mut u64).wrapping_add(index);
                let child = unsafe { *slot };
                if is_immediate(child) {
                    continue;
                }
                if is_permanent(child) {
                    if is_atomic(child) {
                        atomic_inc(child);
                    }
                    continue;
                }
                unsafe { *slot = copy_node(pool, child, &mut worklist) };
            }
        }
        root
    })
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
        if is_immediate(value) {
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
    if is_immediate(left) || is_immediate(right) {
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
        KIND_SUBJECT => {
            // Subjects are equal when they name the same channel: the same
            // process and the same tag.
            let left = subject_payload(left);
            let right = subject_payload(right);
            left.tag == right.tag && std::ptr::eq(&*left.shared, &*right.shared)
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
    if is_small_int(value) {
        return format!("{}", (value as i64) >> 1);
    }
    if is_immediate(value) {
        return match value {
            NIL => "Nil".to_string(),
            FALSE => "False".to_string(),
            TRUE => "True".to_string(),
            EMPTY_LIST => "[]".to_string(),
            other => format!("<unknown immediate {other}>"),
        };
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
                    while !is_immediate(current) {
                        let element = record_field(current, 0);
                        if let Some(text) = &mut chars {
                            let printable = is_small_int(element)
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
                        format!(
                            "@{}({})",
                            record_tag(header),
                            fields(value, header).join(", ")
                        )
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
        KIND_SUBJECT => {
            let payload = subject_payload(value);
            format!("//Subject({}, {})", payload.shared.pid, payload.tag)
        }
        kind => format!("<unknown kind {kind}>"),
    }
}

/// The implementation of `echo`: prints the source location and the value
/// to standard error, then returns the value. Rendering is structural:
/// every value — immediates included — identifies itself at run time.
///
/// # Safety
///
/// The module name pointer must be valid as described for
/// [`gleam_native_panic`]; `message` is a string value or 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_echo(
    value: u64,
    message: u64,
    path: *const u8,
    path_length: u64,
    line: u64,
) -> u64 {
    let path = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(path, path_length as usize))
    };
    let rendered = inspect(value);
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
    eprintln!("runtime error: {name}");
    eprintln!();
    eprintln!("{message}");
    eprintln!();
    // An Erlang-style stack trace from the frame-pointer chain; when the
    // walk resolves nothing (no frame table registered, or an unsupported
    // platform), fall back to the panic site metadata baked into the call.
    let mut trace = String::new();
    let _ = write_stack_trace(current_frame_pointer(), current_stack_top(), &mut |bytes| {
        trace.push_str(std::str::from_utf8(bytes).expect("trace rows are UTF-8"));
    });
    eprintln!("stacktrace:");
    if trace.is_empty() {
        eprintln!("  {module}.{function}:{line}");
    } else {
        eprint!("{trace}");
    }
    // Inside a child process (a test, or a spawned process) the crash
    // kills just that process — its fiber is abandoned without unwinding
    // and links and monitors are notified. The root process's crash ends
    // the program.
    exit_current_abnormally(format!("{name}: {message}"));
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
            "gleam_native_bitarray_int_equals",
            gleam_native_bitarray_int_equals as *const u8,
        ),
        (
            "gleam_native_bitarray_float_equals",
            gleam_native_bitarray_float_equals as *const u8,
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
        ("gleam_native_destroy", gleam_native_destroy as *const u8),
        (
            CURRENT_CONTEXT_SYMBOL,
            gleam_native_current_context as *const u8,
        ),
        (
            MAKE_PERMANENT_SYMBOL,
            gleam_native_make_permanent as *const u8,
        ),
        (
            "gleam_native_make_shared",
            gleam_native_make_shared as *const u8,
        ),
        ("gleam_native_echo", gleam_native_echo as *const u8),
        ("gleam_native_panic", gleam_native_panic as *const u8),
        ("print_int", print_int as *const u8),
        ("print_bool", print_bool as *const u8),
        ("print_float", print_float as *const u8),
        ("println", println as *const u8),
    ]
    .into_iter()
    .chain(process_symbols())
    .collect()
}

#[cfg(test)]
mod fiber_tests {
    use super::*;

    /// The fiber runner executes a body to completion and returns.
    #[test]
    fn fiber_runs_body() {
        let ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let flag = ran.clone();
        run_program_fiber_with(1, move || {
            flag.store(true, std::sync::atomic::Ordering::Release);
        })
        .expect("the fiber program runs");
        assert!(ran.load(std::sync::atomic::Ordering::Acquire));
    }

    /// A spawned process runs, its monitor reports a normal exit, and the
    /// down message arrives through the mailbox — the round trip the test
    /// runner and gleam_otp both rely on.
    #[test]
    fn spawned_process_reports_down() {
        run_program_fiber_with(1, || {
            let ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let flag = ran.clone();
            let child = spawn_child(move || {
                flag.store(true, std::sync::atomic::Ordering::Release);
            });
            let down = monitor_arc(&child);
            let envelope = receive_tags(&[down], None).expect("a down message arrives");
            let reason = envelope.take_value();
            assert_eq!(string_value(reason), "normal");
            let _ = gleam_native_dec(reason);
            assert!(ran.load(std::sync::atomic::Ordering::Acquire));
        })
        .expect("the program runs");
    }

    /// A receive with a timeout returns empty-handed when nothing is sent.
    #[test]
    fn receive_times_out() {
        run_program_fiber_with(1, || {
            let received = receive_tags(&[42], Some(std::time::Duration::from_millis(20)));
            assert!(received.is_none());
        })
        .expect("the program runs");
    }

    /// Prints the round-trip suspend/resume cost; run with
    /// `cargo test -p native-runtime fiber_switch_cost -- --nocapture`.
    #[test]
    fn fiber_switch_cost() {
        const ROUND_TRIPS: u32 = 1_000_000;
        let mut coroutine = corosensei::Coroutine::<(), (), ()>::new(|yielder, ()| {
            for _ in 0..ROUND_TRIPS {
                yielder.suspend(());
            }
        });
        let started = std::time::Instant::now();
        while let corosensei::CoroutineResult::Yield(()) = coroutine.resume(()) {}
        let elapsed = started.elapsed();
        println!(
            "fiber round trip: {:.0} ns over {ROUND_TRIPS} suspend/resume pairs",
            elapsed.as_nanos() as f64 / f64::from(ROUND_TRIPS)
        );
    }
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
        let mut list = EMPTY_LIST;
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
        let result =
            gleam_native_int_add_slow(tag_small_int(SMALL_INT_MAX), tag_small_int(SMALL_INT_MAX));
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
        let array = unsafe { gleam_native_bitarray_append_string(array, make_string("ok"), 0, 0) };
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
        assert_eq!(
            bitarray_value(little_unaligned).bytes,
            vec![232, 0b0011_0000]
        );
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
            float_value(gleam_native_bitarray_read_float(
                floats,
                bits(0),
                bits(16),
                0
            )),
            1.5
        );
        assert_eq!(
            float_value(gleam_native_bitarray_read_float(
                floats,
                bits(16),
                bits(32),
                1
            )),
            2.5
        );
        assert_eq!(
            float_value(gleam_native_bitarray_read_float(
                floats,
                bits(48),
                bits(64),
                0
            )),
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
        assert_eq!(&bitarray_value(utf).bytes[4..], "\u{1F600}".as_bytes());

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
            unsafe { gleam_native_string_starts_with(subject, wrong.as_ptr(), wrong.len() as u64) },
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
        let list_a = make_record(1, &[a, EMPTY_LIST]);
        let list_b = make_record(1, &[b, EMPTY_LIST]);
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
        assert_eq!(
            inspect(gleam_native_float_from_bits(2.5_f64.to_bits())),
            "2.5"
        );
    }
}
