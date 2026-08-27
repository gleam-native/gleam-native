pub const golden_value: String = ""
pub const tag_value: Bool = True

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Float, v0: Int, v1: String) -> List(Int) {
[]
}

pub fn main() {
  let delete = case golden_value, [] {
    "constructor", [8] as whole -> 5 + 1
    _, [constructor, ..rest] -> {
      let m = rest
      let new = m
      constructor
    }
    _, v2 -> 7 % 2
  }
  let prototype = fn(v3) { delete }(True)
  echo [3]
}
