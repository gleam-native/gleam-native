pub const limit_value: Bool = False
pub const tag_value: Bool = False

pub type Promise {
  Record
}

pub type V0 {
  Cv1(List(Int))
  Cv2
  Cv3(value: Float, inner: Bool)
}

pub type V4 {
  Cv5(value: Float)
  Cv6(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(acc: Int, v7: Bool, v8: Int) -> Bool {
"constructor" != "constructor"
}

fn extends(new: Int, v9: Int, this_: Int) -> Float {
case "x" <> "x" {
    _ -> case fn(v10) { 1.0 }(False) {
      constructor -> constructor +. {
        2.0
      }
      v11 -> {
        let this_ = []
        let value = "x"
        v11
      }
      _ -> 0.25
    }
    "a" | "res" -> case fn(v12) { "res" }(4) {
      _ -> {
        0.25
      } -. {
        1.0
      }
      _ | "x" -> 0.1
    }
  }
}

pub fn main() {
  let tag_value = extends(fn(v13) { 3 }(True), 0, {
    let z = []
    let arguments = "b"
    4
  })
  let new = case 10 - 4, fn(v14, v15) { #([7, 42], [1, 3]) }(100, 0.0) {
    _, #([], [a]) if a == 9 || a <= 2 -> fn(v16, v17) { [1, 2] }(100.0, "constructor")
    s, #([constructor], [h, tag_value, ..]) -> [5, 10]
    _, #([], [_, b, ..]) -> []
    _, v18 -> fn(v19, v20) { [] }(True, False)
  }
  echo new
}
