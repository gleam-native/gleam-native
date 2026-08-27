pub type Promise {
  Cv0(value: String, inner: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v1: Bool, v2: Int) -> String {
"ab"
}

pub fn main() {
  let arguments = "res"
  echo case f0(True, 1), arguments {
    _, "bc" -> 4 + 42
    "data", "x" -> case <<1:1>>, {
        100.0
      } +. {
        1.0
      } {
      <<"b":utf8, prototype:8>>, 100.0 if prototype <= 0 -> walk([10, 1], prototype)
      <<_:8>>, 10.0 -> 7
      _, _ -> 0 - 2
    }
    v3, _ -> {
      2 % 1
    } + walk([0, 10], 0)
  }
  echo {
    {
      let value = 1 * 42
      value
    }
  } == {
    case walk([], 5) {
      2 -> fn(v4) { 42 }(False)
      9 | 7 -> 10
      v5 -> v5 - 4
    }
  }
  echo walk(case Cv0("res", False) {
    Cv0("data" <> rest as whole, False) if rest == "b" && whole == "data" -> [100, 5]
    v6 -> [5, 5]
    Cv0("ab", _) | Cv0(_, _) -> {
      let item = 2
      [4, 1]
    }
  }, 1 + 10)
}
