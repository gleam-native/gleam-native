// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

pub type Config {
  Config(host: String, port: Int)
}

fn found(value: Int) -> Result(Int, String) {
  Ok(value)
}

pub fn main() -> Nil {
  let #(a, b) = #(40, 2)
  echo a + b
  let Config(host: host, port: port) = Config("localhost", 8080)
  echo host
  echo port
  let assert [first, ..rest] = [1, 2, 3]
  let assert [second, ..] = rest
  echo first + second
  let assert Ok(value) = found(9000)
  echo value
  let assert "gleam-" <> version = "gleam-1.18"
  echo version
  let assert <<x:16>> = <<2, 1>>
  echo x
  Nil
}
