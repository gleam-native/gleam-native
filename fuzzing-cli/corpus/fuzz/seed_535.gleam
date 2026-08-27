pub const limit_value: Int = 4
pub const euler_value: Int = 7
pub const golden_value: Float = 0.5

pub type V0 {
  Ok(value: String, inner: Int)
  None(value: String)
  Cv1(String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(v2: Bool) -> Int {
walk([0, 0], case None("bc") {
    Cv1("ab" <> rest) -> 5
    _ | None(_) -> 4
    Cv1(_) -> 3 - 4
  })
}

fn default(v3: String, class: #(Int, Float)) -> String {
v3 <> {
    case 42 * 2 {
      7 as whole -> "x"
      _ | 2 -> v3
    }
  }
}

pub fn main() {
  echo fn(v4) { "data" <> {
    v4 <> "x"
  } }("ab")
}
