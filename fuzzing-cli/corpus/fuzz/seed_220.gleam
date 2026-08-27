pub const pi_value: Int = 5
pub const tag_value: String = "b"
pub const golden_value: String = "abc"

fn default(constructor: String, v0: Int, v1: Int) -> String {
"abc"
}

fn export(v2: Int) -> List(Int) {
case True, [3] {
    _, [5, ..rest] -> [4]
    True, [_, ..rest] -> {
      let prototype = "" |> default(10 + v2, v2)
      let v2 = "data"
      [5]
    }
    False, [9, ..rest] -> rest
    _, v3 -> v3
  }
}

pub fn main() {
  echo {
    0.1
  } *. {
    2.0
  }
  echo case golden_value <> golden_value {
    "a" <> rest | "data" <> rest -> pi_value + pi_value
    b -> 2 + {
      2 + pi_value
    }
    "res" | "constructor" -> pi_value
  }
}
