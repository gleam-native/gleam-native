pub const seed_value: Bool = False
pub const pi_value: Bool = False
pub const limit_value: Int = 7

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(value: Float, inner: Int)
}

pub type V3 {
  Cv4
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn extends(prototype: Bool, n: V0) -> Int {
fn(v5, v6) { case v6 +. v6 {
    a -> 42
    1.0 | 1.0 -> 100
  } }("x", 0.1)
}

fn f1(v7: Int, v8: String) -> List(Int) {
[42]
}

fn f2(m: Float) -> Float {
3.14
}

pub fn main() {
  let prototype = case "x", seed_value {
    "constructor" <> rest, True as whole if rest == "ab" && !whole -> fn(v9, v10) { [] }(0, 100.0)
    "bc" <> _, default -> [10, 7]
    _, _ -> f1(5, "ab")
  }
  let prototype = {
    limit_value |> spin(3 - limit_value)
  } < {
    limit_value * limit_value
  }
  echo limit_value
  echo case {
      let prototype = "res"
      Cv1([100], 0)
    } {
    Cv1([pi_value], _) -> {
      let m = "constructor"
      let rest = pi_value + limit_value
      {
        let seed_value = "bc"
        let l = 0.0
        prototype
      }
    }
    Cv1([_, ..rest], _) -> {
      let constructor = limit_value
      let prototype = 2.0
      pi_value
    }
    v11 -> False
  }
}
