pub const seed_value: Int = 2
pub const euler_value: Int = 42
pub const pi_value: String = "b"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Bool, v: Int, new: Bool) -> Float {
{
    {
      0.0
    } +. {
      {
        let l = v
        1.5
      }
    }
  } +. {
    case "abc" {
      arguments -> 100.0
      "data" as whole -> fn(v0) { 100.0 }(True)
    }
  }
}

fn f1(v1: Int, v2: Bool) -> Float {
2.0
}

pub fn main() {
  let m = False
  let value = {
    {
      let l = 1.5
      m
    }
  } || m
  echo case {
      let rest = pi_value
      0.25
    } {
    inner -> case seed_value + seed_value {
      constructor -> []
      4 | 3 -> [0, 42]
      7 -> fn(v3) { [10, 3] }(False)
    }
    b -> fn(v4, v5) { [] }(1, "ab")
    0.0 -> [7]
  }
}
