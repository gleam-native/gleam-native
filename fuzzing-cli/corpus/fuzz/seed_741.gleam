pub const seed_value: Float = 2.0
pub const pi_value: Float = 0.1

fn f0(m: String, item: Bool) -> Int {
case <<"ab":utf8, "a":utf8>> {
    <<7:8>> -> case <<5:8>> {
      <<0:8>> -> 10 % 7
      _ -> 1 * 0
    }
    _ -> case {
        let acc = 10
        let m = [7]
        0.5
      } {
      1.0 -> 42 - 7
      0.0 -> 4
      v0 -> 4
    }
  }
}

fn f1(v1: List(Int)) -> String {
case #("constructor", "x") {
    #("ab" <> rest, "constructor" as whole) if rest != "" || rest != "a" -> "constructor" <> "res"
    #("ab" <> rest, _) -> ""
    constructor -> "x"
  }
}

fn f2(n: String, constructor: String) -> String {
n
}

pub fn main() {
  echo [0]
  echo False
  echo case False {
    False -> case pi_value == pi_value {
      False -> {
        let this_ = ""
        [100, 2]
      }
      v2 -> [5, 1]
      v3 -> fn(v4, v5) { [2, 0] }(1.5, "b")
    }
    False -> [10]
    v6 -> case "data" {
      inner | "data" <> inner -> [42, 5]
      "ab" -> fn(v7) { [] }(3.14)
      inner -> [7, 5]
    }
  }
  echo seed_value
}
