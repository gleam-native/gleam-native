pub const golden_value: Float = 2.0
pub const limit_value: Int = 2
pub const seed_value: Int = 42

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(z: Bool) -> List(Int) {
[]
}

fn f1(v0: Bool, y: Bool, v1: List(Int)) -> String {
case #("b", [4, 1]), #(True, "a") {
    #("bc", [_]), #(True, "a") -> "b"
    #("ab" <> rest, [6, ..tail]) as whole, #(v2, "b") if rest != "res" -> "bc"
    #("b" <> rest as whole, []) as it, #(False as subject_, "res" <> tail) -> rest
    _, v3 -> "ab"
  }
}

pub fn main() {
  echo {
    fn(v4) { walk([7], 3) }(100)
  } + {
    case seed_value + seed_value {
      2 | 1 -> 10
      l -> limit_value
    }
  }
  echo 1.5
  echo walk([5, 42], limit_value - limit_value)
  echo case fn(v5) { [] }(42) {
    [0, b, ..] -> "constructor"
    [constructor, _, ..] -> "bc"
    v6 -> case {
        let v = golden_value
        v
      } {
      b -> "a" <> "a"
      1.5 | 100.0 -> "abc" <> "res"
    }
  }
}
