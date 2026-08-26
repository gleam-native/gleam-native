// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

fn swap(pair: #(Int, Int)) -> #(Int, Int) {
  case pair {
    #(first, second) -> #(second, first)
  }
}

fn scale(point: #(Float, Float), factor: Float) -> #(Float, Float) {
  #(point.0 *. factor, point.1 *. factor)
}

fn describe(pair: #(Int, String)) -> String {
  case pair {
    #(1, word) -> "one " <> word
    #(_, word) if pair.0 > 100 -> "many " <> word
    #(_, word) -> "some " <> word
  }
}

pub fn main() -> Nil {
  // Access, including nested tuples.
  let nested = #(1, #(2, 3))
  echo nested.0
  echo nested.1.0 + nested.1.1

  // Destructuring and reconstruction.
  let swapped = swap(#(40, 2))
  echo swapped.0 - swapped.1

  // Mixed element types and tuple index in guards.
  echo describe(#(1, "apple"))
  echo describe(#(999, "grapes"))
  echo describe(#(7, "pears"))

  // Tuples of floats through a function.
  let scaled = scale(#(1.5, 2.5), 1.5)
  echo scaled.0
  echo scaled.1
  Nil
}
