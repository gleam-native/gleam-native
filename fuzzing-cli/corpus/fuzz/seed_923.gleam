pub type V0 {
  Cv1(value: List(Int))
  Cv2(Int, value: String)
  Cv3(value: String)
}

fn f0(length: Bool) -> Float {
case Cv2(0, "constructor") {
    Cv2(8, "res" as whole) -> {
      0.1
    } /. {
      3.14
    }
    Cv3(_) -> 0.0
    _ -> 3.14
  }
}

pub fn main() {
  let this_ = case fn(v4) { True }("res"), "x" {
    False, "abc" as whole -> True
    _, self_ -> 7 < 5
  }
  let this_ = {
    4 * 100
  } + {
    100 + 3
  }
  echo True
  echo f0(True) +. {
    0.25
  }
  echo case this_ - 0 {
    _ -> this_
    7 -> 10
  }
}
