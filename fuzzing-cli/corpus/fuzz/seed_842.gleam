pub const pi_value: String = "data"
pub const golden_value: Float = 0.1
pub const euler_value: Int = 4

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
  Some(value: Float)
}

pub type Promise {
  Cv3(value: String)
  Cv4(Float, value: String)
}

pub type Symbol {
  Cv5
  Cv6(value: Bool, inner: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(new: Int, v7: Int, m: String) -> Float {
{
    case "data", {
        let rest = v7
        Cv6(True, 100)
      } {
      "data", Cv6(False, v7) if v7 > 0 -> {
        10.0
      } *. {
        0.0
      }
      "abc", Cv5 -> fn(v8) { 100.0 }("b")
      _, Cv5 -> {
        0.0
      } +. {
        1.5
      }
      v9, _ -> 1.5
    }
  } /. {
    10.0
  }
}

fn f1(length: Int) -> List(Int) {
case Cv3("res") {
    Cv4(_, "constructor" <> rest) if rest != "res" -> []
    Cv3("bc" <> rest) -> {
      let length = length
      let rest = length
      []
    }
    _ -> []
  }
}

pub fn main() {
  let m = {
    {
      let golden_value = euler_value
      let constructor = False
      [1]
    }
  } |> walk(42)
  let delete = case {
      let pi_value = "res"
      m
    }, 100 + m {
    y, _ -> [3, 3]
    6, v10 -> [42, 5]
    2, _ -> [2, 3]
  }
  echo {
    {
      let this_ = golden_value <. {
        1.5
      }
      fn(v11, v12) { delete }(True, 0.1)
    }
  } |> walk(m + 42)
  echo {
    {
      let golden_value = delete
      walk(delete, 10)
    }
  } >= euler_value
  echo {
    case "" <> pi_value, Cv5 {
      "abc", Cv6(True, _) -> f0(euler_value, m, "ab")
      "a", _ -> 0.5
      _, Cv6(True, 6) -> golden_value /. {
        2.0
      }
      _, v13 -> f0(euler_value, m, pi_value)
    }
  } +. f0(euler_value + euler_value, walk(delete, euler_value), pi_value <> pi_value)
}
