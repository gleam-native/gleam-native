// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The native implementations behind the Gleam standard library's
//! `@external(native, "runtime", ...)` declarations.
//!
//! Every function follows the external calling convention: arguments are
//! borrowed (never released here), and the returned value is owned by the
//! caller — so a function returning one of its arguments unchanged must
//! increment its reference count first.
//!
//! Functions that take Gleam callbacks cannot exist here: generated Gleam
//! functions use Cranelift's tail calling convention, which Rust cannot
//! call into. The standard library's Gleam sources give those functions
//! native fallback bodies built from the closure-free primitives below.

use num_bigint::BigInt;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::collections::BTreeMap;

use native_runtime::*;

/// Increments and returns the borrowed argument: the identity external.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_identity(value: u64) -> u64 {
    gleam_native_inc(value)
}

/// Renders any value the way `string.inspect` does, reusing `echo`'s
/// renderer.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_inspect_value(value: u64) -> u64 {
    box_string(inspect(value))
}

fn str_value(value: u64) -> &'static str {
    string_value(value).as_str()
}

/// True and False as Gleam values.
fn bool_value(condition: bool) -> u64 {
    if condition { TRUE } else { FALSE }
}

/// Walks a Gleam list, yielding each element (borrowed).
fn list_elements(mut list: u64) -> Vec<u64> {
    let mut elements = Vec::new();
    while list & 1 == 0 {
        elements.push(record_field(list, 0));
        list = record_field(list, 1);
    }
    elements
}

// ---------------------------------------------------------------------------
// gleam/io

/// Prints a string to standard output without a trailing newline.
///
/// # Safety
///
/// The argument must be a string created by this runtime; the Gleam type
/// system upholds this, as for every function below taking strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_print(string: u64) -> u64 {
    use std::io::Write;
    let mut stdout = std::io::stdout();
    let _ = stdout.write_all(string_value(string).as_bytes());
    let _ = stdout.flush();
    NIL
}

/// Prints a string to standard error without a trailing newline.
///
/// # Safety
///
/// See [`gleam_native_print`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_print_error(string: u64) -> u64 {
    use std::io::Write;
    let mut stderr = std::io::stderr();
    let _ = stderr.write_all(string_value(string).as_bytes());
    let _ = stderr.flush();
    NIL
}

/// Prints a string to standard error with a trailing newline.
///
/// # Safety
///
/// See [`gleam_native_print`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_println_error(string: u64) -> u64 {
    eprintln!("{}", string_value(string));
    NIL
}

// ---------------------------------------------------------------------------
// gleam/int

/// Parses a decimal integer with an optional sign, the whole string strict.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_parse(string: u64) -> u64 {
    parse_int_in_base(str_value(string), 10)
}

/// Parses an integer in the given base (2..=36, validated by the standard
/// library before calling).
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_base_parse(string: u64, base: u64) -> u64 {
    let base = ((base as i64) >> 1).clamp(2, 36) as u32;
    parse_int_in_base(str_value(string), base)
}

fn parse_int_in_base(string: &str, base: u32) -> u64 {
    let rest = string
        .strip_prefix('-')
        .or_else(|| string.strip_prefix('+'))
        .unwrap_or(string);
    if rest.is_empty() || !rest.chars().all(|character| character.is_digit(base)) {
        return make_error(NIL);
    }
    match BigInt::parse_bytes(string.to_lowercase().as_bytes(), base) {
        Some(value) => make_ok(retag(value)),
        None => make_error(NIL),
    }
}

/// Renders an integer in the given base (2..=36) with uppercase digits,
/// matching `erlang:integer_to_binary/2`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_to_base_string(value: u64, base: u64) -> u64 {
    let base = ((base as i64) >> 1).clamp(2, 36) as u32;
    box_string(untag(value).to_str_radix(base).to_uppercase())
}

/// Converts an integer to a float.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_to_float(value: u64) -> u64 {
    box_float(untag(value).to_f64().unwrap_or(f64::INFINITY))
}

/// Bitwise AND on arbitrary-precision integers, with Erlang's two's
/// complement semantics for negative values.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_bitwise_and(left: u64, right: u64) -> u64 {
    retag(untag(left) & untag(right))
}

/// Bitwise OR; see [`gleam_native_int_bitwise_and`].
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_bitwise_or(left: u64, right: u64) -> u64 {
    retag(untag(left) | untag(right))
}

/// Bitwise XOR; see [`gleam_native_int_bitwise_and`].
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_bitwise_exclusive_or(left: u64, right: u64) -> u64 {
    retag(untag(left) ^ untag(right))
}

/// Bitwise NOT; see [`gleam_native_int_bitwise_and`].
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_bitwise_not(value: u64) -> u64 {
    retag(!untag(value))
}

/// Arithmetic shift left. A negative shift shifts right, like Erlang's
/// `bsl`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_bitwise_shift_left(value: u64, shift: u64) -> u64 {
    let shift = untag(shift).to_i64().unwrap_or(i64::MAX);
    retag(shift_bigint(untag(value), shift))
}

/// Arithmetic shift right (sign-preserving). A negative shift shifts left,
/// like Erlang's `bsr`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_int_bitwise_shift_right(value: u64, shift: u64) -> u64 {
    let shift = untag(shift).to_i64().unwrap_or(i64::MAX);
    retag(shift_bigint(untag(value), -shift))
}

