pub type Number {
  Record
}

pub type V0 {
  None(value: Int)
  Cv1(String)
}

pub type Symbol {
  Cv2(Bool)
  Cv3(String, value: Int)
}

fn f0(x: Int) -> List(Int) {
[5, 10]
}

pub fn main() {
  let self_ = 2
  let self_ = case "x" <> "res" {
    "constructor" -> 3
    b -> 10 - self_
  }
  echo self_
  echo self_ * self_
  echo 0 - {
    self_ - self_
  }
}
