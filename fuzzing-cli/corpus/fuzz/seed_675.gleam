pub const pi_value: String = "x"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(m: Int) -> List(Int) {
case 42, <<"x":utf8>> {
    rest, <<"x":utf8>> if rest == 0 || rest <= 2 -> []
    8, _ -> {
      let m = [4, 1] |> walk(m)
      let m = {
        let m = True
        [2]
      }
      [100]
    }
    _, _ -> [42, 2]
  }
}

pub fn main() {
  let s = {
    {
      let rest = 10.0
      rest
    }
  } *. {
    {
      2.0
    } +. {
      2.0
    }
  }
  echo False
  echo {
    fn(v0, v1) { pi_value }(True, 100.0)
  } <> "constructor"
  echo {
    case 0.25, 42 {
      0.5, 6 -> {
        let z = "bc"
        "ab"
      }
      new, _ -> pi_value <> "data"
      0.25, _ -> "res" <> pi_value
    }
  } <> {
    case [42, 4] {
      [_, 7, ..] -> "bc"
      [] -> pi_value <> "b"
      [] -> pi_value <> pi_value
      _ -> "abc"
    }
  }
  echo [1, 0]
}
