pub const golden_value: String = "ab"
pub const pi_value: Float = 2.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: List(Int)) -> Int {
7
}

fn f1(default: Bool, this_: V0) -> Int {
{
    spin(100, 1) - 7
  } % 6
}

pub fn main() {
  let golden_value = case <<"x":utf8>> {
    <<_:utf8, _:big-unsigned-8>> -> {
      let golden_value = 0
      True
    }
    <<10:4, 100:8>> -> True
    _ -> {
      let golden_value = 42
      False
    }
  }
  let constructor = fn(v3) { 4 + 100 }("")
  echo 0 - {
    case {
        let x = "x"
        let delete = golden_value
        3.14
      } {
      a -> 7
      0.1 -> constructor
    }
  }
  echo constructor
  echo fn(v4) { [] }(1.0)
  echo {
    case [10, 1], Cv1([], 1) {
      [a, ..rest], Cv1([3], _) -> pi_value
      [2, b, ..], Cv2 if b % 2 == 0 -> fn(v5) { v5 }(0.5)
      [5], v6 -> 3.14
      v7, _ -> pi_value /. {
        1.0
      }
    }
  } >=. {
    case fn(v8) { "ab" }(10) {
      item -> {
        let pi_value = constructor
        1.0
      }
      "res" -> pi_value -. pi_value
    }
  }
}
