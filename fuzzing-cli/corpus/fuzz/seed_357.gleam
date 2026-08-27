pub const golden_value: Float = 0.25

pub type Record {
  Cv0(value: String, inner: String)
  Ok
  Cv1(List(Int), value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(item: List(Int), l: Float, v2: #(String, List(Int))) -> Bool {
True
}

pub fn main() {
  echo {
    let n = {
      let new = 4 + 5
      let new = True
      fn(v3) { "res" }(10.0)
    }
    [1, 0]
  }
}
