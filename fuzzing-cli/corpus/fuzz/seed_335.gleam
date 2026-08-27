pub const limit_value: Float = 0.25
pub const tag_value: String = "a"

pub type V0 {
  Cv1(value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(z: String, v2: Bool) -> Bool {
True
}

fn yield(item: Float, v3: Int, acc: Float) -> Int {
spin({
    v3 |> spin(spin(v3, 4))
  } - 0, v3)
}

fn f2(default: Int, delete: String, pair: Int) -> String {
"b"
}

pub fn main() {
  let s = 0
  echo case "ab" <> tag_value, tag_value <> tag_value {
    "res", z if z != "constructor" -> fn(v4, v5) { z <> "constructor" }(5, 10)
    "bc", "abc" -> case {
        3.14
      } |> yield(s % 5, limit_value /. {
        0.5
      }), #(True, [0]) {
      v6, #(self_, [5]) if v6 <= 2 -> s |> f2(tag_value <> tag_value, s)
      _, #(_, [0]) -> {
        let s = []
        tag_value
      }
      _, v7 -> "b" <> tag_value
    }
    v8, v9 -> v9
  }
  echo 1
  echo 5
}
