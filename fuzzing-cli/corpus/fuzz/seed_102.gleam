pub const golden_value: Float = 0.25
pub const euler_value: Bool = True

pub type V0 {
  None(value: String, inner: Bool)
}

pub type V1 {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(this_: Int, s: V0) -> List(Int) {
[5]
}

pub fn main() {
  echo case fn(v2) { "constructor" }(True), "res" {
    _, "bc" -> [2]
    "a" <> rest, "data" <> tail if tail != "bc" && tail != "ab" -> case {
        let golden_value = tail
        None("constructor", False)
      } {
      s -> []
      None(_, _) -> [7, 5]
    }
    "constructor", "x" -> case 0 - 1 {
      5 | 0 -> [0]
      item -> [3]
    }
    _, v3 -> [4]
  }
  echo {
    let class = case fn(v4) { Record }(True), False {
      Record, l if !l || l -> []
      Record, False -> [42, 4]
      Record, True -> fn(v5) { [3, 1] }(100)
      _, _ -> [42, 3]
    }
    case {
        let golden_value = euler_value
        None("ab", True)
      } {
      v6 -> euler_value
      _ | None(_, _) -> fn(v7) { True }(100.0)
      item -> golden_value >. {
        1.0
      }
    }
  }
  echo euler_value
  echo [4, 10]
}
