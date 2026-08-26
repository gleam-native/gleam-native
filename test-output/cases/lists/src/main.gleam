// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

fn sum(list: List(Int)) -> Int {
  case list {
    [] -> 0
    [first, ..rest] -> first + sum(rest)
  }
}

fn length(list: List(Int)) -> Int {
  case list {
    [] -> 0
    [_, ..rest] -> 1 + length(rest)
  }
}

fn reverse_into(list: List(Int), accumulator: List(Int)) -> List(Int) {
  case list {
    [] -> accumulator
    [first, ..rest] -> reverse_into(rest, [first, ..accumulator])
  }
}

fn join(words: List(String)) -> String {
  case words {
    [] -> ""
    [word] -> word
    [word, ..rest] -> word <> " " <> join(rest)
  }
}

fn shape(list: List(Int)) -> String {
  case list {
    [] -> "empty"
    [_] -> "single"
    [first, second] if first == second -> "twin pair"
    [_, _] -> "pair"
    _ -> "longer"
  }
}

pub fn main() -> Nil {
  let numbers = [5, 1, 9, 3]
  echo sum(numbers)
  echo length(numbers)
  echo sum([40, ..[2]])

  // Reversal: sum of leading pair differs after reversing.
  let leading = case reverse_into(numbers, []) {
    [first, second, ..] -> first * 10 + second
    _ -> -1
  }
  echo leading

  echo join(["lists", "work", "everywhere"])

  echo shape([])
  echo shape([1])
  echo shape([4, 4])
  echo shape([4, 5])
  echo shape(numbers)
  Nil
}
