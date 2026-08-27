pub const golden_value: Float = 0.25

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, v0: Float, arguments: Bool) -> Float {
case 10 - 100, fn(v1, v2) { "a" }(5, "res") {
    _, "b" as whole if whole != "abc" -> 2.0
    constructor, "ab" -> case "res" {
      constructor -> {
        0.0
      } -. {
        1.5
      }
      "x" <> item -> {
        3.14
      } /. {
        3.14
      }
    }
    v3, v4 -> case #([], [100]) {
      #([h, _, ..] as whole, [_]) -> 1.5
      a -> {
        0.1
      } -. {
        0.1
      }
      #([], [0] as whole) -> v0 -. v0
    }
  }
}

pub fn main() {
  let new = {
    3 |> spin(4 + 10)
  } % 4
  echo case "data" <> "bc", [0] {
    "a" <> _, [] -> [0]
    "a", [_] -> [0, 1]
    "bc" <> rest, [5] -> [1]
    _, _ -> [2]
  }
  echo case golden_value >=. golden_value, 5 {
    new, y -> new
    True, default -> True
  }
}
