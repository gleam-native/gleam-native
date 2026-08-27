pub type V0 {
  Cv1
}

pub type V2 {
  Cv3(String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn delete(acc: Int, value: Int) -> Int {
case <<3:16, 100:16, "a":utf8>> {
    <<2:16, _:utf8, _:8>> -> case Cv1 {
      _ -> 2 - value
      b -> 100 - 1
      Cv1 -> value - 2
    }
    _ -> [0] |> walk(7)
  }
}

pub fn main() {
  echo fn(v4) { case Cv1 {
    constructor -> True
    b -> fn(v5) { v4 }(4)
  } }(True)
  echo fn(v6, v7) { v7 }("a", 1)
  echo 5
}
