pub type Object {
  Cv0(value: String, inner: Int)
}

pub type V1 {
  Cv2(value: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn default(length: String, delete: Int, pair: Float) -> String {
case pair -. pair {
    length -> fn(v3, v4) { fn(v5) { "data" }("") }(2, 0)
    2.0 | 10.0 -> {
      fn(v6, v7) { "ab" }("res", 10.0)
    } <> {
      fn(v8, v9) { length }("x", "bc")
    }
    b -> "x" <> {
      "x" <> length
    }
  }
}

pub fn main() {
  let length = {
    let acc = {
      let item = 10
      0.25
    }
    fn(v10, v11) { acc }(True, 2.0)
  }
  let length = {
    let value = [10, 4]
    let acc = "a"
    True
  }
  echo {
    case "", fn(v12) { 4 }(2) {
      "constructor", 0 -> {
        let length = 7
        0.1
      }
      "bc" <> rest, 9 if rest == "bc" -> 0.5
      "b" <> rest, 4 as whole -> fn(v13) { 3.14 }(True)
      _, v14 -> {
        0.1
      } /. {
        1.0
      }
    }
  } -. {
    1.0
  }
}
