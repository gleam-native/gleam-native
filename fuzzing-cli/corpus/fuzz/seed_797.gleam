pub type V0 {
  Some(value: String, inner: String)
  Cv1(List(Int))
  Error
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(rest: Float, class: Bool, value: List(Int)) -> Bool {
case fn(v2, v3) { "bc" }("x", "res"), <<42:8>> {
    "constructor" <> _, <<100:4, _:big-signed-4>> as whole -> !{
      fn(v4) { class }(False)
    }
    "b" <> rest, _ -> {
      1.0
    } >=. {
      {
        let length = value
        2.0
      }
    }
    v5, _ -> class
  }
}

fn f1(v6: Int, y: V0) -> Bool {
False
}

fn f2(v7: Int) -> Bool {
case Cv1([]), 100 + v7 {
    Some(_, v), 4 -> False
    Cv1([0] as whole), 1 -> True
    self_, _ -> True
  }
}

pub fn main() {
  echo 0.25
  echo 1.5
  echo True
}
