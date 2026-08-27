fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Float, x: List(Int), v0: Bool) -> String {
"data"
}

fn f1(value: String, v1: String) -> List(Int) {
[]
}

fn f2(v2: #(List(Int), String)) -> Float {
10.0
}

pub fn main() {
  let new = 3.14
  echo case False, 42 {
    True, _ -> 10
    False as whole, v3 -> {
      0 - v3
    } - v3
    v4, 6 -> f1("constructor", "res") |> walk(1)
    _, _ -> {
      42 + 42
    } - {
      100 + 0
    }
  }
  echo f1("res", "constructor")
}