/// Shifts left for positive amounts and right for negative ones.
fn shift_bigint(value: BigInt, shift: i64) -> BigInt {
    if value.is_zero() {
        return value;
    }
    if shift >= 0 {
        value << (shift as usize)
    } else {
        value >> ((-shift) as usize)
    }
}

// ---------------------------------------------------------------------------
// gleam/float

/// Parses a float written the way Gleam floats are: optional sign, digits,
/// a decimal point, digits, and an optional exponent.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_float_parse(string: u64) -> u64 {
    let string = str_value(string);
    if float_syntax_valid(string) {
        match string.parse::<f64>() {
            Ok(value) => make_ok(box_float(value)),
            Err(_) => make_error(NIL),
        }
    } else {
        make_error(NIL)
    }
}

/// Whether a string matches `[+-]? digits "." digits ([eE] [+-]? digits)?`.
fn float_syntax_valid(string: &str) -> bool {
    let rest = string
        .strip_prefix('-')
        .or_else(|| string.strip_prefix('+'))
        .unwrap_or(string);
    let Some((whole, after_point)) = rest.split_once('.') else {
        return false;
    };
    if whole.is_empty() || !whole.bytes().all(|byte| byte.is_ascii_digit()) {
        return false;
    }
    let (fraction, exponent) = match after_point.split_once(['e', 'E']) {
        Some((fraction, exponent)) => {
            let exponent = exponent
                .strip_prefix('-')
                .or_else(|| exponent.strip_prefix('+'))
                .unwrap_or(exponent);
            (fraction, Some(exponent))
        }
        None => (after_point, None),
    };
    if fraction.is_empty() || !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
        return false;
    }
    match exponent {
        Some(digits) => !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()),
        None => true,
    }
}

/// # Safety
///
/// The argument must be a float created by this runtime, as for every
/// float-taking function below.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_float_ceiling(value: u64) -> u64 {
    box_float(float_value(value).ceil())
}

/// # Safety
///
/// See [`gleam_native_float_ceiling`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_float_floor(value: u64) -> u64 {
    box_float(float_value(value).floor())
}

/// Rounds to the nearest integer, ties away from zero, like
/// `erlang:round/1`.
///
/// # Safety
///
/// See [`gleam_native_float_ceiling`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_float_round(value: u64) -> u64 {
    let rounded = float_value(value).round();
    retag(BigInt::from_f64(rounded).unwrap_or_default())
}

/// Truncates towards zero, like `erlang:trunc/1`.
///
/// # Safety
///
/// See [`gleam_native_float_ceiling`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_float_truncate(value: u64) -> u64 {
    let truncated = float_value(value).trunc();
    retag(BigInt::from_f64(truncated).unwrap_or_default())
}

/// Raises a base to an exponent. The standard library validates the
/// arguments so no NaN can be produced.
///
/// # Safety
///
/// See [`gleam_native_float_ceiling`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_float_power(base: u64, exponent: u64) -> u64 {
    box_float(float_value(base).powf(float_value(exponent)))
}

/// A random float in `[0, 1)`, from a per-thread splitmix64 generator
/// seeded with the current time.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_float_random() -> u64 {
    use std::cell::Cell;
    thread_local! {
        static STATE: Cell<u64> = Cell::new({
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos() as u64)
                .unwrap_or(0x9E3779B97F4A7C15);
            now ^ (&now as *const _ as u64)
        });
    }
    let bits = STATE.with(|state| {
        let mut z = state.get().wrapping_add(0x9E3779B97F4A7C15);
        state.set(z);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    });
    // 53 random mantissa bits make a uniform float in [0, 1).
    box_float((bits >> 11) as f64 / (1u64 << 53) as f64)
}

/// The natural logarithm. The standard library checks the argument is
/// positive before calling.
///
/// # Safety
///
/// See [`gleam_native_float_ceiling`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_float_log(value: u64) -> u64 {
    box_float(float_value(value).ln())
}

/// e raised to the given power.
///
/// # Safety
///
/// See [`gleam_native_float_ceiling`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gleam_native_float_exponential(value: u64) -> u64 {
    box_float(float_value(value).exp())
}

// ---------------------------------------------------------------------------
// gleam/string

/// Whether the first string sorts before the second in code point order.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_string_less_than(left: u64, right: u64) -> u64 {
    bool_value(str_value(left) < str_value(right))
}

/// The `length` bytes from byte `index`, clamped to the string. Callers
/// guarantee the boundaries fall between characters. A negative length
/// takes `|length|` bytes ending at `index`, like `binary:part/3`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_string_byte_slice(string: u64, index: u64, length: u64) -> u64 {
    let bytes = str_value(string).as_bytes();
    let index = (index as i64) >> 1;
    let length = (length as i64) >> 1;
    let (start, end) = if length < 0 {
        (index + length, index)
    } else {
        (index, index + length)
    };
    let start = start.clamp(0, bytes.len() as i64) as usize;
    let end = end.clamp(0, bytes.len() as i64) as usize;
    let slice = bytes[start..end.max(start)].to_vec();
    // The name warns that the caller is responsible for character
    // boundaries, exactly like the Erlang implementation.
    box_string(unsafe { String::from_utf8_unchecked(slice) })
}

/// Everything from the first occurrence of the substring onwards, or the
/// whole string when it does not occur.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_string_crop(string: u64, substring: u64) -> u64 {
    let subject = str_value(string);
    match subject.find(str_value(substring)) {
        Some(index) => box_string(subject[index..].to_string()),
        None => gleam_native_inc(string),
    }
}

