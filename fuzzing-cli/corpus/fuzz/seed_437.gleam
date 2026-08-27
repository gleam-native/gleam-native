pub const pi_value: Bool = True
pub const tag_value: String = "bc"

pub type V0 {
  Cv1
  Cv2
  Cv3
}

pub type V4 {
  Cv5
  Cv6(value: String)
  Cv7(value: Bool, inner: Float)
}

pub type V8 {
  Cv9
  Number(value: List(Int), inner: String)
  Cv10(Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(length: String, v11: V8, rest: V4) -> Int {
walk([4, 1], case <<"a":utf8>> {
    <<5:4>> -> 100
    _ -> {
      let m = 3
      let rest = [0, 3]
      m
    }
  })
}

pub fn main() {
  echo case 0 {
    5 -> 1
    b -> 5
    b -> 3
  }
  echo case {
      let this_ = []
      this_
    } {
    [] -> fn(v12, v13) { True }("x", 2.0)
    [pi_value] if pi_value > 2 && pi_value > 6 -> True
    [] -> fn(v14) { pi_value }(4)
    _ -> case walk([10], 0), 0 - 100 {
      _, pair -> pi_value
      6, 4 -> False
    }
  }
}
