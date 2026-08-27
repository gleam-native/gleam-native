pub const golden_value: Int = 3
pub const euler_value: Float = 1.5
pub const tag_value: Float = 3.14

pub type V0 {
  Record(value: String, inner: Int)
}

pub type V1 {
  Cv2
  Cv3
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int) -> List(Int) {
[42, 1]
}

pub fn main() {
  echo case golden_value, {
      let euler_value = "bc"
      "ab"
    } {
    9, _ -> True
    item, "res" -> case item, {
        let this_ = tag_value
        let constructor = True
        []
      } {
      v4, [euler_value] as whole if v4 <= 1 && v4 % 2 == 0 -> False
      0, [4] as whole -> False
      v5, _ -> True
    }
    _, _ -> golden_value != {
      {
        let arguments = False
        let euler_value = [4, 3]
        golden_value
      }
    }
  }
  echo "b"
  echo 3
  echo fn(v6, v7) { "b" }(False, "abc")
}
