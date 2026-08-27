pub const golden_value: Int = 2
pub const tag_value: Bool = True

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: #(Bool, Bool), v0: List(Int), v1: Float) -> String {
fn(v2, v3) { "constructor" }(0.0, 42)
}

fn export(v4: Float, length: Int) -> List(Int) {
fn(v5) { {
    let v4 = v5
    let x = {
      let length = length
      2
    }
    {
      let self_ = length
      []
    }
  } }("constructor")
}

fn constructor(v6: String, rest: String, delete: List(Int)) -> Int {
case {
      0.0
    } *. {
      0.0
    } {
    10.0 | 3.14 -> 7
    1.5 -> fn(v7, v8) { delete |> walk(1 - 5) }("a", False)
    b -> 7
  }
}

pub fn main() {
  echo ""
  echo f0(case "data" {
    _ -> #(True, True)
    _ -> fn(v9) { #(False, True) }(True)
    "a" -> fn(v10) { #(False, True) }(True)
  }, fn(v11, v12) { export(2.0, 42) }(False, "res"), fn(v13, v14) { {
    1.5
  } /. {
    10.0
  } }(2, "constructor"))
}