/// Whether the string starts with the given prefix (both values).
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_string_has_prefix(string: u64, prefix: u64) -> u64 {
    bool_value(str_value(string).starts_with(str_value(prefix)))
}

/// Splits on the first occurrence of the substring: `Ok(#(before, after))`,
/// or `Error(Nil)` when it does not occur. An empty substring never
/// matches, following `string:split/2`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_string_split_once(string: u64, substring: u64) -> u64 {
    let subject = str_value(string);
    let pattern = str_value(substring);
    if pattern.is_empty() {
        return make_error(NIL);
    }
    match subject.split_once(pattern) {
        Some((before, after)) => make_ok(make_tuple2(
            box_string(before.to_string()),
            box_string(after.to_string()),
        )),
        None => make_error(NIL),
    }
}

/// The string without the given prefix, or unchanged when the prefix does
/// not match.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_string_remove_prefix(string: u64, prefix: u64) -> u64 {
    match str_value(string).strip_prefix(str_value(prefix)) {
        Some(rest) => box_string(rest.to_string()),
        None => gleam_native_inc(string),
    }
}

/// The string without the given suffix, or unchanged when the suffix does
/// not match.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_string_remove_suffix(string: u64, suffix: u64) -> u64 {
    match str_value(string).strip_suffix(str_value(suffix)) {
        Some(rest) => box_string(rest.to_string()),
        None => gleam_native_inc(string),
    }
}

/// The first character's code point and the rest of the string, or code
/// point 0 and the unchanged string when empty. The standard library's URI
/// parser uses this with character-based `gleam_native_string_char_slice`,
/// so the two agree on indices.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_string_pop_char(string: u64) -> u64 {
    let subject = str_value(string);
    match subject.chars().next() {
        Some(character) => make_tuple2(
            tag_small_int(character as i64),
            box_string(subject[character.len_utf8()..].to_string()),
        ),
        None => make_tuple2(tag_small_int(0), gleam_native_inc(string)),
    }
}

/// The `length` characters starting at character `from`, clamped to the
/// string.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_string_char_slice(string: u64, from: u64, length: u64) -> u64 {
    let from = ((from as i64) >> 1).max(0) as usize;
    let length = ((length as i64) >> 1).max(0) as usize;
    box_string(str_value(string).chars().skip(from).take(length).collect())
}

// ---------------------------------------------------------------------------
// gleam/string_tree
//
// A native string tree is either a string or a (possibly nested) list of
// string trees, mirroring Erlang's iodata. The runtime flattens on demand.

/// Flattens a string tree into a single string.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_tree_to_string(tree: u64) -> u64 {
    if tree & 1 == 0 && header_kind(heap_header(tree)) == KIND_STRING {
        return gleam_native_inc(tree);
    }
    box_string(tree_to_string(tree))
}

/// The total size in bytes of the tree's text.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_tree_byte_size(tree: u64) -> u64 {
    let mut size = 0i64;
    let mut worklist = vec![tree];
    while let Some(value) = worklist.pop() {
        if value & 1 == 1 {
            continue;
        }
        let header = heap_header(value);
        match header_kind(header) {
            KIND_STRING => size += string_value(value).len() as i64,
            KIND_RECORD => {
                worklist.push(record_field(value, 1));
                worklist.push(record_field(value, 0));
            }
            _ => {}
        }
    }
    tag_small_int(size)
}

/// A tree holding the first tree's text followed by the second's: a
/// two-element list.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_tree_append(tree: u64, suffix: u64) -> u64 {
    make_list(vec![gleam_native_inc(tree), gleam_native_inc(suffix)])
}

/// # Safety
///
/// The arguments must be string tree values.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_tree_lowercase(tree: u64) -> u64 {
    box_string(tree_to_string(tree).to_lowercase())
}

#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_tree_uppercase(tree: u64) -> u64 {
    box_string(tree_to_string(tree).to_uppercase())
}

/// Replaces every occurrence of the pattern in the flattened text.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_tree_replace(tree: u64, pattern: u64, replacement: u64) -> u64 {
    box_string(tree_to_string(tree).replace(str_value(pattern), str_value(replacement)))
}

/// Splits the flattened text on a separator, returning a list of string
/// trees. An empty separator yields the whole text, like the other
/// targets.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_tree_split(tree: u64, pattern: u64) -> u64 {
    let text = tree_to_string(tree);
    let pattern = str_value(pattern);
    if pattern.is_empty() {
        return make_list(vec![box_string(text)]);
    }
    make_list(
        text.split(pattern)
            .map(|part| box_string(part.to_string()))
            .collect(),
    )
}

/// Whether two trees hold the same text.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_tree_is_equal(left: u64, right: u64) -> u64 {
    bool_value(tree_to_string(left) == tree_to_string(right))
}

/// Whether the tree holds no text at all.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_tree_is_empty(tree: u64) -> u64 {
    bool_value(gleam_native_tree_byte_size(tree) == tag_small_int(0))
}

// ---------------------------------------------------------------------------
// gleam/bit_array

/// A bit array holding the string's UTF-8 bytes.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_from_string(string: u64) -> u64 {
    let bytes = str_value(string).as_bytes().to_vec();
    box_bitarray(BitArrayPayload {
        bits: bytes.len() as u64 * 8,
        bytes,
    })
}

