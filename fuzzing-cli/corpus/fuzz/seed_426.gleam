pub const limit_value: Float = 10.0

pub type V0 {
  Number(value: String, inner: Bool)
  Cv1(Float, Float)
}

fn static(v2: #(Float, List(Int))) -> String {
case False {
    False -> case Number("a", True) {
      Cv1(b, _) -> "a"
      Cv1(_, 0.5) -> fn(v3, v4) { "bc" }(0, False)
      Cv1(1.0, _) | Cv1(_, _) -> "ab"
      v5 -> {
        let v2 = 3
        "a"
      }
    }
    a -> "a"
    inner -> "x" <> "abc"
  }
}

fn delete(l: V0, default: Float) -> Float {
default
}

fn f2(default: Int) -> Float {
1.0
}

pub fn main() {
  echo case 100 {
    8 -> "constructor"
    inner -> "x"
  }
  echo fn(v6, v7) { "bc" }(0.5, 7)
  echo True
}
