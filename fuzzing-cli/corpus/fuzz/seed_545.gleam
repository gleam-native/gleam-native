pub const golden_value: String = "a"
pub const tag_value: Int = 3
pub const pi_value: Int = 2

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, value: #(Bool, Int), y: Float) -> Int {
case <<"ab":utf8, "b":utf8>> {
    <<4:16, s:8>> if s <= 4 -> s + 3
    <<pair:16, _:utf8>> -> 1
    _ -> constructor * constructor
  }
}

fn f1(default: Bool, v0: Bool) -> String {
case f0(0, #(False, 5), 10.0), 4 * 100 {
    _, 2 -> case "constructor", default {
      "" <> rest, acc if acc -> "data" <> rest
      "b", True -> "constructor" <> "bc"
      "a" <> rest, False -> "b"
      _, v1 -> "constructor"
    }
    _, 1 -> "" <> "constructor"
    _, _ -> case [10, 5] {
      [_, ..rest] -> "constructor" <> "x"
      [] -> fn(v2, v3) { "b" }(100, False)
      [] -> ""
      _ -> "x"
    }
  }
}

fn f2(pair: Bool, v4: Int, this_: String) -> Int {
{
    let x = 0.0
    let self_ = case <<"x":utf8>> {
      <<"a":utf8>> -> []
      <<_:8>> -> fn(v5, v6) { [5, 4] }(False, 100)
      _ -> []
    }
    5
  }
}

pub fn main() {
  let n = []
  echo [10, 42]
  echo case <<7:16, "b":utf8>> {
    <<_:4>> -> case <<"constructor":utf8, "data":utf8, "constructor":utf8>> {
      <<"":utf8>> -> True |> f1(fn(v7, v8) { True }(False, False))
      _ -> "constructor"
    }
    <<1:8>> -> golden_value
    _ -> "data"
  }
}
