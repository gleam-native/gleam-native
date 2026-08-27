pub const pi_value: Int = 7
pub const seed_value: Int = 2

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(s: Bool) -> String {
case spin(4, 2), {
      let self_ = 4
      [42]
    } {
    v0, [0, b, ..] -> "ab" <> {
      "b" <> "ab"
    }
    6, [6] -> "b"
    _, [s] -> case [] {
      [x] as whole if x <= 5 || x % 2 == 0 -> "bc"
      [1, ..rest] -> "constructor" <> "constructor"
      _ -> "ab" <> "constructor"
    }
    v1, v2 -> {
      "x" <> ""
    } <> {
      fn(v3, v4) { v4 }(0.1, "bc")
    }
  }
}

fn arguments(item: Int, v5: Bool, this_: Float) -> String {
"ab"
}

fn f2(v6: Float) -> List(Int) {
fn(v7, v8) { [] }("b", 1)
}

pub fn main() {
  echo 0.5
}
