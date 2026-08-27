pub type Symbol {
  Cv0(value: String, inner: List(Int))
  Number(String)
  Cv1
}

pub type Promise {
  Cv2(value: Int, inner: Bool)
  Record(Bool)
}

fn f0(length: String) -> Bool {
True
}

fn f1(v3: #(Int, Float)) -> Float {
0.1
}

fn f2(v4: List(Int), v5: Int) -> Int {
42
}

pub fn main() {
  let arguments = "res" <> ""
  let s = case #(100.0, 0.1) {
    #(0.25, 1.5) -> []
    a -> []
    #(_, 2.0) -> [42, 5]
  }
  echo {
    case f2(s, 0) {
      4 | 3 -> arguments <> "ab"
      _ -> {
        let prototype = "ab"
        let constructor = "x"
        "data"
      }
      v6 -> {
        let class = 0
        let l = 3.14
        arguments
      }
    }
  } <> "constructor"
  echo s
}
