pub const limit_value: Int = 1
pub const pi_value: Int = 2

pub type Symbol {
  Cv0(value: String, inner: List(Int))
  Cv1(List(Int))
  None
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(s: String, v2: #(Float, Bool), length: Bool) -> Int {
0 - {
    {
      let self_ = fn(v3) { length }(1)
      let v = 5 + 42
      v
    }
  }
}

pub fn main() {
  echo "x"
  echo [7, 2]
  echo pi_value
  echo [0, 0]
}
