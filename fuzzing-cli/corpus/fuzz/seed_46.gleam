pub const tag_value: String = ""

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v0: Int, x: Bool) -> Float {
{
    case #([], [100, 4]) {
      item -> {
        0.1
      } +. {
        0.5
      }
      constructor -> 0.5
    }
  } +. {
    fn(v1, v2) { v1 }(10.0, "")
  }
}

pub fn main() {
  echo case 10.0 {
    item -> [42, 2]
    10.0 -> case "constructor" <> "res" {
      _ -> [100, 4]
      _ | "" <> _ -> [100, 5]
    }
    1.5 | 1.5 -> case 2 > 3, {
        let value = tag_value
        True
      } {
      _, z -> fn(v3, v4) { [] }(0, "constructor")
      tag_value, True as whole -> [7, 0]
    }
  }
  echo 2
  echo fn(v5, v6) { [100] }(3, 100)
  echo case tag_value <> "", "a" {
    v7, "data" <> rest as whole if rest == "" && whole != "b" -> 7
    "bc", "b" -> case fn(v8, v9) { [5] }(5, 2.0) {
      [_, 2, ..] -> spin(0, 3)
      [] -> 7 - 2
      [_] -> 2
      _ -> 2
    }
    "x", v10 -> {
      let default = {
        let v10 = 5
        let v10 = 0.0
        tag_value
      }
      7 - 42
    }
    _, v11 -> 3 + 4
  }
}
