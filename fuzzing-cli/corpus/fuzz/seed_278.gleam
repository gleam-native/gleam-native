pub const limit_value: Int = 42
pub const pi_value: Bool = True
pub const golden_value: Bool = True

pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn delete(length: Int) -> Int {
0 - {
    {
      10 + length
    } - 0
  }
}

pub fn main() {
  let limit_value = [0]
  echo case <<"b":utf8, "a":utf8>> {
    <<_:16>> as whole -> True
    _ -> {
      {
        1.0
      } +. {
        0.0
      }
    } >. {
      0.1
    }
  }
  echo case {
      let item = 7
      "a"
    } {
    "a" <> a -> "a"
    "constructor" -> "x"
    v2 -> case 42, 5 > 5 {
      3, _ -> {
        let limit_value = golden_value
        "data"
      }
      5, _ -> "constructor" <> "b"
      6, True -> ""
      _, v3 -> v2 <> v2
    }
  }
}
