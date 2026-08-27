pub const tag_value: Int = 5

pub type Map {
  Cv0(value: String, inner: List(Int))
  Cv1(value: Float)
  Cv2
}

pub type V3 {
  Cv4(Int)
  Cv5(value: Bool)
}

pub type V6 {
  Cv7
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(value: V3, new: String, delete: String) -> String {
case fn(v8, v9) { Cv0("ab", [2, 100]) }(0.0, False) {
    a -> case 4 >= 100, "res" <> delete {
      True, "x" -> delete <> new
      True, "b" -> fn(v10, v11) { delete }(0.25, "res")
      _, _ -> "ab" <> delete
    }
    Cv1(3.14) -> delete
  }
}

fn f1(v12: List(Int)) -> String {
"abc"
}

fn f2(y: Map, v13: Float) -> Bool {
False
}

pub fn main() {
  echo case True {
    _ -> 7
    True -> tag_value
  }
  echo False
}
