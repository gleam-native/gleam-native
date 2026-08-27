pub type V0 {
  Error(value: String, inner: String)
  Cv1(Int)
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(length: Bool, v3: String, v: Float) -> String {
"abc"
}

fn default(pair: Int) -> Float {
case 2.0, {
      let self_ = pair
      "data"
    } {
    1.0 as whole, "b" -> whole -. {
      whole -. {
        0.0
      }
    }
    3.14, _ -> {
      let acc = fn(v4, v5) { True }(2.0, "bc")
      let pair = [0]
      0.1
    }
    v6, _ -> {
      {
        let self_ = v6
        3.14
      }
    } -. {
      0.5
    }
  }
}

fn f2(v7: String, v8: Float, item: List(Int)) -> Int {
1
}

pub fn main() {
  let x = 1
  echo case x + x {
    6 -> case "" {
      _ -> ""
      "constructor" | "bc" -> "bc"
    }
    x -> "data"
    a -> "constructor"
  }
  echo x
}
