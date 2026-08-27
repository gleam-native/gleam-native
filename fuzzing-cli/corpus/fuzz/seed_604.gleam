pub type Promise {
  Cv0(value: String, inner: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn yield(v1: Bool, pair: Float) -> Bool {
{
    {
      "" <> "x"
    } <> {
      "ab" <> "b"
    }
  } == "data"
}

fn f1(v2: Bool, v3: List(Int)) -> Float {
0.5
}

pub fn main() {
  let z = !{
    {
      let prototype = 0.1
      let prototype = ""
      False
    }
  }
  let z = case "res" == "bc" {
    inner -> []
    a -> fn(v4, v5) { [] }("a", 0)
    constructor -> {
      let acc = "ab"
      []
    }
  }
  echo 10.0
  echo yield(!False, {
    10.0
  } *. {
    0.25
  }) |> f1(fn(v6) { [10] }(1))
  echo spin(5 % 6, 10) + {
    fn(v7, v8) { v7 |> spin(v7 |> spin(fn(v9, v10) { v7 }(10.0, True))) }(2, "res")
  }
  echo [10]
}
