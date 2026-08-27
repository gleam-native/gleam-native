pub type V0 {
  Error(value: String, inner: List(Int))
  Cv1(Int, Float)
  Cv2(value: String)
}

pub type Number {
  Record
  Some
  Cv3
}

pub type Record {
  Cv4
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(y: List(Int), v5: String) -> String {
"constructor"
}

fn delete(prototype: String, n: Int, v6: #(String, String)) -> Int {
7
}

pub fn main() {
  let delete = !{
    4 <= 2
  }
  let rest = case {
      let self_ = "ab"
      let s = 1.0
      self_
    }, "ab" {
    n, "x" -> True
    "ab" as whole, "a" -> {
      let delete = False
      let whole = [2]
      delete
    }
    delete, "bc" <> rest -> True
    v7, v8 -> True
  }
  echo []
  echo 100
  echo case 2.0 {
    item -> 2 - spin(5, 1)
    _ -> 7 + 1
  }
  echo False
}
