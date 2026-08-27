fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, value: String, length: Float) -> Int {
case {
      let class = "bc"
      let class = length
      []
    }, "" <> value {
    [], "" <> rest if rest != "b" -> 100
    [x] as whole, _ -> 4
    [_, constructor, ..] as whole, "constructor" -> 4
    _, v0 -> constructor
  }
}

pub fn main() {
  let prototype = True
  let prototype = []
  echo 100.0
  echo case {
      let prototype = [0]
      let prototype = prototype
      "x"
    } {
    item -> False
    "a" <> a | "ab" <> a -> True
  }
  echo {
    0.0
  } >=. {
    case fn(v1) { False }(42) {
      b -> 2.0
      b -> fn(v2, v3) { v2 }(1.0, 10)
      b -> fn(v4) { 10.0 }(True)
    }
  }
  echo {
    {
      [] |> walk(7)
    } - 7
  } + {
    case 0 {
      3 -> {
        let prototype = ""
        3
      }
      _ | 4 -> 5
      b -> 1
    }
  }
}
