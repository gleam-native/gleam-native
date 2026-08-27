pub const seed_value: Bool = False
pub const golden_value: Bool = True

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, v0: Int, v1: String) -> Bool {
case "ab", {
      10.0
    } *. {
      0.0
    } {
    _, v2 -> {
      v0 >= v0
    } && False
    _, 0.5 -> {
      v0 == v0
    } && {
      fn(v3) { constructor }("bc")
    }
    "" <> rest as whole, 0.25 -> case 42 + 1 {
      0 | 7 -> fn(v4) { True }(100)
      9 | 3 -> {
        let constructor = []
        let value = [3, 5]
        True
      }
      v5 -> 2 < 3
    }
  }
}

pub fn main() {
  echo [5, 10]
  echo True
  echo True
  echo {
    {
      fn(v6) { 100.0 }("abc")
    } -. {
      {
        let arguments = 7
        let item = [100]
        100.0
      }
    }
  } <. {
    {
      3.14
    } +. {
      {
        0.0
      } -. {
        0.0
      }
    }
  }
}
