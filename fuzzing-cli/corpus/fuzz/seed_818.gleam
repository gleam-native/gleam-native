pub const limit_value: String = ""

pub type Record {
  Record
  Cv0(List(Int), value: Float)
  Ok(value: String, inner: Float)
}

fn default(v1: Int, v2: String) -> Bool {
True
}

pub fn main() {
  echo case {
      3.14
    } == {
      10.0
    }, 3 {
    False, 2 -> {
      fn(v3, v4) { 7 }(True, True)
    } < {
      42 - 5
    }
    item, 3 -> case limit_value <> limit_value {
      b | "x" <> b -> item
      item | "bc" <> item -> False
    }
    _, _ -> default(4, "x")
  }
  echo 100.0
}