/// The array's size in bits.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_bit_size(array: u64) -> u64 {
    tag_small_int(bitarray_value(array).bits as i64)
}

/// The array's size in whole bytes, rounding partial bytes up.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_byte_size(array: u64) -> u64 {
    tag_small_int(bitarray_value(array).bits.div_ceil(8) as i64)
}

/// A byte-aligned slice: `length` bytes from byte `position`, or bytes
/// ending at `position` for a negative length. Out-of-range positions
/// (including any range touching trailing partial bits) yield
/// `Error(Nil)`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_byte_slice(array: u64, position: u64, length: u64) -> u64 {
    let payload = bitarray_value(array);
    let position = (position as i64) >> 1;
    let length = (length as i64) >> 1;
    let start = position.min(position + length);
    let end = position.max(position + length);
    if start < 0 || (end as u64) * 8 > payload.bits {
        return make_error(NIL);
    }
    let bytes = payload.bytes[start as usize..end as usize].to_vec();
    make_ok(box_bitarray(BitArrayPayload {
        bits: (end - start) as u64 * 8,
        bytes,
    }))
}

/// The array's bytes as a string without UTF-8 validation; callers check
/// with [`gleam_native_bitarray_is_utf8`] first.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_unsafe_to_string(array: u64) -> u64 {
    let payload = bitarray_value(array);
    let bytes = payload.bytes[..(payload.bits / 8) as usize].to_vec();
    box_string(unsafe { String::from_utf8_unchecked(bytes) })
}

/// Whether the array is a whole number of bytes of valid UTF-8.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_is_utf8(array: u64) -> u64 {
    let payload = bitarray_value(array);
    bool_value(
        payload.bits.is_multiple_of(8) && std::str::from_utf8(&payload.bytes).is_ok(),
    )
}

/// Joins a list of bit arrays into one.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_concat(list: u64) -> u64 {
    let mut result = BitArrayPayload {
        bits: 0,
        bytes: Vec::new(),
    };
    for element in list_elements(list) {
        let payload = bitarray_value(element);
        append_bits(&mut result, &payload.bytes, 0, payload.bits);
    }
    box_bitarray(result)
}

const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encodes the array (zero-padded to whole bytes) as base64, with or
/// without `=` padding.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_base64_encode(array: u64, padding: u64) -> u64 {
    let payload = bitarray_value(array);
    let bytes = &payload.bytes;
    let mut encoded = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let group = (chunk[0] as u32) << 16
            | (chunk.get(1).copied().unwrap_or(0) as u32) << 8
            | chunk.get(2).copied().unwrap_or(0) as u32;
        encoded.push(BASE64_ALPHABET[(group >> 18) as usize] as char);
        encoded.push(BASE64_ALPHABET[(group >> 12) as usize & 0x3F] as char);
        if chunk.len() > 1 {
            encoded.push(BASE64_ALPHABET[(group >> 6) as usize & 0x3F] as char);
        }
        if chunk.len() > 2 {
            encoded.push(BASE64_ALPHABET[group as usize & 0x3F] as char);
        }
    }
    if padding == TRUE {
        while !encoded.len().is_multiple_of(4) {
            encoded.push('=');
        }
    }
    box_string(encoded)
}

/// Decodes base64 with optional `=` padding, yielding `Error(Nil)` for
/// invalid input.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_base64_decode(string: u64) -> u64 {
    let text = str_value(string).as_bytes();
    // Strip padding, which may only appear at the end.
    let unpadded = match text {
        [rest @ .., b'=', b'='] => rest,
        [rest @ .., b'='] => rest,
        _ => text,
    };
    if unpadded.len() % 4 == 1 {
        return make_error(NIL);
    }
    let mut bytes = Vec::with_capacity(unpadded.len() * 3 / 4);
    let mut group = 0u32;
    let mut collected = 0u32;
    for &byte in unpadded {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return make_error(NIL),
        };
        group = group << 6 | value as u32;
        collected += 6;
        if collected >= 8 {
            collected -= 8;
            bytes.push((group >> collected) as u8);
        }
    }
    make_ok(box_bitarray(BitArrayPayload {
        bits: bytes.len() as u64 * 8,
        bytes,
    }))
}

/// Encodes the array (zero-padded to whole bytes) as uppercase
/// hexadecimal.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_base16_encode(array: u64) -> u64 {
    let payload = bitarray_value(array);
    let mut encoded = String::with_capacity(payload.bytes.len() * 2);
    for byte in &payload.bytes {
        encoded.push_str(&format!("{byte:02X}"));
    }
    box_string(encoded)
}

/// Decodes hexadecimal in either case, yielding `Error(Nil)` for invalid
/// or odd-length input.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_base16_decode(string: u64) -> u64 {
    let text = str_value(string).as_bytes();
    if !text.len().is_multiple_of(2) {
        return make_error(NIL);
    }
    let mut bytes = Vec::with_capacity(text.len() / 2);
    for pair in text.chunks(2) {
        let digit = |byte: u8| -> Option<u8> { (byte as char).to_digit(16).map(|d| d as u8) };
        match (digit(pair[0]), digit(pair[1])) {
            (Some(high), Some(low)) => bytes.push(high << 4 | low),
            _ => return make_error(NIL),
        }
    }
    make_ok(box_bitarray(BitArrayPayload {
        bits: bytes.len() as u64 * 8,
        bytes,
    }))
}

