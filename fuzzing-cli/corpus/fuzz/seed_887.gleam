pub const tag_value: String = "abc"

pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: #(Bool, List(Int)), length: Float) -> Int {
2
}

fn arguments(v3: Float) -> Int {
42
}

pub fn main() {
  let this_ = False
  let m = []
  echo case "a" <> "abc", "abc" {
    "constructor", acc if acc != "x" -> "constructor"
    "ab" as whole, _ -> whole
    _, v4 -> case tag_value <> v4 {
      inner -> fn(v5) { "" }(0.0)
      "res" <> rest as whole if rest == "constructor" -> "b"
      "abc" -> "res"
    }
  }
  echo fn(v6, v7) { case <<"abc":utf8, "abc":utf8, "x":utf8>> {
    <<"constructor":utf8, _:bytes>> -> v7 *. {
      1.5
    }
    _ -> v7 -. {
      0.5
    }
  } }(5, 2.0)
  echo case [2, 4] |> walk(walk(m, 4)) {
    _ -> 2.0
    2 -> {
      0.25
    } /. {
      1.0
    }
  }
}
