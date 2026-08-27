pub const golden_value: String = "abc"
pub const limit_value: Float = 1.5
pub const pi_value: Int = 10

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type V2 {
  Cv3
  Cv4(value: Bool)
  Cv5
}

fn export(v6: Bool) -> String {
case {
      1.5
    } -. {
      100.0
    } {
    pair -> {
      "ab" <> "x"
    } <> {
      "bc" <> "a"
    }
    a -> "" <> "b"
    10.0 -> fn(v7, v8) { "ab" }(10.0, "b")
  }
}

pub fn main() {
  let item = {
    fn(v9, v10) { golden_value }(0.0, True)
  } <> {
    fn(v11) { "a" }(0.1)
  }
  let value = 0
  echo {
    {
      pi_value % 6
    } + value
  } % 5
  echo 2.0
  echo case limit_value >=. {
      3.14
    } {
    a -> pi_value
    True | True -> pi_value
    item -> pi_value
  }
}
