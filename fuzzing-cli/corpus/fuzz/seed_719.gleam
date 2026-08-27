pub const golden_value: Bool = True
pub const seed_value: Float = 3.14

pub type V0 {
  Number(value: String, inner: List(Int))
  Ok(value: Bool, inner: String)
}

pub type V1 {
  Error(Int, Float)
  Cv2(Bool, value: Bool)
  Cv3
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(class: V1, v4: String) -> List(Int) {
[4]
}

pub fn main() {
  let pair = {
    let item = 4 * 7
    golden_value
  }
  echo {
    {
      let seed_value = golden_value
      let length = {
        2.0
      } -. {
        0.5
      }
      0.0
    }
  } +. {
    100.0
  }
}
