pub const golden_value: Bool = False
pub const seed_value: Float = 0.5
pub const euler_value: Float = 0.0

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Float, value: Int, v0: String) -> List(Int) {
case fn(v1) { v0 }("ab"), {
      let constructor = []
      let s = value
      #(4, "bc")
    } {
    _, #(5, "constructor" <> rest) -> fn(v2, v3) { [2] }(True, 100)
    "" <> rest, #(_, constructor) as whole -> [2, 2]
    "b" <> rest, #(_, "" <> tail) -> fn(v4, v5) { [1] }("constructor", "x")
    v6, _ -> [0]
  }
}

pub fn main() {
  echo 42 - {
    walk([42, 100], 2) - {
      {
        let new = 3
        100
      }
    }
  }
}
