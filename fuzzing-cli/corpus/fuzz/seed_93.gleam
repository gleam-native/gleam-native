pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Error(value: Bool, inner: String)
  Cv2
}

pub type Object {
  Cv3(value: Bool, inner: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(this_: Int, v4: Bool) -> String {
"ab"
}

pub fn main() {
  let arguments = case walk([3, 7], 0) {
    8 -> []
    _ -> []
  }
  let s = [2]
  echo True
}
