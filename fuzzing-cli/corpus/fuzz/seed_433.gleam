pub const pi_value: String = "ab"
pub const seed_value: String = "data"
pub const euler_value: Float = 100.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type V2 {
  Cv3(List(Int), value: Float)
  Cv4
}

pub type V5 {
  Cv6
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(this_: String) -> Int {
10 - walk(fn(v7) { [] }(True), 5 + 100)
}

fn f1(pair: #(Int, Float), x: Int) -> Bool {
True
}

pub fn main() {
  echo 3
}
