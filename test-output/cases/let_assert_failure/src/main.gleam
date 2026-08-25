// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

fn broken() -> Result(Int, String) {
  Error("nope")
}

pub fn main() -> Nil {
  let assert Ok(_) = broken() as "wanted a success"
  Nil
}
