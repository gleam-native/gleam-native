pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(value: Int, class: Bool, v2: Int) -> List(Int) {
[1]
}

pub fn main() {
  echo [10, 4] |> walk(10)
}
