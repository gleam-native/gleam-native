pub const tag_value: Bool = False

pub type V0 {
  Number(value: String, inner: String)
  Error(value: String, inner: Int)
}

pub type V1 {
  Cv2
}

pub type V3 {
  Cv4(Bool, value: Int)
  Ok(Float)
  Cv5(List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(self_: Bool, v6: Bool, v7: #(String, Int)) -> String {
case {
      0.25
    } != {
      0.0
    } {
    _ | True -> {
      let pair = fn(v8, v9) { v9 }(True, 1.0)
      "a" <> "bc"
    }
    False | True -> fn(v10, v11) { v10 <> "res" }("res", 100)
  }
}

pub fn main() {
  echo 0.5
}
