pub const limit_value: Bool = True

pub type V0 {
  Ok(value: String, inner: Int)
}

pub type V1 {
  Cv2(value: Bool, inner: Float)
  Cv3(value: List(Int), inner: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v4: String) -> Int {
3
}

fn default(v5: String, v6: Int) -> String {
case fn(v7) { v5 }(3) {
    _ | "constructor" <> _ -> v5
    "b" | "constructor" <> _ -> case {
        let y = 3.14
        y
      }, "ab" {
      _, v8 -> "abc"
      _, "" <> rest -> rest
      _, "a" -> "x" <> v5
    }
    "x" <> _ -> "res"
  }
}

pub fn main() {
  let limit_value = 5
  echo case {
      let this_ = []
      let y = limit_value
      "constructor"
    } {
    "data" as whole if whole != "b" || whole == "constructor" -> case {
        let pair = True
        let z = False
        1.5
      } {
      inner -> 0 * limit_value
      0.0 | 1.5 -> limit_value - limit_value
      _ -> 3
    }
    "b" <> rest | "constructor" <> rest -> f0(rest <> "")
    "ab" -> {
      fn(v9) { limit_value }(2)
    } % 2
    _ -> limit_value
  }
  echo limit_value
  echo 2.0
}
