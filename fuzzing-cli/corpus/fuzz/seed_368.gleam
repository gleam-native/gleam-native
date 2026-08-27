fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Bool, v0: Float, v1: Bool) -> Int {
case "b", "data" {
    "bc", _ -> 42
    _, _ -> 100 + {
      0 - 4
    }
    "b", "data" -> 5 % 3
  }
}

pub fn main() {
  echo case False {
    _ -> 5
    inner -> 3
  }
  echo {
    let z = case fn(v2) { "bc" }(True) {
      item | "data" <> item -> "data"
      item -> fn(v3, v4) { "x" }(False, 0)
    }
    let default = False
    {
      let pair = []
      let pair = z
      z == "a"
    }
  }
  echo case "res" {
    "a" <> rest as whole -> 100.0
    "x" -> case 10.0, {
        100.0
      } +. {
        3.14
      } {
      _, 0.25 -> 3.14
      1.5, 0.5 -> 10.0
      _, _ -> 0.0
    }
    v5 -> case False {
      item -> {
        10.0
      } -. {
        100.0
      }
      False -> 1.5
    }
  }
  echo {
    fn(v6) { True }(True)
  } || {
    0 <= 4
  }
}
