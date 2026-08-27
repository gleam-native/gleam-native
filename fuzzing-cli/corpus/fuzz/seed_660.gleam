pub const pi_value: Bool = True

pub type V0 {
  Number(value: String, inner: String)
  Record(Bool, value: Int)
  Cv1(value: Bool, inner: Float)
}

pub type V2 {
  Error(Float, value: Bool)
  None
  Cv3(String, Float)
}

fn f0(arguments: Int) -> String {
case Cv1(True, 3.14) {
    Record(_, b) -> case {
        let b = "x"
        7
      } {
      item -> "constructor"
      _ -> fn(v4) { "b" }(False)
    }
    Record(True, 2) -> fn(v5, v6) { "ab" <> "constructor" }(0.25, 2.0)
    _ -> "bc"
  }
}

pub fn main() {
  echo 0.0
  echo "a"
  echo 2
  echo 7
}
