pub const pi_value: Bool = True

pub type V0 {
  Some(value: String, inner: List(Int))
  Cv1(Int, List(Int))
  Cv2
}

pub type V3 {
  Cv4
  Cv5(Float)
  Cv6
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn extends(rest: Float) -> Int {
1
}

pub fn main() {
  echo []
  echo []
}
