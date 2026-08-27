pub const euler_value: Bool = False

pub type Promise {
  Cv0(value: String, inner: Int)
}

pub type Symbol {
  Record(List(Int), Bool)
  Cv1(List(Int), value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(acc: Int) -> List(Int) {
[4]
}

pub fn main() {
  let length = {
    {
      0.25
    } *. {
      0.0
    }
  } -. {
    {
      0.5
    } +. {
      3.14
    }
  }
  let euler_value = length
  echo case [1], {
      let this_ = 0
      length
    } {
    [], 0.0 -> case 10 {
      _ -> "constructor"
      _ -> "a"
      delete -> "ab"
    }
    [9, ..rest], class -> case f0(42), euler_value {
      [7], 3.14 -> fn(v2, v3) { "abc" }(2.0, "bc")
      [], _ -> fn(v4, v5) { "ab" }(False, 0.0)
      [_], 0.25 -> "abc" <> "data"
      _, _ -> "constructor" <> "abc"
    }
    [], 0.5 -> case 7 + 3 {
      constructor -> "" <> "b"
      _ | 9 -> "b"
      inner -> "constructor"
    }
    _, _ -> fn(v6) { "" <> "x" }(False)
  }
}
