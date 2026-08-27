pub const golden_value: String = "bc"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Float, v0: List(Int), this_: Bool) -> List(Int) {
case {
      10.0
    } -. constructor {
    a -> case walk(v0, 10), fn(v1) { 7 }(1.5) {
      _, _ -> fn(v2, v3) { [] }(3, True)
      7, 2 -> fn(v4) { v0 }("bc")
    }
    0.25 | 10.0 -> fn(v5, v6) { [7, 10] }(0.0, "abc")
    1.0 -> []
  }
}

pub fn main() {
  let l = 1
  let this_ = case {
      let acc = 10.0
      let acc = acc
      acc
    } {
    inner -> fn(v7, v8) { 2.0 }(100.0, "b")
    _ -> {
      0.0
    } /. {
      0.5
    }
    v9 -> 1.0
  }
  echo case l, fn(v10) { #("data", [3]) }(False) {
    7 as whole, #(_, [this_, ..rest] as it) if this_ > 1 -> case fn(v11, v12) { whole }(42, "b") {
      inner -> [4]
      0 -> fn(v13, v14) { it }(0.25, 0.1)
      _ -> rest
    }
    _, #(_, [a, 9, ..]) -> [10, 4]
    v15, v16 -> {
      let l = {
        let class = 2
        golden_value
      }
      [7]
    }
  }
  echo fn(v17) { case 0, l - l {
    9, constructor if constructor > 8 -> 1.5
    3, 0 -> this_
    7, v17 -> this_ -. this_
    _, v18 -> fn(v19) { this_ }(3.14)
  } }(False)
  echo []
}
