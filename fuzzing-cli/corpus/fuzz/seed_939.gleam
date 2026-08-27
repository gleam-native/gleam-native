pub type Symbol {
  Record
  Cv0(Int, Float)
}

pub type V1 {
  Cv2(value: List(Int), inner: Bool)
  Some(String)
  Error
}

pub type V3 {
  Number(value: String)
  Cv4
}

fn f0(v5: Symbol, arguments: List(Int)) -> String {
"b"
}

pub fn main() {
  let value = "bc"
  let value = case "ab" {
    "ab" | "data" <> _ -> fn(v6) { False }("b")
    value -> True
  }
  echo 0.0
  echo case 4, Cv0(5, 100.0) {
    s, Cv0(new, 10.0) if s > 9 -> case "data", 10 {
      "data", 6 -> {
        3.14
      } -. {
        0.1
      }
      "x", 3 -> 0.25
      "abc" <> rest, arguments -> 1.0
      _, _ -> fn(v7, v8) { 2.0 }(False, False)
    }
    6, Cv0(2, v9) as whole -> 0.25
    4, Cv0(value, _) -> {
      1.0
    } /. {
      0.5
    }
    v10, v11 -> {
      {
        let value = "res"
        3.14
      }
    } +. {
      fn(v12, v13) { 100.0 }(False, True)
    }
  }
}
