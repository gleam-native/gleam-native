pub const euler_value: String = "a"
pub const limit_value: String = "ab"
pub const seed_value: Float = 10.0

fn f0(m: String, rest: List(Int)) -> String {
case 3 {
    _ -> m
    3 | 7 -> fn(v0) { m }("a")
  }
}

fn f1(item: String, m: String, v1: String) -> Bool {
True
}

pub fn main() {
  let euler_value = limit_value
  let n = case euler_value {
    constructor | "b" <> constructor -> f1("x", "x", euler_value)
    "x" <> a -> f1(limit_value, limit_value, euler_value)
  }
  echo case [0] {
    [] -> case seed_value {
      euler_value -> {
        let euler_value = [2, 3]
        let delete = False
        euler_value
      }
      0.0 -> [100]
      _ -> [2]
    }
    [2] as whole -> case 3 % 2, 42 {
      7, _ -> whole
      2 as whole, 8 -> fn(v2, v3) { [42, 5] }("b", True)
      _, v4 -> {
        let item = v4
        let self_ = seed_value
        whole
      }
    }
    [] -> []
    _ -> [2, 2]
  }
  echo fn(v5) { [7, 1] }(0.0)
  echo seed_value
  echo n
}
