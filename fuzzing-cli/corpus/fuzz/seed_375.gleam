fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Bool, l: Float, x: List(Int)) -> Int {
fn(v0, v1) { 100 }("res", "b")
}

pub fn main() {
  echo 2.0
  echo {
    walk([], 4) * {
      [0, 7] |> walk(5 + 100)
    }
  } - 42
  echo case "abc" <> "constructor" {
    pair | "abc" <> pair -> pair
    item -> case "" {
      "x" <> item | "bc" <> item -> item <> "res"
      inner -> inner <> "abc"
      "res" <> rest | "ab" <> rest -> item <> rest
    }
  }
}
