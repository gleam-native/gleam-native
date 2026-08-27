pub const euler_value: Bool = True
pub const golden_value: String = "bc"
pub const seed_value: Float = 0.1

pub type V0 {
  Cv1(value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn export(acc: String, v2: Int) -> List(Int) {
[10, 2]
}

pub fn main() {
  let seed_value = case golden_value {
    a | "" <> a -> euler_value
    item | "a" <> item -> True
    "bc" <> _ -> {
      let y = []
      let new = golden_value
      euler_value
    }
  }
  let x = {
    let delete = {
      1.0
    } +. {
      1.5
    }
    let value = delete
    3 * 42
  }
  echo x
  echo [10]
  echo 1.0
  echo {
    let seed_value = case spin(x, 3) {
      inner -> x * 5
      _ -> {
        let prototype = euler_value
        let length = []
        100
      }
      4 -> spin(4, 42)
    }
    {
      {
        1.0
      } *. {
        0.0
      }
    } +. {
      {
        1.0
      } /. {
        2.0
      }
    }
  }
}
