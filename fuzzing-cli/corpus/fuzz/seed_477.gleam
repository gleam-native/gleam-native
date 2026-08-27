pub const euler_value: Bool = False

pub type V0 {
  Error(value: String, inner: String)
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(s: #(Float, Bool), v: V0) -> Int {
walk(fn(v2) { [5, 100] }(100.0), 2 - 100)
}

pub fn main() {
  echo [10, 100]
}
