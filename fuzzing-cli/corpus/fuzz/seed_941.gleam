pub const limit_value: Bool = True
pub const euler_value: Int = 7

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(value: Int)
}

pub type Object {
  Some
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(self_: Int, v3: Int, rest: Bool) -> Bool {
3 > 5
}

fn extends(v4: #(String, Float)) -> Int {
walk({
    let v4 = False
    let length = {
      let s = v4
      let this_ = v4
      1
    }
    [100]
  }, 1)
}

fn f2(y: #(Float, Float)) -> Int {
1
}

pub fn main() {
  echo [42]
  echo euler_value
}
