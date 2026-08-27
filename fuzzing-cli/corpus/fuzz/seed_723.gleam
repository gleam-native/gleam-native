pub type V0 {
  None(value: String, inner: Float)
  Cv1(List(Int))
}

pub type V2 {
  Number(Float)
  Ok
  Cv3(Float)
}

pub type Object {
  Cv4(List(Int), Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v5: V2) -> String {
{
    case 42 {
      x -> "b" <> "data"
      2 as whole -> "res"
      3 -> "res"
    }
  } <> {
    case "constructor", None("x", 0.0) {
      _, Cv1([_, ..rest]) as whole -> "x"
      "ab", _ -> "abc" <> ""
      _, _ -> ""
    }
  }
}

fn f1(v6: Int, v7: Float) -> String {
"" <> {
    fn(v8) { Number(0.5) |> f0() }(0.0)
  }
}

pub fn main() {
  echo True
  echo "bc"
}
