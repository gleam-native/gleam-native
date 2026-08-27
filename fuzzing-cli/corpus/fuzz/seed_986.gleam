pub const limit_value: String = "abc"
pub const seed_value: Float = 1.5

pub type V0 {
  Record(value: String, inner: Float)
  Number(List(Int))
  Cv1(value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(default: Int) -> List(Int) {
case [3], <<"a":utf8>> {
    [_], <<l:16, "a":utf8, _:utf8>> -> []
    [b, ..rest], _ -> rest
    _, v2 -> [42]
  }
}

fn f1(value: Float, v3: Bool) -> List(Int) {
fn(v4) { case "data" <> "b" {
    item -> {
      let n = "res"
      let length = value
      [42, 10]
    }
    "bc" | "data" <> _ -> []
    "res" -> class(5)
  } }(0.0)
}

fn yield(v5: Int, self_: Int, length: Int) -> Bool {
True
}

pub fn main() {
  let pair = case #([42, 2], []), 2 {
    #([a, _, ..], []), _ if a % 2 == 0 || a > 0 -> []
    #([3, ..rest], [5, ..tail]), _ -> f1(2.0, True)
    #([3, ..rest], [_, _, ..]), _ -> rest
    _, _ -> fn(v6) { [] }(0.0)
  }
  let delete = case [10] {
    [limit_value] -> limit_value
    [6, ..rest] -> 100 - 0
    [_, ..rest] -> 100
    _ -> 0
  }
  echo case walk(pair, delete) {
    item -> pair
    6 -> pair
    9 | 7 -> pair
  }
  echo case limit_value {
    "x" | "ab" -> case limit_value <> limit_value {
      "x" -> seed_value
      "abc" -> {
        3.14
      } *. seed_value
      b -> seed_value /. {
        3.14
      }
    }
    "constructor" as whole -> case delete {
      0 -> seed_value /. {
        3.14
      }
      constructor -> {
        1.0
      } /. {
        0.5
      }
    }
    "abc" | "x" <> _ -> {
      fn(v7) { 1.0 }(0.1)
    } -. seed_value
    v8 -> 0.0
  }
}
