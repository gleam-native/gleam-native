pub const tag_value: Int = 0
pub const limit_value: Int = 1

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn yield(prototype: #(Int, String), v3: Int) -> Int {
case {
      let pair = v3
      let prototype = []
      4
    } {
    a -> case "b" <> "ab" {
      "b" <> item -> a
      "" <> item -> spin(0, 10)
      _ -> 42
    }
    inner -> 0
  }
}

fn f1(constructor: Int) -> List(Int) {
[]
}

pub fn main() {
  echo case True || True {
    _ -> [0, 3]
    False | True -> f1(tag_value)
  }
  echo case 100.0 {
    tag_value -> [0, 2]
    0.25 | 1.0 -> []
  }
  echo ""
}
