pub const limit_value: Int = 2
pub const euler_value: Float = 0.25
pub const seed_value: String = "abc"

pub type Symbol {
  Cv0(value: String, inner: List(Int))
  Some
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(l: Float, pair: Bool, s: Float) -> Bool {
case [7, 10] {
    [_] -> case [] {
      [_] -> s <. l
      [9] -> {
        let item = 42
        let acc = [10]
        True
      }
      v1 -> 3 != 100
    }
    [4, _, ..] -> pair
    _ -> pair && {
      {
        let pair = "x"
        let class = []
        True
      }
    }
  }
}

fn f1(v: String) -> Float {
0.1
}

fn f2(v2: #(List(Int), String), v3: String, default: Float) -> Float {
default +. default
}

pub fn main() {
  echo {
    {
      {
        let class = [3]
        let seed_value = False
        euler_value
      }
    } /. {
      2.0
    }
  } -. {
    case fn(v4, v5) { [1] }(100, 100), <<"constructor":utf8, "x":utf8>> {
      [a, 0, ..], <<_:utf8>> as whole -> euler_value /. {
        2.0
      }
      [], _ -> 2.0
      v6, _ -> euler_value
    }
  }
}
