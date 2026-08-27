pub const euler_value: Float = 3.14
pub const seed_value: Float = 0.0
pub const golden_value: Int = 10

pub type Map {
  Cv0(value: String, inner: Int)
  Cv1
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v2: String, v3: Bool, v4: Int) -> Float {
2.0
}

pub fn main() {
  let golden_value = spin(4, 10 - golden_value)
  let seed_value = True
  echo euler_value
  echo "data"
}
