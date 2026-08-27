pub const golden_value: String = "b"
pub const pi_value: Float = 1.5
pub const euler_value: Int = 1

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, v0: List(Int), v1: Int) -> Float {
{
    case "a" <> "abc" {
      inner -> {
        let inner = inner
        let v1 = v0
        3.14
      }
      "data" <> rest | "abc" <> rest -> {
        0.25
      } +. {
        1.0
      }
    }
  } -. {
    {
      let n = "b"
      let constructor = constructor + 4
      {
        3.14
      } +. {
        3.14
      }
    }
  }
}

pub fn main() {
  echo case "abc", "x" {
    "constructor", "a" -> case [42, 1], <<2:4>> {
      [], <<_:big-signed-4, "x":utf8>> as whole -> "abc"
      [x, _, ..], <<5:16>> as whole -> "ab" <> "data"
      _, v2 -> {
        let x = []
        golden_value
      }
    }
    "abc" <> rest, v3 -> "ab"
    v4, v5 -> {
      {
        let class = "a"
        v4
      }
    } <> "constructor"
  }
  echo {
    {
      {
        let prototype = "bc"
        pi_value
      }
    } *. {
      {
        0.0
      } +. {
        0.0
      }
    }
  } +. {
    case 3 % 1 {
      item -> 2.0
      6 | 3 -> {
        let golden_value = True
        2.0
      }
      v6 -> {
        let class = True
        pi_value
      }
    }
  }
  echo case golden_value <> "a", fn(v7) { "" }("b") {
    y, "abc" -> case "a" <> golden_value {
      a | "abc" <> a -> [0, 7]
      "constructor" <> item -> [0]
    }
    "data", v8 -> []
    _, v9 -> case <<"":utf8, "constructor":utf8, "":utf8>> {
      <<4:16>> -> []
      _ -> {
        let class = pi_value
        let s = 42
        [2]
      }
    }
  }
  echo {
    {
      euler_value + euler_value
    } * 0
  } - 10
}
