pub const pi_value: String = "res"
pub const golden_value: Float = 1.0
pub const seed_value: Int = 5

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(z: Int, v: Bool) -> String {
case fn(v2, v3) { 1 }(100, True) {
    _ | 1 -> {
      "b" <> "res"
    } <> "x"
    v4 -> ""
    8 -> case {
        10.0
      } *. {
        0.5
      } {
      v -> "ab" <> "b"
      b -> ""
    }
  }
}

fn extends(y: Bool) -> List(Int) {
case {
      0.1
    } -. {
      1.0
    } {
    constructor -> case Cv1([3], 7) {
      constructor -> {
        let y = 0
        [2]
      }
      Cv1([h, 0, ..], 1) -> {
        let item = [0, 3]
        [0, 0]
      }
    }
    10.0 as whole -> [3, 2]
    0.0 -> []
  }
}

fn f2(class: V0, arguments: Int) -> String {
"b"
}

pub fn main() {
  let z = "data"
  echo case {
      3.14
    } /. {
      2.0
    } {
    2.0 as whole -> "bc" != {
      z <> "a"
    }
    a -> seed_value == seed_value
  }
  echo case golden_value {
    2.0 -> [1, 42]
    1.0 -> [100, 1]
    0.0 -> fn(v5, v6) { {
      let length = 1.5
      let item = True
      []
    } }(0, 3.14)
    _ -> [0]
  }
  echo {
    {
      let l = golden_value
      2
    }
  } >= {
    case fn(v7, v8) { golden_value }("constructor", True) {
      b -> 2
      _ -> 0 - 100
      item -> fn(v9) { 7 }(42)
    }
  }
  echo case {
      let this_ = golden_value
      let z = True
      this_
    } {
    0.5 as whole -> False
    _ -> True
    item -> case 42, 3 {
      0, 4 -> True || False
      _, v10 -> True
    }
  }
}
