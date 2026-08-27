pub type V0 {
  Cv1(value: List(Int))
  Cv2(String)
}

pub type Symbol {
  Number
  Cv3(value: List(Int))
}

fn f0(rest: Bool, v4: Int) -> Float {
0.0
}

fn f1(length: List(Int), v5: Symbol, v6: #(Int, Int)) -> String {
case 1 != 0, {
      let v = 1
      let item = True
      v
    } {
    _, 2 as whole if whole <= 7 -> "abc"
    _, 9 -> {
      "a" <> "a"
    } <> {
      {
        let s = "res"
        "res"
      }
    }
    True, _ -> case {
        let self_ = False
        Number
      }, Number {
      Number, Number -> "constructor"
      Number, Cv3([]) as whole -> "b"
      Number as whole, v7 -> "abc"
      _, _ -> "ab"
    }
    _, _ -> fn(v8, v9) { "ab" }(True, 4)
  }
}

pub fn main() {
  echo []
  echo {
    let item = 100
    [4, 2]
  }
  echo case {
      let prototype = 1
      let value = True
      Cv1([])
    } {
    Cv2(inner) -> case {
        10.0
      } /. {
        1.0
      }, [1] {
      3.14, [constructor] if constructor == 0 -> False
      _, [] -> False
      2.0, [5] -> False
      _, v10 -> !True
    }
    Cv2("x" <> rest) -> {
      fn(v11, v12) { True }(4, 0.5)
    } || False
    v13 -> case Cv3([100, 5]), v13 {
      _, Cv1([_, ..rest]) -> True
      _, Cv2("abc") -> True
      v14, _ -> True
    }
  }
  echo case f1([], Cv3([]), #(5, 5)), [] {
    "ab" <> rest, [6, ..tail] if rest != "" && rest == "ab" -> case {
        0.1
      } -. {
        3.14
      } {
      constructor -> [1, 3]
      1.0 | 1.0 -> tail
    }
    "b" as whole, [constructor, ..rest] as it -> []
    _, [constructor, ..rest] -> case fn(v15, v16) { constructor }(0.25, "data"), False && True {
      4, False -> []
      5, True -> rest
      _, v17 -> [0, 1]
    }
    _, _ -> [4, 42]
  }
}
