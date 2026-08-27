pub const seed_value: Int = 0
pub const limit_value: String = "abc"
pub const pi_value: Bool = False

pub type V0 {
  Record(value: String, inner: List(Int))
  Error(value: String, inner: Float)
  Number(value: List(Int), inner: Int)
}

pub type Object {
  Cv1
  Cv2
  Cv3(value: Float)
}

fn f0(prototype: Int) -> Bool {
True
}

fn f1(v4: Float, default: List(Int)) -> Int {
1
}

pub fn main() {
  echo {
    {
      fn(v5) { limit_value }("abc")
    } <> {
      fn(v6) { limit_value }(7)
    }
  } <> {
    case {
        let m = limit_value
        let m = 10.0
        [1]
      } {
      [6, ..rest] -> limit_value
      [a, ..rest] -> {
        let a = pi_value
        let length = rest
        limit_value
      }
      v7 -> limit_value
    }
  }
  echo case Cv2, Cv1 {
    Cv2 as whole, Cv2 -> {
      let seed_value = 1 - seed_value
      let n = limit_value
      False
    }
    Cv3(_), Cv1 -> {
      "x" == limit_value
    } && {
      seed_value > seed_value
    }
    v8, v9 -> pi_value
  }
  echo {
    1.5
  } |> f1([7])
}
