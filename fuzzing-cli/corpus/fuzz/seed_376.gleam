pub const seed_value: Int = 7

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, s: Bool, v0: Bool) -> List(Int) {
[]
}

fn constructor(pair: Bool) -> Float {
{
    let pair = !pair
    case {
        0.1
      } -. {
        3.14
      } {
      delete -> fn(v1) { delete }(0)
      0.0 | 0.5 -> {
        2.0
      } -. {
        1.5
      }
    }
  }
}

pub fn main() {
  let pair = spin(seed_value, seed_value) % 7
  let prototype = 4 + {
    1 % 5
  }
  echo False && {
    {
      10.0
    } <. {
      True |> constructor()
    }
  }
}
