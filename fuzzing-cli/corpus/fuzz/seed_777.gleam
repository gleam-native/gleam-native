pub const golden_value: Int = 5

pub type Number {
  Cv0(value: String, inner: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v1: Float) -> String {
"data"
}

fn f1(l: String, v2: String) -> Bool {
case fn(v3, v4) { "" }(100, 0.5) {
    a | "abc" <> a -> case Cv0("x", [2]) {
      v5 -> {
        3.14
      } <=. {
        0.5
      }
      Cv0("bc" <> rest as whole, [a, 4, ..]) -> {
        0.0
      } >=. {
        2.0
      }
    }
    inner | "data" <> inner -> fn(v6, v7) { True }(True, 1.5)
  }
}

pub fn main() {
  echo [100]
  echo 1.0
  echo fn(v8, v9) { golden_value }(False, 2)
}
