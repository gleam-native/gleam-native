pub const tag_value: Int = 7

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Number(value: Int)
}

pub type V2 {
  Cv3(Bool)
  Cv4(List(Int), value: Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(z: List(Int)) -> Int {
{
    let x = 5
    let x = "data"
    0
  }
}

fn constructor(this_: Float, arguments: String) -> Float {
case 10 |> spin({
      let this_ = arguments
      5
    }) {
    a -> case "" {
      "x" | "a" -> this_
      inner -> this_
    }
    a -> this_
  }
}

pub fn main() {
  let tag_value = case 5 - 2 {
    5 | 0 -> [5, 5]
    b -> [3]
    a -> [3, 4]
  }
  echo {
    {
      "a" <> "ab"
    } <> {
      "" <> ""
    }
  } <> {
    case {
        let m = 0.5
        Cv4([4], False)
      } {
      Cv4([constructor], True) -> fn(v5, v6) { "ab" }(1, 10)
      Cv3(_) as whole -> "res"
      Cv3(False) | Cv3(_) -> "b"
      _ -> fn(v7) { "" }(0.0)
    }
  }
}
