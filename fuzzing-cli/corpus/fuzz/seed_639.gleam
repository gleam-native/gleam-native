pub const seed_value: Int = 2
pub const pi_value: String = "ab"
pub const euler_value: Int = 42

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, v0: List(Int), class: #(Float, Bool)) -> Float {
{
    let z = case "b" <> "a", [1] {
      "abc", [h] if h % 2 == 0 || h % 2 == 0 -> constructor
      "ab" <> rest, [8] -> {
        10.0
      } == {
        0.25
      }
      l, [b] -> True
      _, v1 -> constructor
    }
    let delete = case <<"b":utf8>> {
      <<_:utf8>> as whole -> [7, 5]
      <<_:8, _:8>> -> v0
      _ -> v0
    }
    case 0.1, fn(v2) { v2 }(5) {
      10.0, prototype -> {
        let length = constructor
        let v = 42
        0.25
      }
      _, 7 -> 0.5
      10.0, 3 -> {
        0.0
      } +. {
        0.0
      }
      _, _ -> 0.25
    }
  }
}

pub fn main() {
  let euler_value = seed_value
  let rest = [10, 7]
  echo case 1 {
    constructor -> {
      fn(v3) { pi_value }(True)
    } == {
      "x" <> pi_value
    }
    1 -> case {
        let l = rest
        let s = pi_value
        0.1
      } {
      0.25 -> 2 < euler_value
      _ | 1.5 -> 3 < seed_value
    }
    7 | 4 -> {
      100.0
    } >. {
      0.0
    }
  }
  echo case [], False {
    [1, ..rest], True -> []
    [], False -> {
      let v = fn(v4, v5) { v4 }(4, False)
      rest
    }
    _, _ -> case 42 {
      constructor -> []
      2 -> rest
    }
  }
  echo {
    let y = {
      3.14
    } != {
      {
        let l = True
        0.0
      }
    }
    let delete = 0 % 1
    {
      {
        let pi_value = 2.0
        pi_value
      }
    } -. {
      0.25
    }
  }
}
