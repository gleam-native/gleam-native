pub const seed_value: Float = 10.0
pub const golden_value: Int = 10

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Number(Float)
  Cv2(value: String)
}

pub type V3 {
  Cv4(Float)
  Cv5(value: Int)
}

pub type V6 {
  Cv7
  Cv8(value: Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(default: V0, x: Bool) -> Int {
spin({
    4 * 1
  } + 3, spin(5, 0) + 4)
}

fn delete(v9: Float, constructor: Bool, v10: V3) -> Float {
0.5
}

pub fn main() {
  let prototype = [100]
  echo "a"
  echo golden_value
  echo prototype
}
