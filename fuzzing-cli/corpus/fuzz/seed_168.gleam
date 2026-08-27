pub const golden_value: String = "b"
pub const euler_value: Bool = False
pub const limit_value: Bool = True

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v0: Int, s: Int) -> Bool {
constructor < 3
}

fn f1(v1: String) -> Int {
case True, [] {
    this_, [_] -> 100
    False, [a] -> case v1 {
      "constructor" <> item if item != "x" -> a - a
      _ -> 7
    }
    True, [a, _, ..] -> spin({
      let this_ = a
      100
    }, a)
    _, v2 -> 0 + {
      42 * 7
    }
  }
}

pub fn main() {
  let l = 42
  echo 3.14
  echo {
    0.25
  } +. {
    {
      0.0
    } +. {
      10.0
    }
  }
  echo 0.5
}
