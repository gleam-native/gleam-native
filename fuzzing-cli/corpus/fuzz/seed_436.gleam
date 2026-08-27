pub const seed_value: Bool = True
pub const tag_value: String = "b"

pub type Object {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(default: String) -> String {
default <> {
    case default {
      "bc" <> _ -> {
        let m = 4
        let default = True
        "x"
      }
      "abc" | "res" <> _ -> default
      item -> item <> item
    }
  }
}

pub fn main() {
  let seed_value = static(tag_value) |> static()
  let y = case tag_value <> seed_value {
    b -> {
      2.0
    } -. {
      0.5
    }
    item -> 10.0
  }
  echo case Record {
    inner -> [1, 42]
    constructor -> [4, 0]
    _ -> [1, 1]
  }
  echo {
    let value = fn(v0, v1) { fn(v2) { tag_value }(False) }(2.0, "res")
    let s = case 1 {
      inner -> [10]
      item -> []
      y -> [100]
    }
    static("constructor")
  }
  echo case fn(v3, v4) { y }(0.25, "x") {
    inner -> [42]
    constructor -> [5, 2]
    _ -> case [1] {
      [9] as whole -> [3, 42]
      [_, 4, ..] -> fn(v5, v6) { [] }(0, "abc")
      [5, ..rest] as whole -> {
        let item = True
        rest
      }
      _ -> fn(v7, v8) { [42] }(1.0, 10)
    }
  }
  echo seed_value
}
