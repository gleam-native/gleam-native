pub const seed_value: String = "bc"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(default: List(Int)) -> Int {
{
    100 - {
      [] |> walk(default |> walk(10))
    }
  } % 2
}

fn static(v0: #(Float, Float), this_: Int, v1: Bool) -> Int {
walk([0, 100], fn(v2) { 2 }(True)) + {
    {
      0 + 1
    } % 3
  }
}

pub fn main() {
  let s = case fn(v3) { [0] }(0.5) {
    [2] -> {
      2.0
    } -. {
      2.0
    }
    [] as whole -> fn(v4, v5) { 0.0 }(True, 4)
    _ -> 2.0
  }
  let rest = 10.0
  echo case <<"b":utf8>> {
    <<2:16>> -> "data"
    <<_:utf8>> -> {
      "data" <> "ab"
    } <> {
      {
        let prototype = [100, 1]
        let class = seed_value
        "constructor"
      }
    }
    _ -> seed_value
  }
  echo rest
  echo case rest *. rest {
    constructor -> case rest /. {
        2.0
      }, [] {
      _, [4, 3, ..] -> 1.0
      10.0 as whole, [7] -> 0.0
      v6, v7 -> 0.0
    }
    inner -> 3.14
    10.0 -> rest
  }
  echo 0.25
}
