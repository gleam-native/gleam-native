pub type Number {
  Cv0(value: String, inner: List(Int))
}

pub type V1 {
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn extends(delete: Int, n: Int) -> Float {
case "constructor" {
    a -> 2.0
    _ -> {
      let n = fn(v3) { [] }(False)
      0.0
    }
    _ | "x" -> 100.0
  }
}

fn f1(s: String, new: Int, prototype: Float) -> String {
fn(v4, v5) { {
    s <> s
  } <> {
    fn(v6, v7) { "" }("x", 0.1)
  } }(3.14, 42)
}

fn static(v8: Bool) -> String {
f1({
    {
      let arguments = 1
      "a"
    }
  } <> f1("ab", 2, 1.0), {
    2 % 2
  } + {
    100 * 7
  }, 0.5)
}

pub fn main() {
  let this_ = 3
  let this_ = case this_, walk([2], 0) {
    _, 8 -> "res"
    4, _ -> {
      let self_ = []
      let self_ = self_
      "constructor"
    }
    v9, _ -> "res"
  }
  echo f1(this_ <> {
    this_ <> this_
  }, {
    let this_ = False && True
    5
  }, 1.5)
  echo "res"
  echo True
  echo case "" <> this_ {
    "b" | "bc" -> case fn(v10, v11) { #(True, [10]) }("data", ""), True {
      #(_, [_, 4, ..]), True -> False
      #(_, [_, h, ..]), this_ -> True
      #(True, [this_, ..rest]), False -> False
      v12, v13 -> 0 >= 3
    }
    "x" -> False
    "b" <> rest -> case Cv2, Cv2 {
      v14, Cv2 -> True
      Cv2, v15 -> True
      v16, v17 -> True
    }
    v18 -> fn(v19) { v19 && True }(True)
  }
}
