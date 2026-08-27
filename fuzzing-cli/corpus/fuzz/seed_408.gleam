pub const golden_value: Int = 10
pub const pi_value: Float = 100.0
pub const limit_value: Int = 100

pub type V0 {
  Some(value: String, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v1: Int) -> String {
"data" <> {
    case True, {
        let v = 3.14
        let rest = False
        "constructor"
      } {
      False, "constructor" <> _ -> "" <> "a"
      _, "data" -> "b"
      _, "a" <> rest -> rest
      _, v2 -> v2
    }
  }
}

pub fn main() {
  let golden_value = case walk([], golden_value) {
    b -> "b"
    b -> "ab" <> "res"
  }
  let v = case [5], Some("constructor", 100.0) {
    [x], Some("data", 2.0) -> True
    [constructor, ..rest], Some("res", 0.0 as whole) -> True
    [h, 9, ..], Some(_, 100.0) -> True
    _, _ -> True
  }
  echo case [1] {
    [] -> {
      pi_value *. {
        0.0
      }
    } +. {
      fn(v3, v4) { pi_value }(1.0, "")
    }
    [] -> fn(v5) { fn(v6, v7) { pi_value }("constructor", True) }("res")
    [1, 0, ..] as whole -> {
      pi_value -. pi_value
    } +. pi_value
    _ -> case {
        let golden_value = v
        1.5
      } {
      2.0 as whole -> {
        let self_ = pi_value
        self_
      }
      _ -> {
        let rest = "data"
        1.0
      }
    }
  }
  echo []
}
