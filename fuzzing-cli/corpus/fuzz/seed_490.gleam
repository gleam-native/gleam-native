pub const pi_value: String = "x"

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(value: Bool)
}

pub type Object {
  Error(Bool, value: Bool)
}

pub type V3 {
  Ok
  Cv4
  Cv5
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn default(self_: Bool, v6: Bool) -> String {
"bc"
}

fn f1(v7: Bool) -> Bool {
case "res" {
    "res" <> rest if rest == "ab" -> {
      let item = 10.0
      let item = item +. {
        2.0
      }
      v7 || False
    }
    "bc" -> 1 >= 7
    _ -> True
  }
}

pub fn main() {
  echo fn(v8, v9) { case Error(True, True), default(True, False) {
    Error(delete, v10), "x" -> []
    Error(True, _), "data" -> []
    _, "a" -> [1, 100]
    v11, v12 -> [10, 1]
  } }(3.14, 1)
}
