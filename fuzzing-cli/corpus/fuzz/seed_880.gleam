pub const euler_value: String = "bc"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(class: List(Int)) -> Bool {
False
}

fn f1(item: Float, rest: Int, x: Float) -> Int {
case "data", 3 + rest {
    "data", item -> case <<"constructor":utf8, "bc":utf8>> {
      <<5:8, default:8>> -> [] |> walk(42 + 2)
      <<_:utf8>> -> 10
      _ -> 1 + item
    }
    _, _ -> rest
  }
}

pub fn main() {
  let euler_value = case f1(0.5, 7, 0.5) {
    _ -> True
    3 -> fn(v0, v1) { True }(4, "data")
    _ | 1 -> False
  }
  let new = "a"
  echo 0.0
}
