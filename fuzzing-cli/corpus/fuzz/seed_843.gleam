pub const limit_value: String = "b"
pub const golden_value: Float = 1.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(Int, value: String)
  Cv3(Float, String)
}

fn export(v4: Bool) -> String {
case 0, v4 {
    4, False -> "a"
    3, True -> ""
    _, _ -> "res"
  }
}

pub fn main() {
  let golden_value = export(False)
  let golden_value = case [], 10 {
    [2], 8 as whole -> ""
    [1, 3, ..] as whole, _ -> golden_value <> limit_value
    [x] as whole, _ -> limit_value <> "abc"
    _, v5 -> limit_value <> golden_value
  }
  echo case {
      let pair = False
      limit_value
    } {
    _ -> False
    _ -> {
      let s = limit_value <> limit_value
      let m = {
        let x = "bc"
        let constructor = 0.1
        True
      }
      100 >= 100
    }
  }
}
