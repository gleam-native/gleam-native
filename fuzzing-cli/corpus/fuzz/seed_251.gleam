pub const limit_value: Int = 100
pub const euler_value: Bool = False
pub const pi_value: String = "x"

pub type V0 {
  Ok(value: String, inner: String)
  Cv1(String)
  Some(value: Int, inner: Float)
}

fn f0(length: Bool, new: V0) -> List(Int) {
[]
}

pub fn main() {
  echo {
    case pi_value, fn(v2, v3) { pi_value }(False, True) {
      _, _ -> 10
      v4, "bc" <> _ as whole -> fn(v5) { limit_value }(1.0)
      "res" <> rest, limit_value -> 2 * 4
    }
  } > 4
  echo f0(case {
      let this_ = 4
      "a"
    } {
    constructor -> euler_value
    "constructor" -> True
  }, case {
      let value = []
      10.0
    }, pi_value {
    constructor, "data" <> rest as whole if constructor >. 0.25 -> Cv1("bc")
    3.14, "constructor" -> Ok("", "res")
    _, _ -> Cv1("constructor")
  })
  echo case pi_value <> pi_value {
    "bc" <> _ -> case fn(v6, v7) { euler_value }("a", False) {
      _ -> "x"
      limit_value -> pi_value <> pi_value
      item -> pi_value
    }
    "ab" <> _ | "res" <> _ -> {
      pi_value <> pi_value
    } <> pi_value
    constructor -> constructor <> "bc"
  }
  echo f0(fn(v8) { False }(3), Cv1("res"))
}
