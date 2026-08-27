pub const pi_value: Bool = False
pub const golden_value: Float = 0.0
pub const seed_value: Bool = False

pub type V0 {
  Cv1(value: List(Int))
  Cv2(Float)
}

pub type V3 {
  Cv4(value: Float)
  Cv5(value: Float, inner: List(Int))
  Cv6(String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(x: Float, length: Bool) -> Bool {
case Cv6("x") {
    a -> case fn(v7, v8) { #(42, 100.0) }(5, 3.14) {
      #(_, 0.5) -> {
        100.0
      } != x
      #(4, _) -> 7 <= 4
      a -> False
    }
    Cv5(_, [2, x, ..]) -> case {
        let length = x
        let x = 0.25
        False
      } {
      b -> b
      v9 -> {
        let s = []
        False
      }
      False -> length
    }
  }
}

fn yield(m: Int) -> Float {
{
    case {
        let acc = "b"
        let x = 7
        5
      } {
      default -> {
        let default = "a"
        1.5
      }
      b -> {
        1.5
      } -. {
        3.14
      }
      3 -> {
        let l = False
        let n = l
        0.1
      }
    }
  } /. {
    1.0
  }
}

pub fn main() {
  let golden_value = 5
  let pi_value = case 0, [2] {
    _, [pi_value, 7, ..] -> fn(v10) { golden_value }(False)
    4, [_, _, ..] -> {
      let seed_value = 0.1
      let class = [5]
      golden_value
    }
    _, v11 -> golden_value
  }
  echo {
    let golden_value = 1.0
    let y = seed_value
    case seed_value {
      item -> pi_value |> yield()
      v12 -> {
        0.5
      } /. {
        10.0
      }
    }
  }
}
