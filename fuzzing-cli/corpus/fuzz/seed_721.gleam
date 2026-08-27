pub const tag_value: Bool = True
pub const limit_value: Bool = True

pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(new: Float, v2: Int) -> Bool {
{
    case "x", {
        let v2 = v2
        100.0
      } {
      "constructor", 2.0 -> 10.0
      "bc", 2.0 -> {
        0.1
      } /. {
        0.5
      }
      v3, _ -> 10.0
    }
  } != {
    fn(v4, v5) { new }(False, "ab")
  }
}

fn f1(v6: Bool, default: List(Int)) -> String {
case Cv1 {
    Cv1 -> "abc"
    _ | Cv1 -> case Cv1, default {
      Cv1 as whole, [v6, ..rest] -> "res"
      Cv1, [2, ..rest] -> "a" <> ""
      v7, _ -> "x"
    }
    _ -> case #(True, [10]) {
      a -> "res"
      #(_, [9, 2, ..]) | #(False, [_, _, ..]) -> "a"
      #(True, [0]) -> "ab"
    }
  }
}

pub fn main() {
  let v = case #(True, [0]) {
    #(_, [9, tag_value, ..]) if tag_value > 3 -> True || limit_value
    constructor -> {
      0.25
    } |> export([42] |> walk(7))
    item -> fn(v8, v9) { tag_value }("", 2.0)
  }
  echo []
}
