pub const tag_value: Int = 4

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(arguments: String) -> Int {
3
}

pub fn main() {
  echo case False, fn(v0, v1) { "res" }(1, 3) {
    True, "ab" <> rest if rest == "bc" -> False
    _, "constructor" <> rest if rest == "ab" -> {
      {
        100.0
      } +. {
        1.0
      }
    } <. {
      {
        let z = []
        let z = 1
        10.0
      }
    }
    True, "res" -> case True {
      item -> fn(v2) { item }(2.0)
      item -> True
      v3 -> {
        let acc = [10]
        let l = "ab"
        True
      }
    }
    _, _ -> True
  }
  echo {
    case {
        let value = "data"
        True
      } {
      False | True -> 1.5
      True -> 1.0
    }
  } -. {
    case False || False {
      False -> 1.5
      constructor -> 0.0
      False -> {
        3.14
      } -. {
        3.14
      }
    }
  }
  echo fn(v4, v5) { [] }(0.0, False)
}
