pub const limit_value: String = "bc"
pub const seed_value: Int = 5
pub const euler_value: Float = 1.5

pub type V0 {
  Cv1(value: List(Int))
  Cv2(List(Int))
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn new(m: Int) -> Bool {
False
}

fn static(prototype: List(Int), x: V0, item: List(Int)) -> List(Int) {
case 4 {
    inner -> {
      let v = {
        let self_ = item
        "a"
      }
      [100]
    }
    8 as whole -> item
  }
}

pub fn main() {
  echo case fn(v3) { limit_value }(True), static([0, 1], Record, [100, 5]) {
    _, [a, ..rest] if a == 6 -> euler_value
    "res" <> rest, [euler_value] if euler_value == 7 && euler_value > 0 -> case <<"a":utf8>>, {
        let s = 0.5
        []
      } {
      <<7:8>>, [b, 4, ..] as whole if b <= 2 -> 0.5
      _, [6, ..rest] -> fn(v4) { 2.0 }(4)
      _, _ -> 0.25
    }
    arguments, [7] -> {
      {
        let arguments = False
        3.14
      }
    } /. {
      3.14
    }
    v5, _ -> {
      {
        let v = "ab"
        euler_value
      }
    } -. {
      3.14
    }
  }
  echo limit_value
  echo {
    let self_ = case <<"b":utf8>>, Record {
      <<"":utf8>> as whole, Cv1([a, ..rest]) -> True
      <<"ab":utf8>>, Cv1([]) -> new(seed_value)
      _, Record -> False
      v6, v7 -> {
        let self_ = True
        False
      }
    }
    let default = case new(4) {
      item -> euler_value
      False as whole -> euler_value +. {
        3.14
      }
    }
    5
  }
  echo []
}
