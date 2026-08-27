pub const euler_value: Bool = False
pub const tag_value: Bool = True

pub type Number {
  Record
  Cv0(value: String, inner: String)
}

pub type Record {
  Cv1(value: Float, inner: Bool)
  Cv2(value: Float, inner: Bool)
  Cv3(value: Float, inner: Float)
}

pub type Map {
  Cv4
  Error(String)
  Cv5(String, value: Float)
}

fn f0(class: List(Int), l: Int, m: Int) -> Bool {
10 > l
}

pub fn main() {
  echo []
  echo 0 - 7
  echo case {
      let euler_value = 42
      let s = [0, 4]
      "ab"
    } {
    "ab" -> 2
    "abc" <> inner if inner == "data" || inner == "bc" -> {
      2 + 100
    } % 4
    b -> 2 - 42
  }
}
