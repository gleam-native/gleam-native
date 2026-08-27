pub const tag_value: Bool = True
pub const pi_value: Int = 0
pub const euler_value: Bool = True

pub type Symbol {
  Cv0(value: String, inner: String)
}

pub type V1 {
  Cv2
  Cv3(List(Int), value: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn default(value: List(Int)) -> List(Int) {
value
}

fn yield(self_: Int) -> Int {
self_ - {
    self_ % 6
  }
}

pub fn main() {
  echo case fn(v4, v5) { [] }(0.1, 1.5) {
    [2, ..rest] -> 3.14
    [euler_value, ..rest] -> case "ab" <> "", 42 {
      "bc", 8 -> 0.25
      _, m -> {
        1.5
      } +. {
        1.5
      }
      _, 3 -> {
        1.0
      } -. {
        3.14
      }
    }
    _ -> 3.14
  }
  echo default({
    let n = 0 - 3
    let pi_value = 0.5
    [100, 1]
  })
  echo "a"
}
