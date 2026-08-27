pub const golden_value: String = "data"

pub type V0 {
  Cv1(value: List(Int))
  Cv2(String)
  Cv3
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v: Int, n: #(Int, String)) -> List(Int) {
[3]
}

pub fn main() {
  echo golden_value
  echo False
  echo case Cv2("x") {
    Cv2("a") | Cv3 -> spin(5 * 4, 1 + 42)
    constructor -> case 4 % 7 {
      item -> 0 - 0
      b -> spin(b, 42)
      b -> b + 1
    }
    b -> case 1 {
      1 | 6 -> 2 + 42
      b -> b
    }
  }
}
