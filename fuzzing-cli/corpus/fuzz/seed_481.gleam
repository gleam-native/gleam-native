pub const pi_value: Float = 0.1
pub const limit_value: Bool = True
pub const golden_value: Int = 100

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(y: String) -> String {
case y {
    "data" -> "bc"
    inner -> "res" <> y
  }
}

fn extends(v0: Bool, x: Int, v1: Int) -> String {
{
    case True {
      b -> {
        let v0 = 3.14
        ""
      }
      True -> constructor("res")
      False | True -> constructor("x")
    }
  } <> {
    "a" <> "data"
  }
}

pub fn main() {
  let z = golden_value
  let v = case True {
    True -> [42, 42]
    True | True -> [10, 7]
    False -> [3]
  }
  echo "res" <> {
    "constructor" <> {
      limit_value |> extends(0, {
        let z = [5]
        let z = "constructor"
        4
      })
    }
  }
  echo case "res", #("", True) {
    constructor, #("bc" <> rest as whole, _) -> pi_value /. {
      1.0
    }
    "data", #(item, _) -> pi_value
    "constructor", #(_, False) -> case fn(v2) { #(False, True) }(2.0) {
      #(True, False) -> pi_value
      constructor -> pi_value
      #(True, v) -> pi_value
    }
    v3, _ -> {
      let v3 = "data"
      let constructor = limit_value && limit_value
      pi_value
    }
  }
}
