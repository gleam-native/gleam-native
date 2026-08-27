pub const pi_value: String = "b"

pub type V0 {
  Record(value: String, inner: Float)
  Cv1(value: Float, inner: Float)
  Cv2(value: Float)
}

pub type V3 {
  Error(List(Int))
  None(String, value: List(Int))
}

fn yield(v4: Bool) -> List(Int) {
fn(v5, v6) { [5, 100] }(True, False)
}

pub fn main() {
  let value = case fn(v7) { pi_value }(1.0) {
    "b" | "bc" -> fn(v8) { [] }("abc")
    _ -> [2]
  }
  echo {
    {
      1.5
    } *. {
      {
        1.5
      } +. {
        1.0
      }
    }
  } /. {
    1.0
  }
  echo case {
      let pair = "b"
      let pi_value = "a"
      10
    } {
    _ | 8 -> fn(v9, v10) { 0 % 5 }(2, 3)
    _ -> 100
  }
  echo True
  echo case fn(v11, v12) { v11 }(7, "constructor") {
    b -> fn(v13, v14) { value }(0.25, True)
    a -> value
  }
}
