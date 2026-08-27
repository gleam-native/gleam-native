pub type V0 {
  Cv1(value: List(Int))
  Cv2(Int)
  Cv3(List(Int))
}

pub type V4 {
  None
  Cv5
  Number(List(Int), value: Int)
}

pub type Symbol {
  Cv6(Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(x: List(Int), prototype: V0, pair: Bool) -> List(Int) {
x
}

fn f1(v7: String, v8: Int) -> Float {
0.0
}

pub fn main() {
  let s = fn(v9) { 5 + 4 }("bc")
  echo "data" <> {
    case True || True {
      False -> "x"
      False -> "constructor" <> "b"
      _ -> "a"
    }
  }
  echo "x"
  echo fn(v10) { False }(10.0)
}
