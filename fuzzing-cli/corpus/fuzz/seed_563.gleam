pub const pi_value: Int = 5
pub const tag_value: Int = 5

pub type Record {
  Cv0(value: String, inner: Float)
  Cv1(value: Bool)
}

pub type V2 {
  None
  Cv3(value: List(Int), inner: Float)
}

pub type V4 {
  Cv5
  Number(Float)
}

fn f0(value: V4, this_: String, prototype: V2) -> Bool {
True
}

pub fn main() {
  echo case <<"b":utf8, "data":utf8>> {
    <<7:1>> as whole -> True
    _ -> True
  }
  echo []
  echo case Cv3([4, 4], 1.5) {
    None as whole -> case 2.0 {
      a -> "bc"
      100.0 -> "bc"
    }
    _ -> fn(v6, v7) { fn(v8) { "ab" }(5) }("x", True)
    None -> "b"
  }
}
