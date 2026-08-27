pub const limit_value: Float = 1.0
pub const golden_value: Float = 10.0
pub const euler_value: Int = 0

pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: Float)
  Cv3(Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(v4: List(Int), v5: Bool, default: List(Int)) -> List(Int) {
[3, 7]
}

pub fn main() {
  let l = case Cv2(3.14) {
    Cv1([b, ..rest]) -> {
      3.14
    } +. {
      0.0
    }
    constructor -> limit_value
  }
  echo 10
  echo [2]
  echo [3, 3]
  echo {
    case golden_value +. {
        2.0
      } {
      b -> False && True
      0.1 -> False
      a -> False
    }
  } && {
    fn(v6) { False }("b")
  }
}
