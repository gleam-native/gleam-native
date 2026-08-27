pub const seed_value: String = "abc"
pub const pi_value: String = "res"
pub const tag_value: Bool = False

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(self_: Float) -> List(Int) {
[3, 1]
}

pub fn main() {
  echo case 7 {
    _ -> 0.5
    4 -> case Cv2(3.14), [5] {
      Cv1([9], 6), [] as whole -> 3.14
      Cv1([1], 0), [h] -> 100.0
      v3, v4 -> {
        0.25
      } -. {
        0.25
      }
    }
  }
  echo case {
      let self_ = [5]
      Cv2(0.0)
    } {
    inner -> False && {
      {
        let inner = "constructor"
        let pi_value = tag_value
        tag_value
      }
    }
    Cv1(_, a) -> tag_value || True
  }
  echo {
    {
      {
        10.0
      } -. {
        1.5
      }
    } *. {
      {
        0.1
      } -. {
        2.0
      }
    }
  } -. {
    case [] {
      [b, 1, ..] -> {
        3.14
      } -. {
        1.5
      }
      [_, tag_value, ..] if tag_value <= 0 -> {
        1.5
      } /. {
        2.0
      }
      [2, ..rest] -> 0.0
      _ -> fn(v5, v6) { 0.1 }(4, 42)
    }
  }
  echo {
    1.5
  } -. {
    {
      {
        let this_ = tag_value
        100.0
      }
    } -. {
      {
        0.1
      } +. {
        0.1
      }
    }
  }
}
