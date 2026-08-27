pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(prototype: V0, v2: V0) -> List(Int) {
[0]
}

fn f1(y: V0) -> String {
case 0.5, <<"constructor":utf8, "res":utf8>> {
    _, <<m:big-unsigned-8, _:little-unsigned-16>> as whole if m > 8 -> case 42 + 1 {
      _ | 6 -> fn(v3, v4) { "res" }(0, "b")
      1 as whole -> {
        let arguments = [42]
        "x"
      }
      a -> "x" <> "bc"
    }
    v5, <<"x":utf8, 3:8>> -> {
      "data" <> "a"
    } <> "abc"
    10.0, _ -> {
      {
        let self_ = False
        "abc"
      }
    } <> "data"
    _, _ -> fn(v6, v7) { v7 <> "a" }(2, "a")
  }
}

pub fn main() {
  let y = walk([100, 1], 5)
  echo case "bc" <> "data", "" <> "constructor" {
    new, "x" -> y
    "ab" as whole, _ -> y
    _, _ -> {
      y + y
    } * 42
  }
  echo fn(v8, v9) { fn(v10) { v10 -. v10 }(100.0) }("bc", "data")
  echo case "abc" <> "constructor", 42 {
    "abc", 8 -> 100
    "a", 1 -> fn(v11) { [] |> walk(walk([], y)) }("abc")
    v12, v13 -> 5
  }
  echo "x"
}
