pub const tag_value: Bool = True
pub const pi_value: Float = 2.0
pub const limit_value: Int = 10

pub type V0 {
  Record(value: String, inner: Int)
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(n: Bool, v2: V0) -> Int {
fn(v3, v4) { 4 }(0.1, "bc")
}

pub fn main() {
  let y = case {
      let pair = tag_value
      let self_ = "a"
      tag_value
    } {
    inner -> [0]
    item -> fn(v5) { [2, 4] }(True)
    item -> fn(v6, v7) { [100, 42] }("a", 4)
  }
  let tag_value = case Cv1, {
      let acc = tag_value
      let self_ = 2.0
      True
    } {
    Cv1, value -> limit_value - 10
    Record("bc", 6) as whole, False -> arguments(tag_value, Cv1)
    _, v8 -> limit_value
  }
  echo {
    let limit_value = {
      tag_value - tag_value
    } >= 10
    {
      let constructor = "x" <> "bc"
      let constructor = []
      pi_value
    }
  }
  echo case {
      let prototype = pi_value
      "res"
    }, "" {
    "constructor", "ab" <> rest as whole if rest != "constructor" -> fn(v9) { [] }("ab")
    _, _ -> [5]
  }
  echo "abc"
}
