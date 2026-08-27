pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(v3: Int, s: List(Int)) -> List(Int) {
case {
      2.0
    } -. {
      2.0
    } {
    100.0 | 1.5 -> s
    constructor -> fn(v4) { [10, 2] }(2)
  }
}

pub fn main() {
  let z = {
    100.0
  } <=. {
    1.5
  }
  let z = {
    0.0
  } >=. {
    1.5
  }
  echo []
}
