pub const golden_value: String = "data"
pub const seed_value: Int = 1
pub const tag_value: Int = 100

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(List(Int), value: String)
  Cv3(Float)
}

fn f0(constructor: Bool, v4: Bool) -> List(Int) {
[]
}

fn f1(this_: Bool, constructor: Int) -> Bool {
False
}

pub fn main() {
  let y = {
    let prototype = !True
    fn(v5) { "abc" }("data")
  }
  let default = case golden_value <> golden_value, Cv3(1.5) {
    "x", Cv2([7, ..rest], "data" <> tail) -> 1
    "data", Cv3(_) -> 1
    _, v6 -> 7 * seed_value
  }
  echo {
    case y <> golden_value {
      b -> {
        2.0
      } +. {
        2.0
      }
      _ -> {
        100.0
      } +. {
        10.0
      }
      "bc" <> constructor | "res" <> constructor -> 3.14
    }
  } <. {
    0.1
  }
}
