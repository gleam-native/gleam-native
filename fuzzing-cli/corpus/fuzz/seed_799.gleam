fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(s: Bool) -> List(Int) {
case [100] {
    [_] -> case fn(v0) { 2 }(True) {
      3 | 0 -> [1]
      b -> []
    }
    [7, 5, ..] -> []
    _ -> []
  }
}

pub fn main() {
  let m = case 10.0, {
      let length = []
      let m = 1.0
      #(True, [5, 1])
    } {
    0.0, #(_, [h, ..rest]) -> "b"
    0.5, #(_, [4, ..rest]) -> {
      let rest = 0.5
      "b"
    }
    _, #(True, [x, ..rest]) as whole -> "abc"
    v1, v2 -> "x" <> "x"
  }
  let constructor = {
    3.14
  } /. {
    2.0
  }
  echo 0.25
  echo {
    case #("abc", True) {
      inner -> m <> m
      #("constructor", _) | #("res" <> _, True) -> m
    }
  } <> {
    case 10 {
      item -> m
      5 as whole -> m
    }
  }
  echo [3]
  echo 42
}
