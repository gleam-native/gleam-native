fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(this_: Bool) -> List(Int) {
case {
      0.1
    } /. {
      10.0
    } {
    2.0 -> [7]
    _ | 0.0 -> fn(v0, v1) { {
      let self_ = True
      [7]
    } }(1.0, 100.0)
  }
}

fn yield(v2: Float, m: Float) -> String {
{
    {
      fn(v3, v4) { "constructor" }("a", True)
    } <> {
      "ab" <> "bc"
    }
  } <> {
    fn(v5) { "ab" <> "ab" }(5)
  }
}

fn f2(v6: Bool, acc: #(List(Int), List(Int))) -> String {
"abc"
}

pub fn main() {
  let m = 2.0
  let m = case {
      let m = 0.5
      let constructor = [5]
      1
    }, True |> constructor() {
    3, [x, _, ..] -> {
      let m = "bc"
      let arguments = 100.0
      False
    }
    7 as whole, [] -> True && True
    v7, _ -> {
      let v7 = m
      True
    }
  }
  echo case 1, "a" <> "data" {
    5, "b" -> "b" <> "res"
    0, _ -> "constructor"
    v8, _ -> ""
  }
  echo case True {
    a -> 0.25
    _ -> {
      let item = yield(2.0, 0.25)
      let rest = 42 <= 10
      0.0
    }
  }
  echo {
    case "bc" <> "data", "x" {
      "x", "res" <> rest if rest != "b" -> {
        0.1
      } /. {
        10.0
      }
      "ab", "res" <> _ -> {
        1.5
      } +. {
        10.0
      }
      _, "data" -> {
        let l = "a"
        let z = m
        1.0
      }
      v9, _ -> {
        let v = 5
        let m = m
        1.5
      }
    }
  } +. {
    case <<100:4>> {
      <<_:16, 1:16, _:little-signed-8>> -> {
        1.0
      } +. {
        1.0
      }
      <<_:16, _:utf8>> -> 0.25
      <<1:4>> -> {
        10.0
      } /. {
        0.5
      }
      v10 -> 0.0
    }
  }
  echo {
    0.1
  } +. {
    1.0
  }
}
