fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(constructor: Int) -> String {
"data"
}

fn f1(prototype: List(Int)) -> List(Int) {
case 10 - 100 {
    v0 -> fn(v1) { fn(v2, v3) { [42, 7] }(0.25, True) }(4)
    item -> case prototype |> walk(4) {
      5 | 1 -> []
      class -> prototype
    }
    9 -> [1]
  }
}

pub fn main() {
  let s = case {
      let m = 42
      True
    } {
    True -> True || True
    inner -> 4 >= 100
  }
  echo case 2.0 {
    1.5 | 0.0 -> "ab"
    constructor -> "res"
    _ -> "x"
  }
  echo False
  echo 0.1
  echo [3]
}
