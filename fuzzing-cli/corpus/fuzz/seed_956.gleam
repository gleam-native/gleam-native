pub const golden_value: Int = 7
pub const pi_value: Float = 10.0
pub const limit_value: Bool = True

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Float, v0: #(String, String), value: List(Int)) -> Int {
3 |> spin(1 - 3)
}

pub fn main() {
  echo pi_value
  echo 0 - 42
  echo 0
  echo case golden_value {
    a -> {
      fn(v1) { "" }(3)
    } <> {
      "x" <> "abc"
    }
    inner -> case pi_value, [100, 7] {
      0.5 as whole, [8, 2, ..] -> "abc" <> "b"
      constructor, [6, ..rest] -> "ab" <> "x"
      _, v2 -> "ab" <> "data"
    }
    inner -> "constructor"
  }
}
