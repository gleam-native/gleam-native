pub type V0 {
  Cv1
  Cv2
  Cv3(value: List(Int))
}

pub type Map {
  Some
  Cv4(String, Float)
  Number
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(s: #(Bool, Bool), v5: String) -> Int {
5
}

pub fn main() {
  let l = case True, fn(v6, v7) { Cv2 }(True, 2.0) {
    True, Cv2 -> False
    True, _ -> True
    v8, _ -> fn(v9, v10) { v8 }(2, "abc")
  }
  echo case fn(v11, v12) { Number }(False, True) {
    Cv4(_, item) -> case {
        let value = l
        let x = "data"
        False
      } {
      False -> {
        let item = [3, 0]
        let n = 7
        ""
      }
      True | True -> "ab" <> "constructor"
      b -> "b"
    }
    Some -> "b" <> {
      "abc" <> "abc"
    }
    Number | Number -> case "bc" <> "res" {
      l -> "bc"
      inner -> inner <> "ab"
    }
  }
}
