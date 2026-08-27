pub const limit_value: Bool = True
pub const euler_value: Float = 3.14

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: Bool, item: Int, arguments: Bool) -> Bool {
!v2
}

pub fn main() {
  echo limit_value
  echo case #([0], [10]), fn(v3) { 2 }(0.25) {
    #([1, ..rest], [7]), _ -> fn(v4) { {
      0.5
    } *. {
      3.14
    } }(False)
    #([a, ..rest], [2]), 2 -> euler_value
    #([0, x, ..], [b, ..rest]), _ -> 0.25
    _, _ -> 0.25
  }
}
