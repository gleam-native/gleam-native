pub const tag_value: String = "abc"
pub const golden_value: Int = 2
pub const limit_value: String = "a"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(value: #(List(Int), List(Int))) -> Float {
case fn(v0, v1) { "res" }(0.25, False) {
    constructor -> case [4], "data" <> constructor {
      [1, 6, ..], constructor if constructor != "bc" -> {
        100.0
      } +. {
        0.25
      }
      [constructor, x, ..], "abc" <> rest if x > 2 -> {
        0.1
      } -. {
        1.0
      }
      [x, ..rest], "b" -> {
        2.0
      } *. {
        0.25
      }
      v2, v3 -> 1.5
    }
    a | "constructor" <> a -> 3.14
  }
}

fn f1(v4: Int) -> Float {
{
    case fn(v5, v6) { v4 }(True, True) {
      1 -> 1.5
      a -> 10.0
    }
  } -. {
    10.0
  }
}

fn delete(v7: String, s: List(Int), v8: String) -> String {
"data"
}

pub fn main() {
  echo {
    case tag_value <> tag_value {
      "res" -> 0.25
      "x" -> fn(v9) { 100.0 }("abc")
      _ -> 100.0
    }
  } *. {
    walk([100], golden_value) |> f1()
  }
  echo [0]
  echo {
    {
      let acc = fn(v10, v11) { [7] }(3.14, False)
      0.0
    }
  } -. {
    1.0
  }
  echo [4, 1]
}
