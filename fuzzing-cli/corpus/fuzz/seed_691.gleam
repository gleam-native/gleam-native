fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(s: String) -> Bool {
walk(fn(v0) { [5, 42] }(2), 0 + 42) >= {
    case [3] {
      [] -> 1
      [constructor] -> [1, 2] |> walk(3)
      v1 -> 100
    }
  }
}

fn extends(v2: String, m: Float) -> String {
"ab"
}

pub fn main() {
  echo case [], 100.0 {
    [], 0.5 -> extends(fn(v3, v4) { v3 }("ab", 5), {
      let acc = 42
      100.0
    })
    [], _ -> fn(v5, v6) { "" }(1, "a")
    [b, constructor, ..], _ -> case b - constructor {
      _ -> fn(v7) { "res" }(5)
      _ | 7 -> "ab"
    }
    _, _ -> "a"
  }
  echo case #(2.0, []) {
    #(3.14 as whole, []) -> 10.0
    #(_, [b, ..rest]) -> 0.0
    _ -> case {
        let self_ = 10.0
        42
      } {
      l -> 0.1
      1 as whole -> {
        let length = False
        0.1
      }
    }
  }
  echo {
    fn(v8) { [] |> walk(2 + 2) }(True)
  } > {
    case [5, 1] {
      [0] -> 3
      [h] if h <= 6 && h == 2 -> 0
      [x, _, ..] -> 5 % 4
      _ -> 5 - 4
    }
  }
}
