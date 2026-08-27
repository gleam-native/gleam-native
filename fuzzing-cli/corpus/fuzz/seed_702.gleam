pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: Float, v3: #(Int, String)) -> Int {
[] |> walk(5)
}

fn f1(v4: Int, z: List(Int), l: String) -> Int {
4
}

pub fn main() {
  let arguments = case "" <> "b", fn(v5, v6) { v6 }(2.0, "data") {
    "ab" <> rest, "ab" if rest != "x" -> fn(v7, v8) { False }(4, 0.0)
    _, "a" -> False
    v9, _ -> fn(v10) { True }(1.0)
  }
  let length = "x"
  echo case {
      let arguments = 1.0
      "a"
    } {
    "b" as whole if whole != "x" && whole != "bc" -> {
      let whole = {
        1.5
      } -. {
        100.0
      }
      let s = whole
      "ab"
    }
    constructor -> case "abc" {
      "" <> _ -> ""
      _ -> "abc"
    }
  }
  echo []
}
