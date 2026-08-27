pub const tag_value: Bool = True

pub type Number {
  Record
  Cv0(Float, Float)
  Cv1(value: Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn extends(z: Int, delete: Int, value: Int) -> Int {
z
}

pub fn main() {
  echo [5]
  echo [0, 10]
}
