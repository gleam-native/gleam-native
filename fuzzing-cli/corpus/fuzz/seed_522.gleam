pub const euler_value: Bool = True

pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: #(Float, Bool), v: List(Int), default: String) -> Bool {
True
}

fn f1(v3: Float, v4: String) -> String {
case [4], fn(v5) { "abc" }("bc") {
    [], "ab" as whole if whole != "" && whole != "ab" -> {
      fn(v6) { v6 }("data")
    } <> ""
    [v4, ..rest], "" <> tail -> case v4, fn(v7) { tail }(True) {
      new, _ -> "a"
      7, "b" <> rest -> "b" <> "data"
    }
    [7], "b" -> case 2 {
      6 -> "constructor" <> v4
      9 -> "a"
      _ -> v4
    }
    _, v8 -> v4
  }
}

pub fn main() {
  let m = f1({
    1.5
  } /. {
    0.5
  }, "a")
  let z = euler_value
  echo m
}
