pub type V0 {
  Cv1
  Cv2
}

pub type V3 {
  Cv4(value: Int)
  Cv5(Float, value: Bool)
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

fn constructor(m: V6, v8: #(Int, Bool), value: List(Int)) -> Bool {
case fn(v9, v10) { #(0.25, False) }(5, 2.0), fn(v11, v12) { False }(0.1, 100.0) {
    #(_, False) as whole, True -> case {
        let whole = value
        2.0
      } {
      a -> False
      _ -> False
      _ -> {
        let m = 10.0
        False
      }
    }
    #(10.0, True), True -> {
      5 < 7
    } && {
      fn(v13) { True }("")
    }
    #(0.25 as whole, False), v8 -> v8
    v14, _ -> case "a" <> "bc" {
      item | "ab" <> item -> {
        1.0
      } <=. {
        3.14
      }
      "b" <> _ -> False || False
    }
  }
}

fn f1(v15: Bool, v16: Int) -> List(Int) {
fn(v17) { [] }(100.0)
}

fn f2(class: Float, delete: String, item: V0) -> Bool {
False
}

pub fn main() {
  echo f1(False, case 100 - 4 {
    0 | 5 -> 5
    item -> 0
  })
  echo True
}
