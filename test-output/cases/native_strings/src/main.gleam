// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

@external(native, "runtime", "print_bool")
pub fn print_bool(value: Bool) -> Nil

@external(native, "runtime", "gleam_native_string_length")
fn length(string: String) -> Int

@external(native, "runtime", "gleam_native_string_compare")
fn compare(left: String, right: String) -> Int

@external(native, "runtime", "gleam_native_string_uppercase")
fn uppercase(string: String) -> String

@external(native, "runtime", "gleam_native_string_reverse")
fn reverse(string: String) -> String

@external(native, "runtime", "gleam_native_string_slice")
fn slice(string: String, start: Int, length: Int) -> String

@external(native, "runtime", "gleam_native_string_split")
fn split(string: String, on: String) -> List(String)

@external(native, "runtime", "gleam_native_string_pop_grapheme")
fn pop_grapheme(string: String) -> Result(#(String, String), Nil)

@external(native, "runtime", "gleam_native_string_contains")
fn contains(string: String, needle: String) -> Bool

@external(native, "runtime", "gleam_native_string_trim")
fn trim(string: String) -> String

@external(native, "runtime", "gleam_native_int_to_string")
fn int_to_string(value: Int) -> String

fn join(words: List(String), separator: String) -> String {
  case words {
    [] -> ""
    [word] -> word
    [word, ..rest] -> word <> separator <> join(rest, separator)
  }
}

fn shout_each(string: String) -> String {
  // Runtime-built results destructure in ordinary Gleam patterns.
  case pop_grapheme(string) {
    Ok(#(head, rest)) -> uppercase(head) <> shout_each(rest)
    Error(Nil) -> ""
  }
}

pub fn main() -> Nil {
  print_int(length("héllo"))
  print_int(compare("apple", "banana"))
  println(uppercase("héllo"))
  println(reverse("stressed"))
  println(slice("héllo wörld", 6, 5))
  println(join(split("a,b,c", ","), " + "))
  println(shout_each("héllo"))
  print_bool(contains("wibble", "bb"))
  println(trim("   tidy   "))
  println(int_to_string(9223372036854775808))
}
