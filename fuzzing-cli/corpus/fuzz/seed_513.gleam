pub const limit_value: String = ""

pub type V0 {
  Record(value: String, inner: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(y: Bool, rest: Int, z: List(Int)) -> Bool {
True
}

fn f1(v1: List(Int), v: Bool) -> List(Int) {
v1
}

pub fn main() {
  let class = [1, 3]
  let s = case <<5:8>> {
    <<self_:4, "res":utf8>> -> class
    <<_:4>> -> class
    _ -> f1([5, 5], True)
  }
  echo {
    let this_ = 3.14
    fn(v2, v3) { fn(v4, v5) { "constructor" }(0.5, False) }("bc", 2.0)
  }
  echo {
    case {
        let class = 2
        let n = class
        False
      } {
      v6 -> "x"
      _ -> "data"
      False as whole -> limit_value <> "constructor"
    }
  } <> {
    {
      {
        let s = True
        let limit_value = 3.14
        "a"
      }
    } <> {
      "x" <> limit_value
    }
  }
}
