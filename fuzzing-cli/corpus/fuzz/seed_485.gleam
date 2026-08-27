fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(delete: #(String, Float)) -> String {
"data"
}

pub fn main() {
  echo 100
  echo case {
      100.0
    } -. {
      100.0
    } {
    constructor -> [5]
    inner -> case [] {
      [inner, a, ..] -> [4]
      [a, 7, ..] -> [3, 10]
      _ -> [4]
    }
    0.25 -> [4, 10]
  }
}
