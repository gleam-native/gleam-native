pub const golden_value: Int = 4
pub const pi_value: Int = 0
pub const euler_value: String = ""

pub type Object {
  Cv0(value: String, inner: Int)
}

pub type Map {
  Some(List(Int), List(Int))
  None(value: Int)
  Record
}

fn delete(v1: List(Int), m: Bool, v2: Float) -> Bool {
case 3, fn(v3) { Cv0("constructor", 42) }(10) {
    _, Cv0("x", 3 as whole) if whole <= 2 && whole > 3 -> case v1 {
      [3, 8, ..] -> 2 != whole
      [4, ..rest] as whole -> {
        let v1 = 1.0
        let m = whole
        True
      }
      _ -> fn(v4, v5) { v5 }(4, True)
    }
    s, Cv0("constructor" <> rest as whole, 4) -> case 4 % 3 {
      9 as whole if whole == 2 && whole <= 9 -> {
        0.5
      } >. v2
      _ -> {
        let x = True
        True
      }
      4 -> False
    }
    _, _ -> True || {
      !False
    }
  }
}

pub fn main() {
  echo case 2, {
      0.5
    } +. {
      0.25
    } {
    l, 0.5 -> {
      l + pi_value
    } + 42
    _, _ -> {
      fn(v6) { golden_value }(1)
    } - pi_value
  }
  echo case #(0.25, 10) {
    #(100.0, 6 as whole) -> case whole % 1 {
      item -> whole
      constructor -> golden_value * constructor
    }
    inner -> golden_value
    #(1.5, v7) -> pi_value
  }
}
