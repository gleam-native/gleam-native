pub type V0 {
  Some(value: String, inner: String)
  None
}

pub type Symbol {
  Cv1(String, List(Int))
}

pub type V2 {
  Cv3
}

fn export(v4: Float, v5: List(Int), v6: String) -> Int {
42
}

pub fn main() {
  let y = case {
      let acc = True
      Cv1("data", [4])
    } {
    _ -> {
      let z = [2, 10]
      4
    }
    Cv1("abc", [_]) | Cv1(_, _) -> {
      0.5
    } |> export([1], fn(v7) { "constructor" }("bc"))
    Cv1("a", [5, x, ..] as whole) -> fn(v8) { x }("data")
  }
  echo case fn(v9) { [4] }(10.0) {
    [_, 0, ..] -> ""
    [] -> case Some("abc", "res") {
      _ -> "b"
      acc -> "x"
    }
    [] -> "a"
    v10 -> "abc"
  }
  echo {
    let acc = [1, 4]
    100
  }
  echo "x"
}
