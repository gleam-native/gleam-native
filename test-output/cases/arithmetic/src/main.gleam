// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

const answer = 42

const limits = #(1, 100)

const defaults = [42, 7]

fn sum(list: List(Int)) -> Int {
  case list {
    [] -> 0
    [first, ..rest] -> first + sum(rest)
  }
}

pub fn main() -> Nil {
  echo answer
  echo limits.1 - limits.0
  echo sum(defaults)
  // Truncating division, dividend-sign remainder, zero rules.
  echo -7 / 2
  echo -7 % 2
  echo 1 / 0
  echo 0.1 +. 0.2
  echo 1.0 /. 0.0 == 0.0
  echo 2 <= 2
  echo [1, 2] == [1, 2]
  echo #(1, "a") == #(1, "b")
  // echo passes its value through.
  let through = echo 21 * 2
  echo through
  Nil
}
