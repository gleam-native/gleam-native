pub type Map {
  Cv0(value: String, inner: String)
  Cv1(Int, List(Int))
  Cv2(value: List(Int), inner: List(Int))
}

pub type V3 {
  Cv4(Int)
  Ok(value: List(Int), inner: Float)
  Cv5(value: Bool, inner: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn yield(v6: Bool) -> String {
""
}

pub fn main() {
  let this_ = False
  echo 1.5
  echo case {
      let class = "data"
      False
    } {
    False -> fn(v7, v8) { True }(1.5, False)
    _ -> False
    False -> {
      10 - 100
    } < 2
  }
  echo fn(v9, v10) { case Cv0("constructor", ""), walk([], 7) {
    Cv0("data" as whole, class), v11 -> []
    Cv0("bc" <> rest, v12), 5 -> {
      let item = []
      let v10 = item
      [1]
    }
    v13, _ -> fn(v14) { [] }(0.25)
  } }(42, 10)
  echo this_
}
