pub const golden_value: Bool = False

pub type Object {
  Cv0(value: String, inner: Int)
}

pub type V1 {
  Some(value: Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v2: Int, v3: #(List(Int), Bool)) -> String {
case #(10, "a"), Some(False) {
    #(3, v4), Some(True) if v4 != "res" -> v4
    #(_, "res"), Some(False as whole) if whole && whole -> {
      "abc" <> "a"
    } <> "b"
    #(_, "a"), Some(_) -> "res"
    _, _ -> fn(v5, v6) { "a" <> "constructor" }(True, 0.5)
  }
}

fn f1(acc: Object, m: Object) -> Float {
10.0
}

pub fn main() {
  let class = "res"
  let z = fn(v7) { {
    1.0
  } <=. {
    0.1
  } }("bc")
  echo "ab" <> {
    case [] {
      [class, _, ..] as whole if class <= 6 -> f0(class, #([1], False))
      [3] -> class
      [] -> ""
      v8 -> class <> "res"
    }
  }
  echo {
    10.0
  } *. {
    10.0
  }
}
