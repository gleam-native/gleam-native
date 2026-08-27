pub const euler_value: String = "abc"

pub type V0 {
  Cv1
  Cv2
  Cv3(Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(v4: Float, v5: #(Float, List(Int)), s: #(Float, Float)) -> String {
"ab"
}

fn f1(v6: List(Int), l: String, v7: V0) -> String {
l
}

pub fn main() {
  echo case [1, 3] |> f1("x", {
      let this_ = False
      Cv2
    }) {
    "abc" <> rest | "res" <> rest -> case {
        10.0
      } |> static(#(3.14, [4]), {
        let euler_value = []
        let delete = True
        #(0.25, 2.0)
      }), "bc" <> rest {
      "b", "data" as whole -> 3.14
      "constructor", self_ if self_ != "constructor" -> 0.5
      "x", _ -> {
        3.14
      } -. {
        10.0
      }
      _, v8 -> {
        let s = 4
        let y = [3]
        1.5
      }
    }
    "a" <> rest -> case fn(v9) { [0] }(0.25) {
      [2] -> {
        0.25
      } +. {
        2.0
      }
      [constructor] as whole -> fn(v10, v11) { 0.5 }("abc", True)
      [8, ..rest] -> 10.0
      _ -> 0.1
    }
    v12 -> fn(v13) { 0.1 }(1)
  }
  echo {
    case 100 <= 1, "ab" {
      True, "x" <> rest if rest == "a" || rest == "a" -> 10
      x, "x" <> rest -> 5 + 0
      v14, v15 -> {
        let acc = euler_value
        let class = 3
        class
      }
    }
  } + 42
}
