pub type Record {
  Cv0(value: String, inner: Float)
  Cv1
  Cv2(value: Int)
}

pub type V3 {
  Cv4(value: List(Int))
  Cv5(List(Int))
  Cv6
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(constructor: Int, v7: String, v8: Bool) -> Int {
case constructor {
    constructor -> 0
    5 -> {
      constructor * 7
    } + 4
  }
}

pub fn main() {
  echo fn(v9, v10) { case v9, fn(v11) { v9 }(1) {
    "data", "x" as whole if whole != "" || whole == "data" -> ""
    v12, "bc" <> rest if v12 != "a" -> "constructor" <> v12
    "abc", "x" <> rest -> v9
    _, _ -> "a"
  } }("bc", 0.5)
  echo "bc"
  echo [3, 4]
}
