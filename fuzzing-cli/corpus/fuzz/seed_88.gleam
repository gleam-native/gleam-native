pub const pi_value: Int = 1
pub const euler_value: Bool = False

pub type Number {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn class(v0: Int, constructor: Int) -> Bool {
True
}

pub fn main() {
  let prototype = case fn(v1, v2) { Record }(1.5, 7), fn(v3, v4) { v4 }(0.0, 2.0) {
    Record as whole, 0.0 -> 100.0
    Record, 0.1 -> {
      0.5
    } *. {
      0.1
    }
    v5, _ -> {
      1.5
    } *. {
      10.0
    }
  }
  echo spin(pi_value, 42)
  echo case <<"":utf8, "ab":utf8, 1:8>> {
    <<_:utf8, _:utf8>> as whole -> {
      "bc" <> "abc"
    } == {
      {
        let self_ = "abc"
        let self_ = euler_value
        "ab"
      }
    }
    <<"":utf8, _:big-signed-16, 0:8>> -> euler_value
    v6 -> fn(v7) { !False }("bc")
  }
  echo "abc" == {
    "data" <> "ab"
  }
  echo euler_value
}
