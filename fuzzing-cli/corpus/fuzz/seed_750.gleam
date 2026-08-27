pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type V2 {
  Number
  Cv3
  Cv4(value: Bool)
}

pub type Symbol {
  Cv5(String, Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn new(s: String, v6: Float, v7: V0) -> List(Int) {
[7, 4]
}

pub fn main() {
  let acc = case {
      let n = [10, 10]
      Cv3
    } {
    b -> 0.0
    constructor -> {
      let default = "data"
      0.0
    }
  }
  let acc = case {
      let item = ""
      []
    } {
    [acc] -> [1]
    [a] -> [10]
    v8 -> v8
  }
  echo case 0 {
    _ | 3 -> case 100 + 100 {
      constructor -> "a" <> "bc"
      5 -> "x"
      b -> "ab"
    }
    v9 -> "data"
  }
}
