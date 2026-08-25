// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

// Arbitrary precision integers, matching Erlang exactly. The JavaScript
// target legitimately diverges here (its integers are 64-bit floats), so
// this case runs on the Erlang and native targets only.

fn classify(n: Int) -> String {
  case n {
    9223372036854775808 -> "big literal"
    _ -> "not"
  }
}

pub fn main() -> Nil {
  // Promotion and demotion around the small integer boundary.
  echo 4611686018427387903 * 4
  echo 9223372036854775808 - 9223372036854775807
  echo 9223372036854775808 > 5
  echo classify(9223372036854775808)
  echo classify(5)
  // Negating the most negative small integer promotes.
  let min = -4611686018427387904
  echo -min
  Nil
}