/// The whole array read as an unsigned big-endian integer, paired with its
/// size in bits; used to compare bit arrays.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bitarray_to_int_and_size(array: u64) -> u64 {
    let payload = bitarray_value(array);
    let mut value = BigInt::ZERO;
    let whole = (payload.bits / 8) as usize;
    for &byte in &payload.bytes[..whole] {
        value = (value << 8) + byte;
    }
    let leftover = payload.bits % 8;
    if leftover > 0 {
        let partial = payload.bytes[whole] >> (8 - leftover);
        value = (value << leftover) + partial;
    }
    make_tuple2(retag(value), tag_small_int(payload.bits as i64))
}

// ---------------------------------------------------------------------------
// gleam/dict
//
// A dict is a heap object of kind `KIND_DICT`; the payload and its
// structural ordering live in the core runtime, which must understand
// dicts for equality, destruction, and `echo`. Transient dicts share the
// representation; the standard library uses them linearly so in-place
// mutation is safe.

/// A copy of the dict's map with every key and value's reference count
/// incremented.
fn clone_dict_map(dict: u64) -> BTreeMap<DictKey, u64> {
    dict_payload(dict)
        .map
        .iter()
        .map(|(key, value)| {
            (DictKey(gleam_native_inc(key.0)), gleam_native_inc(*value))
        })
        .collect()
}

/// A new empty dict.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dict_new() -> u64 {
    box_dict(DictPayload {
        map: BTreeMap::new(),
    })
}

/// The number of entries.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dict_size(dict: u64) -> u64 {
    tag_small_int(dict_payload(dict).map.len() as i64)
}

/// `Ok(value)` for the key, or `Error(Nil)`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dict_get(dict: u64, key: u64) -> u64 {
    match dict_payload(dict).map.get(&DictKey(key)) {
        Some(value) => make_ok(gleam_native_inc(*value)),
        None => make_error(NIL),
    }
}

/// Whether the key is present. The key comes first, matching
/// `maps:is_key/2`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dict_has_key(key: u64, dict: u64) -> u64 {
    bool_value(dict_payload(dict).map.contains_key(&DictKey(key)))
}

/// A new dict with the entry added or replaced; the original is unchanged.
/// The key and value come first, matching `maps:put/3`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dict_insert(key: u64, value: u64, dict: u64) -> u64 {
    let mut map = clone_dict_map(dict);
    if let Some(previous) = map.insert(DictKey(gleam_native_inc(key)), gleam_native_inc(value)) {
        let _ = gleam_native_dec(previous);
        // The map keeps the existing key on replacement; release the extra
        // reference the new key took.
        let _ = gleam_native_dec(key);
    }
    box_dict(DictPayload { map })
}

/// The entries as a list of `#(key, value)` tuples in the dict's
/// (structural) order.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dict_to_list(dict: u64) -> u64 {
    make_list(
        dict_payload(dict)
            .map
            .iter()
            .map(|(key, value)| {
                make_tuple2(gleam_native_inc(key.0), gleam_native_inc(*value))
            })
            .collect(),
    )
}

/// A mutable copy of the dict.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dict_to_transient(dict: u64) -> u64 {
    box_dict(DictPayload {
        map: clone_dict_map(dict),
    })
}

/// Freezes a transient: the standard library uses transients linearly, so
/// the same object simply becomes the dict.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dict_from_transient(transient: u64) -> u64 {
    gleam_native_inc(transient)
}

/// Adds or replaces an entry in place. The key and value come first,
/// matching `maps:put/3`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dict_transient_insert(key: u64, value: u64, transient: u64) -> u64 {
    let map = &mut dict_payload_mut(transient).map;
    if let Some(previous) = map.insert(DictKey(gleam_native_inc(key)), gleam_native_inc(value)) {
        let _ = gleam_native_dec(previous);
        let _ = gleam_native_dec(key);
    }
    gleam_native_inc(transient)
}

/// Removes an entry in place, if present. The key comes first, matching
/// `maps:remove/2`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dict_transient_delete(key: u64, transient: u64) -> u64 {
    let map = &mut dict_payload_mut(transient).map;
    if let Some((removed_key, removed_value)) = map.remove_entry(&DictKey(key)) {
        let _ = gleam_native_dec(removed_key.0);
        let _ = gleam_native_dec(removed_value);
    }
    gleam_native_inc(transient)
}

// ---------------------------------------------------------------------------
// gleam/dynamic and gleam/dynamic/decode

/// Names a dynamic value's type as well as the representation allows.
/// Booleans and `Nil` share the small-integer representation with `Int`,
/// so they classify as `Int`.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_classify(value: u64) -> u64 {
    let name = if value & 1 == 1 {
        "Int"
    } else {
        let header = heap_header(value);
        match header_kind(header) {
            KIND_BIGINT => "Int",
            KIND_FLOAT => "Float",
            KIND_STRING => "String",
            KIND_BITARRAY => "BitArray",
            KIND_CLOSURE => "Function",
            KIND_DICT => "Dict",
            KIND_RECORD => match record_display(header) {
                DISPLAY_LIST => "List",
                DISPLAY_TUPLE => "Array",
                DISPLAY_OK | DISPLAY_ERROR => "Result",
                DISPLAY_SOME | DISPLAY_NONE => "Option",
                _ => "Custom type",
            },
            _ => "Unknown",
        }
    };
    box_string(name.to_string())
}

