pub const euler_value: Int = 100
pub const pi_value: Int = 100
pub const tag_value: String = "a"

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, rest: Bool, v0: List(Int)) -> Bool {
{
    {
      {
        0.1
      } -. {
        10.0
      }
    } +. {
      1.0
    }
  } <. {
    3.14
  }
}

fn f1(value: #(Bool, Float), v1: String) -> List(Int) {
[]
}

fn constructor(v2: String, m: String) -> Bool {
True
}

pub fn main() {
  let pi_value = 10.0
  let pi_value = case <<"x":utf8>> {
    <<_:utf8>> as whole -> [5, 5]
    _ -> [2, 4]
  }
  echo case {
      3.14
    } <. {
      0.0
    } {
    constructor -> case spin(5, euler_value) {
      8 | 1 -> 10.0
      4 | 5 -> {
        0.1
      } *. {
        3.14
      }
      v3 -> {
        let constructor = pi_value
        2.0
      }
    }
    b -> 100.0
    _ | False -> {
      1.0
    } -. {
      100.0
    }
  }
  echo 1.5
}
