fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(acc: #(List(Int), List(Int))) -> List(Int) {
case "b" {
    "data" | "res" -> [2, 7]
    "a" -> case {
        let acc = False
        [100]
      } {
      [2, 2, ..] -> [100, 7]
      [1, ..rest] -> [0]
      [] -> [1]
      v0 -> []
    }
    constructor -> fn(v1) { [4] }(7)
  }
}

fn f1(v2: Int, pair: Bool, v3: #(List(Int), List(Int))) -> List(Int) {
case "x", [42, 5] {
    "" <> _, [1, ..rest] -> fn(v4) { #([], []) |> constructor() }("")
    "bc" as whole, [3] as it -> case 10.0, v2 - v2 {
      10.0, 6 -> constructor(#([10, 100], [1]))
      _, 3 -> [10, 4]
      10.0, 2 -> fn(v5) { it }(100)
      _, v6 -> constructor(v3)
    }
    "bc" <> _, [5, ..rest] as whole -> []
    v7, _ -> constructor(fn(v8, v9) { v3 }(10, 10.0))
  }
}

pub fn main() {
  let x = case 0 + 7 {
    5 -> ""
    7 -> {
      let length = "ab"
      let acc = []
      length
    }
    v10 -> fn(v11) { "res" }(10.0)
  }
  let value = []
  echo {
    case "res" <> x, 42 {
      "ab" <> rest, _ -> fn(v12) { 0 }("data")
      "bc" <> rest, new -> {
        let new = True
        42
      }
      v13, _ -> 100 - 42
    }
  } - {
    case 5 {
      3 -> 100
      _ -> fn(v14, v15) { 7 }("res", "constructor")
      a -> a
    }
  }
}
