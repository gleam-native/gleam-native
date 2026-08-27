fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, acc: Bool, prototype: Float) -> List(Int) {
[1]
}

fn class(v0: #(Int, Int), v1: Int, m: List(Int)) -> String {
{
    case 5 {
      a -> fn(v2) { "ab" }(3.14)
      item -> fn(v3) { "" }(3.14)
      inner -> "data"
    }
  } <> "abc"
}

fn f2(v4: Int, l: Bool, v5: Float) -> Bool {
case "ab" {
    _ | "constructor" -> False
    b | "constructor" <> b -> case [] {
      [7, x, ..] -> fn(v6, v7) { v7 }("res", False)
      [h, _, ..] -> l
      [7] -> !True
      _ -> l && True
    }
  }
}

pub fn main() {
  echo {
    {
      let delete = "abc"
      let delete = #(7, 5) |> class(100, f0(1, True, 0.25))
      1.0
    }
  } -. {
    2.0
  }
}
