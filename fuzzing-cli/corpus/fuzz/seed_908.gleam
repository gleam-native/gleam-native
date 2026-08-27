pub const pi_value: Float = 3.14
pub const euler_value: Int = 42
pub const limit_value: String = "ab"

pub type V0 {
  Cv1(value: List(Int))
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(n: Int, v3: Int, item: Float) -> List(Int) {
[0, 100]
}

fn yield(delete: Bool, value: List(Int)) -> Float {
{
    let rest = False
    {
      let acc = [3]
      {
        1.0
      } /. {
        2.0
      }
    }
  }
}

pub fn main() {
  echo case limit_value <> "data", Cv2 {
    _, _ -> True
    "" <> rest, Cv2 as whole -> case rest {
      item | "" <> item -> False
      "" <> rest -> True
      v4 -> False
    }
    _, Cv1([euler_value] as whole) -> {
      fn(v5) { limit_value }(0.0)
    } == {
      {
        let arguments = 7
        let default = 7
        "a"
      }
    }
  }
  echo f0(euler_value, spin(spin(euler_value, 5), spin(euler_value, 0)), pi_value)
  echo [0, 3]
}
