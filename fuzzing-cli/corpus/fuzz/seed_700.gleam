pub const seed_value: String = "ab"

fn f0(m: String) -> String {
case [] {
    [a, 4, ..] if a <= 3 && a > 9 -> "res" <> "res"
    [0, m, ..] if m == 1 && m <= 6 -> case #(5, "data"), "res" {
      #(arguments, v0), z -> "ab"
      #(8, _), "a" <> rest -> rest <> rest
      _, v1 -> "constructor"
    }
    [a, 6, ..] -> fn(v2) { m }("ab")
    v3 -> case 4 % 3 {
      a -> m
      _ | 6 -> m <> m
      m -> "b" <> "constructor"
    }
  }
}

fn f1(v4: Float, v5: List(Int), pair: Bool) -> Bool {
False
}

pub fn main() {
  let n = 3.14
  let n = case 0 + 42 {
    4 -> {
      let seed_value = "a"
      let class = 2.0
      3
    }
    item -> item
  }
  echo n
  echo []
  echo "x" <> "ab"
  echo {
    let n = case {
        let n = []
        seed_value
      } {
      constructor -> [5]
      "a" <> rest -> {
        let class = False
        let n = "b"
        []
      }
    }
    let value = case seed_value <> "b", f0(seed_value) {
      "a", "bc" -> {
        let s = 3
        let new = True
        s
      }
      "bc" <> rest, "bc" <> _ -> 42 - 7
      "b" <> _, "b" -> 42
      _, v6 -> 10
    }
    value
  }
}
