// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

fn deep(n: Int) -> Int {
  case n {
    0 -> 0
    _ -> 1 + deep(n - 1)
  }
}

pub fn main() -> Nil {
  print_int(deep(200_000))
}
