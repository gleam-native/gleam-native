pub const pi_value: Bool = False
pub const euler_value: Bool = True
pub const golden_value: String = "abc"

pub type V0 {
  Error(value: String, inner: List(Int))
  Cv1
}

fn f0(v2: Bool, v3: Int) -> Bool {
False
}

fn f1(this_: Int) -> Int {
0
}

fn constructor(v4: String) -> Int {
7
}

pub fn main() {
  let y = case False || euler_value, Error("bc", [2, 5]) {
    True as whole, Cv1 -> whole || True
    False, _ -> True
    _, Cv1 -> False
    v5, v6 -> !v5
  }
  echo case <<"x":utf8>>, "a" {
    <<4:16>>, _ -> case golden_value {
      "bc" <> rest -> "a"
      a -> {
        let golden_value = False
        let length = 4
        a
      }
    }
    <<"x":utf8>>, "res" <> rest if rest == "a" || rest == "bc" -> "x"
    <<_:utf8, arguments:8>>, constructor -> case Error("x", []) {
      constructor -> golden_value
      _ | Cv1 -> fn(v7) { "x" }(False)
    }
    _, v8 -> case y |> f0(3 * 0) {
      True -> ""
      constructor -> golden_value
      _ | False -> "constructor"
    }
  }
  echo False
}
