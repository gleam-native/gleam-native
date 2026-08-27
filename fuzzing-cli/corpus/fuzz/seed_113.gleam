pub const limit_value: String = "bc"
pub const golden_value: Float = 3.14

pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(l: Int, new: List(Int)) -> Int {
case fn(v2) { True }("b") {
    True -> 3
    a -> 10 + {
      3 - l
    }
  }
}

pub fn main() {
  echo case True || False {
    a -> {
      let n = golden_value
      [5]
    }
    False -> [5, 100]
    constructor -> case True, limit_value <> limit_value {
      _, v3 -> {
        let class = constructor
        [10]
      }
      True, "b" <> rest -> fn(v4, v5) { [] }(0.0, "constructor")
    }
  }
  echo case True, limit_value <> limit_value {
    False, "data" -> case arguments(10, [3]) {
      3 -> "abc"
      _ -> limit_value
    }
    True, "constructor" -> "a" <> {
      limit_value <> limit_value
    }
    v6, "a" -> "x"
    _, _ -> {
      "ab" <> "data"
    } <> ""
  }
  echo {
    case #(3, [10]), "x" {
      #(_, [_, ..rest]), _ -> limit_value <> limit_value
      #(6 as whole, [_] as it), "data" if whole % 2 == 0 -> {
        let length = [7, 2]
        limit_value
      }
      #(9, [x]), _ -> limit_value
      v7, _ -> fn(v8) { "res" }(5)
    }
  } <> {
    fn(v9, v10) { limit_value }(0.5, 1.5)
  }
  echo golden_value
}
