pub const limit_value: Int = 5
pub const seed_value: String = "data"

pub type Number {
  Cv0(value: String, inner: List(Int))
  None
  Number(Bool, Bool)
}

pub type V1 {
  Error(Bool)
  Cv2(value: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(rest: Int, n: Int, x: String) -> Float {
{
    {
      0.25
    } -. {
      {
        0.5
      } /. {
        2.0
      }
    }
  } -. {
    0.5
  }
}

fn constructor(x: List(Int)) -> Int {
4
}

fn f2(new: #(Int, Int), l: List(Int)) -> Int {
7
}

pub fn main() {
  let rest = case {
      let acc = limit_value
      Cv0("ab", [0, 3])
    } {
    _ -> []
    Number(constructor, _) -> []
    Number(True, True as whole) -> {
      let limit_value = 10.0
      let whole = 42
      []
    }
  }
  let seed_value = rest
  echo fn(v3) { case Error(False), {
      let l = "bc"
      Cv2(42)
    } {
    Error(False), Error(_) -> limit_value
    Cv2(_) as whole, rest -> 10 - limit_value
    Error(x), _ -> 42
    _, _ -> fn(v4, v5) { 10 }("", "bc")
  } }(False)
  echo fn(v6) { {
    1.5
  } +. {
    1.0
  } }("abc")
}
