pub const pi_value: String = "data"

pub type Symbol {
  Record
}

pub type V0 {
  None(value: Bool)
}

pub type V1 {
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(n: Int, z: List(Int), default: Float) -> String {
""
}

fn f1(x: Int) -> String {
"abc"
}

pub fn main() {
  let v = case "a" {
    "res" -> "bc"
    inner -> inner
  }
  let default = True
  echo v
}
