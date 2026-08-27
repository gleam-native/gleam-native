pub const seed_value: Int = 2

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(value: Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v3: Int) -> Int {
case "res" <> "bc", fn(v4) { "a" }(2.0) {
    "ab", "a" <> rest -> 3
    "x", "bc" <> rest -> v3
    v5, _ -> {
      {
        let acc = v3
        let v5 = 1
        acc
      }
    } - v3
  }
}

fn f1(v6: Bool, length: Float) -> Int {
spin(f0(42) * 3, fn(v7, v8) { 1 }(0, ""))
}

pub fn main() {
  let new = 100.0
  echo fn(v9, v10) { case Cv1([0], 2) {
    Cv1(_, item) -> seed_value
    a -> {
      let s = seed_value
      seed_value
    }
  } }("", "constructor")
}
