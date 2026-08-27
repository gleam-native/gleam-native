pub type V0 {
  Cv1
  Cv2
  Cv3(Float, String)
}

pub type V4 {
  Cv5(value: List(Int), inner: Int)
  None(Bool)
  Cv6(String, String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v7: List(Int), m: String, v8: Int) -> String {
case "b" == "ab" {
    True -> m
    True | True -> ""
    _ -> {
      let v8 = {
        let this_ = v8
        m
      }
      m <> m
    }
  }
}

fn f1(new: V4, s: V4) -> Bool {
True
}

fn f2(l: String) -> Bool {
False
}

pub fn main() {
  let self_ = {
    let new = [3, 1]
    let value = [0, 0]
    "a" <> "x"
  }
  let delete = case 10 {
    _ | 0 -> 3.14
    _ | 9 -> 0.1
  }
  echo case "ab" <> "data", {
      let item = 100
      let acc = []
      self_
    } {
    "constructor" as whole, "x" <> rest -> self_ <> {
      {
        let item = delete
        let new = whole
        self_
      }
    }
    "abc", _ -> self_
    "constructor" <> _, "res" -> f0([], "data", {
      let pair = delete
      3
    })
    v9, v10 -> {
      "ab" <> "data"
    } <> v10
  }
  echo [3]
}
