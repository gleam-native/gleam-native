pub const seed_value: String = "bc"
pub const euler_value: Int = 4

pub type V0 {
  Error(value: String, inner: List(Int))
  Cv1(Float, Int)
  Cv2
}

pub type V3 {
  None
  Cv4(String, List(Int))
}

fn f0(v5: String, v6: String) -> Bool {
case 5 - 1, "res" {
    s, "bc" -> True || {
      !True
    }
    4, "a" <> rest -> True
    _, v7 -> {
      fn(v8) { 3.14 }("ab")
    } != {
      0.0
    }
  }
}

fn export(length: String, v9: V3) -> String {
length
}

pub fn main() {
  echo f0("constructor", case seed_value |> export(None) {
    "res" -> seed_value <> seed_value
    seed_value -> seed_value |> export(None)
    a -> "abc" <> "a"
  })
}
