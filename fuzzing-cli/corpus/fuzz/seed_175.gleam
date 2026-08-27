pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: Int, inner: Float)
}

pub type Object {
  Cv3(List(Int), value: Float)
}

pub type V4 {
  Cv5(String, Bool)
}

fn f0(v6: Int, v7: Int, v8: Int) -> Float {
case v7 + 1, [4] {
    v9, [x] -> case {
        let x = False
        let rest = 0.0
        [1, 10]
      } {
      [] -> {
        10.0
      } +. {
        10.0
      }
      [3, ..rest] as whole -> 0.5
      [_] -> 0.25
      _ -> 0.25
    }
    7, [_, 1, ..] -> {
      let v6 = "data" <> "constructor"
      {
        10.0
      } +. {
        10.0
      }
    }
    1, [6, ..rest] -> {
      0.25
    } /. {
      2.0
    }
    _, _ -> 0.5
  }
}

fn constructor(y: V0, v10: List(Int), item: Int) -> List(Int) {
case "x", "ab" {
    "res", "abc" -> case fn(v11) { Cv5("abc", True) }(100) {
      b -> [3]
      Cv5("a" <> rest, True) -> v10
    }
    _, "x" <> _ -> [5]
    v12, v13 -> [100, 42]
  }
}

fn f2(v14: Float, rest: Bool, default: Float) -> Float {
case 42, [5, 0] {
    9 as whole, [x, ..rest] as it -> default
    4, [4, b, ..] if b > 7 -> default
    1, [9, ..rest] -> fn(v15, v16) { default -. {
      3.14
    } }(False, 0)
    _, v17 -> case fn(v18, v19) { 1 }(10, 100) {
      _ -> 0.1
      constructor -> fn(v20, v21) { default }(True, 1)
    }
  }
}

pub fn main() {
  let n = {
    42 + 5
  } + 42
  let n = 2.0
  echo case #(True, "data") {
    #(v22, "abc") -> case {
        let v22 = True
        let x = []
        100.0
      }, Cv5("", False) {
      0.25, _ -> "abc"
      0.0, _ -> "" <> "a"
      v23, _ -> "a"
    }
    #(True, "x" as whole) if whole != "a" && whole != "ab" -> case fn(v24, v25) { 100 }(100.0, False) {
      _ -> "res" <> "abc"
      class -> whole <> whole
      6 | 0 -> "b" <> whole
    }
    inner -> {
      "bc" <> "x"
    } <> {
      "ab" <> "bc"
    }
  }
}
