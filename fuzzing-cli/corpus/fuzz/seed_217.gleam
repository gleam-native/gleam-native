pub const euler_value: Bool = True
pub const seed_value: Float = 100.0
pub const limit_value: Int = 4

pub type Number {
  Cv0(value: String, inner: Int)
  Cv1
  Some(Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(acc: List(Int), v2: Int, v3: Bool) -> List(Int) {
case v2 {
    item -> acc
    v4 -> fn(v5) { [7, 0] }("res")
  }
}

fn default(v6: List(Int), v7: Int) -> Int {
4
}

pub fn main() {
  echo []
  echo 0.25
}
