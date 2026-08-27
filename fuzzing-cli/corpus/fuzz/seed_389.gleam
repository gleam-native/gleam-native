pub const limit_value: Bool = False

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(class: String) -> List(Int) {
case 2.0 {
    0.25 | 1.0 -> [3]
    item -> []
  }
}

pub fn main() {
  echo fn(v0, v1) { case "res" {
    _ -> limit_value && limit_value
    item | "x" <> item -> limit_value
  } }(False, True)
}
