pub const euler_value: String = "abc"

pub type Promise {
  Cv0(value: String, inner: List(Int))
}

pub type V1 {
  Cv2
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(v4: List(Int), v5: Bool) -> String {
case "data" <> "x", fn(v6, v7) { Cv2 }(True, "x") {
    "bc" <> rest as whole, Cv3 -> "res" <> whole
    "x" <> _, _ -> case #([4, 10], "") {
      #([_, _, ..], "b") -> "bc" <> "ab"
      #([constructor, ..rest], _) -> {
        let rest = 0.5
        "bc"
      }
      _ -> "ab"
    }
    _, v8 -> case {
        let s = 10
        v4
      } {
      [] -> "" <> "b"
      [3] -> "a"
      [7] -> {
        let self_ = 2.0
        let v8 = 5
        "abc"
      }
      v9 -> {
        let x = v9
        "b"
      }
    }
  }
}

pub fn main() {
  echo fn(v10, v11) { [1] }(True, 42)
  echo case Cv2, {
      let l = 2.0
      1
    } {
    Cv2, v12 if v12 > 1 || v12 > 1 -> case #("a", False) {
      #(_, False) | #("ab" <> _, False) -> False
      a -> {
        let a = ""
        let n = []
        False
      }
    }
    Cv3, _ -> {
      False && True
    } && True
    Cv2, 9 -> True
    v13, v14 -> True
  }
}
