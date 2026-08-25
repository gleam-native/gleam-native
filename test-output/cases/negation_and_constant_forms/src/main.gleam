// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

pub type Config {
  Config(host: String, port: Int, secure: Bool)
}

pub const default = Config("localhost", 80, False)

const data = <<1, 300:16-little, 2.5:32, "hé":utf8, 1:1, 0:3>>

const wrap = Ok

fn map(list: List(a), with: fn(a) -> b) -> List(b) {
  case list {
    [] -> []
    [first, ..rest] -> [with(first), ..map(rest, with)]
  }
}

pub fn main() -> Nil {
  let yes = True
  echo !yes
  let n = 42
  echo -n
  let config = Config(..default, port: 443, secure: !default.secure)
  echo config.port
  echo config.secure

  case data {
    <<1, small:16-little, f:32-float, text:bytes-size(3), pad:4>> -> {
      echo small
      echo f == 2.5
      echo text == <<"hé":utf8>>
      echo pad
      Nil
    }
    _ -> Nil
  }

  let wrapped = case map([1, 2], wrap) {
    [Ok(a), Ok(b)] -> a + b
    _ -> -1
  }
  echo wrapped
  Nil
}
