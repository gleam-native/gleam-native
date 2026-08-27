pub type V0 {
  Cv1
  Cv2
  Cv3(value: List(Int))
}

pub type V4 {
  Cv5(value: List(Int), inner: Int)
  Cv6
}

pub type V7 {
  Cv8(value: Bool, inner: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(class: Int, l: Int) -> Int {
{
    case {
        0.25
      } +. {
        10.0
      } {
      inner -> fn(v9) { [] }("bc")
      3.14 | 10.0 -> {
        let class = "ab"
        let class = False
        [5, 42]
      }
      1.5 as whole -> fn(v10) { [3, 2] }(10)
    }
  } |> walk(l)
}

fn export(pair: List(Int)) -> Float {
0.1
}

fn f2(arguments: Bool) -> Int {
100
}

pub fn main() {
  let m = 10 * {
    [10, 1] |> walk(3 + 42)
  }
  let m = True
  echo case fn(v11, v12) { "b" }(True, "data") {
    b -> {
      let prototype = fn(v13, v14) { "constructor" }(0, 0.5)
      let z = fn(v15, v16) { [4, 4] }(10, 1.5)
      b
    }
    _ -> {
      let acc = {
        let prototype = False
        let prototype = m
        3
      }
      let item = "" <> "constructor"
      ""
    }
    a | "ab" <> a -> case fn(v17) { Cv5([2], 10) }(1.5), fn(v18, v19) { 5 }(True, "b") {
      m, 0 -> a <> a
      Cv6 as whole, 2 as it -> a <> ""
      Cv6, 7 -> "a"
      v20, v21 -> "data" <> a
    }
  }
  echo "ab"
}
