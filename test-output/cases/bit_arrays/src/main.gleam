// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

fn parse_frame(packet: BitArray) -> String {
  // Length-prefixed frame: 1-byte length, then that many bytes.
  case packet {
    <<length, payload:bytes-size(length), rest:bits>> ->
      case payload, rest {
        <<"hi">>, <<>> -> "greeting"
        _, _ ->
          case length {
            3 -> "frame of three"
            _ -> "other frame"
          }
      }
    _ -> "malformed"
  }
}

pub fn main() -> Nil {
  // Dynamic length-prefixed parsing, including rejection.
  echo parse_frame(<<2, "hi":utf8>>)
  echo parse_frame(<<3, 7, 8, 9>>)
  echo parse_frame(<<5, 1>>)

  // Unaligned segments and the Erlang reference value for <<1000:12>>.
  let assert <<first, tail:4>> = <<1000:12>>
  echo first
  echo tail
  case <<1:1, 0:1, 1:1, 0:5, 10:4, 3:4>> {
    <<a:1, _:1, c:1, _:5, high:4, low:4>> -> {
      echo a * 100 + c * 10 + high - low
      Nil
    }
    _ -> Nil
  }

  // Endianness and signedness.
  let assert <<value:16-little>> = <<1, 2>>
  echo value
  case <<255>> {
    <<n:signed>> -> {
      echo n
      Nil
    }
    _ -> Nil
  }

  // Floats at 16, 32, and 64 bits round-trip through patterns.
  case <<1.5:16, 2.5:32-little, 3.25>> {
    <<a:16-float, b:32-float-little, c:float>> -> {
      echo a
      echo b
      echo c
      Nil
    }
    _ -> Nil
  }

  // Wide reads produce big integers; sizes may use arithmetic.
  let width = 2
  let assert <<head:size(width * 8), _:bits>> = <<513:16, 42>>
  echo head

  // Encodings, splices, and equality.
  echo <<"AB":utf16-little>> == <<65, 0, 66, 0>>
  echo <<<<0xAB>>:bits-size(4)>> == <<0xA:4>>
  Nil
}
