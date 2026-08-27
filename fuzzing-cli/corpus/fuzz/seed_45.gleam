pub const golden_value: Float = 10.0

pub type V0 {
  None(value: String, inner: Bool)
  Cv1(String)
  Cv2(Int, value: String)
}

pub type V3 {
  Cv4(List(Int))
  Cv5(Int)
}

pub type V6 {
  Some
  Cv7
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn new(acc: Float) -> String {
case Cv7 {
    _ | Some -> "data"
    Cv7 -> {
      let acc = []
      let acc = fn(v8) { acc }(10)
      "res" <> ""
    }
  }
}

fn extends(v9: Int, v10: Float, self_: Int) -> Float {
case fn(v11) { self_ }("ab") {
    _ | 4 -> case self_ {
      0 -> 3.14
      _ -> v10 -. v10
      constructor -> 0.1
    }
    0 | 1 -> 3.14
  }
}

fn arguments(v12: List(Int), v13: V0, default: Int) -> String {
"data" <> "a"
}

pub fn main() {
  let golden_value = True || {
    False && True
  }
  echo extends(1, 0.0, spin(7, 0) - 10)
}
