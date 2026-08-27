pub const euler_value: Int = 3

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(prototype: String) -> Float {
{
    case 4, [42, 3] {
      0, [4, 7, ..] -> 3.14
      5, [4, 6, ..] as whole -> {
        let prototype = 0.1
        let whole = True
        0.0
      }
      7 as whole, [_] -> {
        let value = [0, 7]
        0.25
      }
      v0, _ -> 10.0
    }
  } /. {
    3.14
  }
}

fn f1(constructor: #(Int, List(Int)), v1: List(Int), v2: List(Int)) -> Float {
{
    case {
        let length = 5
        1
      } {
      constructor -> {
        0.1
      } -. {
        1.5
      }
      _ -> {
        0.0
      } +. {
        0.0
      }
    }
  } /. {
    0.5
  }
}

fn f2(v3: Int, class: Bool, pair: Int) -> Float {
10.0
}

pub fn main() {
  echo euler_value
  echo {
    5 - euler_value
  } + {
    spin(euler_value, 0) - 42
  }
  echo {
    fn(v4, v5) { "abc" }("data", 0.25)
  } != {
    {
      "abc" <> "bc"
    } <> {
      fn(v6, v7) { v7 }(42, "b")
    }
  }
  echo case [] {
    [] as whole -> case !True {
      inner -> [3, 10]
      whole -> fn(v8, v9) { [1, 10] }("bc", 100)
      True -> fn(v10, v11) { whole }(4, "res")
    }
    [3] -> {
      let s = fn(v12, v13) { [10] }("data", "data")
      let euler_value = True
      fn(v14, v15) { [100] }(True, 0)
    }
    _ -> case spin(42, 1) {
      item -> fn(v16, v17) { [4, 42] }(7, True)
      4 | 7 -> fn(v18) { [100, 42] }("bc")
    }
  }
}
