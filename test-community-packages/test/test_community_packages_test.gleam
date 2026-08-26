// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 The Gleam contributors

import gleeunit
import gleeunit/should

@target(erlang)
pub fn main() {
  gleeunit.main()
}

@target(javascript)
pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn hello_world_test() {
  1
  |> should.equal(1)
}
