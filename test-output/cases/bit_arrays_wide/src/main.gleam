// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

// Bit array features JavaScript does not support: reads wider than its
// 52-bit integers, and native endianness. Erlang and native share the
// snapshot.

pub fn main() -> Nil {
  // A wide read produces a big integer.
  let assert <<wide:size(128)>> = <<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0>>
  echo wide == 256
  echo wide

  // Native endianness (little on every supported machine).
  echo <<258:16-native>> == <<258:16-little>>
  Nil
}
