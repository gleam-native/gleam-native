pub const limit_value: Float = 0.25

pub type V0 {
  Ok(value: String, inner: Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(this_: Float, v1: Float) -> String {
case Ok("constructor", 0.25) {
    Ok("b", 0.0 as whole) -> "res"
    constructor -> case fn(v2, v3) { v3 }(0.1, "x"), True {
      "" <> _, True -> {
        let n = 0.1
        let default = "bc"
        ""
      }
      _, True as whole -> "ab"
      v4, v5 -> "a"
    }
  }
}

fn arguments(class: List(Int), item: String) -> Int {
2
}

fn export(v6: List(Int)) -> List(Int) {
case "bc", Ok("res", 0.25) {
    "bc" as whole, Ok(_, 1.0) -> case 4, 4 >= 5 {
      2, _ -> {
        let s = 100.0
        let self_ = [10, 1]
        []
      }
      3, False as whole -> v6
      _, _ -> v6
    }
    "ab", Ok("abc" <> _, 0.25) as whole -> v6
    v6, _ -> [5, 7]
  }
}

pub fn main() {
  let x = case 7 - 10 {
    _ | 3 -> ""
    4 -> f0(1.0, 100.0)
  }
  echo case 100 {
    item -> {
      let item = {
        let limit_value = x
        let acc = item
        True
      }
      let x = 10
      !item
    }
    v7 -> False
  }
}
