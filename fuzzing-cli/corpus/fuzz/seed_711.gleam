pub const tag_value: String = ""
pub const euler_value: Float = 0.1

pub type V0 {
  Ok(value: String, inner: String)
  Cv1
}

pub type V2 {
  Record(value: Float, inner: String)
  Error
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(x: Float, rest: Bool, l: Bool) -> List(Int) {
case fn(v3) { "a" }(False), l {
    "ab", True as whole if whole -> []
    _, False -> [4]
    "constructor", True as whole -> case 100 {
      item -> []
      8 as whole -> []
    }
    _, v4 -> [7]
  }
}

pub fn main() {
  echo euler_value
}