/// Builds a tuple from a list of dynamic values.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_list_to_tuple(list: u64) -> u64 {
    let elements = list_elements(list);
    let record = gleam_native_record_new(0, elements.len() as u64, DISPLAY_TUPLE as u64);
    for (index, element) in elements.iter().enumerate() {
        unsafe {
            *((record as *mut u64).add(1 + index)) = gleam_native_inc(*element);
        }
    }
    record
}

/// `Ok(value)` when the dynamic value is an integer, `Error(0)` otherwise.
/// Booleans and `Nil` share the integer representation and so also decode.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dynamic_int(value: u64) -> u64 {
    if value & 1 == 1 || header_kind(heap_header(value)) == KIND_BIGINT {
        make_ok(gleam_native_inc(value))
    } else {
        make_error(tag_small_int(0))
    }
}

/// `Ok(value)` when the dynamic value is a float, `Error(0.0)` otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dynamic_float(value: u64) -> u64 {
    if value & 1 == 0 && header_kind(heap_header(value)) == KIND_FLOAT {
        make_ok(gleam_native_inc(value))
    } else {
        make_error(box_float(0.0))
    }
}

/// `Ok(bits)` when the dynamic value is a bit array or a string (whose
/// UTF-8 bytes become the array), `Error(<<>>)` otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dynamic_bit_array(value: u64) -> u64 {
    if value & 1 == 1 {
        return make_error(box_bitarray(BitArrayPayload {
            bits: 0,
            bytes: Vec::new(),
        }));
    }
    match header_kind(heap_header(value)) {
        KIND_BITARRAY => make_ok(gleam_native_inc(value)),
        KIND_STRING => make_ok(gleam_native_bitarray_from_string(value)),
        _ => make_error(box_bitarray(BitArrayPayload {
            bits: 0,
            bytes: Vec::new(),
        })),
    }
}

/// `Ok(elements)` when the dynamic value is a list (or a tuple, whose
/// fields become the elements), `Error(Nil)` otherwise. The empty list
/// shares the small-integer representation, so integer 0 also decodes as
/// the empty list.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_dynamic_list(value: u64) -> u64 {
    if value == NIL {
        return make_ok(NIL);
    }
    if value & 1 == 1 {
        return make_error(NIL);
    }
    let header = heap_header(value);
    if header_kind(header) != KIND_RECORD {
        return make_error(NIL);
    }
    match record_display(header) {
        DISPLAY_LIST => make_ok(gleam_native_inc(value)),
        DISPLAY_TUPLE => {
            let fields: Vec<u64> = (0..record_arity(header))
                .map(|index| gleam_native_inc(record_field(value, index)))
                .collect();
            make_ok(make_list(fields))
        }
        _ => make_error(NIL),
    }
}

/// `Ok(dict)` when the dynamic value is a dict, `Error(Nil)` otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_decode_dict(value: u64) -> u64 {
    if value & 1 == 0 && header_kind(heap_header(value)) == KIND_DICT {
        make_ok(gleam_native_inc(value))
    } else {
        make_error(NIL)
    }
}

fn make_some(value: u64) -> u64 {
    let record = gleam_native_record_new(0, 1, DISPLAY_SOME as u64);
    unsafe { *((record as *mut u64).add(1)) = value };
    record
}

fn make_none() -> u64 {
    gleam_native_record_new(1, 0, DISPLAY_NONE as u64)
}

/// Indexes into a dynamic container: dicts by any key; tuples and the
/// first eight elements of lists by integer. Mirrors the Erlang
/// implementation's semantics, including its error kinds.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_bare_index(data: u64, key: u64) -> u64 {
    let key_is_int =
        key & 1 == 1 || header_kind(heap_header(key)) == KIND_BIGINT;
    if data & 1 == 0 {
        let header = heap_header(data);
        match header_kind(header) {
            KIND_DICT => {
                return match dict_payload(data).map.get(&DictKey(key)) {
                    Some(value) => make_ok(make_some(gleam_native_inc(*value))),
                    None => make_ok(make_none()),
                };
            }
            KIND_RECORD if record_display(header) == DISPLAY_LIST && key_is_int => {
                let index = (key as i64) >> 1;
                if key & 1 == 1 && (0..8).contains(&index) {
                    let mut current = data;
                    let mut position = 0;
                    while current & 1 == 0 {
                        if position == index {
                            return make_ok(make_some(gleam_native_inc(
                                record_field(current, 0),
                            )));
                        }
                        position += 1;
                        current = record_field(current, 1);
                    }
                }
                return make_error(box_string("Indexable".to_string()));
            }
            KIND_RECORD if key_is_int => {
                let arity = record_arity(header) as i64;
                let index = (key as i64) >> 1;
                if key & 1 == 1 && (0..arity).contains(&index) {
                    return make_ok(make_some(gleam_native_inc(record_field(
                        data,
                        index as u32,
                    ))));
                }
                return make_ok(make_none());
            }
            _ => {}
        }
    }
    if key_is_int {
        make_error(box_string("Indexable".to_string()))
    } else {
        make_error(box_string("Dict".to_string()))
    }
}

/// Whether the dynamic value is null. The native target has no null
/// values, but `Nil`'s representation is recognised.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_is_null(value: u64) -> u64 {
    bool_value(value == NIL)
}

// ---------------------------------------------------------------------------
// gleam/uri

