fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: String, v: List(Int), n: Int) -> Bool {
case {
      let constructor = "data"
      "abc"
    }, 100 {
    _, _ -> fn(v0, v1) { True }(0.0, 100)
    "b", _ -> False
  }
}

fn f1(v2: String) -> Float {
{
    3.14
  } +. {
    1.5
  }
}

fn f2(v3: Bool, v4: Float, s: #(Float, List(Int))) -> Float {
v4
}

pub fn main() {
  echo 0 - {
    {
      let new = "res" <> ""
      let new = "bc" <> new
      42 * 42
    }
  }
  echo {
    "bc" <> "x"
  } != "a"
  echo []
  echo {
    {
      fn(v5, v6) { 10 }(3.14, True)
    } * {
      4 - 2
    }
  } * walk([2, 3], [10, 3] |> walk(4))
}
