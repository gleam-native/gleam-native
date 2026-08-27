pub const limit_value: Int = 42
pub const golden_value: String = "data"

pub type Object {
  Cv0(value: String, inner: Int)
  Error(value: List(Int))
  Cv1(String, Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v2: Object, l: Int, v3: String) -> Float {
1.0
}

pub fn main() {
  let self_ = []
  let limit_value = 42
  echo 4
  echo {
    {
      let y = []
      let self_ = self_
      golden_value
    }
  } <> golden_value
}
