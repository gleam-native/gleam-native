pub const golden_value: Int = 1

pub type Number {
  Cv0(value: String, inner: Float)
  Error(Int, value: String)
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: #(Float, String)) -> Int {
walk({
    let l = fn(v3, v4) { 4 }(False, "b")
    let delete = 100.0
    [2]
  }, 7 * 5)
}

pub fn main() {
  echo {
    0.0
  } +. {
    3.14
  }
}
