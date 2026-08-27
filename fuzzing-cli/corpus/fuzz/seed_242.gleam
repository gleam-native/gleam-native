pub const pi_value: Bool = True

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Bool, m: #(List(Int), Float), this_: Int) -> List(Int) {
[5, 5]
}

pub fn main() {
  let value = case 42 {
    item -> "a" <> "x"
    a -> {
      let l = ""
      let default = [100, 7]
      "b"
    }
  }
  echo {
    case [0], 4 % 7 {
      [4, ..rest], _ -> value
      [b], 5 -> value <> value
      _, _ -> value
    }
  } <> {
    {
      value <> value
    } <> value
  }
}