/// Splits a query string like `a=1&b=2` into decoded key-value pairs.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_uri_parse_query(query: u64) -> u64 {
    let mut pairs = Vec::new();
    for section in str_value(query).split('&') {
        if section.is_empty() {
            continue;
        }
        let (key, value) = match section.split_once('=') {
            Some((key, value)) => (key, value),
            None => (section, ""),
        };
        if key.is_empty() {
            continue;
        }
        let (Some(key), Some(value)) = (
            percent_decode_bytes(key, true),
            percent_decode_bytes(value, true),
        ) else {
            return make_error(NIL);
        };
        pairs.push(make_tuple2(box_string(key), box_string(value)));
    }
    make_ok(make_list(pairs))
}

/// Percent-encodes everything except RFC 3986 unreserved characters and
/// the sub-delimiters the Erlang implementation keeps.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_percent_encode(string: u64) -> u64 {
    let mut encoded = String::new();
    for byte in str_value(string).bytes() {
        let keep = byte.is_ascii_alphanumeric()
            || matches!(byte, b'!' | b'$' | b'\'' | b'(' | b')' | b'*' | b'+' | b'-' | b'.' | b'_' | b'~');
        if keep {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    box_string(encoded)
}

/// Decodes percent-escapes, yielding `Error(Nil)` for malformed escapes
/// or byte sequences that are not UTF-8.
#[unsafe(no_mangle)]
pub extern "C" fn gleam_native_percent_decode(string: u64) -> u64 {
    match percent_decode_bytes(str_value(string), false) {
        Some(decoded) => make_ok(box_string(decoded)),
        None => make_error(NIL),
    }
}

/// Decodes `%XX` escapes (and `+` as space in query mode) into a string,
/// or `None` when an escape is malformed or the result is not UTF-8.
fn percent_decode_bytes(text: &str, query: bool) -> Option<String> {
    let source = text.as_bytes();
    let mut bytes = Vec::with_capacity(source.len());
    let mut index = 0;
    while index < source.len() {
        match source[index] {
            b'%' => {
                let high = (*source.get(index + 1)? as char).to_digit(16)?;
                let low = (*source.get(index + 2)? as char).to_digit(16)?;
                bytes.push((high * 16 + low) as u8);
                index += 3;
            }
            b'+' if query => {
                bytes.push(b' ');
                index += 1;
            }
            byte => {
                bytes.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8(bytes).ok()
}

/// The symbols this crate contributes to the JIT's registry. Ahead-of-time
/// compilation links them from the static library instead.
pub fn symbols() -> Vec<(&'static str, *const u8)> {
    vec![
        ("gleam_native_identity", gleam_native_identity as *const u8),
        (
            "gleam_native_inspect_value",
            gleam_native_inspect_value as *const u8,
        ),
        ("gleam_native_print", gleam_native_print as *const u8),
        (
            "gleam_native_print_error",
            gleam_native_print_error as *const u8,
        ),
        (
            "gleam_native_println_error",
            gleam_native_println_error as *const u8,
        ),
        ("gleam_native_int_parse", gleam_native_int_parse as *const u8),
        (
            "gleam_native_int_base_parse",
            gleam_native_int_base_parse as *const u8,
        ),
        (
            "gleam_native_int_to_base_string",
            gleam_native_int_to_base_string as *const u8,
        ),
        (
            "gleam_native_int_to_float",
            gleam_native_int_to_float as *const u8,
        ),
        (
            "gleam_native_int_bitwise_and",
            gleam_native_int_bitwise_and as *const u8,
        ),
        (
            "gleam_native_int_bitwise_or",
            gleam_native_int_bitwise_or as *const u8,
        ),
        (
            "gleam_native_int_bitwise_exclusive_or",
            gleam_native_int_bitwise_exclusive_or as *const u8,
        ),
        (
            "gleam_native_int_bitwise_not",
            gleam_native_int_bitwise_not as *const u8,
        ),
        (
            "gleam_native_int_bitwise_shift_left",
            gleam_native_int_bitwise_shift_left as *const u8,
        ),
        (
            "gleam_native_int_bitwise_shift_right",
            gleam_native_int_bitwise_shift_right as *const u8,
        ),
        (
            "gleam_native_float_parse",
            gleam_native_float_parse as *const u8,
        ),
        (
            "gleam_native_float_ceiling",
            gleam_native_float_ceiling as *const u8,
        ),
        (
            "gleam_native_float_floor",
            gleam_native_float_floor as *const u8,
        ),
        (
            "gleam_native_float_round",
            gleam_native_float_round as *const u8,
        ),
        (
            "gleam_native_float_truncate",
            gleam_native_float_truncate as *const u8,
        ),
        (
            "gleam_native_float_power",
            gleam_native_float_power as *const u8,
        ),
        (
            "gleam_native_float_random",
            gleam_native_float_random as *const u8,
        ),
        ("gleam_native_float_log", gleam_native_float_log as *const u8),
        (
            "gleam_native_float_exponential",
            gleam_native_float_exponential as *const u8,
        ),
        (
            "gleam_native_string_less_than",
            gleam_native_string_less_than as *const u8,
        ),
        (
            "gleam_native_string_byte_slice",
            gleam_native_string_byte_slice as *const u8,
        ),
        (
            "gleam_native_string_crop",
            gleam_native_string_crop as *const u8,
        ),
        (
            "gleam_native_string_has_prefix",
            gleam_native_string_has_prefix as *const u8,
        ),
        (
            "gleam_native_string_split_once",
            gleam_native_string_split_once as *const u8,
        ),
        (
            "gleam_native_string_remove_prefix",
            gleam_native_string_remove_prefix as *const u8,
        ),
        (
            "gleam_native_string_remove_suffix",
            gleam_native_string_remove_suffix as *const u8,
        ),
        (
            "gleam_native_string_pop_char",
            gleam_native_string_pop_char as *const u8,
        ),
        (
            "gleam_native_string_char_slice",
            gleam_native_string_char_slice as *const u8,
        ),
        (
            "gleam_native_tree_to_string",
            gleam_native_tree_to_string as *const u8,
        ),
        (
            "gleam_native_tree_byte_size",
            gleam_native_tree_byte_size as *const u8,
        ),
        (
            "gleam_native_tree_append",
            gleam_native_tree_append as *const u8,
        ),
        (
            "gleam_native_tree_lowercase",
            gleam_native_tree_lowercase as *const u8,
        ),
        (
            "gleam_native_tree_uppercase",
            gleam_native_tree_uppercase as *const u8,
        ),
        (
            "gleam_native_tree_replace",
            gleam_native_tree_replace as *const u8,
        ),
        (
            "gleam_native_tree_split",
            gleam_native_tree_split as *const u8,
        ),
        (
            "gleam_native_tree_is_equal",
            gleam_native_tree_is_equal as *const u8,
        ),
        (
            "gleam_native_tree_is_empty",
            gleam_native_tree_is_empty as *const u8,
        ),
        (
            "gleam_native_bitarray_from_string",
            gleam_native_bitarray_from_string as *const u8,
        ),
        (
            "gleam_native_bitarray_bit_size",
            gleam_native_bitarray_bit_size as *const u8,
        ),
        (
            "gleam_native_bitarray_byte_size",
            gleam_native_bitarray_byte_size as *const u8,
        ),
        (
            "gleam_native_bitarray_byte_slice",
            gleam_native_bitarray_byte_slice as *const u8,
        ),
        (
            "gleam_native_bitarray_unsafe_to_string",
            gleam_native_bitarray_unsafe_to_string as *const u8,
        ),
        (
            "gleam_native_bitarray_is_utf8",
            gleam_native_bitarray_is_utf8 as *const u8,
        ),
        (
            "gleam_native_bitarray_concat",
            gleam_native_bitarray_concat as *const u8,
        ),
        (
            "gleam_native_bitarray_base64_encode",
            gleam_native_bitarray_base64_encode as *const u8,
        ),
        (
            "gleam_native_bitarray_base64_decode",
            gleam_native_bitarray_base64_decode as *const u8,
        ),
        (
            "gleam_native_bitarray_base16_encode",
            gleam_native_bitarray_base16_encode as *const u8,
        ),
        (
            "gleam_native_bitarray_base16_decode",
            gleam_native_bitarray_base16_decode as *const u8,
        ),
        (
            "gleam_native_bitarray_to_int_and_size",
            gleam_native_bitarray_to_int_and_size as *const u8,
        ),
        ("gleam_native_dict_new", gleam_native_dict_new as *const u8),
        ("gleam_native_dict_size", gleam_native_dict_size as *const u8),
        ("gleam_native_dict_get", gleam_native_dict_get as *const u8),
        (
            "gleam_native_dict_has_key",
            gleam_native_dict_has_key as *const u8,
        ),
        (
            "gleam_native_dict_insert",
            gleam_native_dict_insert as *const u8,
        ),
        (
            "gleam_native_dict_to_list",
            gleam_native_dict_to_list as *const u8,
        ),
        (
            "gleam_native_dict_to_transient",
            gleam_native_dict_to_transient as *const u8,
        ),
        (
            "gleam_native_dict_from_transient",
            gleam_native_dict_from_transient as *const u8,
        ),
        (
            "gleam_native_dict_transient_insert",
            gleam_native_dict_transient_insert as *const u8,
        ),
        (
            "gleam_native_dict_transient_delete",
            gleam_native_dict_transient_delete as *const u8,
        ),
        ("gleam_native_classify", gleam_native_classify as *const u8),
        (
            "gleam_native_list_to_tuple",
            gleam_native_list_to_tuple as *const u8,
        ),
        (
            "gleam_native_dynamic_int",
            gleam_native_dynamic_int as *const u8,
        ),
        (
            "gleam_native_dynamic_float",
            gleam_native_dynamic_float as *const u8,
        ),
        (
            "gleam_native_dynamic_bit_array",
            gleam_native_dynamic_bit_array as *const u8,
        ),
        (
            "gleam_native_dynamic_list",
            gleam_native_dynamic_list as *const u8,
        ),
        (
            "gleam_native_decode_dict",
            gleam_native_decode_dict as *const u8,
        ),
        (
            "gleam_native_bare_index",
            gleam_native_bare_index as *const u8,
        ),
        ("gleam_native_is_null", gleam_native_is_null as *const u8),
        (
            "gleam_native_uri_parse_query",
            gleam_native_uri_parse_query as *const u8,
        ),
        (
            "gleam_native_percent_encode",
            gleam_native_percent_encode as *const u8,
        ),
        (
            "gleam_native_percent_decode",
            gleam_native_percent_decode as *const u8,
        ),
    ]
}
