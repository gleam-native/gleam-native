pub const tag_value: Bool = False

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(acc: Int) -> String {
"bc"
}

fn default(y: List(Int)) -> Bool {
case False && True {
    _ -> False
    True | True -> !True
    False -> case fn(v0, v1) { v1 }(False, 0) {
      _ | 2 -> True
      _ | 4 -> fn(v2) { v2 }(False)
    }
  }
}

fn f2(v3: Int, item: Int, default: List(Int)) -> String {
{
    constructor(1) <> {
      {
        let default = 3.14
        let prototype = 2.0
        "abc"
      }
    }
  } <> "a"
}

pub fn main() {
  let tag_value = tag_value
  echo case [100, 3] {
    [b] -> "x"
    [3] -> "constructor"
    [x, ..rest] -> {
      x - 0
    } |> constructor()
    v4 -> spin(4, 7) |> constructor()
  }
}
