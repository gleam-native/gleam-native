// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

fn size(n: Int) -> String {
  case n {
    1 | 2 | 3 -> "small"
    100 -> "big"
    n if n < 0 -> "negative"
    _ -> "medium"
  }
}

fn same(a: Int, b: Int) -> String {
  case a, b {
    x, y if x == y -> "same"
    x, y if !{ x < y } -> "descending"
    _, _ -> "ascending"
  }
}

fn describe(word: String) -> String {
  case word {
    "hi" -> "greeting"
    "say " <> words -> words
    _ -> "unknown"
  }
}

fn floaty(f: Float) -> String {
  case f {
    1.5 -> "one and a half"
    _ -> "other"
  }
}

pub fn main() -> Nil {
  echo size(2)
  echo size(100)
  echo size(-5)
  echo size(50)
  echo same(3, 3)
  echo same(9, 4)
  echo same(2, 8)
  echo describe("hi")
  echo describe("say hello")
  echo describe("zap")
  echo floaty(1.5)
  echo floaty(2.5)
  Nil
}
