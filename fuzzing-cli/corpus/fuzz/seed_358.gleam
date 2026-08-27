pub const euler_value: Bool = False
pub const seed_value: Int = 100
pub const limit_value: Int = 5

pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: Float)
  Error(value: List(Int))
}

pub type V3 {
  Cv4(value: String, inner: Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v5: String) -> Float {
{
    1.5
  } *. {
    0.25
  }
}

fn f1(l: String) -> Bool {
True
}

pub fn main() {
  let pair = case "bc", 0.0 {
    _, euler_value -> []
    "ab", 100.0 -> {
      let l = limit_value
      let y = 0.5
      [0, 100]
    }
  }
  let seed_value = fn(v6) { {
    let value = euler_value
    let v6 = 10.0
    "a"
  } }(True)
  echo case Cv2(0.25), limit_value |> spin(limit_value) {
    Error([_, ..rest]), _ -> case limit_value {
      constructor -> seed_value
      9 -> seed_value <> seed_value
    }
    Error([]), _ -> seed_value <> {
      {
        let seed_value = seed_value
        let delete = 1.5
        seed_value
      }
    }
    _, v7 -> seed_value
  }
}
