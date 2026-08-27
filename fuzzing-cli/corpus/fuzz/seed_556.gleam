pub const seed_value: Bool = True

pub type Promise {
  Cv0(value: String, inner: Float)
  Cv1(String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: Promise) -> Float {
0.25
}

fn f1(z: Int) -> Int {
case Cv1("res") {
    class -> z
    Cv0(constructor, _) if constructor != "a" -> walk([100], fn(v3) { 10 }(100))
    Cv0("bc", 1.0) -> z
  }
}

fn f2(s: Int, v: #(String, List(Int))) -> Bool {
case fn(v4) { Cv0("abc", 1.5) }(3.14) {
    Cv1("abc" <> rest) -> True
    _ -> {
      let z = {
        let this_ = s
        let v = [3, 10]
        1.0
      }
      let class = ""
      True && False
    }
    Cv0(_, 1.0) -> False && True
  }
}

pub fn main() {
  echo {
    let x = case {
        let constructor = True
        "data"
      } {
      constructor | "data" <> constructor -> 0.5
      "a" | "bc" <> _ -> {
        let rest = [2]
        let z = 1.5
        z
      }
    }
    "b" <> {
      fn(v5, v6) { "constructor" }(True, False)
    }
  }
  echo 2 |> f2(#("bc", []))
  echo fn(v7, v8) { [0] }("b", True)
  echo "bc"
}
