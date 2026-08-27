pub const pi_value: Float = 3.14
pub const tag_value: Int = 7
pub const limit_value: String = "res"

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

fn static(z: Int, v2: V0) -> List(Int) {
[]
}

pub fn main() {
  let value = {
    let default = fn(v3) { [42] }(True)
    let l = tag_value
    "constructor" <> limit_value
  }
  echo case False {
    True -> {
      {
        let v = tag_value
        True
      }
    } || {
      fn(v4) { v4 }(False)
    }
    _ -> {
      pi_value +. {
        1.5
      }
    } <=. {
      3.14
    }
    constructor -> {
      let prototype = static(tag_value, Cv1([7], 100))
      0 < tag_value
    }
  }
  echo {
    {
      3.14
    } *. {
      2.0
    }
  } >. {
    case #("bc", True), {
        let z = pi_value
        let new = tag_value
        1.0
      } {
      #(_, _), 2.0 as whole -> whole *. whole
      #("res" <> _ as whole, False), limit_value if limit_value >=. 3.14 -> {
        let item = value
        1.5
      }
      #("b", pi_value) as whole, _ -> 0.0
      _, v5 -> pi_value
    }
  }
  echo tag_value + {
    2 * {
      0 - tag_value
    }
  }
  echo {
    case 100 + tag_value {
      8 | 3 -> 42
      item -> 3
      _ -> 2 % 2
    }
  } |> static(Cv1([], 10))
}
