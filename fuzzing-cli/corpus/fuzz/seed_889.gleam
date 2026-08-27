pub type Symbol {
  Cv0(value: String, inner: Int)
  Cv1(Int, Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, v2: Int, delete: Bool) -> String {
"bc"
}

pub fn main() {
  echo case "x" {
    item -> fn(v3) { "data" }(False)
    "b" <> constructor | "data" <> constructor -> constructor <> {
      {
        let length = constructor
        let constructor = "a"
        ""
      }
    }
    "constructor" -> "bc"
  }
  echo {
    100.0
  } /. {
    2.0
  }
  echo {
    let this_ = 4
    let item = case fn(v4, v5) { v5 }(3, 0.1) {
      10.0 -> [5]
      _ | 10.0 -> [0]
    }
    {
      let y = fn(v6, v7) { "b" }(10, True)
      let delete = False
      1.0
    }
  }
  echo "x"
}
