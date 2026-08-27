pub type V0 {
  Error(value: String, inner: Int)
  Cv1
}

pub type V2 {
  Cv3(String)
  Cv4
}

fn class(v: String) -> Bool {
{
    case {
        let v = True
        let pair = True
        "res"
      }, 1 * 1 {
      "abc" <> rest, 1 as whole if whole > 6 -> v == v
      "ab" <> rest, _ -> fn(v5, v6) { True }(7, 1.5)
      _, v7 -> fn(v8, v9) { False }(2, 2.0)
    }
  } && {
    !{
      fn(v10, v11) { v11 }(1.5, True)
    }
  }
}

fn extends(v12: Bool) -> Int {
{
    {
      10 - 0
    } * {
      2 * 4
    }
  } * {
    100 - {
      5 - 42
    }
  }
}

pub fn main() {
  let new = case fn(v13) { [] }(1.0) {
    [8, ..rest] as whole -> {
      let rest = 5
      let value = whole
      value
    }
    [a, x, ..] as whole -> [42, 3]
    _ -> fn(v14, v15) { [10] }(0.5, "constructor")
  }
  echo {
    case {
        let n = 0.1
        "ab"
      } {
      _ -> {
        0.1
      } +. {
        1.0
      }
      new -> 1.5
    }
  } *. {
    case {
        let acc = 1.0
        let acc = False
        "b"
      } {
      "res" -> 0.1
      "abc" | "bc" -> 10.0
      _ -> {
        let s = True
        1.0
      }
    }
  }
  echo [5]
  echo new
  echo True
}
