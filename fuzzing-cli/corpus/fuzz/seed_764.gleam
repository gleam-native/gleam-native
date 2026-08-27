pub const pi_value: Float = 0.25
pub const euler_value: Int = 0

pub type V0 {
  Cv1(value: List(Int))
  Cv2
}

pub type Symbol {
  None(value: List(Int))
  Cv3(String, value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(class: Int) -> String {
{
    let pair = 100
    let pair = 2
    "res"
  }
}

pub fn main() {
  let x = case True {
    True -> [3, 0]
    False -> [100]
  }
  echo case euler_value, euler_value {
    3, _ -> case euler_value - euler_value {
      1 | 8 -> pi_value +. pi_value
      _ | 3 -> pi_value +. pi_value
      2 | 0 -> 100.0
    }
    6, x -> case {
        let x = euler_value
        let pi_value = [3]
        2.0
      }, 42 {
      x, _ -> pi_value /. {
        2.0
      }
      v4, 6 as whole -> v4 -. pi_value
    }
    _, _ -> pi_value
  }
}
