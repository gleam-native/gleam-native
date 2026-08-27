pub const golden_value: Int = 7
pub const tag_value: Bool = True

pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(this_: List(Int), default: Bool, y: String) -> List(Int) {
case "res" {
    "res" <> rest -> {
      let this_ = y <> rest
      [4]
    }
    _ -> this_
    "b" -> [7, 0]
  }
}

fn f1(v: Int, v2: Bool) -> Bool {
v >= v
}

pub fn main() {
  echo case {
      let tag_value = 0.5
      let this_ = True
      [0]
    } {
    [b, ..rest] if b % 2 == 0 -> fn(v3, v4) { {
      let golden_value = False
      rest
    } }(False, "bc")
    [] -> []
    [3, 6, ..] -> {
      let value = {
        10.0
      } == {
        10.0
      }
      let rest = "res"
      f0([1], value, rest)
    }
    _ -> case [] {
      [9] -> fn(v5, v6) { [42, 42] }("", True)
      [] -> f0([], True, "x")
      _ -> [5, 5]
    }
  }
  echo 2.0
}
